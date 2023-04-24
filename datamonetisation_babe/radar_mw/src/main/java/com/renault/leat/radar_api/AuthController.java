package com.renault.leat.radar_api;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.charset.StandardCharsets;
import java.security.NoSuchAlgorithmException;
import java.security.spec.InvalidKeySpecException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import com.google.common.hash.Hashing;

import com.renault.leat.radar_api.MWProperty;
import com.renault.leat.radar_api.Storage;

import org.apache.http.NameValuePair;
import org.apache.http.auth.AuthScope;
import org.apache.http.auth.UsernamePasswordCredentials;
import org.apache.http.client.CredentialsProvider;
import org.apache.http.client.entity.UrlEncodedFormEntity;
import org.apache.http.client.methods.CloseableHttpResponse;
import org.apache.http.client.methods.HttpPost;
import org.apache.http.entity.StringEntity;
import org.apache.http.impl.client.BasicCredentialsProvider;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.HttpClientBuilder;
import org.apache.http.impl.client.HttpClients;
import org.springframework.web.bind.annotation.GetMapping;
import org.apache.http.message.BasicNameValuePair;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.server.ResponseStatusException;

//@CrossOrigin(origins = { "${app.security.cors.origin}" })
@RestController
public class AuthController {

    private final JwtHelper jwtHelper;
    private final UserDetailsService userDetailsService;
    private final PasswordEncoder passwordEncoder;

    public AuthController(JwtHelper jwtHelper, UserDetailsService userDetailsService, PasswordEncoder passwordEncoder) {
        this.jwtHelper = jwtHelper;
        this.userDetailsService = userDetailsService;
        this.passwordEncoder = passwordEncoder;

    }

    @PostMapping(path = "/login", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public LoginResult login(@RequestParam String username, @RequestParam String password) {
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            Map<String, String> claims = new HashMap<>();
            claims.put("username", username);
            String authorities = userDetails.getAuthorities().stream().map(GrantedAuthority::getAuthority)
                    .collect(Collectors.joining(","));
            claims.put("authorities", authorities);
            claims.put("userId", String.valueOf(1));

            String jwt = jwtHelper.createJwtForClaims(username, claims);
            return new LoginResult(jwt);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    /**
     * Method to get the asset token
     * 
     * @param username
     * @param password
     * @return
     * @throws IOException
     */
    @PostMapping(path = "/assettoken", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public LoginResult getAssetToken(@RequestParam String username, @RequestParam String password) throws IOException {
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            Map<String, String> claims = new HashMap<>();
            claims.put("username", username);
            String authorities = userDetails.getAuthorities().stream().map(GrantedAuthority::getAuthority)
                    .collect(Collectors.joining(","));
            claims.put("authorities", authorities);
            claims.put("userId", String.valueOf(1));

            // 1) Get the Asset Token
            String jwt = jwtHelper.createJwtForClaims(username, claims);
            // 2) Get the Hash of the Token
            String sha256hex = Hashing.sha256().hashString(jwt, StandardCharsets.UTF_8).toString();
            // 3) Transfer the token to the respective Radar Manufacturer as a JSON

            HttpPost httpPost = new HttpPost(MWProperty.radarOEMURL);
            /*
             * StringBuilder json = new StringBuilder(); json.append("{");
             * json.append("\"AssetId\":\"1\","); json.append("\"AssetToken\":jwt");
             * json.append("}");
             * 
             * CredentialsProvider provider = new BasicCredentialsProvider();
             * provider.setCredentials(AuthScope.ANY, new
             * UsernamePasswordCredentials("radar_oem", "leat_radar"));
             */
            // CloseableHttpClient client =
            // HttpClientBuilder.create().setDefaultCredentialsProvider(provider).build();
            CloseableHttpClient client = HttpClients.createDefault();
            /*
             * StringEntity entity = new StringEntity(json.toString());
             * httpPost.setEntity(entity);
             */
            int retrievedAssetId = Storage.getAssetIndex();
            System.out.println("ASSET" + retrievedAssetId);
            Storage.addAssetId(retrievedAssetId);
            Storage.incrementAssetIndex();

            List<NameValuePair> urlParameters = new ArrayList<>();
            urlParameters.add(new BasicNameValuePair("username", "radar_oem"));
            urlParameters.add(new BasicNameValuePair("password", "leat_radar"));
            urlParameters.add(new BasicNameValuePair("AssetId", String.valueOf(retrievedAssetId)));
            urlParameters.add(new BasicNameValuePair("AssetToken", jwt));
            urlParameters.add(new BasicNameValuePair("Content-type", "application/x-www-form-urlencoded"));
            urlParameters.add(new BasicNameValuePair("Accept", "application/x-www-form-urlencoded"));

            httpPost.setEntity(new UrlEncodedFormEntity(urlParameters));
            // httpPost.setHeader();
            // httpPost.setHeader();
            // httpPost.setHeader("Accept", "application/json");
            // httpPost.setHeader("Content-type", "application/json");
            CloseableHttpResponse response = client.execute(httpPost);
            // assertThat(response.getStatusLine().getStatusCode(), equalTo(200));
            client.close();
            // System.out.println("YES");
            // AddTo Storage

            // 4) Return the Hash of the JWT Token
            return new LoginResult(sha256hex);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/assetexists", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public LoginResult assetExists(@RequestParam String username, @RequestParam String password,
            @RequestParam String assetId) throws IOException {
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {

            boolean result = Storage.containsAssetId(assetId);

            // 4) Return the Hash of the JWT Token
            return new LoginResult(String.valueOf(result));
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");

    }

    /**
     * Method to get the asset service token
     * 
     * @param username
     * @param password
     * @return
     * @throws IOException
     */
    @PostMapping(path = "/assetservicetoken", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public LoginResult getAssetServiceToken(@RequestParam String username, @RequestParam String password)
            throws IOException {
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            Map<String, String> claims = new HashMap<>();
            claims.put("username", username);
            String authorities = userDetails.getAuthorities().stream().map(GrantedAuthority::getAuthority)
                    .collect(Collectors.joining(","));
            claims.put("authorities", authorities);
            claims.put("userId", String.valueOf(1));

            // 1) Get the Asset Token
            String jwt = jwtHelper.createJwtForClaims(username, claims);
            // 2) Get the Hash of the Token
            String sha256hex = Hashing.sha256().hashString(jwt, StandardCharsets.UTF_8).toString();
            // 3) Transfer the token to the respective Radar Manufacturer as a JSON
            CloseableHttpClient client = HttpClients.createDefault();
            HttpPost httpPost = new HttpPost(MWProperty.vehicleOEMURL);
            /*
             * StringBuilder json = new StringBuilder(); json.append("{");
             * json.append("\"AssetId\":\"1\","); json.append("\"AssetToken\":jwt");
             * json.append("}");
             * 
             * StringEntity entity = new StringEntity(json.toString());
             * httpPost.setEntity(entity); httpPost.setHeader("Accept", "application/json");
             * httpPost.setHeader("Content-type", "application/json");
             */

            int retrievedAssetServiceId = Storage.getAssetServiceIndex();
            System.out.println("ASSET" + retrievedAssetServiceId);
            Storage.addAssetServiceId(retrievedAssetServiceId);
            Storage.incrementAssetServiceIndex();

            List<NameValuePair> urlParameters = new ArrayList<>();
            urlParameters.add(new BasicNameValuePair("username", "vehicle_oem"));
            urlParameters.add(new BasicNameValuePair("password", "leat_vehicle"));
            urlParameters.add(new BasicNameValuePair("AssetServiceId", String.valueOf(retrievedAssetServiceId)));
            urlParameters.add(new BasicNameValuePair("AssetServiceToken", jwt));
            urlParameters.add(new BasicNameValuePair("Content-type", "application/x-www-form-urlencoded"));
            urlParameters.add(new BasicNameValuePair("Accept", "application/x-www-form-urlencoded"));
            httpPost.setEntity(new UrlEncodedFormEntity(urlParameters));
            // httpPost.setHeader("username", "vehicle_oem");
            // httpPost.setHeader("password", "leat_vehicle");
            // httpPost.setHeader("AssetId", String.valueOf(retrievedAssetServiceId));
            // httpPost.setHeader("AssetToken", jwt);
            // httpPost.setHeader("Content-type", "application/x-www-form-urlencoded");
            // httpPost.setHeader("Accept", "application/x-www-form-urlencoded");
            CloseableHttpResponse response = client.execute(httpPost);
            // assertThat(response.getStatusLine().getStatusCode(), equalTo(200));
            // client.close();
            // AddTo Storage
            // 4) Return the Hash of the JWT Token
            return new LoginResult(sha256hex);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/assetserviceexists", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public LoginResult assetServiceExists(@RequestParam String username, @RequestParam String password,
            @RequestParam String assetServiceId) throws IOException {
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {

            boolean result = Storage.containsAssetServiceId(assetServiceId);

            // 4) Return the Hash of the JWT Token
            return new LoginResult(String.valueOf(result));
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    /**
     * Method to post the access token in the Radar database
     * 
     * @param AssetId
     * @param AssetToken
     * @return
     */
    @PostMapping(path = "/postassettoken", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<AssetFeedback> setAssetToken(@RequestParam(value = "username") String username,
            @RequestParam String password,
            @RequestParam String AssetId, @RequestParam String AssetToken) {
        System.out.println("ASSETrec" + AssetId);
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            AssetFeedback assetFeedback = new AssetFeedback();
            assetFeedback.setAssetId(AssetId);
            assetFeedback.setAssetToken(AssetToken);
            Storage.addAssetToken(assetFeedback);
            return ResponseEntity.ok(assetFeedback);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    /**
     * Method to post the access token in the Radar database
     * 
     * @param AssetId
     * @param AssetToken
     * @return
     */
    @PostMapping(path = "/postassetservicetoken", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<AssetFeedback> setAssetServiceToken(@RequestParam String username,
            @RequestParam String password, @RequestParam String AssetServiceId,
            @RequestParam String AssetServiceToken) {
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            AssetFeedback assetFeedback = new AssetFeedback();
            assetFeedback.setAssetId(AssetServiceId);
            assetFeedback.setAssetToken(AssetServiceToken);
            Storage.addAssetServiceToken(assetFeedback);
            return ResponseEntity.ok(assetFeedback);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/getassettokens", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> getAssetTokens(@RequestParam String username,
            @RequestParam String password) {
        System.out.println("VERIFIEDcall");
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            System.out.println("VERIFIED");
            String response = "";
            List<AssetFeedback> assetFeedbacks = Storage.getAssetTokens();
            for (AssetFeedback assetFeedback : assetFeedbacks) {
                response = response + ";" + assetFeedback.toString() + ";";
            }
            return ResponseEntity.ok(response);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/getassetservicetokens", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> getAssetServiceTokens(@RequestParam String username, @RequestParam String password) {
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            String response = "";
            List<AssetFeedback> assetFeedbacks = Storage.getAssetServiceTokens();
            for (AssetFeedback assetFeedback : assetFeedbacks) {
                response = response + ";" + assetFeedback.toString() + ";";
            }
            return ResponseEntity.ok(response);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/getdeleteassetid", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> getAssetId(@RequestParam String username, @RequestParam String password) {
        System.out.println("VERIFIEDcall");
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            System.out.println("VERIFIED");
            String response = "";
            int latestAssetId = 0;
            if (Storage.getAssetIds().size() > 0) {
                latestAssetId = Storage.getAssetIds().get(Storage.getAssetIds().size() - 1);
                List<Integer> assetList = new ArrayList<>();
                assetList.add(latestAssetId);
                Storage.removeAssetId(assetList);
            }

            return ResponseEntity.ok(String.valueOf(latestAssetId));
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/getdeleteassetserviceid", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> getAssetServiceId(@RequestParam String username,
            @RequestParam String password) {
        System.out.println("VERIFIEDcall");
        UserDetails userDetails;
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            System.out.println("VERIFIED");
            String response = "";
            int latestAssetServiceId = 0;
            if (Storage.getAssetServiceIds().size() > 0) {
                latestAssetServiceId = Storage.getAssetServiceIds().get(Storage.getAssetServiceIds().size() - 1);
                List<Integer> assetSrvcList = new ArrayList<>();
                assetSrvcList.add(latestAssetServiceId);
                Storage.removeAssetServiceId(assetSrvcList);
            }
            return ResponseEntity.ok(String.valueOf(latestAssetServiceId));
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    /**
     * Method to post the vehicle data in the OEM database
     * 
     * @param VehicleId
     * @param RadarData
     * @return
     */
    @PostMapping(path = "/postData", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<RadarData> postData(@RequestParam String VehicleId, @RequestParam String RadarData) {
        RadarData radarData = new RadarData();
        radarData.setCarId(VehicleId);
        radarData.setRadarDetails(RadarData);
        Storage.addRadarData(radarData);
        return ResponseEntity.ok(radarData);
    }

    /**
     * Method to get the vehicle data in the OEM database
     * 
     * @param VehicleId
     * @param RadarData
     * @return
     */
    @PostMapping(path = "/getData", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<RadarData> getData() {
        RadarData radarData = Storage.getRadarData();
        return ResponseEntity.ok(radarData);
    }

    @PostMapping(path = "/getPublicKey", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> getPublicKey(@RequestParam String username,
            @RequestParam String password)
            throws NoSuchAlgorithmException, InvalidKeySpecException, IOException, URISyntaxException {
        UserDetails userDetails;
        String pubKey = "";
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }
        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            System.out.println("VERIFIED");
            pubKey = CryptoUtil.getPublicKeyString(MWProperty.radarOEMIndex);
            return ResponseEntity.ok(pubKey);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/getEncryptedBid", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> getEncryptedBid(@RequestParam String username,
            @RequestParam String password, @RequestParam String data) {
        UserDetails userDetails;
        String encryptedString = "";
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }
        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            System.out.println("VERIFIED");
            encryptedString = CryptoUtil.getEncryptedData(data, MWProperty.radarOEMIndex);
            return ResponseEntity.ok(encryptedString);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/getDecryptedBid", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> getDecryptedBid(@RequestParam String username,
            @RequestParam String password, @RequestParam String encryptedData) {
        UserDetails userDetails;
        String decryptedString = "";
        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            System.out.println("VERIFIED");
            decryptedString = CryptoUtil.getDecryptedData(encryptedData, MWProperty.radarOEMIndex);
            return ResponseEntity.ok(decryptedString);
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/isDeployed", consumes = { MediaType.APPLICATION_FORM_URLENCODED_VALUE })
    public ResponseEntity<String> isDeployed(@RequestParam String username,
            @RequestParam String password) {
        UserDetails userDetails;

        try {
            userDetails = userDetailsService.loadUserByUsername(username);
        } catch (UsernameNotFoundException e) {
            throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User Not Found");
        }

        if (passwordEncoder.matches(password, userDetails.getPassword())) {
            return ResponseEntity.ok("I am Deployed by RADAR OEM");
        }
        throw new ResponseStatusException(HttpStatus.UNAUTHORIZED, "User not authenticated");
    }

    @PostMapping(path = "/deployementStatus")
    public ResponseEntity<String> deployementStatus() {
        return ResponseEntity.ok("I am Deployed by RADAR OEM");

    }

}
