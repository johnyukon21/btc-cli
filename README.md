# btc-cli
A Bitcoin command line tool written in Rust that 
* interacts with a full node via the p2p protocol
* interacts with a full node via the rpc interface
* interacts with an electrum server
* provides key management utilities 


**DISCLAIMER: this project is experimental and the code is not  production ready. DO NOT USE IN PRODUCTION!!!!**

#### Usage Examples:

#### The "key-management" subcommand:
This sub-command provides utilities for key management. 
```
➜  btc-cli git:(development) ✗ cargo run key-management --action generate-xpub-from-seed --seed="000102030405060708090a0b0c0d0e0a" --network bitcoin --path "m/0/1"
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/main key-management --action generate-xpub-from-seed --seed=000102030405060708090a0b0c0d0e0a --network bitcoin --path m/0/1`
xpub: xpub6BJNgwRThSd6DEqbxGYdYPuEWhKFtzEMPPxmDHLfWdWjHEWaeGvQukNtAjw2rFpYb4cUzvJF1VG8pwWcr7bv9oMJqHaJqLfYhUy4hknreS9
public key: 02fb62886536544edbd8d8c257ab7014f54988b386c867f9d6ec68b5305af1beb9
➜  btc-cli git:(development) ✗
```

#### The "p2p" subcommand:
This sub-command enables the user to interact with a bitcoin node using the  p2p protocol
```
➜  btc-cli git:(development) ✗ cargo run  p2p --action raw-message --node localhost:8333 --message "f9beb4d976657273696f6e000000000066000000be61b8277f1101000d04000000000000f00f4d5c00000000000000000000000000000000000000000000ffff5bf08c80b4bd0d04000000000000000000000000000000000000000000000000faa99559cc68a1c1102f5361746f7368693a302e31372e312f938c080001"
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/main p2p --action raw-message --node 'localhost:8333' --message f9beb4d976657273696f6e000000000066000000be61b8277f1101000d04000000000000f00f4d5c00000000000000000000000000000000000000000000ffff5bf08c80b4bd0d04000000000000000000000000000000000000000000000000faa99559cc68a1c1102f5361746f7368693a302e31372e312f938c080001`
|f9beb4d9 76657273 696f6e00 00000000| ....version..... 00000000
|66000000 ed89e2e7 7f110100 0d040000| f............... 00000010
|00000000 b92de35c 00000000 00000000| .....-.\........ 00000020
|00000000 00000000 00000000 00000000| ................ 00000030
|00000000 00000d04 00000000 00000000| ................ 00000040
|00000000 00000000 00000000 00000000| ................ 00000050
|17409d72 31d05343 102f5361 746f7368| .@.r1.SC./Satosh 00000060
|693a302e 31382e30 2fdfcd08 0001|     i:0.18.0/.....   00000070
                                                       0000007e
➜  btc-cli git:(development) ✗

```

#### The "rpc" subcommand:
This subcommand enables the user to talk to a bitcoin node via JSON RPC
(only password auth implemented for now)
```
➜  btc-cli git:(development) ✗ cargo run rpc --node http://localhost:8332 --username user --action get-best-block-hash
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/main rpc --node 'http://localhost:8332' --username user --action get-best-block-hash`
Type in the RPC password:

best block hash: 000000000000000000141688aac2f0da13de43612e683666593b6e6a1afe7faf
➜  btc-cli git:(development) ✗
```

#### The "electrum" subcommand:
This subcommand enables the user to talk to an elecrtrum server. 
NOT IMPLEMENTED YET