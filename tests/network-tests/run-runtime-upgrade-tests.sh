#!/usr/bin/env bash
set -e

SCRIPT_PATH="$(dirname "${BASH_SOURCE[0]}")"
cd $SCRIPT_PATH

rm ./output.json || :

# Location that will be mounted to /spec in containers
# This is where the initial balances files and generated chainspec files will be located.
DATA_PATH=$PWD/data
mkdir -p ${DATA_PATH}

# The latest docker image tag to use for joystream/node (testing profile)
RUNTIME=${RUNTIME:=latest}
TARGET_RUNTIME=${TARGET_RUNTIME:=target}

# Source of funds for all new accounts that are created in the tests.
TREASURY_INITIAL_BALANCE=${TREASURY_INITIAL_BALANCE:="100000000"}
TREASURY_ACCOUNT_URI=${TREASURY_ACCOUNT_URI:="//Bob"}
TREASURY_ACCOUNT=$(docker run --pull never --rm joystream/node:${RUNTIME} key inspect ${TREASURY_ACCOUNT_URI} --output-type json | jq .ss58Address -r)

echo >&2 "treasury account from suri: ${TREASURY_ACCOUNT}"

# Default initial balances
function generate_config_files() {
    echo "{
  \"balances\":[
    [\"$TREASURY_ACCOUNT\", $TREASURY_INITIAL_BALANCE]
  ],
  \"vesting\":[]
}" >${DATA_PATH}/initial-balances.json

    # Override initial balances from external source
    if [[ $INITIAL_BALANCES == http* ]]; then
        echo >&2 "fetching ${INITIAL_BALANCES}"
        wget -O ${DATA_PATH}/initial-balances.json ${INITIAL_BALANCES}
    else
        if [ ! -z "$INITIAL_BALANCES" ]; then
            if jq -e . >/dev/null 2>&1 <<<"$INITIAL_BALANCES"; then
                echo >&2 "Detected some valid JSON in INITIAL_BALANCES"
                echo $INITIAL_BALANCES >${DATA_PATH}/initial-balances.json
            else
                echo >&2 "Failed to parse INITIAL_BALANCES as JSON, or got false/null"
            fi
        fi
    fi
}

# Create a chain spec file
function create_raw_chain_spec() {
    docker run --pull never --rm -v ${DATA_PATH}:/spec --entrypoint ./chain-spec-builder joystream/node:${RUNTIME} \
        generate \
        --authorities 1 \
        --nominators 1 \
        --endowed 1 \
        --initial-balances-path /spec/initial-balances.json \
        --deployment dev \
        --chain-spec-path /spec/chain-spec.json \
        --keystore-path /spec/keystore

    # Convert the chain spec file to a raw chainspec file
    docker run --pull never --rm -v ${DATA_PATH}:/spec joystream/node:${RUNTIME} build-spec \
        --raw --disable-default-bootnode \
        --chain /spec/chain-spec.json >${DATA_PATH}/chain-spec-raw.json

}

# Start a chain with generated chain spec
function start_joystream_node {
    docker-compose -f ../../docker-compose.yml run -d -v ${DATA_PATH}:/spec \
        --name joystream-node \
        -p 9944:9944 -p 9933:9933 joystream-node \
        --validator --unsafe-ws-external --unsafe-rpc-external \
        --rpc-methods Unsafe --rpc-cors=all -l runtime \
        --chain /spec/chain-spec-forked.json --pruning=archive --no-telemetry --no-mdns \
        --no-hardware-benchmarks \
        --keystore-path /spec/keystore/auth-0 \
        --base-path /data
}

#######################################
# Copy the new wasm to the data directory
# Globals:
#   TARGET_RUNTIME
#   DATA_PATH
# Arguments:
#   None
#######################################
function set_new_runtime_wasm_path() {
    docker create --name target-node joystream/node:${TARGET_RUNTIME}
    docker cp target-node:/joystream/runtime.compact.compressed.wasm ${DATA_PATH}/new_runtime.wasm
    docker rm target-node
}

#######################################
# use fork-off to generate a chainspec file with the current s
# Globals:
#   DATA_PATH
#   SCRIPT_PATH
# Arguments:
#   None
#######################################
function fork_off_init() {
    # chain-spec-raw already existing

    # http endpoint where to get metadata from mainnet
    if [[ -z $WS_RPC_ENDPOINT ]]; then
        export WS_RPC_ENDPOINT="wss://65.108.208.60.nip.io/ws-rpc"
    fi
    # http endpoint where to download storage data
    if [[ -z $HTTP_RPC_ENDPOINT ]]; then
        export HTTP_RPC_ENDPOINT="http://65.108.208.60.nip.io/pioneer"
    fi

    # download the raw storage state
    if ! [[ -f ${DATA_PATH}/storage.json ]]; then
        echo >&2 "fetching state storage from $HTTP_RPC_ENDPOINT"
        curl $HTTP_RPC_ENDPOINT -H \
            "Content-type: application/json" -d \
            '{"jsonrpc":"2.0","id":1,"method":"state_getPairs","params":["0x"]}' \
            >${DATA_PATH}/storage.json
        echo >&2 "storage trie downloaded at ${DATA_PATH}/storage.json"
    fi

    yarn workspace api-scripts tsnode-strict --max-old-space-size=6144 \
        src/fork-off.ts ${DATA_PATH} ${WS_RPC_ENDPOINT}
}

#######################################
# Write initial genesis state to disk
# Globals:
#   DATA_PATH
# Arguments:
#   None
#######################################
function init_chain_db() {
    # write the initial genesis state to db, in order to avoid waiting for an arbitrary amount of time
    # when starting the node to startup. it can take a significant amount of time
    # if the initial state is large.
    # exporting should give some essential tasks errors but they are harmless https://github.com/paritytech/substrate/issues/10583
    echo >&2 "exporting state"
    docker-compose -f ../../docker-compose.yml run --rm \
        -v ${DATA_PATH}:/spec joystream-node export-state \
        --chain /spec/chain-spec-forked.json \
        --base-path /data --pruning archive >${DATA_PATH}/exported-state.json
}

# entrypoint
function main {
    if [ $TARGET_RUNTIME == $RUNTIME ]; then
        echo >&2 "Same tag for runtime and target runtime aborting..."
        exit 0
    fi

    # 0. Generate config files
    generate_config_files
    echo >&2 "config files generated"
    # 1. create empty raw chainspec
    create_raw_chain_spec
    echo >&2 "chainspec generated"
    # 2. clone live chainspec with fork it
    fork_off_init
    echo >&2 "storage downloaded & dumped into the raw chainspec"
    # 3. set path to new runtime.wasm
    set_new_runtime_wasm_path
    echo >&2 "new wasm path set"

    # 4. early chain db init
    export JOYSTREAM_NODE_TAG=${RUNTIME}
    init_chain_db
    echo >&2 "chain db initialized"
    # 5. start node using new version
    export JOYSTREAM_NODE_TAG=${TARGET_RUNTIME}
    start_joystream_node
    echo >&2 "joystream node starting"

    # Start a query-node
    # Its important to remember the query-node processor is starting with an empty state
    # and not inline with the chain state. This means some integration test scenarios might
    # not function correctly, but we need it to at least have the council election and runtime upgrade
    # proposal to be created and passed.
    ../../query-node/start.sh

    # Wait for chain and query node to get in sync
    sleep 200

    # 6. Bootstrap storage infra because we need to run content-directory tests after runtime upgrade
    if [ "${NO_STORAGE}" != true ]; then
        ./start-storage.sh
    fi

    # Do some setup and checks before the upgrade
    ./run-test-scenario.sh preRuntimeUpgrade

    ./run-test-scenario.sh runtimeUpgrade

    sleep 20

    ./run-test-scenario.sh postRuntimeUpgrade
}

# main entrypoint
main
