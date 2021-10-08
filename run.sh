#!/bin/bash

# download configs
indexer-explorer init --chain-id mainnet --download-genesis --download-config
mkdir -p /root/.near/mainnet
mv ~/.near/genesis.json /root/.near/mainnet/genesis.json
mv ~/.near/node_key.json /root/.near/mainnet/node_key.json
curl https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json --output /root/.near/mainnet/config.json

# set as archival node
cat /root/.near/mainnet/config.json | jq '.archive = true' > /root/.near/mainnet/config.json.tmp && mv /root/.near/mainnet/config.json.tmp /root/.near/mainnet/config.json

# run indexer
indexer-explorer --home-dir /root/.near/mainnet run --store-genesis sync-from-block --height 0
