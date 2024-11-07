# Cyferio storage contract

Cyferio storage contract which enables saving arbitrary data into Babylon with bitcoin timestamped, and checking
whether data is Bitcoin finalized.

Not for production. Best used as example.

Also part of e2e test-suite.

## Development

Explorer: https://www.babylonexplorer.com/

### How to start

1. Optimize contract
```bash
cargo run-script optimize
```

2. Store contract
```bash
babylond tx wasm store ./artifacts/storage_contract-aarch64.wasm --from=user --gas=auto --gas-prices=1ubbn --gas-adjustment=1.3 --chain-id="euphrates-0.5.0" -b=sync --yes --log_format=json --node=https://rpc-euphrates.devnet.babylonlabs.io
```

3. Get code id
```bash
curl -s -X GET "https://lcd-euphrates.devnet.babylonlabs.io/cosmwasm/wasm/v1/code" -H "accept: application/json" | jq -r '.'

codeId=$(curl -s -X GET "https://lcd-euphrates.devnet.babylonlabs.io/cosmwasm/wasm/v1/code" -H "accept: application/json" | jq -r '.code_infos[-1].code_id'); echo $codeId
```

4. Instantiate contract
```bash
babylond tx wasm instantiate $codeId '{}' --from=user --no-admin --label="cyferio_storage" --gas-prices=1ubbn --gas-adjustment=1.3 --chain-id="euphrates-0.5.0" -b=sync --yes --log_format=json --node=https://rpc-euphrates.devnet.babylonlabs.io
```

5. Get contract address
```bash
curl -s -X GET "https://lcd-euphrates.devnet.babylonlabs.io/cosmwasm/wasm/v1/code/$codeId/contracts" -H "accept: application/json" | jq -r '.'
```

```bash
address="$(curl -s -X GET "https://lcd-euphrates.devnet.babylonlabs.io/cosmwasm/wasm/v1/code/$codeId/contracts" -H "accept: application/json" | jq -r '.contracts[-1]')"; echo $address
```

6. Execute contract
```bash
data='hello world'
dataHex=$(echo -n $data | xxd -ps -c0)
executeMsg="{ \"save_data\": { \"save_data\": { \"data\": \"$dataHex\", \"da_height\": 0 } } }"; echo $executeMsg

babylond tx wasm execute $address "$executeMsg" --from=user --gas-prices=1ubbn --gas-adjustment=1.3 --chain-id="euphrates-0.5.0" -b=sync --yes --log_format=json --node=https://rpc-euphrates.devnet.babylonlabs.io
```

7. Query contract
```bash
babylond query wasm contract-state smart $address '{ "query_data": { "query_data": { "da_height": 0 } } }' --node=https://rpc-euphrates.devnet.babylonlabs.io -o json | jq -r '.'
```