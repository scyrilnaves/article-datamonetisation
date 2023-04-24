#!/usr/bin/env node

import substrate_data from "./substrate_data_lib.js";
import * as child from 'child_process';
import * as path from 'path';
import * as async from "async";

if (process.argv.length <= 3) {
    console.error("Required 2 argument: \n\tlimit, ex: 10000\n\ttx/sec, ex: 10 (tx/sec)\n\tnb_processes, ex: 10 (optional)")
    process.exit(1);
}

//PROD URL
const url = "ws://substrate-ws.unice.cust.tasfrance.com";
// DEV URL
//const url = "ws://127.0.0.1:9944";

const limit = parseInt(process.argv[2]);
const nb_processes = process.argv[4] ? parseInt(process.argv[4]) : 14; //parseInt(process.argv[4]);
const wait_time = (nb_processes / parseFloat(process.argv[3])) * 1000;// parseFloat(process.argv[3]) * 1000;
var processes_arr = [];
var processes_exited = 0;
var processes_finished = 0;
var processes_init_ok = 0;
var processes_prepare_ok = 0;

var tot_success = 0;
var tot_failed = 0;
var tot_finished = 0;
var tot_prepared_finished = 0;

var firstBlock = {
    hash: '0x95f9902f60296a61e9fc5b687475ee53f0ecfc4f722996515a314d4b0eadfd22',
    number: '60906'
};
var lastBlock = {
    hash: '0x313031342394db57b3f2756c8a6c2979defad17583b7b1a8ef7bf379e26408b9',
    number: '60909'
};
var start_time;
var stop_time;

function do_benchmark(callback) {
    console.log("Benchmark settings:")
    // console.log("\t", nb_processes * (1 / parseFloat(process.argv[3])), "Tx/sec")
    console.log("\t", parseFloat(process.argv[3]), "Tx/sec")
    console.log("\t wait_time=", wait_time, "ms (in each thread)")
    console.log("\t nb threads=", nb_processes, "threads")
    console.log("Start processes")
    // Create the worker.
    for (let i = 0; i < nb_processes; i++) {
        processes_arr[i] = child.fork(path.join(".", "/sender.js"), [i, nb_processes]);


        //handle messages
        processes_arr[i].on('message', async (message) => {
            if (message.cmd == "init_worker") { //first init all workers
                processes_arr[i].send({ cmd: "init" });
            }
            else if (message.cmd == "init_ok") { //wait all processes init ok
                processes_init_ok++;
                if (processes_init_ok == nb_processes) { //all processes ready
                    //start send all processes 
                    console.log("All processes synced")
                    console.log(limit)
                    for (let j = 0; j < nb_processes; j++)
                        processes_arr[j].send({ cmd: "prepare", limit: parseInt(limit / nb_processes) }); //start send
                }
            }
            else if (message.cmd == "prepare_ok") {
                processes_prepare_ok++;
                if (processes_prepare_ok == nb_processes) { //all processes ready
                    //start send all processes 
                    console.log("All processes prepared")
                    console.log(`Total prepared finished: ${tot_prepared_finished}`)
                    await substrate_data.sleep(5000); //wait a little

                    processes_arr[0].send({ cmd: "get_head", type: "first" }); //get first block
                }
            }
            else if (message.cmd == "get_head_ok" && message.type == "first") { //if we got first block (start sending)
                firstBlock = {
                    hash: message.hash,
                    number: message.number
                }
                start_time = new Date();
                for (let j = 0; j < nb_processes; j++)
                    processes_arr[j].send({ cmd: "send", wait_time: wait_time }); //start send
            }
            else if (message.cmd == "get_head_ok" && message.type == "last") { //if we got last block (exit process)
                lastBlock = {
                    hash: message.hash,
                    number: message.number
                }
                for (let j = 0; j < nb_processes; j++)
                    processes_arr[j].send({ cmd: "exit" }); //exit when done
            }
            else if (message.cmd == "prepare_stats") {
                tot_prepared_finished += message.finished;
            }
            else if (message.cmd == "send_stats") {
                // console.log(message)
                tot_success += message.success;
                tot_failed += message.failed;
                tot_finished += message.finished;
            }
            else if (message.cmd == "send_ok") {
                processes_finished++;
                if (processes_finished == nb_processes) { //all processes finished
                    //start send all processes 
                    console.log("All processes finished")
                    stop_time = new Date();
                    await substrate_data.sleep(18000); //wait a little

                    processes_arr[0].send({ cmd: "get_head", type: "last" }); //get last block
                }
            }
        });


        //handle exit
        processes_arr[i].on('exit', async () => {
            //proccess exit
            processes_exited++;
            if (processes_exited == nb_processes) {
                //if all processes exited -> stop main process
                console.log("All processes exited.")
                console.log("Done main worker")

                console.log(`Total success: ${tot_success}`)
                console.log(`Total failed: ${tot_failed}`)
                console.log(`Total finished: ${tot_finished}`)

                await substrate_data.sleep(10000); //wait a little
                callback()
            }
        });
    }
}

function do_stats(callback) {
    (async () => {
        console.log("Retrieving stats");

        console.log("firstBlock", firstBlock)
        console.log("lastBlock", lastBlock)

        var api = await substrate_data.initApi(url);

        await substrate_data.print_header(api);

        var nb_blocks = 0;
        var current_hash = lastBlock.hash; //init the current_hash var
        var block = await api.rpc.chain.getBlock(current_hash); //init the block var
        var stop_timestamp = new Date(parseInt(block.block.extrinsics[0].method.args.toString().slice(0, 10)));
        var tot_tx = 0;
        while (firstBlock.hash !== current_hash) {
            tot_tx += block.block.extrinsics.length - 1;
            block = await api.rpc.chain.getBlock(current_hash);
            current_hash = block.block.header.parentHash.toString();
            nb_blocks++;
            process.stdout.write(".");
        }

        process.stdout.write("\n");
        var start_timestamp = new Date(parseInt(block.block.extrinsics[0].method.args.toString().slice(0, 10)));
        console.log("stop_timestamp ", stop_timestamp)
        console.log("start_timestamp ", start_timestamp)
        var date_diff = (stop_timestamp - start_timestamp);
        var time_diff = (stop_time - start_time);
        console.log("Done in " + date_diff + " sec");
        console.log("Done in " + time_diff + " sec");
        console.log("Transactions " + tot_tx)
        console.log("Transactions/sec " + tot_tx / date_diff)
        console.log("Blocks " + nb_blocks)
        console.log("Done retrieving stats")
        callback()
    })()
}

async.series([
    do_benchmark,
    do_stats
], () => {
    console.log("End main program")
    process.exit(0);
})