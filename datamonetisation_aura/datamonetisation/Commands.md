Reference:

Old Tuto:
https://www.shawntabrizi.com/substrate-collectables-workshop/#/Extras/Auction/creating-an-auction


Latest Tuto:
https://substrate.dev/substrate-how-to-guides/docs/tutorials/Kitties/basic-setup


Latest V2 Code:
https://github.com/substrate-developer-hub/substrate-node-template/blob/tutorials/kitties/pallets/kitties/src/lib.rs

Assume a Kitty as either a Data Bundle or Data Service with certain qualities:

Step 1: Sell or Buy data Bundle or Data Service

Step 2: Convert the direct data bundle or data service as an auction

Step 3: Integrate with an off chain worker to handle data collection or data service api

Step 4: Finish with benchmarking

Base:
https://substrate.dev/docs/en/tutorials/build-a-dapp/pallet 

Other Blog:
https://blog.knoldus.com/rpc-to-call-a-runtime-api-easily-in-substrate/ 

Substrate Template Used:
===========================
tag=monthly-2021-08#4d28ebeb

RunTime Module vs Smart contract?
=======================================
https://stackoverflow.com/questions/56040779/when-should-i-build-a-substrate-runtime-module-versus-a-substrate-smart-contract

Changed Files:
=======================

pallets/template/src/lib.rs
runtime/src/lib.rs

Oauth API PART
================
https://howtodoinjava.com/spring-boot2/oauth2-auth-server/

Radar_auth = contains the actual authorization part

Radar_resource = verifies the oauth token and retuns the data needed
Build: ./mvnw spring-boot:run

https://medium.com/swlh/stateless-jwt-authentication-with-spring-boot-a-better-approach-1f5dbae6c30f

keytool 
password = password

1) Denied Access
curl --get http://localhost:8080/user


2) Login to get JWT
curl -d "username=vehicle_oem&password=leat_vehicle" -H "Content-Type: application/x-www-form-urlencoded" -X POST http://localhost:8080/login

curl -d "username=vehicle_oem&password=leat_vehicle" -H "Content-Type: application/x-www-form-urlencoded" -X POST http://localhost:8080/assettoken

curl -d "username=radar_oem&password=leat_radar" -H "Content-Type: application/x-www-form-urlencoded" -X POST http://localhost:9090/assetservicetoken


3) Get the User details
curl --get http://localhost:8080/user --header "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJyZW5hdWx0IiwibmJmIjoxNjMxNzEzMTcyLCJleHAiOjE2MzE3OTk1NzIsInVzZXJJZCI6IjEiLCJhdXRob3JpdGllcyI6IlVTRVIiLCJ1c2VybmFtZSI6InJlbmF1bHQifQ.Ktsg_084LPg8KSZnKqdloRjdHBQzEeuBGAka8CHcrUIA6kubvMGBq03qWYKhUP-_FrBZKOd5eb2DuUt24K0TLcaM-meGNtUvSDU-0wVZIxEwgSTHbVZ2QRf9eNuSkcW7s1QHg29hzxZ2_f2KHNZWqVjSs4JxqExXYPkxLhidYT8d_22oLWcDMtnfdUZ6fhmsRZ-jV0h-sB_zV0z3dBY9ZNL_KduYhdCGzXpGPjfpieYJAieDqc2P1Gy2N1gk88eCKvsYAs011egSBmhRGy-fJuU_Y4rlvdxa5I6pjec_vvWBMVMxGtyxLjHCFn8VQW59DnUA5hEOVRzgv7l_IiZrvA"

5) Get the radar data
curl --get http://localhost:8080/user/api/radardata --header "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJyZW5hdWx0IiwibmJmIjoxNjMyNDAwNTQwLCJleHAiOjE2MzI0ODY5NDAsInVzZXJJZCI6IjEiLCJhdXRob3JpdGllcyI6IlVTRVIiLCJ1c2VybmFtZSI6InJlbmF1bHQifQ.a5E-Nag2NgQ4RYSsh2MSUQ4klJKE-ByYmkEZnaqKKHNS4ZNQ-vuPw1L3UYB_AQ2RYNtzVkJC6ZjIhS0KK-uKKkxrjkRQ-e3HySrvop_DScj6SROs75iPD1EhaaRnxRG7VKJkztWv9S2Kcx1IyulbweggzRxZfTVUqJu-V9xGIgQkd3LLqODpAoO_ZKKNFJX4CHR7GSKuenD_jjc5P3Tba40FFcIyLeldyKqknJ6dJfDxoC96ovp01Q3YtCHwfXeaOUK0bmnLDDzSNm1QGuQkAMWZ4heKFu3mywgW9tucqlpoyIeJtwmEVdF55wriduYJ9sTYr5l-7Ds2Mks5RxVg8A"

For Adding Dependencies:

https://github.com/shawntabrizi/substrate-module-template/blob/master/HOWTO.md#adding-new-dependencies

https://substrate.dev/docs/en/knowledgebase/runtime/macros#frame-v1-vs-v2

Latest Offchain example: https://github.com/paritytech/substrate/blob/master/frame/example-offchain-worker/src/lib.rs

Offchain example other: https://docs.rs/sp-runtime/2.0.0-rc4/src/sp_runtime/offchain/http.rs.html#198-201

Other misc help: https://substrate.dev/recipes/runtime-printing.html

https://docs.rs/sp-runtime/2.0.0-rc4/src/sp_runtime/offchain/http.rs.html#198-201

Parse Library: (c=Cannot be used as client library)
https://docs.rs/httparse/1.5.1/httparse/

ALL Tutorials:
==================
https://substrate.dev/awesome-substrate/

Reward Coin:
https://github.com/substrate-developer-hub/substrate-how-to-guides/blob/main/example-code/template-node/pallets/reward-coin/src/lib.rs#L1-L249

Locakable:
https://github.com/substrate-developer-hub/substrate-how-to-guides/blob/main/example-code/template-node/pallets/lockable-currency/src/lib.rs

Mint Token:
https://github.com/substrate-developer-hub/substrate-how-to-guides/blob/main/example-code/template-node/pallets/mint-token/src/lib.rs#L1-L130 

https://docs.serde.rs/serde_json/


TO DO: 
Proof
First Bid
Second Bid
Finalised Assets or Transferred

Store Token for Asset
Store Toekn for Asset Service

Similar for Assetservice

To Add Logic to move funds to else where and transfer it back to the owner (escrow account/ funds locking)

For monthly-2021-08: TAG
Follow Repo: substate_old
Compile successfully
==================================================
Updated RUST to latest.
==================================================
For monthly-2021-10
Followed the below REPO to make minor changes
==================================================
https://github.com/substrate-developer-hub/substrate-node-template/blob/jc/kitties-workshop/pallets/kitties/src/lib.rs

Some other Links:
=================
Hex Literal conversion: https://stackoverflow.com/questions/60983883/how-to-encode-the-hex-string-representation-of-an-account-id-in-substrate-using

Main Pallet Examples: https://github.com/paritytech/substrate/blob/master/frame/balances/src/lib.rs 
https://docs.substrate.io/v3/runtime/frame/#prebuilt-pallets

Extrinsic without transaction fee:
==================================
https://stackoverflow.com/questions/62780202/how-to-create-an-extrinsic-without-transaction-fee

SIM Pallet: https://github.com/lucgerrits/substrate-node-template/tree/5ef22f55fa88a7bb4d02f0918962304abef3b672/runtime

Transaction Byte Fees: https://githubmemory.com/repo/open-web3-stack/open-runtime-module-library/issues/433

SomeOtherExamples: https://github.com/substrate-developer-hub/substrate-node-template/blob/sl/seminar/pallet-basics/runtime/src/lib.rs

My bitBucket: https://bitbucket.org/cyrilnavessamuel/data_auction/src/master/

UTXO: https://hackernoon.com/building-a-blockchain-in-rust-and-substrate-a-step-by-step-guide-for-developers-kc223ybp

Front : To check: https://polkadot.js.org/apps/#/extrinsics
ToDo:
====
1) Sequence Diagram
2) Check Functionality
3) Deploy on TAS
4) Prepare Clients
5) Finish

POLKADOT JS APP:
docker run --rm -it --name polkadot-ui -e WS_URL=ws://127.0.0.1:9944 -p 80:80 jacogr/polkadot-js-apps:latest
FRONT END APPS:
================
https://github.com/polkadot-js/apps

yarn --> to build

yarn run start


============================
Cloud Deploy:
1) cloud/remote-accesss-benchmark/big_test.sh
2) cloud/cloud-deployments/delete-substrate-net.sh
Change Number of Nodes:
3) cloud/cloud-deployments/genNodeYaml.sh