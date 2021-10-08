#!/bin/bash

# download config
curl https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json --output /root/.near/mainnet/config.json
# set as archival node
cat ~/.near/mainnet/config.json | jq '.archive = true' > ~/.near/mainnet/config.json

# run indexer
indexer-explorer --home-dir ~/.near/mainnet run --store-genesis sync-from-block --height 0
