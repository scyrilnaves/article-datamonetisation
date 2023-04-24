#!/usr/bin/env node
//https://medium.com/coinmonks/starting-with-polkadot-development-part-iii-234fc5e13687
// Import
// https://brightinventions.pl/blog/develop-your-own-cryptocurrency-with-substrate-2/

// To RUN : node initialise_datapallet.js
import { ApiPromise, WsProvider } from '@polkadot/api';

import { Keyring } from '@polkadot/keyring';

import { BN } from 'bn.js';

import axios from 'axios';
import qs from 'qs';


// Initialise the provider to connect to the local node
//  const provider = new WsProvider('ws://substrate-ws.unice.cust.tasfrance.com');



async function main() {
  // Instantiate the API
  const provider = new WsProvider('ws://substrate-ws.unice.cust.tasfrance.com');

  const asset_admin_pub_key = 'http://vehicleapi.unice.cust.tasfrance.com/getPublicKey';
  console.log("For all the Exposed Endpoints of the Vehicle and Radaar API");
  console.log("http://vehicleapi.unice.cust.tasfrance.com/actuator/mappings");
  console.log("http://radarapi.unice.cust.tasfrance.com/actuator/mappings");

  const vehicle_get_data = 'http://vehicleapi.unice.cust.tasfrance.com/getData';

  const get_asset_token = 'http://radarapi.unice.cust.tasfrance.com/getassettokens';

  const asset_admin_encrypt_url = 'http://vehicleapi.unice.cust.tasfrance.com/getEncryptedBid';

  const asset_admin_decrypt_url = 'http://vehicleapi.unice.cust.tasfrance.com/getDecryptedBid';

  const vehicle_post_data = 'http://vehicleapi.unice.cust.tasfrance.com/postData';


  //const provider = new WsProvider('ws://127.0.0.1:9944');

  //const asset_admin_pub_key = 'http://localhost:8080/getPublicKey';

  //const asset_admin_encrypt_url = 'http://localhost:8080/getEncryptedBid';

  //const asset_admin_decrypt_url = 'http://localhost:8080/getDecryptedBid';

  const sleeptime = 20000;

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });

  // Constuct the keyring after the API (crypto has an async init)
  const keyring = new Keyring({ type: 'sr25519' });

  // Add Alice to our keyring with a hard-deived path (empty phrase, so uses dev)
  const alice = keyring.addFromUri('//Alice');

  // Add Bob
  const bob = keyring.addFromUri('//Bob');

  // Add Charlie
  const charlie = keyring.addFromUri('//Charlie');

  const vehicle = keyring.createFromUri("trip frown invite narrow nominee plunge first earn pledge kitchen legend layer", { name: 'vehicle' }, 'sr25519');

  const metadata = await api.rpc.state.getMetadata()

  console.log("Step A1:Inititialised Accounts");
  console.log('==========================================================================================================================================')
  console.log('                                                                                                                                          ')
  console.log('==========================================================================================================================================')

  console.log("CHECK BELOW API RESPONSE for VEHICLE and RADAR OEM");
  var data = qs.stringify({
  });
  var config = {
    method: 'post',
    //url: 'http://localhost:8080/getEncryptedBid',
    url: 'http://vehicleapi.unice.cust.tasfrance.com/deployementStatus',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    data: data
  };

  await axios(config)
    .then(function (response) {
      console.log(JSON.stringify(response.data));
    })
    .catch(function (error) {
      console.log(error);
    });

  var data = qs.stringify({
  });
  var config = {
    method: 'post',
    //url: 'http://localhost:8080/getEncryptedBid',
    url: 'http://radarapi.unice.cust.tasfrance.com/deployementStatus',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    data: data
  };

  await axios(config)
    .then(function (response) {
      console.log(JSON.stringify(response.data));
    })
    .catch(function (error) {
      console.log(error);
    });

  console.log("For all the Exposed Endpoints of the Vehicle and Radaar API");
  console.log("http://vehicleapi.unice.cust.tasfrance.com/actuator/mappings");
  console.log("http://radarapi.unice.cust.tasfrance.com/actuator/mappings");
  console.log('==========================================================================================================================================')
  console.log('                                                                                                                                          ')
  console.log('==========================================================================================================================================')

  // Step 1: Add an Escrow Account
  const escrow_tx = await api.tx.templateModule.addEscrow()

  const { nonce: alice_nonce, data: balance } = await api.query.system.account(alice.address);

  const hash = await escrow_tx.signAndSend(alice, { nonce: -1 });

  console.log("Step A2:Added ALICE as escrow account");
  console.log('==========================================================================================================================================')
  console.log('                                                                                                                                          ')
  console.log('==========================================================================================================================================')

  await sleep(sleeptime);
  console.log('sleeping')
  // Step 2: Add an Asset Admin
  const addassetadmin_tx = await api.tx.templateModule.addAssetAdmin(bob.address)

  const addasset_hash = await addassetadmin_tx.signAndSend(alice, { nonce: -1 });

  console.log("Step A3: Added BOB as ASSET ADMIN or VEHICLE OEM");
  console.log('==========================================================================================================================================')
  console.log('                                                                                                                                          ')
  console.log('==========================================================================================================================================')

  // Step 3: Add an Asset Service Admin
  const addassetserviceadmin_tx = await api.tx.templateModule.addAssetServiceAdmin(charlie.address)

  const addassetservice_hash = await addassetserviceadmin_tx.signAndSend(alice, { nonce: -1 });

  console.log("Step A4: Added CHARLIE as ASSET SERVICE ADMIN or RADAR OEM");
  console.log('==========================================================================================================================================')
  console.log('                                                                                                                                          ')
  console.log('==========================================================================================================================================')

  // Step 4: Add a Vehicle by Asset Admin (bob)
  const addvehicle_tx = await api.tx.templateModule.addVehicle(vehicle.address)

  const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

  const addvehicle_hash = await addvehicle_tx.signAndSend(bob, { nonce: -1 });

  console.log("Step A5: Added VEHICLE for PARTICIPATION BY VEHICLE OEM: BOB");
  console.log('==========================================================================================================================================')
  console.log('                                                                                                                                          ')
  console.log('==========================================================================================================================================')

  if (true) {

    // Step 5: Broadcast Asset Demand by Asset Service Admin (charlie)
    const createassetdemand_tx = await api.tx.templateModule.createAssetDemand(200, 500, 1) // Criteria, Price, Validation

    const { nonce: charlie_nonce, data: charlie_balance } = await api.query.system.account(charlie.address);

    const createassetdemand_hash = await createassetdemand_tx.signAndSend(charlie, { nonce: -1 });

    console.log("Step 1: Broadcasted Asset Demand with Criteria: 200, 500, 1 as Criteria, Price, Validation BY VEHICLE OEM: BOB");
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    // Get the Public Key from Asset Admin

    var asset_pub_key = '';

    var data = qs.stringify({
      'username': 'vehicle_oem',
      'password': 'leat_vehicle'
    });
    var config = {
      method: 'post',
      url: asset_admin_pub_key,
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      },
      data: data
    };

    await axios(config)
      .then(function (response) {
        asset_pub_key = JSON.stringify(response.data);
      })
      .catch(function (error) {
        console.log(error);
      });

    console.log("Retrieved the Public Key of Vehicle OEM in response to demand for Creation of Asset as NFT")
    console.log("This is Offchain action via vehicleapi.unice.cust.tasfrance");
    console.log('PUB KEY' + asset_pub_key);
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    //Step6: Create Asset by Asset Admin (bob)
    const createasset_tx = await api.tx.templateModule.createAsset(600, 200, 1, 550, 1, asset_pub_key) // Price, Criteria, Validation, Price Threshold, Privacy, PubKey

    const new_bob_nonce = bob_nonce + 1;

    const createasset_hash = await createasset_tx.signAndSend(bob, { nonce: -1 });

    console.log("Asset Details: 600, 200, 1, 550, 1, asset_pub_key as Price, Criteria, Validation, Price Threshold, Privacy, PubKey ")

    console.log("Step 2: Created a new NFT Token with tx Hash" + createasset_hash + "by Vehicle OEM ")
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    //Step 7: Get Asset Count
    console.log('sleeping')
    await sleep(sleeptime);

    const asset_count = await api.query.templateModule.allAssetsCount()

    console.log('Retrieved AssetCount in the Data Market OnChain' + asset_count);
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    // Start of Asset Finalisation
    for (let i = 0; i < asset_count; i++) {
      // Step 8: Get Asset Id
      const asset_id = await api.query.templateModule.allAssetsArray(i)

      console.log('Retrieved the Minted NFT AssetId: ' + asset_id);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      console.log('Radar OEM Gets the Encrypted Price to do the bidding');

      // Get the Encrypted Price
      var encryptedprice = '';

      var data = qs.stringify({
        'username': 'vehicle_oem',
        'password': 'leat_vehicle',
        'data': '500'
      });
      var config = {
        method: 'post',
        url: asset_admin_encrypt_url,
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        data: data
      };

      await axios(config)
        .then(function (response) {
          encryptedprice = JSON.stringify(response.data);
        })
        .catch(function (error) {
          console.log(error);
        });

      console.log('Encrypted Price' + encryptedprice);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')
      //Step 9: Bid Asset by Asset Service Admin (Charlie)  

      const bidasset_tx = await api.tx.templateModule.bidAsset(asset_id, 600, encryptedprice) // Asset Id, buy price, encrypted price

      //const new_charlie_nonce = charlie_nonce + i + 1;

      const bidasset_hash = await bidasset_tx.signAndSend(charlie, { nonce: -1 });

      console.log("Asset NFT Token Bid is submitted by the Radar OEM @ Price: 600 but is encrypted")

      console.log('bidasset_hash' + bidasset_hash);

      console.log('Vehicle OEM gets the Encrypted Price OnChain for the Bid and Decrypts it Offchain');

      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      // Retrieve the Public Key for an asset and use it for Decryption from the storage
      // Get the Decrypted Price
      var decryptedprice = '';

      var data = qs.stringify({
        'username': 'vehicle_oem',
        'password': 'leat_vehicle',
        'encryptedData': encryptedprice
      });
      var config = {
        method: 'post',
        url: asset_admin_decrypt_url,
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        data: data
      };

      await axios(config)
        .then(function (response) {
          decryptedprice = JSON.stringify(response.data);
        })
        .catch(function (error) {
          console.log(error);
        });

      console.log('Decrypted Price' + decryptedprice);

      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      //Step 10:  AcceptBid Asset by Asset  Admin (Bob)  

      //const accept_bidasset_tx = await api.tx.templateModule.acceptBid(asset_id, 550) // Asset Id, Bid threshold 
      const accept_bidasset_tx = await api.tx.templateModule.acceptBidPrivacy(asset_id) // Asset Id, Bid threshold 
      // Bid Price should be greater than or equal to Bid threshold 

      const accept_bidasset_hash = await accept_bidasset_tx.signAndSend(bob, { nonce: -1 });

      console.log('Acceptation of NFT Asset Bid of Radar OEM: Charlie by Vehicle OEM with price: 570, TransactionHash:' + accept_bidasset_hash);
      console.log('accept_bidasset_hash' + accept_bidasset_hash);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      console.log("Vehicle submit Data Offchain to the Vehicle OEM Data Storage API ")

      //Step 10: Vehicle Posts the Data

      var data = qs.stringify({
        'VehicleId': 'V245',
        'RadarData': 'NICE 06560'
      });
      var config = {
        method: 'post',
        url: vehicle_post_data,
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        data: data
      };

      await axios(config)
        .then(function (response) {
          console.log(JSON.stringify(response.data));
        })
        .catch(function (error) {
          console.log(error);
        });
      /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
      console.log("We get the data to check if in storage");
      /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
      var getdata = qs.stringify({
      });
      var getconfig = {
        method: 'post',
        url: vehicle_get_data,
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        data: getdata
      };

      await axios(getconfig)
        .then(function (response) {
          console.log(JSON.stringify(response.data));
        })
        .catch(function (error) {
          console.log(error);
        });
      //////////////////////////////////////////////////////////////////////////
      console.log("Back to OnChain Actions")
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')


      //Step 10: Submit Asset Proof by Vehicle

      const submitassetproof_tx = await api.tx.templateModule.submitAssetProof(asset_id, '0xbc0560970bbc96e4b0b7a709582fc752d1d9ffac2f6b8578c9e0ee5b4d9ecc65') // Asset Id, Proof

      //const { nonce: vehicle_nonce, data: vehicle_balance } = await api.query.system.account(vehicle.address);

      const submitassetproof_hash = await submitassetproof_tx.signAndSend(vehicle, { nonce: -1 });

      console.log("Step 3: Vehicle Submits the Obfuscated Hash OnChain to the Data Market, Tx Hash", + submitassetproof_hash);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      //Step 11: Submit Asset Proof Final by Asset Admin

      const submitassetprooffinal_tx = await api.tx.templateModule.submitAssetProofFinal(asset_id, '0x64ff825edd1f9accd746861f664c1fd2d508a705abe9559e76e9af512675c766') // Asset Id, Proof

      //const { nonce: bobi_nonce, data: bobi_balance } = await api.query.system.account(bob.address);

      const submitassetprooffinal_hash = await submitassetprooffinal_tx.signAndSend(bob, { nonce: -1 });

      console.log("Vehicle OEM Processes the Data submitted Offchain");

      console.log("Step 4: Vehicle OEM submits the Final Hash of the Processed Data OnChain:", submitassetprooffinal_hash);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      await sleep(sleeptime);
      console.log('sleeping')

      //Step 12: Finalise Transfer Asset by Asset Admin

      const transferasset_tx = await api.tx.templateModule.transferAsset(charlie.address, asset_id) // Asset Id, buyeraddreess

      //const { nonce: bobi_nonce, data: bobi_balance } = await api.query.system.account();

      const transferasset_tx_hash = await transferasset_tx.signAndSend(bob, { nonce: -1 });

      console.log(' Step 5: NFT Token @' + asset_id + "is transfrred from Vehicle OEM to Radar OEM for Transaction" + transferasset_tx_hash);

      console.log('Transfered ASSET' + transferasset_tx_hash);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      await sleep(sleeptime);
      console.log('sleeping')

      //Step 13: Finalise Asset Transfer to remove counter for offchain worker

      const deleteOutstandingAssetIndex_tx = await api.tx.templateModule.deleteOutstandingAssetIndex() // Asset Id, buyeraddreess

      const deleteOutstandingAssetIndex_tx_hash = await deleteOutstandingAssetIndex_tx.signAndSend(bob, { nonce: -1 });

      console.log('Deleted Outstanding Asset Index' + deleteOutstandingAssetIndex_tx_hash);
      console.log('Putinto Idle the Offchain Workner which is listening on Onchain');
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      // Step 14: Submit Asset Review by Asset Service Admin (Charlie)  

      const submit_asset_review_tx = await api.tx.templateModule.submitAssetReview(asset_id, 5) // Asset Id, Rating Review

      //const new_charlie_nonce = charlie_nonce + i + 1;

      const submit_asset_review_hash = await submit_asset_review_tx.signAndSend(charlie, { nonce: -1 });

      console.log('submit_asset_review_hash' + submit_asset_review_hash);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

    }

    console.log('OFFCHAIN VERFICATION')
    console.log('OAuth Token Storage VERIFICATION')

    var tokendata = qs.stringify({
      'username': 'radar_oem',
      'password': 'leat_radar'
    });
    var tokenconfig = {
      method: 'post',
      url: get_asset_token,
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      },
      data: tokendata
    };

    await axios(tokenconfig)
      .then(function (response) {
        console.log(JSON.stringify(response.data));
      })
      .catch(function (error) {
        console.log(error);
      });
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')
    console.log('FINISHED ASSET BID TRANSACTION LIFECYCLE')
    //await sleep(20000);
    ////////////////////////////////////////////////////////// Asset Service////////////////////////////////
  }
}

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

main().catch(console.error).finally(() => process.exit(-1));

//main()