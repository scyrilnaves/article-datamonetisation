package com.renault.leat.radar_api;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.io.StringReader;
import java.net.URISyntaxException;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.KeyFactory;
import java.security.KeyPair;
import java.security.NoSuchAlgorithmException;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.Security;
import java.security.Signature;
import java.security.SignatureException;
import java.security.interfaces.RSAPublicKey;
import java.security.spec.InvalidKeySpecException;
import java.security.spec.PKCS8EncodedKeySpec;
import java.security.spec.X509EncodedKeySpec;
import java.util.Base64;
import java.util.UUID;
import org.apache.commons.io.FileUtils;

import javax.crypto.Cipher;

import com.google.common.hash.Hashing;

import org.bouncycastle.util.io.pem.PemObject;
import org.bouncycastle.util.io.pem.PemReader;

import net.i2p.crypto.eddsa.EdDSASecurityProvider;

public final class CryptoUtil {
    // https://howtodoinjava.com/java15/java-eddsa-example/
    // https://snipplr.com/view/18368/saveload--private-and-public-key-tofrom-a-file

    private static final String pubkeyfile = "nodekeys/public";

    private static final String privatekeyfile = "nodekeys/private";

    public static File getFileFromResource(String fileName) throws URISyntaxException {

        ClassLoader classLoader = CryptoUtil.class.getClassLoader();
        URL resource = classLoader.getResource(fileName);
        if (resource == null) {
            throw new IllegalArgumentException("file not found! " + fileName);
        } else {

            // failed if files have whitespaces or special characters
            // return new File(resource.getFile());

            return new File(resource.toURI());
        }

    }

    private static File getFileFromResourceAsStream(String fileName) {

        // The class loader that loaded the class
        ClassLoader classLoader = CryptoUtil.class.getClassLoader();
        InputStream inputStream = classLoader.getResourceAsStream(fileName);

        File file = new File("/tmp/file.txt");

        try {
            FileUtils.copyInputStreamToFile(inputStream, file);
        } catch (Exception exception) {
            System.out.print(exception.toString());
        }
        // the stream holding the file content
        if (inputStream == null) {
            throw new IllegalArgumentException("file not found! " + fileName);
        } else {
            return file;
        }

    }

    public static KeyPair getKeyPair(int index)
            throws IOException, NoSuchAlgorithmException, InvalidKeySpecException, URISyntaxException {
        // Read Public Key of Node
        File pubfile = getFileFromResourceAsStream(pubkeyfile + "/node" + index);
        FileInputStream publicfis = new FileInputStream(pubfile);
        byte[] encodedPublicKey = new byte[(int) pubfile.length()];
        publicfis.read(encodedPublicKey);
        publicfis.close();
        // Read Private Key of Node
        File privatefile = getFileFromResourceAsStream(privatekeyfile + "/node" + index);
        FileInputStream privatefis = new FileInputStream(privatefile);
        byte[] encodedPrivateKey = new byte[(int) privatefile.length()];
        privatefis.read(encodedPrivateKey);
        privatefis.close();
        // Generate Key Pair
        KeyFactory keyFactory = KeyFactory.getInstance("RSA");
        X509EncodedKeySpec publicKeySpec = new X509EncodedKeySpec(
                encodedPublicKey);
        PublicKey publicKey = keyFactory.generatePublic(publicKeySpec);

        PKCS8EncodedKeySpec privateKeySpec = new PKCS8EncodedKeySpec(
                encodedPrivateKey);
        PrivateKey privateKey = keyFactory.generatePrivate(privateKeySpec);

        return new KeyPair(publicKey, privateKey);

    }

    public static PublicKey getPublicKey(int index)
            throws IOException, NoSuchAlgorithmException, InvalidKeySpecException, URISyntaxException {
        File pubfile = getFileFromResourceAsStream(pubkeyfile + "/node" + index);
        FileInputStream publicfis = new FileInputStream(pubfile);
        byte[] encodedPublicKey = new byte[(int) pubfile.length()];
        publicfis.read(encodedPublicKey);
        publicfis.close();
        KeyFactory keyFactory = KeyFactory.getInstance("RSA");
        X509EncodedKeySpec publicKeySpec = new X509EncodedKeySpec(
                encodedPublicKey);
        PublicKey publicKey = keyFactory.generatePublic(publicKeySpec);
        return publicKey;

    }

    public static String getPublicKeyString(int index)
            throws IOException, NoSuchAlgorithmException, InvalidKeySpecException, URISyntaxException {
        File pubfile = getFileFromResourceAsStream(pubkeyfile + "/node" + index);
        FileInputStream publicfis = new FileInputStream(pubfile);
        byte[] encodedPublicKey = new byte[(int) pubfile.length()];
        publicfis.read(encodedPublicKey);
        publicfis.close();
        KeyFactory keyFactory = KeyFactory.getInstance("RSA");
        X509EncodedKeySpec publicKeySpec = new X509EncodedKeySpec(
                encodedPublicKey);
        PublicKey publicKey = keyFactory.generatePublic(publicKeySpec);
        String pubkeystring = Base64.getEncoder().encodeToString(publicKey.getEncoded());
        return pubkeystring;

    }

    public static PrivateKey getPrivateKey(int index)
            throws IOException, NoSuchAlgorithmException, InvalidKeySpecException, URISyntaxException {
        File privatefile = getFileFromResourceAsStream(privatekeyfile + "/node" + index);
        FileInputStream privatefis = new FileInputStream(privatefile);
        byte[] encodedPrivateKey = new byte[(int) privatefile.length()];
        privatefis.read(encodedPrivateKey);
        privatefis.close();
        KeyFactory keyFactory = KeyFactory.getInstance("RSA");
        PKCS8EncodedKeySpec privateKeySpec = new PKCS8EncodedKeySpec(
                encodedPrivateKey);
        PrivateKey privateKey = keyFactory.generatePrivate(privateKeySpec);
        return privateKey;

    }

    public static String getEncryptedData(String data, int index) {
        byte[] encryptedMessageBytes = null;
        try {
            Cipher encryptCipher = Cipher.getInstance("RSA");
            encryptCipher.init(Cipher.ENCRYPT_MODE, getPublicKey(index));
            byte[] secretMessageBytes = data.getBytes(StandardCharsets.UTF_8);
            encryptedMessageBytes = encryptCipher.doFinal(secretMessageBytes);
        } catch (Exception e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }
        String encodedMessage = Base64.getEncoder().encodeToString(encryptedMessageBytes);
        return encodedMessage;
    }

    public static String getDecryptedData(String encryptedData, int index) {
        byte[] decryptedMessageBytes = null;
        try {
            Cipher decryptCipher = Cipher.getInstance("RSA");
            decryptCipher.init(Cipher.DECRYPT_MODE, getPrivateKey(index));
            decryptedMessageBytes = decryptCipher.doFinal(Base64.getMimeDecoder().decode(encryptedData));
        } catch (Exception e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }
        String decryptedMessage = new String(decryptedMessageBytes, StandardCharsets.UTF_8);
        return decryptedMessage;

    }

    public static String getSignature(int index, String message) throws NoSuchAlgorithmException, SignatureException,
            InvalidKeyException, InvalidKeySpecException, IOException, URISyntaxException {
        PrivateKey privateKey = getPrivateKey(index);
        byte[] msg = message.getBytes();
        Signature signature = Signature.getInstance("NONEwithRSA");
        signature.initSign(privateKey);
        signature.update(msg);
        byte[] s = signature.sign();
        // To get singature as String

        String encodedString = Base64.getEncoder().encodeToString(s);
        return encodedString;

    }

    // Verify a signature
    // Used for transaction verification
    public static boolean verify(int index, String signatureinput, String message) throws NoSuchAlgorithmException,
            SignatureException, InvalidKeyException, InvalidKeySpecException, IOException, URISyntaxException {
        PublicKey publicKey = getPublicKey(index);
        byte[] msgbytes = message.getBytes();
        byte[] signaturebytes = Base64.getMimeDecoder().decode(signatureinput);
        Signature signature = Signature.getInstance("NONEwithRSA");
        signature.initVerify(publicKey);
        signature.update(msgbytes);
        boolean bool = signature.verify(signaturebytes);
        return bool;
    }

    // Verify a signature by using public key passed as string
    // Used for block verification
    public static boolean verify(String pubKeyStr, String signatureinput, String message)
            throws NoSuchAlgorithmException,
            SignatureException, InvalidKeyException, InvalidKeySpecException, IOException {
        byte[] pubKeyBytes = Base64.getDecoder().decode(pubKeyStr);
        KeyFactory keyFactory = KeyFactory.getInstance("RSA");
        X509EncodedKeySpec publicKeySpec = new X509EncodedKeySpec(
                pubKeyBytes);
        PublicKey publicKey = keyFactory.generatePublic(publicKeySpec);
        byte[] msgbytes = CryptoUtil.getHash(message).getBytes();
        byte[] signaturebytes = Base64.getMimeDecoder().decode(signatureinput);
        Signature signature = Signature.getInstance("NONEwithRSA");
        signature.initVerify(publicKey);
        signature.update(msgbytes);
        boolean bool = signature.verify(signaturebytes);
        return bool;
    }

    // UUID Version 4 Pseudo Random Generator
    public static String getUniqueIdentifier() {
        UUID uuid = UUID.randomUUID();
        return uuid.toString();
    }

    // Get SHA256 Hash of a String Data in String format
    public static String getHash(String data) {
        String sha256Hex = Hashing.sha256().hashString(data, StandardCharsets.UTF_8).toString();
        return sha256Hex;
    }

    private CryptoUtil() {
        throw new java.lang.UnsupportedOperationException("This is a utility class and cannot be instantiated");
    }

}
