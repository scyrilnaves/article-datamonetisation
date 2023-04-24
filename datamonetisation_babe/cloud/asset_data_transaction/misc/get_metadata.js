#!/usr/bin/env node
//https://medium.com/coinmonks/starting-with-polkadot-development-part-iii-234fc5e13687
// Import
// https://brightinventions.pl/blog/develop-your-own-cryptocurrency-with-substrate-2/

// To RUN : node initialise_datapallet.js
import { ApiPromise, WsProvider } from '@polkadot/api';

import { Keyring } from '@polkadot/keyring';

import { BN } from 'bn.js';
/* async function main () {
  // Initialise the provider to connect to the local node
  const provider = new WsProvider('ws://substrate-ws.unice.cust.tasfrance.com');

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });

  // Retrieve the chain & node information information via rpc calls
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);

  //Get 

  console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
} */


async function main() {
  // Instantiate the API
  //const provider = new WsProvider('ws://substrate-ws.unice.cust.tasfrance.com');

  const provider = new WsProvider('ws://127.0.0.1:9944');

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

  //console.log('version: ' + metadata.version)
  //console.log('Magic number: ' + metadata.magicNumber)
  // GET METHODS METADATA
  console.log('Metadata: ' + JSON.stringify(metadata.asLatest.toHuman(), null, 2))

  // Listen to events
  if (false) {
    api.query.system.events((events) => {
      console.log(`\nReceived ${events.length} events:`);

      // Loop through the Vec<EventRecord>
      events.forEach((record) => {
        // Extract the phase, event and the event types
        const { event, phase } = record;
        const types = event.typeDef;

        // Show what we are busy with
        console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
        //console.log(`\t\t${event.meta.documentation.toString()}`);

        // Loop through each of the parameters, displaying the type and data
        event.data.forEach((data, index) => {
          console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
        });
      });
    });
  }

  if (false) {

    // Step 1: Add an Escrow Account
    const escrow_tx = await api.tx.templateModule.addEscrow()

    const { nonce: alice_nonce, data: balance } = await api.query.system.account(alice.address);

    const hash = await escrow_tx.signAndSend(alice, { nonce: alice_nonce });

    await sleep(7000);
    // Step 2: Add an Asset Admin
    const addassetadmin_tx = await api.tx.templateModule.addAssetAdmin(bob.address)

    const addasset_hash = await addassetadmin_tx.signAndSend(alice, { nonce: alice_nonce + 1 });

    // Step 3: Add an Asset Service Admin
    const addassetserviceadmin_tx = await api.tx.templateModule.addAssetServiceAdmin(charlie.address)

    const addassetservice_hash = await addassetserviceadmin_tx.signAndSend(alice, { nonce: alice_nonce + 2 });

    // Step 4: Add a Vehicle by Asset Admin (bob)
    const addvehicle_tx = await api.tx.templateModule.addVehicle(vehicle.address)

    const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

    const addvehicle_hash = await addvehicle_tx.signAndSend(bob, { nonce: bob_nonce });

    if (true) {

      // Step 5: Broadcast Asset Demand by Asset Service Admin (charlie)
      const createassetdemand_tx = await api.tx.templateModule.createAssetDemand(200, 500) // Criteria, Price

      const { nonce: charlie_nonce, data: charlie_balance } = await api.query.system.account(charlie.address);

      const createassetdemand_hash = await createassetdemand_tx.signAndSend(charlie, { nonce: charlie_nonce });

      //Step6: Create Asset by Asset Admin (bob)
      const createasset_tx = await api.tx.templateModule.createAsset(600, 200, 550) // Price, Criteria, Price Threshold

      const new_bob_nonce = bob_nonce + 1;

      const createasset_hash = await createasset_tx.signAndSend(bob, { nonce: new_bob_nonce });

      //Step 7: Get Asset Count

      await sleep(20000);

      const asset_count = await api.query.templateModule.allAssetsCount()

      console.log('AssetCount' + asset_count);

      // Start of Asset Finalisation
      for (let i = 0; i < asset_count; i++) {
        // Step 8: Get Asset Id
        const asset_id = await api.query.templateModule.allAssetsArray(i)

        console.log('AssetId' + asset_id);

        //Step 9: Buy Asset by Asset Service Admin (Charlie)  

        const buyasset_tx = await api.tx.templateModule.buyAsset(asset_id, 570) // Asset Id, buy price

        const new_charlie_nonce = charlie_nonce + i + 1;

        const buyeasset_hash = await buyasset_tx.signAndSend(charlie, { nonce: -1 });

        console.log('buyeasset_hash' + buyeasset_hash);

        //Step 10: Submit Asset Proof by Vehicle

        const submitassetproof_tx = await api.tx.templateModule.submitAssetProof(asset_id, '0xbc0560970bbc96e4b0b7a709582fc752d1d9ffac2f6b8578c9e0ee5b4d9ecc65') // Asset Id, Proof

        const { nonce: vehicle_nonce, data: vehicle_balance } = await api.query.system.account(vehicle.address);

        const submitassetproof_hash = await submitassetproof_tx.signAndSend(vehicle, { nonce: -1 });

        //Step 11: Submit Asset Proof Final by Asset Admin

        const submitassetprooffinal_tx = await api.tx.templateModule.submitAssetProofFinal(asset_id, '0x64ff825edd1f9accd746861f664c1fd2d508a705abe9559e76e9af512675c766') // Asset Id, Proof

        const { nonce: bobi_nonce, data: bobi_balance } = await api.query.system.account(bob.address);

        const submitassetprooffinal_hash = await submitassetprooffinal_tx.signAndSend(bob, { nonce: -1 });

        await sleep(20000);

        //Step 12: Finalise Trasnfer Asset by Asset Admin

        const transferasset_tx = await api.tx.templateModule.transferAsset(charlie.address, asset_id) // Asset Id, buyeraddreess

        //const { nonce: bobi_nonce, data: bobi_balance } = await api.query.system.account();

        const transferasset_tx_hash = await transferasset_tx.signAndSend(bob, { nonce: -1 });

        console.log('Transfered ASSET' + transferasset_tx_hash);

        await sleep(60000);

        //Step 13: Finalise Asset Transfer to remove counter for offchain worker

        const deleteOutstandingAssetIndex_tx = await api.tx.templateModule.deleteOutstandingAssetIndex() // Asset Id, buyeraddreess

        const deleteOutstandingAssetIndex_tx_hash = await deleteOutstandingAssetIndex_tx.signAndSend(bob, { nonce: -1 });

        console.log('Deleted Outstanding Asset Index' + deleteOutstandingAssetIndex_tx_hash);

      }
      console.log('FINISHED ASSET')
      //await sleep(20000);
    }
    ////////////////////////////////////////////////////////// Asset Service////////////////////////////////

    if (false) {
      console.log('STRATED ASSET SERVICE')

      //Step A (Fund amount in Vehicle)
      // To have more than 2000 advisable
      const transfer_to_vehicle = api.tx.balances.transfer(vehicle.address, 3000);

      //const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

      const transfer_to_vehicle_hash = await transfer_to_vehicle.signAndSend(bob, { nonce: -1 });

      //Step 1: Create an Asset Service by Asset Service Admin (Charlie) 
      const { nonce: charlieservice_nonce, data: charlieservice_balance } = await api.query.system.account(charlie.address);

      const createassetservice_tx = await api.tx.templateModule.createAssetService(600, 200, 550) // Price, Criteria, Price Threshold

      const createassetservice_hash = await createassetservice_tx.signAndSend(charlie, { nonce: charlieservice_nonce });

      await sleep(7000);

      //Step2 Get Asset Service Count

      const assetservice_count = await api.query.templateModule.allAssetServicesCount()

      console.log('AssetServiceCount' + assetservice_count);

      // Start of Asset Finalisation
      for (let j = 0; j < assetservice_count; j++) {
        // Step 3: Get Asset Id
        const assetservice_id = await api.query.templateModule.allAssetServicesArray(j)

        console.log('AssetServiceId' + assetservice_id);

        // Step 4: Broadcast Asset Service by Asset Service Admin
        const broadcastassetservice_tx = await api.tx.templateModule.broadcastAssetService(assetservice_id.toString(), 590) // Asset Id, buy price

        const broadcastassetservice_hash = await broadcastassetservice_tx.signAndSend(charlie, { nonce: -1 });

        console.log('broadcastassetservice_hash' + broadcastassetservice_hash);


        // Step 5: Vehciles submit their interest for Asset Service

        const submitassetserviceinterest_tx = await api.tx.templateModule.assetServiceInterest(assetservice_id.toString(), 590) // Asset Id, buy price

        const { nonce: vehicle_nonce, data: vehicle_balance } = await api.query.system.account(vehicle.address);

        const submitassetserviceinterest_hash = await submitassetserviceinterest_tx.signAndSend(vehicle, { nonce: -1 });

        console.log('submitassetserviceinterest_hash' + submitassetserviceinterest_hash);

        // Step6: Get Service Interest for the Asset Service by by Asset Admin
        // Skip This as it can be optional
        /*
        await sleep(7000);
        const keys = await api.query.templateModule.serviceInterestCounter.keys()
        const entries = await api.query.templateModule.serviceInterestCounter.entries()
      
        */


        //Step 7: Vehicle OEM (BOB) buys the asset 
        const buyassetserviceinterest_tx = await api.tx.templateModule.buyAssetService(assetservice_id.toString(), 590) // Asset Id, buy price

        //const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

        const buyassetserviceinterest_hash = await buyassetserviceinterest_tx.signAndSend(bob, { nonce: -1 });

        console.log('buyassetserviceinterest_hash' + buyassetserviceinterest_hash);

        //Step 8: Radar OEM - Asset Service Admin (Charlie) submits the proof of the API Version and Release Hash to the blockchain
        const submitassetserviceproofinal_tx = await api.tx.templateModule.submitAssetServiceProofFinal(assetservice_id.toString(), '0x64ff825edd1f9accd746861f664c1fd2d508a705abe9559e76e9af512675c766');

        const submitassetserviceproofinal_hash = await submitassetserviceproofinal_tx.signAndSend(charlie, { nonce: -1 });

        const { nonce: vehiclenew_nonce, data: vehiclenew_balance } = await api.query.system.account(vehicle.address);

        console.log('Balance SERVICE' + vehiclenew_balance);


        console.log('Result' + submitassetserviceproofinal_hash);

        console.log('FINISHED ASSET SERVICE');

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