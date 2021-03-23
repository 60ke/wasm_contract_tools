
#!/bin/bash
cargo +nightly -Z unstable-options build  -q --release --out-dir . --target wasm32-unknown-unknown
cp ./target/json/abi.json ./
echo -e "ABI-------"
cat ./target/json/abi.json
echo -e "\n"
echo -e "BINCODE-------"
/root/wasm_contract_tools/compile-tool/readhex sample.wasm
rm sample.wasm
#cargo clean
    