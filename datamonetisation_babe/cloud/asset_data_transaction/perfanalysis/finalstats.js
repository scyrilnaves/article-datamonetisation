#!/usr/bin/env node

import substrate_data from "./substrate_data_lib.js";
import * as child from 'child_process';
import * as path from 'path';
import * as async from "async";

//PROD URL
const url = "ws://substrate-ws.unice.cust.tasfrance.com";
// DEV URL
//const url = "ws://127.0.0.1:9944";

var startBlockHash = process.argv[2];

console.log("Input Hash", startBlockHash)

function do_stats(callback) {
    (async () => {
        console.log("Retrieving stats");


        var api = await substrate_data.initApi(url);

        var nb_blocks = 0;
        var block = await api.rpc.chain.getBlock(startBlockHash); //init the block var


        var startblockno = parseInt(block.block.header.number.toString());

        var startblockTimeStampQuery = await api.query.timestamp.now.at(startBlockHash);
        var breakup = ""

        // Set to O if not yet found
        // Set to 1 if found
        var startBlockFound = 0
        var startblockTimeStamp

        var iterationLimit = 50
        var iterationcounter = 0
        var totaltxscounter = 0
        var totaltxs = 50000
        var finalblockTimeStamp
        for (var i = startblockno; iterationcounter <= iterationLimit; i++) {
            var blockHash = await api.rpc.chain.getBlockHash(i);
            // returns SignedBlock
            var signedBlock = await api.rpc.chain.getBlock(blockHash);
            var length = signedBlock.block.extrinsics.length
            if (length <= 1 && startBlockFound == 0) {
                iterationcounter += 1;
            } else {
                totaltxscounter += length;
                iterationcounter += 1;
                var finalblocktimeQuery = await api.query.timestamp.now.at(blockHash);
                finalblockTimeStamp = finalblocktimeQuery.toNumber();
                if (startBlockFound == 0) {
                    startblockTimeStamp = finalblockTimeStamp;
                    startBlockFound = 1
                }
                console.log("--------");
                console.log("BlockNo", i);
                console.log("ExtrinsicFound", length);
                console.log("--------");
                breakup += i + ": " + length + "| "
                if (totaltxscounter >= totaltxs) {
                    console.log("completed")
                    break;
                }
            }

        }
        console.log("breakup", breakup);
        console.log("firstTimeStamp", startblockTimeStamp);
        console.log("endTimeStamp", finalblockTimeStamp);
        console.log("totaltxfound", totaltxscounter);
        var timeDifference = (finalblockTimeStamp - startblockTimeStamp) / 1000
        var finalTPS = totaltxscounter / timeDifference
        var invalidTxs = totaltxs - totaltxscounter

        console.log("TimeDifferencen secs", timeDifference)
        console.log("Finalised TPS", finalTPS)
        if (invalidTxs > 0) {
            console.log("Invalid Transaction", invalidTxs)
        } else {
            console.log("Invalid Transaction", 0)
        }


        console.log("Done retrieving stats")
        callback()

    })()
}

async.series([
    do_stats
], () => {
    console.log("End main program")
    process.exit(0);
})