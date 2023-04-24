#!/usr/bin/env node
//https://medium.com/coinmonks/starting-with-polkadot-development-part-iii-234fc5e13687
// Import
// https://brightinventions.pl/blog/develop-your-own-cryptocurrency-with-substrate-2/

// To RUN : node initialise_datapallet.js
import axios from 'axios';
import qs from 'qs';
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
  //var axios = require('axios');
  // var qs = require('qs');
  var data = qs.stringify({
    'username': 'vehicle_oem',
    'password': 'leat_vehicle',
    'data': '500'
  });
  var config = {
    method: 'post',
    url: 'http://localhost:8080/getEncryptedBid',
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



}
function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

main().catch(console.error).finally(() => process.exit(-1));

//main()