#!/usr/bin/env node
//https://medium.com/coinmonks/starting-with-polkadot-development-part-iii-234fc5e13687
// Import
// https://brightinventions.pl/blog/develop-your-own-cryptocurrency-with-substrate-2/

// To RUN : node initialise_datapallet.js
import { ApiPromise, WsProvider } from '@polkadot/api';

import { Keyring } from '@polkadot/keyring';

import { BN } from 'bn.js';


async function main() {
  // Instantiate the API
  const provider = new WsProvider('ws://substrate-ws.unice.cust.tasfrance.com');

  //const provider = new WsProvider('ws://127.0.0.1:9944');

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


  ////////////////////////////////////////////////////////// Asset Service////////////////////////////////

  if (true) {
    console.log('STARTED ASSET SERVICE')

    //Step 5: (Fund amount in Vehicle)
    // To have more than 2000 advisable
    const transfer_to_vehicle = api.tx.balances.transfer(vehicle.address, 3000);

    //const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

    const transfer_to_vehicle_hash = await transfer_to_vehicle.signAndSend(bob, { nonce: -1 });

    //Step 6: Create an Asset Service by Asset Service Admin (Charlie) 
    const { nonce: charlieservice_nonce, data: charlieservice_balance } = await api.query.system.account(charlie.address);

    const createassetservice_tx = await api.tx.templateModule.createAssetService(600, 200, 550) // Price, Criteria, Price Threshold

    const createassetservice_hash = await createassetservice_tx.signAndSend(charlie, { nonce: charlieservice_nonce });

    await sleep(sleeptime);

    //Step 7: Get Asset Service Count

    const assetservice_count = await api.query.templateModule.allAssetServicesCount()

    console.log('AssetServiceCount' + assetservice_count);

    // Start of Asset Finalisation
    for (let j = 0; j < assetservice_count; j++) {
      // Step 8: Get Asset Id
      const assetservice_id = await api.query.templateModule.allAssetServicesArray(j)

      console.log('AssetServiceId' + assetservice_id);

      // Step 9: Broadcast Asset Service by Asset Service Admin
      const broadcastassetservice_tx = await api.tx.templateModule.broadcastAssetService(assetservice_id.toString(), 590) // Asset Id, Criteria

      const broadcastassetservice_hash = await broadcastassetservice_tx.signAndSend(charlie, { nonce: -1 });

      console.log('broadcastassetservice_hash' + broadcastassetservice_hash);


      // Step 10: Vehicles submit their interest for Asset Service

      const submitassetserviceinterest_tx = await api.tx.templateModule.assetServiceInterest(assetservice_id.toString(), 590) // Asset Id, buy price

      const { nonce: vehicle_nonce, data: vehicle_balance } = await api.query.system.account(vehicle.address);

      const submitassetserviceinterest_hash = await submitassetserviceinterest_tx.signAndSend(vehicle, { nonce: -1 });

      console.log('submitassetserviceinterest_hash' + submitassetserviceinterest_hash);

      if (true) {

        //Step 11: Vehicle OEM (BOB) submit bids the asset service 
        const bidassetserviceinterest_tx = await api.tx.templateModule.bidAssetService(assetservice_id.toString(), 590, '0') // Asset Id, buy price, encryptedprice

        //const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

        const bidassetserviceinterest_hash = await bidassetserviceinterest_tx.signAndSend(bob, { nonce: -1 });

        console.log('bidassetserviceinterest_hash' + bidassetserviceinterest_hash);

        //Step 12: RADAR OEM (Charile) accept bids the asset service 
        const acceptbidassetserviceinterest_tx = await api.tx.templateModule.acceptBidService(assetservice_id.toString(), 490) // Asset Id, buy price

        //const { nonce: bob_nonce, data: bob_balance } = await api.query.system.account(bob.address);

        const acceptbidassetserviceinterest_hash = await acceptbidassetserviceinterest_tx.signAndSend(bob, { nonce: -1 });

        console.log('acceptbidassetserviceinterest_hash' + acceptbidassetserviceinterest_hash);


        //Step 13: Radar OEM - Asset Service Admin (Charlie) submits the proof of the API Version and Release Hash to the blockchain
        const submitassetserviceproofinal_tx = await api.tx.templateModule.submitAssetServiceProofFinal(assetservice_id.toString(), '0x64ff825edd1f9accd746861f664c1fd2d508a705abe9559e76e9af512675c766');

        const submitassetserviceproofinal_hash = await submitassetserviceproofinal_tx.signAndSend(charlie, { nonce: -1 });

        const { nonce: vehiclenew_nonce, data: vehiclenew_balance } = await api.query.system.account(vehicle.address);

        console.log('Balance SERVICE' + vehiclenew_balance);

        console.log('Result' + submitassetserviceproofinal_hash);

        //Step 14: Finalise Transfer Asset by Asset Service Admin

        const transferassetservice_tx = await api.tx.templateModule.transferAssetService(bob.address, assetservice_id.toString()) // Asset Id, buyeraddreess

        //const { nonce: bobi_nonce, data: bobi_balance } = await api.query.system.account();

        const transferassetservice_tx_hash = await transferassetservice_tx.signAndSend(charlie, { nonce: -1 });

        console.log('Transfered ASSET Service' + transferassetservice_tx_hash);

        await sleep(sleeptime);

        //Step 15: Finalise Asset Transfer to remove counter for offchain worker

        const deleteOutstandingAssetServiceIndex_tx = await api.tx.templateModule.deleteOutstandingAssetServiceIndex() // Asset Id, buyeraddreess

        const deleteOutstandingAssetServiceIndex_tx_hash = await deleteOutstandingAssetServiceIndex_tx.signAndSend(charlie, { nonce: -1 });

        console.log('Deleted Outstanding Asset Service Index' + deleteOutstandingAssetServiceIndex_tx_hash);

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