To get shell in benchmark pod

# ./rancher kubectl exec -it -n substrate-net $(./rancher kubectl -n substrate-net get pods | awk '/benchmark-/{printf $1}') -- /bin/bash 

# ensure to add substrate-ws.unice.cust.tasfrance.com 185.52.32.4 in /etc/hosts

# docker run --rm -it --name polkadot-ui -e WS_URL=ws://substrate-ws.unice.cust.tasfrance.com -p 80:80 jacogr/polkadot-js-apps:latest

docker run --rm -it --name polkadot-ui -e WS_URL=ws://127.0.0.1:9944 -p 80:80 jacogr/polkadot-js-apps:latest

#  http://localhost:80

cargo build --release

// To format code
cargo fmt

./target/release/node-template --tmp --dev --port 30333 --ws-port 9944 --ws-external --rpc-cors all -l txpool=trace,sync=info

./target/release/node-template --tmp --dev --port 30333 --ws-port 9944 --ws-external

cd /home/renault/Documents/code/substrate/datamarket-new/datamarket/cloud/data-transaction-js/bin/ws

delete node_modules package.json
rm -rf node_modules/
rm -rf package.json

npm init
npm install @polkadot/api --save
npm install @polkadot/keyring --save
npm install @polkadot/util --save
npm install axios --save
npm install qs --save

add  "type": "module", to package.json

node initialise_datapallet.js

node asset_tx_simple.js

node asset_service_tx_simple.js

./mvnw spring-boot:run
---------------------------------------------

https://github.com/paritytech/substrate/blob/master/bin/node/cli/src/chain_spec.rs
https://substrate.stackexchange.com/questions/368/how-to-change-consensus-engine-from-poa-to-pos
https://github.com/kaichaosun/substrate-stencil/commits/master
https://github.com/kaichaosun/substrate-stencil/commit/593e9bf845ee094ff642cef26de8cafafbe4efc9#diff-594440db8adfc5d1b37711309312bfa06b49e488309186ad062a896778cf9fbf
distributed systems sukumar ghosh as reference



---------------------------------------------

https://github.com/paritytech/substrate/tree/polkadot-v0.9.28/frame/examples/offchain-worker

https://github.com/paritytech/substrate/blob/polkadot-v0.9.28/bin/node/runtime/src/lib.rs

https://docs.substrate.io/reference/how-to-guides/offchain-workers/offchain-transactions/

-----------------------------------------------
cd  ~/.cargo
rm -rf ~/.cargo
https://docs.substrate.io/install/linux/

https://substrate.recipes/off-chain-workers/storage.html

https://github.com/JoshOrndorff/recipes/blob/master/text/off-chain-workers/storage.md

----------------------
Radar_mw and OEM_mw are duplicates: No need to change
Password defined in Security Config
----------------------
BA:
https://github.com/paritytech/substrate/blob/master/bin/node/cli/src/chain_spec.rs
https://substrate.stackexchange.com/questions/368/how-to-change-consensus-engine-from-poa-to-pos
https://github.com/kaichaosun/substrate-stencil/commits/master
https://github.com/kaichaosun/substrate-stencil/commit/593e9bf845ee094ff642cef26de8cafafbe4efc9#diff-594440db8adfc5d1b37711309312bfa06b49e488309186ad062a896778cf9fbf

Version4: Update
---------------------
rm -rf .git in subfolder
git rm --cached substrate-node-template
Babe wrk by and then updated to latest commit of it
===========================
V3 to V4 Pallette:
1) Added Necessary Dependency
2) Remove Value Query for Default Trait Problem in AccountId for Storage
3) Self::escrowaccount(0) --> Self::escrowaccount(0).unwrap()
4) Genesis config ---> remove Admin
5) the trait `EncodeLike<<T as frame_system::Config>::AccountId>` is not implemented for `std::option::Option<<T as frame_system::Config>::AccountId>` 
ERROR::::
#[scale_info(skip_type_params(T))] 
#[codec(mel_bound())] 
Added over Struct
--https://substrate.stackexchange.com/questions/619/how-to-fix-parity-scale-codecmaxencodedlen-is-not-implemented-for-t?rq=1
6) Another over pallet: #[pallet::without_storage_info]
https://stackoverflow.com/questions/70206199/substrate-tutorials-trait-maxencodedlen-is-not-implemented-for-vecu8
7) https://github.com/paritytech/subport/issues/150 [default features=false]
8) feeless: pallet_transaction_payments https://substrate.stackexchange.com/questions/1713/how-we-can-implement-gasless-feeless-transaction/1715

----------------------------------------------------------
Nice but not used: https://sequencediagram.org/

Reference: https://plantuml.com/sequence-diagram

Used: http://www.plantuml.com/plantuml/uml/hLJ1RXCn4BtxAmRr0b5oWDuYKZMR5hYKLct51Q5gQayIgruxihqj_NkywmLxKe90GiebC_FUl9attjCGqSUkDU9Lt59BOFHc6pSUdqNKqH0iGw085yvIr06ZTbO8bD5vECE85-WVKlhetdGqXh_GLaj3TvVd5tTBEDfu8lkRLCDJrwZG_yGyap7kwKMcOPb2bn3ttZtjTsbu-3p8z0-Ed4Ux8I4MYs42EACMhOARVW-mM8XAoUphCDYtsKcPTVwPI6LH0rvO5mdS8x4PfXKWjq4fdLVQRY0wC4uEgvp2xVEEfd2pwqdTUjIU8bTUIqgHt4CKkja9t_0S6kzGhJ34Ufx3YEOGGZ3UznKiDVb84LRE1h8HrgFvqDPNmNhKwEOlqFjkd5lLUv72um5YW5ltG1OkrsloQOt5C2QRMRNoX94omzsoSFGIufR6PfM35HNtC8l8V8fxARsEv3L2hrlRgDkX9hhw7lXs3eji_b5ayrz57jvsRKYZLIykbnORNgOEctZhqOR4pD5NMmpqTWuVEgjA2Sk-cyELTsuDwLVUOEaPUFbn8jCMpNXb2PNB63ihJmFBBUfA8k-qb1J6AJBsJzFb1PI7bOpdB_66hFgLNQOiVzavpS-xyciEroH9Fz9-JVxKPVPo7_lqryIn-JBhxsrf80whL1w74sjTgdjvfSifBryPBH_W4vaTj6IxmRh9HDiGVITxay9aqbzf2j9tR7U6Kg3k_lhu3MUqpivwFDhq8zU5H1m2RaYSabLTQtu0