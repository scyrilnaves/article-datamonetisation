#echo "Get the Shell for the Test Machine"
./rancher kubectl exec -it -n substrate-net $(./rancher kubectl -n substrate-net get pods | awk '/benchmark-/{printf $1}') -- bash

1) apt install -y git

2) git clone https://cyrilnavessamuel@bitbucket.org/cyrilnavessamuel/aurapalletvanilla.git
password: ATBB3rbLRHD7LjLFDgwfdcSdRE371D13DC7D
===============================================
3) docker login
https://hub.docker.com/
cyrilthese
aujourdhui

4) service docker start

5) ./deploycontainer.sh

6) docker pull cyrilthese/datamonetisation_auradual:latest

7) docker run -it cyrilthese/datamonetisation_auradual:latest ./node-template

prometheus url: prometheus:9090
annotations: no need to change but check the grafana id it is susceptible to change.
________________________________

http://vehicleapi.unice.cust.tasfrance.com/assettoken
http://radarapi.unice.cust.tasfrance.com/assetservicetoken


http://radarapi.unice.cust.tasfrance.com/getPublicKey
http://radarapi.unice.cust.tasfrance.com/isDeployed

http://vehicleapi.unice.cust.tasfrance.com/getPublicKey
http://vehicleapi.unice.cust.tasfrance.com/isDeployed

http://vehicleapi.unice.cust.tasfrance.com/deployementStatus
http://radarapi.unice.cust.tasfrance.com/deployementStatus



http://vehicleapi.unice.cust.tasfrance.com/actuator/mappings
http://radarapi.unice.cust.tasfrance.com/actuator/mappings

localhost:8080/getPublicKey
localhost:8080/actuator/mappings

Spring Boot JAR
==================
docker pull cyrilthese/oem-api
docker run -it -p 8080:8080 cyrilthese/oem-api
_______________
docker pull cyrilthese/radar-api
docker run -it -p 8080:8080 cyrilthese/radar-api

================
Make sure to note Grafana Id while doing deployment and analysis
Ingress for POST Request added in the REPO

=====================
DEMO:
Deploy:
cloud/asset_data_transaction/perfexecution
./startOnlyDeploy.sh
------------------------
cloud/asset_data_transaction/multi
npm install

npm dedupe : doesnt work

https://github.com/polkadot-js/apps/blob/master/package.json
add resolutions in package.json to fix the conflicting package error
=================
Export All Data
-------------------
pip3 install pandas
sudo apt-get install python-numpy python-scipy python-matplotlib
pip3 install numpy
pip3 install matplotlib
pip3 install scipy
pip3 install SciencePlots

=============================
Important URL:
const HTTP_REMOTE_REQUEST_ASSET: &str = "http://vehicleapi.unice.cust.tasfrance.com/assettoken";
const HTTP_REMOTE_REQUEST_ASSET_VALID: &str ="http://vehicleapi.unice.cust.tasfrance.com/assettoken";
const HTTP_REMOTE_REQUEST_ASSET_SERVICE: &str ="http://radarapi.unice.cust.tasfrance.com/assetservicetoken";
const HTTP_REMOTE_REQUEST_ASSET_SERVICE_VALID: &str ="http://radarapi.unice.cust.tasfrance.com/assetservicetoken";

docker run --rm -it --name polkadot-ui -e WS_URL=ws://substrate-ws.unice.cust.tasfrance.com -p 80:80 jacogr/polkadot-js-apps:latest

localhost:80
================================
Test:
1) Make sure to build the 4 docker containers:
Blockchain Node: under datamarket/v4_radar_monetisation : ./deploycontainer.sh
Client: under datamarket/cloud/asset_data_transaction/perfclient: ./deploycontainer.sh
Vehicle_OEM: under datamarket/oem_mw : ./buildndeploy.sh
Radar_OEM: under datamarket-new/datamarket/radar_mw: ./buildndeploy.sh
Asumming Ingress and Grafana is configured

2) For Demo use datamarket/cloud/asset_data_transaction/multiflow
execute node

3) For test Start the deployement and test
under /datamarket/cloud/asset_data_transaction/perfexecution: ./start_test.sh
Get the start time and end time for the tags

4) For analysis
under /datamarket/cloud/asset_data_transaction/perfanalysis: ./export_all_data.sh starttimeinmillisecs endtimeinmillsecs

========== FINISH ==============
cargo build --release
=========
./target/release/node-template --tmp --dev --port 30333 --ws-port 9944 --ws-external
=========
// To format code
cargo fmt
=========

--pool-kbytes 100000 --pool-limit 100000    

Maximum number of kilobytes of all transactions stored in the pool [default: 20480]
--pool-limit Maximum number of transactions in the transaction pool [default: 8192]


https://substrate.stackexchange.com/questions/4769/getting-unknownblock-state-already-discarded-while-fetch-block-data-from-vali
--blocks-pruning
--state-pruning 600 \\

Run finalstats.js to get the actual results
--------------------
Issues in Babe: Refer attached DOC : BABE_Issues

-------------
docker system prune --volumes 
:clean all unused files