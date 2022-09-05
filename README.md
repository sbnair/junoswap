# junoswap

#compile the default pool and upload it into juno node, instantiate pool based on codeid, example is given below

# wasmswap is being used in packages - ultra base package

#sudo docker exec -i juno_node_1 junod tx wasm instantiate 20  \
# '{
#  "name": "default pool",
#  "owner": "juno17w059ne838lnwz6t0hntuusr72wzfzhk7ml8k7"
#  }' --amount 5000000ujunox  --label "fanfury swap" --from "juno17w059ne838lnwz6t0hntuusr72wzfzhk7ml8k7" --chain-id "testing" --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block -y --admin "juno1j75mrz3hksdyjuf3h9wtvn9nd47qaw3tks9fpp"

# compile the junoswap oracle and then upload it, based on contract address of previous default pool that you get following code can be executed

# sudo docker exec -i juno_node_1  junod tx wasm instantiate 21 '{
#  "pool_contract_address": "juno1xz4cya4qm2ws6nzperhvc40wdjcq4872fl6d3j2s4cytyx8j80eqv8mshp"
#  }' --amount 500000ujunox  --label "fanfury swap pair" --from "juno17w059ne838lnwz6t0hntuusr72wzfzhk7ml8k7" --chain-id "testing" --gas-prices 0.1ujunox --gas 400000 --gas-adjustment 1.3 -b block -y --admin "juno17w059ne838lnwz6t0hntuusr72wzfzhk7ml8k7"
