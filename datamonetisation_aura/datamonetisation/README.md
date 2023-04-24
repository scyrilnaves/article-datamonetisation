Commands to execute:
========================

cargo build --release

Link:
=========
https://corporatefinanceinstitute.com/resources/knowledge/other/vickrey-auction/

// To format code
cargo fmt

//https://wiki.polkadot.network/docs/build-node-management

./target/release/node-template --tmp --dev --port 30333 --ws-port 9944 --ws-external --rpc-cors all -l txpool=trace,sync=info

./target/release/node-template --tmp --dev --port 30333 --ws-port 9944 --ws-external

// complex transaction: https://polkadot.js.org/docs/api/start/api.tx.wrap/

./mvnw spring-boot:run

cd apps

Build: yarn

Start: yarn run start


Only for building pallet:
============================
cargo build -p pallet-template

=====================================================================
Alternative:
============
./target/release/node-template --chain=local --base-path /tmp/validator1 --alice --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a --port 30333 --ws-port 9944 --ws-external --rpc-cors all

FrontEnd:

Better: https://github.com/polkadot-js/apps.git

Alternate: https://github.com/substrate-developer-hub/substrate-front-end-template.git

https://polkadot.js.org/apps/#/extrinsics

// No hardlining Asset and Asset Service Admin ( can be introperable only Radar OEM or Vehicle OEM)
Transaction Seaquence
No |           Actor                     | Function                                             |      Parameters                           | Times
1  |        Admin(Alice)                 | Add Escrow                                                                                       | 1
2  |        Escrow (Alice)               | Add Asset Admin (Bob)                                                                            | 1
3  |        Escrow (Alice)               | Add Asset Service Admin (Charlie)                                                                | 1
4  |        Asset Admin (Bob)            | Add Vehicle (Dave)                                                                               | 1
5  |        Asset Service Admin(Charlie) | Publish an Asset Demand Event                        | Asset Price, Criteria                     | 1
6  |        Asset Admin (Bob)            | Create an Asset Demand                               | Asset Price, Criteria, Threshold Price    | 1
7  |        Asset Service Admin(Charlie) | Buy Asset                                            | Asset Id, Buy Price                       | 1
8  |        Vehicle(Dave)                | Submit Asset Proof + Offchain Data Transfer to Cloud | Asset Id, Asset Proof                     | No of Vehicles
9  |        




To get shell in benchmark pod

# ./rancher kubectl exec -it -n substrate-net $(./rancher kubectl -n substrate-net get pods | awk '/benchmark-/{printf $1}') -- /bin/bash 

# ensure to add substrate-ws.unice.cust.tasfrance.com 185.52.32.4 in /etc/hosts

# docker run --rm -it --name polkadot-ui -e WS_URL=ws://substrate-ws.unice.cust.tasfrance.com -p 80:80 jacogr/polkadot-js-apps:latest

docker run --rm -it --name polkadot-ui -e WS_URL=ws://127.0.0.1:9944 -p 80:80 jacogr/polkadot-js-apps:latest

#  http://localhost:80

Build transaction JS

npm config rm http-proxy
npm config rm proxy
npm config rm https-proxy
npm install --verbose

./bin/genAccounts.js 10000 10