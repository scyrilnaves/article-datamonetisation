node-template 4.0.0-dev-e32038af99b
Substrate DevHub <https://github.com/substrate-developer-hub>
A fresh FRAME-based Substrate node, ready for hacking.

USAGE:
    node-template [OPTIONS]
    node-template <SUBCOMMAND>

OPTIONS:
        --alice
            Shortcut for `--name Alice --validator` with session keys for `Alice` added to keystore

        --allow-private-ipv4
            Always accept connecting to private IPv4 addresses (as specified in
            [RFC1918](https://tools.ietf.org/html/rfc1918)). Enabled by default for chains marked as
            "local" in their chain specifications, or when `--dev` is passed

        --blocks-pruning <COUNT>
            Specify the number of finalized blocks to keep in the database.
            
            Default is to keep all blocks.
            
            NOTE: only finalized blocks are subject for removal!

        --bob
            Shortcut for `--name Bob --validator` with session keys for `Bob` added to keystore

        --bootnodes <ADDR>...
            Specify a list of bootnodes

        --chain <CHAIN_SPEC>
            Specify the chain specification.
            
            It can be one of the predefined ones (dev, local, or staging) or it can be a path to a
            file with the chainspec (such as one exported by the `build-spec` subcommand).

        --charlie
            Shortcut for `--name Charlie --validator` with session keys for `Charlie` added to
            keystore

    -d, --base-path <PATH>
            Specify custom base path

        --database <DB>
            Select database backend to use
            
            [possible values: rocksdb, paritydb, paritydb-experimental, auto]

        --dave
            Shortcut for `--name Dave --validator` with session keys for `Dave` added to keystore

        --db-cache <MiB>
            Limit the memory the database cache can use

        --detailed-log-output
            Enable detailed log output.
            
            This includes displaying the log target, log level and thread name.
            
            This is automatically enabled when something is logged with any higher level than
            `info`.

        --dev
            Specify the development chain.
            
            This flag sets `--chain=dev`, `--force-authoring`, `--rpc-cors=all`, `--alice`, and
            `--tmp` flags, unless explicitly overridden.

        --disable-log-color
            Disable log color output

        --discover-local
            Enable peer discovery on local networks.
            
            By default this option is `true` for `--dev` or when the chain type is
            `Local`/`Development` and false otherwise.

        --enable-log-reloading
            Enable feature to dynamically update and reload the log filter.
            
            Be aware that enabling this feature can lead to a performance decrease up to factor six
            or more. Depending on the global logging level the performance decrease changes.
            
            The `system_addLogFilter` and `system_resetLogFilter` RPCs will have no effect with this
            option not being set.

        --enable-offchain-indexing <ENABLE_OFFCHAIN_INDEXING>
            Enable Offchain Indexing API, which allows block import to write to Offchain DB.
            
            Enables a runtime to write directly to a offchain workers DB during block import.

        --eve
            Shortcut for `--name Eve --validator` with session keys for `Eve` added to keystore

        --execution <STRATEGY>
            The execution strategy that should be used by all execution contexts
            
            [possible values: native, wasm, both, native-else-wasm]

        --execution-block-construction <STRATEGY>
            The means of execution used when calling into the runtime while constructing blocks
            
            [possible values: native, wasm, both, native-else-wasm]

        --execution-import-block <STRATEGY>
            The means of execution used when calling into the runtime for general block import
            (including locally authored blocks)
            
            [possible values: native, wasm, both, native-else-wasm]

        --execution-offchain-worker <STRATEGY>
            The means of execution used when calling into the runtime while using an off-chain
            worker
            
            [possible values: native, wasm, both, native-else-wasm]

        --execution-other <STRATEGY>
            The means of execution used when calling into the runtime while not syncing, importing
            or constructing blocks
            
            [possible values: native, wasm, both, native-else-wasm]

        --execution-syncing <STRATEGY>
            The means of execution used when calling into the runtime for importing blocks as part
            of an initial sync
            
            [possible values: native, wasm, both, native-else-wasm]

        --ferdie
            Shortcut for `--name Ferdie --validator` with session keys for `Ferdie` added to
            keystore

        --force-authoring
            Enable authoring even when offline

    -h, --help
            Print help information

        --in-peers <COUNT>
            Maximum number of inbound full nodes peers
            
            [default: 25]

        --in-peers-light <COUNT>
            Maximum number of inbound light nodes peers
            
            [default: 100]

        --ipc-path <PATH>
            DEPRECATED, IPC support has been removed

        --ipfs-server
            Join the IPFS network and serve transactions over bitswap protocol

        --kademlia-disjoint-query-paths
            Require iterative Kademlia DHT queries to use disjoint paths for increased resiliency in
            the presence of potentially adversarial nodes.
            
            See the S/Kademlia paper for more information on the high level design as well as its
            security improvements.

        --keystore-path <PATH>
            Specify custom keystore path

        --keystore-uri <KEYSTORE_URI>
            Specify custom URIs to connect to for keystore-services

    -l, --log <LOG_PATTERN>...
            Sets a custom logging filter. Syntax is <target>=<level>, e.g. -lsync=debug.
            
            Log levels (least to most verbose) are error, warn, info, debug, and trace. By default,
            all targets log `info`. The global log level can be set with -l<level>.

        --listen-addr <LISTEN_ADDR>...
            Listen on this multiaddress.
            
            By default: If `--validator` is passed: `/ip4/0.0.0.0/tcp/<port>` and
            `/ip6/[::]/tcp/<port>`. Otherwise: `/ip4/0.0.0.0/tcp/<port>/ws` and
            `/ip6/[::]/tcp/<port>/ws`.

        --max-parallel-downloads <COUNT>
            Maximum number of peers from which to ask for the same blocks in parallel.
            
            This allows downloading announced blocks from multiple peers. Decrease to save traffic
            and risk increased latency.
            
            [default: 5]

        --max-runtime-instances <MAX_RUNTIME_INSTANCES>
            The size of the instances cache for each runtime.
            
            The default value is 8 and the values higher than 256 are ignored.

        --name <NAME>
            The human-readable name for this node.
            
            The node name will be reported to the telemetry server, if enabled.

        --no-grandpa
            Disable GRANDPA voter when running in validator mode, otherwise disable the GRANDPA
            observer

        --no-mdns
            Disable mDNS discovery.
            
            By default, the network will use mDNS to discover other nodes on the local network. This
            disables it. Automatically implied when using --dev.

        --no-private-ipv4
            Always forbid connecting to private IPv4 addresses (as specified in
            [RFC1918](https://tools.ietf.org/html/rfc1918)), unless the address was passed with
            `--reserved-nodes` or `--bootnodes`. Enabled by default for chains marked as "live" in
            their chain specifications

        --no-prometheus
            Do not expose a Prometheus exporter endpoint.
            
            Prometheus metric endpoint is enabled by default.

        --no-telemetry
            Disable connecting to the Substrate telemetry server.
            
            Telemetry is on by default on global chains.

        --node-key <KEY>
            The secret key to use for libp2p networking.
            
            The value is a string that is parsed according to the choice of `--node-key-type` as
            follows:
            
            `ed25519`: The value is parsed as a hex-encoded Ed25519 32 byte secret key, i.e. 64 hex
            characters.
            
            The value of this option takes precedence over `--node-key-file`.
            
            WARNING: Secrets provided as command-line arguments are easily exposed. Use of this
            option should be limited to development and testing. To use an externally managed secret
            key, use `--node-key-file` instead.

        --node-key-file <FILE>
            The file from which to read the node's secret key to use for libp2p networking.
            
            The contents of the file are parsed according to the choice of `--node-key-type` as
            follows:
            
            `ed25519`: The file must contain an unencoded 32 byte or hex encoded Ed25519 secret key.
            
            If the file does not exist, it is created with a newly generated secret key of the
            chosen type.

        --node-key-type <TYPE>
            The type of secret key to use for libp2p networking.
            
            The secret key of the node is obtained as follows:
            
            * If the `--node-key` option is given, the value is parsed as a secret key according to
            the type. See the documentation for `--node-key`.
            
            * If the `--node-key-file` option is given, the secret key is read from the specified
            file. See the documentation for `--node-key-file`.
            
            * Otherwise, the secret key is read from a file with a predetermined, type-specific name
            from the chain-specific network config directory inside the base directory specified by
            `--base-dir`. If this file does not exist, it is created with a newly generated secret
            key of the chosen type.
            
            The node's secret key determines the corresponding public key and hence the node's peer
            ID in the context of libp2p.
            
            [default: ed25519]
            [possible values: ed25519]

        --offchain-worker <ENABLED>
            Should execute offchain workers on every block.
            
            By default it's only enabled for nodes that are authoring new blocks.
            
            [default: when-validating]
            [possible values: always, never, when-validating]

        --one
            Shortcut for `--name One --validator` with session keys for `One` added to keystore

        --out-peers <COUNT>
            Specify the number of outgoing connections we're trying to maintain
            
            [default: 25]

        --password <PASSWORD>
            Password used by the keystore. This allows appending an extra user-defined secret to the
            seed

        --password-filename <PATH>
            File that contains the password used by the keystore

        --password-interactive
            Use interactive shell for entering the password used by the keystore

        --pool-kbytes <COUNT>
            Maximum number of kilobytes of all transactions stored in the pool
            
            [default: 20480]

        --pool-limit <COUNT>
            Maximum number of transactions in the transaction pool
            
            [default: 8192]

        --port <PORT>
            Specify p2p protocol TCP port

        --prometheus-external
            Expose Prometheus exporter on all interfaces.
            
            Default is local.

        --prometheus-port <PORT>
            Specify Prometheus exporter TCP Port

        --public-addr <PUBLIC_ADDR>...
            The public address that other nodes will use to connect to it. This can be used if
            there's a proxy in front of this node

        --reserved-nodes <ADDR>...
            Specify a list of reserved node addresses

        --reserved-only
            Whether to only synchronize the chain with reserved nodes.
            
            Also disables automatic peer discovery.
            
            TCP connections might still be established with non-reserved nodes. In particular, if
            you are a validator your node might still connect to other validator nodes and collator
            nodes regardless of whether they are defined as reserved nodes.

        --rpc-cors <ORIGINS>
            Specify browser Origins allowed to access the HTTP & WS RPC servers.
            
            A comma-separated list of origins (protocol://domain or special `null` value). Value of
            `all` will disable origin validation. Default is to allow localhost and
            <https://polkadot.js.org> origins. When running in --dev mode the default is to allow
            all origins.

        --rpc-external
            Listen to all RPC interfaces.
            
            Default is local. Note: not all RPC methods are safe to be exposed publicly. Use an RPC
            proxy server to filter out dangerous methods. More details:
            <https://docs.substrate.io/main-docs/build/custom-rpc/#public-rpcs>. Use
            `--unsafe-rpc-external` to suppress the warning if you understand the risks.

        --rpc-max-payload <RPC_MAX_PAYLOAD>
            DEPRECATED, this has no affect anymore. Use `rpc_max_request_size` or
            `rpc_max_response_size` instead

        --rpc-max-request-size <RPC_MAX_REQUEST_SIZE>
            Set the the maximum RPC request payload size for both HTTP and WS in megabytes. Default
            is 15MiB

        --rpc-max-response-size <RPC_MAX_RESPONSE_SIZE>
            Set the the maximum RPC response payload size for both HTTP and WS in megabytes. Default
            is 15MiB

        --rpc-max-subscriptions-per-connection <RPC_MAX_SUBSCRIPTIONS_PER_CONNECTION>
            Set the the maximum concurrent subscriptions per connection. Default is 1024

        --rpc-methods <METHOD SET>
            RPC methods to expose.
            
            - `unsafe`: Exposes every RPC method.
            - `safe`: Exposes only a safe subset of RPC methods, denying unsafe RPC methods.
            - `auto`: Acts as `safe` if RPC is served externally, e.g. when `--{rpc,ws}-external` is
              passed, otherwise acts as `unsafe`.
            
            [default: auto]
            [possible values: auto, safe, unsafe]

        --rpc-port <PORT>
            Specify HTTP RPC server TCP port

        --runtime-cache-size <RUNTIME_CACHE_SIZE>
            Maximum number of different runtimes that can be cached
            
            [default: 2]

        --state-cache-size <Bytes>
            Specify the state cache size
            
            [default: 67108864]

        --state-pruning <PRUNING_MODE>
            Specify the state pruning mode, a number of blocks to keep or 'archive'.
            
            Default is to keep only the last 256 blocks, otherwise, the state can be kept for all of
            the blocks (i.e 'archive'), or for all of the canonical blocks (i.e
            'archive-canonical').

        --sync <SYNC_MODE>
            Blockchain syncing mode.
            
            - `full`: Download and validate full blockchain history.
            - `fast`: Download blocks and the latest state only.
            - `fast-unsafe`: Same as `fast`, but skip downloading state proofs.
            - `warp`: Download the latest state and proof.
            
            [default: full]
            [possible values: full, fast, fast-unsafe, warp]

        --telemetry-url <URL VERBOSITY>
            The URL of the telemetry server to connect to.
            
            This flag can be passed multiple times as a means to specify multiple telemetry
            endpoints. Verbosity levels range from 0-9, with 0 denoting the least verbosity.
            Expected format is 'URL VERBOSITY', e.g. `--telemetry-url 'wss://foo/bar 0'`.

        --tmp
            Run a temporary node.
            
            A temporary directory will be created to store the configuration and will be deleted at
            the end of the process.
            
            Note: the directory is random per process execution. This directory is used as base path
            which includes: database, node key and keystore.
            
            When `--dev` is given and no explicit `--base-path`, this option is implied.

        --tracing-receiver <RECEIVER>
            Receiver to process tracing messages
            
            [default: log]
            [possible values: log]

        --tracing-targets <TARGETS>
            Sets a custom profiling filter. Syntax is the same as for logging: <target>=<level>

        --two
            Shortcut for `--name Two --validator` with session keys for `Two` added to keystore

        --tx-ban-seconds <SECONDS>
            How long a transaction is banned for, if it is considered invalid. Defaults to 1800s

        --unsafe-pruning
            THIS IS A DEPRECATED CLI-ARGUMENT.
            
            It has been preserved in order to not break the compatibility with the existing scripts.
            Enabling this option will lead to a runtime warning. In future this option will be
            removed completely, thus specifying it will lead to a start up error.
            
            Details: <https://github.com/paritytech/substrate/issues/8103>

        --unsafe-rpc-external
            Listen to all RPC interfaces.
            
            Same as `--rpc-external`.

        --unsafe-ws-external
            Listen to all Websocket interfaces.
            
            Same as `--ws-external` but doesn't warn you about it.

    -V, --version
            Print version information

        --validator
            Enable validator mode.
            
            The node will be started with the authority role and actively participate in any
            consensus task that it can (e.g. depending on availability of local keys).

        --wasm-execution <METHOD>
            Method for executing Wasm runtime code
            
            [default: compiled]
            [possible values: interpreted-i-know-what-i-do, compiled]

        --wasm-runtime-overrides <PATH>
            Specify the path where local WASM runtimes are stored.
            
            These runtimes will override on-chain runtimes when the version matches.

        --wasmtime-instantiation-strategy <STRATEGY>
            The WASM instantiation method to use.
            
            Only has an effect when `wasm-execution` is set to `compiled`.
            
            The copy-on-write strategies are only supported on Linux. If the copy-on-write variant
            of a strategy is unsupported the executor will fall back to the non-CoW equivalent.
            
            The fastest (and the default) strategy available is `pooling-copy-on-write`.
            
            The `legacy-instance-reuse` strategy is deprecated and will be removed in the future. It
            should only be used in case of issues with the default instantiation strategy.
            
            [default: pooling-copy-on-write]
            [possible values: pooling-copy-on-write, recreate-instance-copy-on-write, pooling,
            recreate-instance, legacy-instance-reuse]

        --ws-external
            Listen to all Websocket interfaces.
            
            Default is local. Note: not all RPC methods are safe to be exposed publicly. Use an RPC
            proxy server to filter out dangerous methods. More details:
            <https://docs.substrate.io/main-docs/build/custom-rpc/#public-rpcs>. Use
            `--unsafe-ws-external` to suppress the warning if you understand the risks.

        --ws-max-connections <COUNT>
            Maximum number of WS RPC server connections

        --ws-max-out-buffer-capacity <WS_MAX_OUT_BUFFER_CAPACITY>
            DEPRECATED, this has no affect anymore. Use `rpc_max_response_size` instead

        --ws-port <PORT>
            Specify WebSockets RPC server TCP port

SUBCOMMANDS:
    benchmark
            Sub-commands concerned with benchmarking
    build-spec
            Build a chain specification
    chain-info
            Db meta columns information
    check-block
            Validate blocks
    export-blocks
            Export blocks
    export-state
            Export the state of a given block into a chain spec
    help
            Print this message or the help of the given subcommand(s)
    import-blocks
            Import blocks
    key
            Key management cli utilities
    purge-chain
            Remove the whole chain
    revert
            Revert the chain to a previous state
    try-runtime
            Try some command against runtime state. Note: `try-runtime` feature must be enabled
