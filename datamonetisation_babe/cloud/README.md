# Substrate

Folder containing the files required to deploy Substrate in Kubernetes.

## Substrate local node

A complete example of substrate + SIM use case is in [./substrate-node-template](./substrate-node-template). It has two docker-compose files:

* single node example: [./substrate-node-template/docker-compose.yml](./substrate-node-template/docker-compose.yml)
* multi-node example: [./substrate-node-template/docker-compose-aura.yml](./substrate-node-template/docker-compose-aura.yml)

### Setup the multi-node substrate + SIM implementation

The best way to test the Substrate + SIM use case implementation is to run the multi-node example with the following command line:
```bash
docker-compose --project-name substrate_sim -f docker-compose-aura.yml up --scale substrate-peer=4
#note: the project name is required to get a fixed container name for peers. Ex: substrate_sim_substrate-peer_1
#The container name is used to point prometheus scraper to the right container
```

Next, to view and send transactions you'll use the polkadot-js telemetry browser application that will connect directly to your local multi-node setup:
[https://polkadot.js.org/apps/#/settings?rpc=ws://127.0.0.1:9944](https://polkadot.js.org/apps/#/settings?rpc=ws://127.0.0.1:9944)

**Attention**: Don't forget to add [additional types](./substrate-node-template/additional_types.json) in the settings of the polkadot-js telemetry app to read the crashes state corretly:
```json
{
  "CrashType": {
    "block_number": "BlockNumber",
    "data": "Vec<u8>"
  }
}
```

### Test the multi-node setup

To test the SIM module in substrate you have to send a few transactions in the following order:

1. As **sudo**, send a `sim` transaction with the action `storeFactory(factory_id)`, ex: Alice (is sudo), adds Bob as a factory.
Use: [https://polkadot.js.org/apps/#/sudo](https://polkadot.js.org/apps/#/sudo)
2. As `Bob`, send a `sim` **extrinsic** with the action `storeCar(car_id)`, ex: Bob (as a factory), adds Charlie as a car.
Use: [https://polkadot.js.org/apps/#/extrinsics](https://polkadot.js.org/apps/#/extrinsics)
3. As `Charlie`, send a `sim` **extrinsic** with the action `storeCrash(data_hash)`, ex: Charlie (as a car), adds a crash with Charlie as a data_hash=0xa591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e (=sha256sum of "Hello World").
4. Repeat X times step 3 to store new crashes...

**Note**: All the steps above can be executed in the polkadot-js telemetry browser app. 

### View the state

To actually view the data stored on-chain you can request state data in the polkadot-js telemetry browser app: [https://polkadot.js.org/apps/#/chainstate](https://polkadot.js.org/apps/#/chainstate)


### HTTP REST API for Substrate

I added at the last minute an HTTP REST API tool that allows us to communicate with one node using single HTTP requests.
The REST API is built by Parity and is called [substrate-api-sidecar](https://github.com/paritytech/substrate-api-sidecar).

More info about the endpoints: [https://paritytech.github.io/substrate-api-sidecar/dist/](https://paritytech.github.io/substrate-api-sidecar/dist/)

### TODO

* build a simple client (in C++ ?) to send benchmark transaction to Substrate