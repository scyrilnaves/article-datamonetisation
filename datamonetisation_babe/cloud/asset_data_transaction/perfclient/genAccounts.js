#!/usr/bin/env node

// Import
import substrate_data from "./substrate_data_lib.js";
import { writeFileSync, unlinkSync, existsSync } from 'fs'
const filename_accounts = "admins.json"

if (process.argv.length < 1) {
    console.error("Required 1 argument: \n\tnumber of accounts to generate. Ex: 1000\n\tnumber of admin accounts to generate. Ex: 10")
    process.exit(1);
}

const args = process.argv.slice(2);
console.log(`args ${args}`)

const admin_pair_size = parseInt(args);
async function main() {
    console.log("Start genAccounts.js...")
    console.log("Delete old file...")
    if (existsSync(filename_accounts)) {
        unlinkSync(filename_accounts);
    }

    console.log(`Creating ${admin_pair_size} accounts...`)
    var ADMIN_PAIRS = [];
    for (let i = 0; i < admin_pair_size; i++) {
        //console.clear();
        //console.log(`Loading account list ${i * 100 / account_pair_size}%`)
        console.log(`iteration ${i} `);
        ADMIN_PAIRS.push(substrate_data.accounts.genMnemonic())
    }
    writeFileSync(filename_accounts, JSON.stringify(ADMIN_PAIRS, null, 1), { encoding: "utf8", flag: "a+" })

    console.log("Done")
}


///////////////////////////////End Generation////////////////
main().catch(console.error).finally(() => process.exit(0));