#!/usr/bin/env node

import substrate_data from "./substrate_data_lib.js";
import * as child from 'child_process';
import * as path from 'path';
import * as os from 'os';

// if (process.argv.length <= 2) {
//     console.error("Required 3 argument: \n\twait_time, ex: 1 (sec)\n\tnb_processes, ex: 2")
//     process.exit(1);
// }

//PROD URL
const url = "ws://substrate-ws.unice.cust.tasfrance.com";
// DEV URL
//const url = "ws://127.0.0.1:9944";
const wait_time = 100; //parseFloat(process.argv[2]) * 1000;
const nb_processes = 10; //Math.round(cpuCount/1.5); //parseInt(process.argv[3]);


console.log("Start init_v2.js...")
console.log("Settings:")
console.log("\t", nb_processes * (1 / (wait_time / 1000)), "Tx/sec (times the number of factories !)")
console.log("\t", nb_processes, "processes")
console.log("\t", wait_time, "wait time (ms)")

async function initAssetParticipants() {
    //init
    var api = await substrate_data.initApi(url);
    substrate_data.accounts.makeAllAdmins()

    const admin_array = substrate_data.accounts.getAllAdmins();
    console.log("Send new Asset Admin...")
    await substrate_data.sleep(2000); //wait a little
    var is_some_added = false;
    for (let i = 0; i < admin_array.length; i++) {
        var already_admin = await substrate_data.print_admins(api, admin_array[i].address);
        try {
            if (already_admin) {
                console.log(`Already Admin:\t ${admin_array[i].address}`)
            } else {
                await substrate_data.send.new_asset_admin(api, admin_array[i]); //alice is factory
                is_some_added = true;
                await substrate_data.sleep(500); //wait a little: get error "tx outdated" when one account sends too many tx/sec
            }
        } catch (e) {
            console.log("Init factory failed. Maybe already there or concurrent process already send it.")
            console.log(e)
        }
    }
    console.log("Wait block finalised")
    if (is_some_added)
        await substrate_data.sleep(18000); //wait block finalised
}

async function initEssentials() {
    //init
    var api = await substrate_data.initApi(url);
    await substrate_data.send.new_escrow(api);
    await substrate_data.send.new_asset_service_admin(api);
    console.log(`Init Essential Completed`);
    await substrate_data.sleep(18000);
}

// Add Alice as Escrow Account and Asset Service Admins
await initEssentials();

// Add Generated Addess to the Asset Admins
await initAssetParticipants();

console.log("End processes")

process.exit(0);