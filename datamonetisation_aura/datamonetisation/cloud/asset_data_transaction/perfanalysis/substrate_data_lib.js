//
//A lib to simplify my life with our use case and Substrate
//
import { Keyring } from '@polkadot/keyring';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { createHash, randomBytes } from 'crypto';
import { cryptoWaitReady, mnemonicGenerate } from '@polkadot/util-crypto';
import { readFileSync, existsSync } from 'fs'
const filename_accounts = "accounts.json"
const filename_admins = "admins.json"

// init keyring
var keyring;
// init alice and charlie accounts
var alice;
var bob;
var charlie;
var ACCOUNT_PAIRS = {};
var ADMINS_ACCOUNT_PAIRS = {};
var ADMINS_ACCOUNT_PAIRS_NONCES = {};
var FACTORIES_ACCOUNT_PAIRS_NONCES = {};

async function get_balance(api, address) {
    let { data: balance } = await api.query.system.account(address);
    return balance.free.toBigInt();
}

async function get_sudo_keyPair(api) {
    let key = await api.query.sudo.key();
    return keyring.getPair(key.toString());
}

async function get_admin(api, admin_address) {
    let data = await api.query.templateModule.assetAdmin(admin_address);
    return data;
}

async function get_cars(api) {
    let data = await api.query.simModule.cars.keys();
    let a = data.map(({ args: [carId] }) => carId);
    return a;
}

async function get_car(api, car_id) {
    let data = await api.query.simModule.cars(car_id);
    return data;
}

async function get_crashes(api) {
    let data = await api.query.simModule.crashes.keys();
    let a = data.map(({ args: [carId] }) => carId);
    return a;
}

async function get_crash(api, car_id) {
    let data = await api.query.simModule.crashes(car_id);
    return data;
}

function getKeyring() {
    return keyring;
}

var substrate_data = {
    initApi: async function (url) { //if a process if given then we only get a piece of the accounts        
        // Construct
        const wsProvider = new WsProvider(url);

        await cryptoWaitReady();
        keyring = new Keyring({ type: 'sr25519' });

        // init alice and charlie accounts
        alice = getKeyring().addFromUri('//Alice', { name: 'Alice default' });
        bob = getKeyring().addFromUri('//Bob', { name: 'Bob default' });
        charlie = getKeyring().addFromUri('//Charlie', { name: 'Charlie default' });

        // substrate_data.accounts.makeAll(process_id, tot_processes) //init all accounts

        const api = await ApiPromise.create({ provider: wsProvider });

        ACCOUNT_PAIRS[0] = [];
        ADMINS_ACCOUNT_PAIRS[0] = [];
        return api;
    },
    accounts: {
        alice: () => { return alice; },
        bob: () => { return bob; },
        charlie: () => { return charlie; },
        genMnemonic: () => {
            return mnemonicGenerate();
        },
        genFromMnemonic: (mnemonic, name) => {
            //TODO: maybe use createFromUri ???
            // https://polkadot.js.org/docs/keyring/start/create/#revisiting-crypto
            return getKeyring().createFromUri(mnemonic, { name: name }, 'sr25519');
        },
        makeAll: (process_id = -1, tot_processes = -1) => {
            substrate_data.accounts.makeAllFactories(process_id, tot_processes);
            substrate_data.accounts.makeAllAccounts(process_id, tot_processes);
        },
        makeAllAdmins: (process_id = -1, tot_processes = -1) => {
            if (existsSync(filename_admins)) {
                var file_content = readFileSync(filename_admins, 'utf-8');
                try {
                    var file_json_arr = JSON.parse(file_content);
                    var tot_accounts = file_json_arr.length;

                    if (process_id == -1 || tot_processes == -1) {
                        console.log("Loading all admin accounts...");
                        ADMINS_ACCOUNT_PAIRS[0] = [];
                        ADMINS_ACCOUNT_PAIRS_NONCES[0] = [];
                        for (let i = 0; i < tot_accounts; i++) {
                            // if (i % 500 == 0)
                            //     console.log(`\n${(i * 100) / tot_accounts}%`)
                            //console.log('pushing');
                            //console.log(file_json_arr[i]);
                            ADMINS_ACCOUNT_PAIRS[0].push(substrate_data.accounts.genFromMnemonic(file_json_arr[i], `Admin Account ${i}`))
                            ADMINS_ACCOUNT_PAIRS_NONCES[0].push(0); //init 0
                        }
                    } else {
                        var account_count = parseInt(tot_accounts / tot_processes);
                        var account_start_index = process_id;
                        ADMINS_ACCOUNT_PAIRS[process_id] = [];
                        ADMINS_ACCOUNT_PAIRS_NONCES[process_id] = [];
                        for (let i = (account_start_index * account_count); i < ((account_start_index + 1) * account_count); i++) {
                            ADMINS_ACCOUNT_PAIRS[process_id].push(substrate_data.accounts.genFromMnemonic(file_json_arr[i], `Admin Account ${i}`))
                            ADMINS_ACCOUNT_PAIRS_NONCES[process_id].push(0); //init 0
                        }
                    }
                } catch (e) {
                    console.log(e);
                    console.log("Can't load Admin account file: " + filename_admins);
                }

            } else {
                console.error(`${filename_factories} doesn't exist. Use genAccounts.js to build one.`)
                process.exit(1);
            }
        },
        makeAllAccounts: (process_id = -1, tot_processes = -1) => {
            if (existsSync(filename_accounts)) {
                var file_content = readFileSync(filename_accounts, 'utf-8');
                try {
                    var file_json_arr = JSON.parse(file_content);
                    var tot_accounts = file_json_arr.length;

                    if (process_id == -1 || tot_processes == -1) {
                        console.log("Loading all accounts...");
                        ACCOUNT_PAIRS[0] = [];
                        ACCOUNT_PAIRS_NONCES[0] = [];
                        for (let i = 0; i < tot_accounts; i++) {
                            // if (i % 500 == 0)
                            //     console.log(`\n${(i * 100) / tot_accounts}%`)
                            ACCOUNT_PAIRS[0].push(substrate_data.accounts.genFromMnemonic(file_json_arr[i], `Account ${i}`))
                            ACCOUNT_PAIRS_NONCES[0].push(0); //init 0
                        }
                    } else {
                        var account_count = parseInt(tot_accounts / tot_processes);
                        var account_start_index = process_id;
                        ACCOUNT_PAIRS[process_id] = [];
                        ACCOUNT_PAIRS_NONCES[process_id] = [];
                        for (let i = (account_start_index * account_count); i < ((account_start_index + 1) * account_count); i++) {
                            ACCOUNT_PAIRS[process_id].push(substrate_data.accounts.genFromMnemonic(file_json_arr[i], `Account ${i}`))
                            ACCOUNT_PAIRS_NONCES[process_id].push(0); //init 0
                        }
                    }
                } catch (e) {
                    console.log(e);
                    console.log("Can't load account file: " + filename_accounts);
                }
            } else {
                console.error(`${filename_accounts} doesn't exist. Use genAccounts.js to build one.`)
                process.exit(1);
            }
        },
        getAllAccounts: (process_id = -1) => {
            if (process_id == -1)
                return ACCOUNT_PAIRS[0];
            return ACCOUNT_PAIRS[process_id];
        },
        getAllAdmins: (process_id = -1) => {
            if (process_id == -1)
                return ADMINS_ACCOUNT_PAIRS[0];
            return ADMINS_ACCOUNT_PAIRS[process_id];
        },
        getAllAdminsNonces: (process_id = -1) => {
            if (process_id == -1)
                return ADMINS_ACCOUNT_PAIRS_NONCES[0];
            return ADMINS_ACCOUNT_PAIRS_NONCES[process_id];
        },
        getAllFactoriesNonces: (process_id = -1) => {
            if (process_id == -1)
                return FACTORIES_ACCOUNT_PAIRS_NONCES[0];
            return FACTORIES_ACCOUNT_PAIRS_NONCES[process_id];
        }
    },
    print_admins: async function (api, admin_address) {
        var admin = await get_admin(api, admin_address);
        if (admin.words && admin.words[0] == 0) {
            console.log(`[+] ${admin_address} is NOT a Admin.`)
            return false;
        } else {
            let admin_blockHash = await api.rpc.chain.getBlockHash(admin.words[0]);
            console.log(`[+] ${admin_address} is a Admin.\n\tAdded in block number: ${admin.words[0]}\n\tBlock Hash: ${admin_blockHash}"`);
            return true;
        }
    },
    print_cars: async function (api, expand = false) {
        var cars = await get_cars(api);
        console.log(`[+] Stored cars: ${Object.keys(cars).length}`);
        if (expand) {
            for (const [key, car_id] of Object.entries(cars)) {
                let car = await get_car(api, car_id);
                console.log(`[+] Car: ${car_id}\n\tAdded by: ${car[0]}\n\tJoined on block: ${car[1]}`);
            }
        }
    },
    print_crashes: async function (api) {
        var crashes = await get_crashes(api);
        console.log(`[+] Stored cars that had crashed at least once: ${Object.keys(crashes).length}`);

        for (const [key, car_id] of Object.entries(crashes)) {
            let car_crashes = await get_crash(api, car_id);
            // console.log(`[+] Car: ${car_id} \n\t${car_crashes}`);
            console.log(`[+] Car: ${car_id}`);
            console.log(`\t-Total crashes: ${car_crashes.length}`);
            for (const crash of car_crashes) {
                console.log(`\t\tCrashed at block: ${crash.block_number}`);
                console.log(`\t\tData hash ${crash.data}\n`);
            }
        }
    },
    print_header: async (api, process_id = -1, tot_processes = -1) => {
        // Retrieve the last timestamp
        const now = await api.query.timestamp.now();
        const now_human = new Date(now.toNumber()).toISOString();
        // Genesis Hash
        const genesisHash = api.genesisHash.toHex()
        const last_block = await api.rpc.chain.getHeader();
        console.log("---------------------------------------------------------------------------------------------");
        console.log(`[+] Genesis Hash: ${genesisHash}`);
        console.log(`[+] Node time: ${now} (${now_human})`);
        console.log(`[+] Last block: ${last_block.number.toString()} -> ${last_block.hash.toString()}`);
        console.log(`[+] Accounts:`);
        console.log(`\t Alice:\t\t${substrate_data.accounts.alice().address}`);
        console.log(`\t Bob:\t\t${substrate_data.accounts.bob().address}`);
        console.log(`\t Charlie:\t${substrate_data.accounts.charlie().address}`);
        //console.log(`[+] Auto generated accounts: ${substrate_data.accounts.getAllAccounts(process_id).length}`);
        //console.log(`[+] Auto generated factories accounts: ${substrate_data.accounts.getAllAdmins(process_id).length}`);
        console.log("---------------------------------------------------------------------------------------------");

    },
    send: {
        prepare_new_car_crash: async function (api, car, nonce = -1, verbose = false) {
            // const nonce = await api.rpc.system.accountNextIndex(car.address);

            // var rnd_bytes = randomBytes(32);
            //process.hrtime().toString()
            // var data = "Hello world:" + rnd_bytes.toString("hex");
            var data = "Hello world";
            const hash = createHash('sha256');
            hash.update(data);
            var data_sha256sum = hash.digest();
            // console.log(`Hash "${data}" = ${data_sha256sum.toString("hex")}`)
            // Sign and send a new crash from Bob car
            const tx = api.tx.simModule.storeCrash(data_sha256sum.buffer.toString())
            // const tx_signed = tx.sign(car, { nonce: nonce , era: 0 });
            const tx_signed = await tx.signAsync(car, { nonce: nonce, era: 0 });
            if (verbose)
                console.log(`Transaction sent: ${tx}`);
            return tx_signed;
        },
        new_car_crash: async function (api, car, nonce = -1, verbose = false) {
            // const nonce = await api.rpc.system.accountNextIndex(car.address);

            // var rnd_bytes = randomBytes(32);
            //process.hrtime().toString()
            // var data = "Hello world:" + rnd_bytes.toString("hex");
            var data = "Hello world";
            const hash = createHash('sha256');
            hash.update(data);
            var data_sha256sum = hash.digest();
            // console.log(`Hash "${data}" = ${data_sha256sum.toString("hex")}`)
            // Sign and send a new crash from Bob car
            const tx = await api.tx.simModule
                .storeCrash(data_sha256sum.buffer.toString())
                .signAndSend(car,
                    { nonce: nonce },
                );
            if (verbose)
                console.log(`Transaction sent: ${tx}`);
            return tx;
        },
        new_escrow: async function (api) {
            // https://polkadot.js.org/docs/api/start/api.tx.wrap/#sudo-use
            // const sudoPair = await get_sudo_keyPair(api);

            const alicepair = substrate_data.accounts.alice();
            // const { nonce } = await api.query.system.account(sudoPair.address);
            // console.log(`nonce: ${nonce}`)
            let tx = await api.tx.templateModule.addEscrow().signAndSend(alicepair, { nonce: -1 });
            //console.log(`Transaction sent: ${tx}`);
            return tx;
        },
        new_asset_service_admin: async function (api) {
            // https://polkadot.js.org/docs/api/start/api.tx.wrap/#sudo-use
            // const sudoPair = await get_sudo_keyPair(api);

            const alicepair = substrate_data.accounts.alice();
            // const { nonce } = await api.query.system.account(sudoPair.address);
            // console.log(`nonce: ${nonce}`)
            let tx = await api.tx.templateModule.addAssetServiceAdmin(alicepair.address).signAndSend(alicepair, { nonce: -1 });
            //console.log(`Transaction sent: ${tx}`);
            return tx;
        },
        new_asset_admin: async function (api, asset_admin) {
            // https://polkadot.js.org/docs/api/start/api.tx.wrap/#sudo-use
            // const sudoPair = await get_sudo_keyPair(api);

            const alicepair = substrate_data.accounts.alice();
            // const { nonce } = await api.query.system.account(sudoPair.address);
            // console.log(`nonce: ${nonce}`)
            let tx = await api.tx.templateModule.addAssetAdmin(asset_admin.address).signAndSend(alicepair, { nonce: -1 });
            //console.log(`Transaction sent: ${tx}`);
            return tx;
        },
        create_asset: async function (api, assetadmin, nonce = -1, verbose = false) {
            // Sign and create a new asset
            // Price, Criteria, Validation, Price Threshold, Privacy, PubKey
            const createasset_tx = await api.tx.templateModule.createAsset(600, 200, 1, 550, 1, '0x64ff825edd1f9accd746861f664c1fd2d508a705abe9559e76e9af512675c766');
            // const tx_signed = tx.sign(car, { nonce: nonce , era: 0 });
            //console.log(`Transaction strat`);
            //console.log(assetadmin.address);
            const tx_signed = await createasset_tx.signAsync(assetadmin, { nonce: nonce, era: 0 });
            if (verbose)
                console.log(`Transaction sent: ${tx_signed}`);
            return tx_signed;
        },
        new_car: async function (api, factory, car, nonce = -1) {
            let tx = await api.tx.simModule
                .storeCar(car.address)
                .signAndSend(factory,
                    { nonce: nonce },
                );
            // console.log(`Transaction sent: ${tx}`);
            return tx;
        },
        // new_owner: async function (api, sudo_account, car) {
        //     // https://polkadot.js.org/docs/api/start/api.tx.wrap/#sudo-use
        //     let tx = await api.tx.sudo(
        //         api.tx.simModule.storeCar(car.address)
        //     ).signAndSend(sudo_account, { nonce: -1 });
        //     console.log(`Transaction sent: ${tx}`);
        // }
    },
    sleep: function (ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    },
    getLastBlock: async (api) => {
        return await api.rpc.chain.getHeader();
    }
}

export default substrate_data = substrate_data