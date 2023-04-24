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


async function main() {
  // Instantiate the API
  const provider = new WsProvider('ws://substrate-ws.unice.cust.tasfrance.com');

  const asset_service_admin_pub_key = 'http://radarapi.unice.cust.tasfrance.com/getPublicKey';

  const asset_service_admin_encrypt_url = 'http://radarapi.unice.cust.tasfrance.com/getEncryptedBid';

  const asset_service_admin_decrypt_url = 'http://radarapi.unice.cust.tasfrance.com/getDecryptedBid';

  //const provider = new WsProvider('ws://127.0.0.1:9944');

  //const asset_service_admin_pub_key = 'http://localhost:9090/getPublicKey';

  //const asset_service_admin_encrypt_url = 'http://localhost:9090/getEncryptedBid';

  //const asset_service_admin_decrypt_url = 'http://localhost:9090/getDecryptedBid';

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

  //const sudoKey = await api.query.sudo.key();

  //const sudoPair = keyring.getPair(sudoKey);

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


  ////////////////////////////////////////////////////////// Asset Service////////////////////////////////

  if (true) {

    //Step 5: (Fund amount in Vehicle)
    // To have more than 2000 advisable
    const transfer_to_vehicle = api.tx.balances.transfer(vehicle.address, 3000);

    //const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

    const transfer_to_vehicle_hash = await transfer_to_vehicle.signAndSend(bob, { nonce: -1 });

    console.log('Fund The New Vehicle Account with Enough Funds : 3000 to Purchase Asseet Service, tx hash' + transfer_to_vehicle_hash)
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    // Get the Public Key from Asset Service Admin

    var asset_service_pub_key = '';

    var data = qs.stringify({
      'username': 'radar_oem',
      'password': 'leat_radar'
    });
    var config = {
      method: 'post',
      url: asset_service_admin_pub_key,
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      },
      data: data
    };

    await axios(config)
      .then(function (response) {
        asset_service_pub_key = JSON.stringify(response.data);
      })
      .catch(function (error) {
        console.log(error);
      });

    console.log("Retrieved the Public Key of Radar OEM in response to demand for Creation of Asset Service Offering as NFT")
    console.log("This is Offchain action via radarapi.unice.cust.tasfrance");
    console.log('PUB KEY' + asset_service_pub_key);
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    console.log('STARTED ASSET SERVICE TRANSACTION LIFECYCLE')

    //Step 6: Create an Asset Service by Asset Service Admin (Charlie) 
    const { nonce: charlieservice_nonce, data: charlieservice_balance } = await api.query.system.account(charlie.address);

    // Price, Criteria, Price Threshold, Validation, Privacy Boolean, PubKey
    const createassetservice_tx = await api.tx.templateModule.createAssetService(600, 200, 550, 1, 1, asset_service_pub_key)

    const createassetservice_hash = await createassetservice_tx.signAndSend(charlie, { nonce: -1 });

    console.log("Asset Service Details: 600, 200, 1, 550, 1, asset_service_pub_key as Price, Criteria, Validation, Price Threshold, Privacy Boolean, PubKey ")

    console.log("Step 1: Created a new NFT Asset Service Token with tx Hash" + createassetservice_hash + "by Radar OEM ")
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    await sleep(sleeptime);
    console.log('sleeping')

    //Step 7: Get Asset Service Count

    const assetservice_count = await api.query.templateModule.allAssetServicesCount()

    console.log('Retrieved Asset Service NFT TOKEN Count in the Data Market OnChain' + assetservice_count);
    console.log('==========================================================================================================================================')
    console.log('                                                                                                                                          ')
    console.log('==========================================================================================================================================')

    // Start of Asset Finalisation
    for (let j = 0; j < assetservice_count; j++) {
      // Step 8: Get Asset Id
      const assetservice_id = await api.query.templateModule.allAssetServicesArray(j)

      console.log('Retrieved the Minted NFT Asset Service Id: ' + assetservice_id);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      // Step 9: Broadcast Asset Service by Asset Service Admin
      const broadcastassetservice_tx = await api.tx.templateModule.broadcastAssetService(assetservice_id.toString(), 590) // Asset Service Id, Criteria

      const broadcastassetservice_hash = await broadcastassetservice_tx.signAndSend(charlie, { nonce: -1 });

      console.log("Radar OEM Broadcast the Create Asset Service NFT to EveryOne ")

      console.log('broadcastassetservice_hash' + broadcastassetservice_hash);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')


      // Step 10: Vehicles submit their interest for Asset Service

      const submitassetserviceinterest_tx = await api.tx.templateModule.assetServiceInterest(assetservice_id.toString(), 590) // Asset Id, buy price

      const { nonce: vehicle_nonce, data: vehicle_balance } = await api.query.system.account(vehicle.address);

      const submitassetserviceinterest_hash = await submitassetserviceinterest_tx.signAndSend(vehicle, { nonce: -1 });

      console.log("Vehicles Submit / Consent to the Asset Service NFT Token ")

      console.log('submitassetserviceinterest_hash' + submitassetserviceinterest_hash);
      console.log('==========================================================================================================================================')
      console.log('                                                                                                                                          ')
      console.log('==========================================================================================================================================')

      if (true) {

        console.log("Vehicle OEM then finalises the purchases of the Asset Service NFT Token")
        console.log("Price of the Asset Service Token is transferred from the accepted vehicles (Service Interest Storage) to the Escrow Account")
        //Step 11: Vehicle OEM (BOB) buys the asset service 
        const buyassetserviceinterest_tx = await api.tx.templateModule.buyAssetService(assetservice_id.toString(), 590) // Asset Id, buy price

        //const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

        const buyassetserviceinterest_hash = await buyassetserviceinterest_tx.signAndSend(bob, { nonce: -1 });

        console.log('buyassetserviceinterest_hash' + buyassetserviceinterest_hash);
        console.log('==========================================================================================================================================')
        console.log('                                                                                                                                          ')
        console.log('==========================================================================================================================================')

        //Step 12: Radar OEM - Asset Service Admin (Charlie) submits the proof of the API Version and Release Hash to the blockchain
        const submitassetserviceproofinal_tx = await api.tx.templateModule.submitAssetServiceProofFinal(assetservice_id.toString(), '0x64ff825edd1f9accd746861f664c1fd2d508a705abe9559e76e9af512675c766');

        const submitassetserviceproofinal_hash = await submitassetserviceproofinal_tx.signAndSend(charlie, { nonce: -1 });

        console.log("Radar OEM - Asset Service Admin (Charlie) submits the proof of the API Version and Release Hash to the blockchain", submitassetserviceproofinal_hash)

        console.log('==========================================================================================================================================')
        console.log('                                                                                                                                          ')
        console.log('==========================================================================================================================================')
        const { nonce: vehiclenew_nonce, data: vehiclenew_balance } = await api.query.system.account(vehicle.address);

        console.log('Balance of the Vehicle' + vehiclenew_balance);

        //console.log('Result' + submitassetserviceproofinal_hash);

        //Step 13: Finalise Transfer Asset by Asset Service Admin

        const transferassetservice_tx = await api.tx.templateModule.transferAssetService(bob.address, assetservice_id.toString()) // Asset Id, buyeraddreess

        //const { nonce: bobi_nonce, data: bobi_balance } = await api.query.system.account();

        const transferassetservice_tx_hash = await transferassetservice_tx.signAndSend(charlie, { nonce: -1 });

        console.log("Oath Token submitted OnChain is also transferred Offchain by the Offchain Worker to the API ")

        console.log('Transfered ASSET Service' + transferassetservice_tx_hash);

        console.log('==========================================================================================================================================')
        console.log('                                                                                                                                          ')
        console.log('==========================================================================================================================================')

        await sleep(sleeptime);
        console.log('sleeping')

        //Step 13: Finalise Asset Transfer to remove counter for offchain worker

        const deleteOutstandingAssetServiceIndex_tx = await api.tx.templateModule.deleteOutstandingAssetServiceIndex() // Asset Id, buyeraddreess

        const deleteOutstandingAssetServiceIndex_tx_hash = await deleteOutstandingAssetServiceIndex_tx.signAndSend(charlie, { nonce: -1 });

        console.log('Deleted Outstanding Asset Service Index' + deleteOutstandingAssetServiceIndex_tx_hash);
        console.log('Putinto Idle the Offchain Workner which is listening on Onchain');
        console.log('==========================================================================================================================================')
        console.log('                                                                                                                                          ')
        console.log('==========================================================================================================================================')

        // Step 14: Submit Asset Service Review by Vehicle 
        // Only vehicles can review

        const submit_asset_service_review_vehicle_tx = await api.tx.templateModule.submitAssetServiceReview(assetservice_id, 4) // Asset Service Id, Rating Review

        //const new_charlie_nonce = charlie_nonce + i + 1;

        const submit_asset_service_review_vehicle_hash = await submit_asset_service_review_vehicle_tx.signAndSend(vehicle, { nonce: -1 });

        console.log('Submit the review for the Asset purchased by the Vehicle Only as they are the end users');

        console.log('Step 6: submit_asset_service_review_vehicle_hash' + submit_asset_service_review_vehicle_hash);
        console.log('==========================================================================================================================================')
        console.log('                                                                                                                                          ')
        console.log('==========================================================================================================================================')

        if (false) {
          //NOT VALID ASSET ADMIN IS NOT USER
          // Step 15: Submit Asset Service Review by Asset Admin (Charlie)  

          const submit_asset_service_review_tx = await api.tx.templateModule.submitAssetServiceReview(assetservice_id, 5) // Asset Service Id, Rating Review

          //const new_charlie_nonce = charlie_nonce + i + 1;

          const submit_asset_service_review_hash = await submit_asset_service_review_tx.signAndSend(bob, { nonce: -1 });

          console.log('submit_asset_service_review_hash' + submit_asset_service_review_hash);
        }

        console.log('OFFCHAIN VERFICATION')
        console.log('OAuth Token Storage VERIFICATION')

        var data = qs.stringify({
          'username': 'vehicle_oem',
          'password': 'leat_vehicle'
        });
        var config = {
          method: 'post',
          url: 'http://vehicleapi.unice.cust.tasfrance.com/getassetservicetokens',
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

        console.log('==========================================================================================================================================')
        console.log('                                                                                                                                          ')
        console.log('==========================================================================================================================================')
        console.log('FINISHED ASSET SERVICE NFT Purchase Lifecycle');
      }

    }
  }
}

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

main().catch(console.error).finally(() => process.exit(-1));

//main()