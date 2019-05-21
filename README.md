# btc-cli
A Bitcoin command line tool written in Rust that 
* interacts with a full node via the p2p protocol
* interacts with a full node via the rpc interface
* interacts with an electrum server
* provides key management utilities 


**DISCLAIMER: this project is experimental and the code is not  production ready. DO NOT USE IN PRODUCTION!!!!**

#### Installation
1. Install rust: https://www.rust-lang.org/tools/install
2. `cargo install btc-cli`

#### Usage Examples:

#### The "key-management" subcommand:
This sub-command provides utilities for key management. 
```
# ~/.cargo/bin/btc-cli key-management --action generate-xpub-from-seed --seed="000102030405060708090a0b0c0d0e0a" --network bitcoin --path "m/0/1"
xpub: xpub6BJNgwRThSd6DEqbxGYdYPuEWhKFtzEMPPxmDHLfWdWjHEWaeGvQukNtAjw2rFpYb4cUzvJF1VG8pwWcr7bv9oMJqHaJqLfYhUy4hknreS9
public key: 02fb62886536544edbd8d8c257ab7014f54988b386c867f9d6ec68b5305af1beb9
```

#### The "p2p" subcommand:
This sub-command enables the user to interact with a bitcoin node using the  p2p protocol
``` bash
# ~/.cargo/bin/btc-cli p2p --action raw-message --node localhost:8333 --message "f9beb4d976657273696f6e000000000066000000be61b8277f1101000d04000000000000f00f4d5c00000000000000000000000000000000000000000000ffff5bf08c80b4bd0d04000000000000000000000000000000000000000000000000faa99559cc68a1c1102f5361746f7368693a302e31372e312f938c080001"
|f9beb4d9 76657273 696f6e00 00000000| ....version..... 00000000
|66000000 757ee13c 7f110100 0d040000| f...u~.<........ 00000010
|00000000 fd85e35c 00000000 00000000| .......\........ 00000020
|00000000 00000000 00000000 00000000| ................ 00000030
|00000000 00000d04 00000000 00000000| ................ 00000040
|00000000 00000000 00000000 00000000| ................ 00000050
|60e534e6 3b60ff1a 102f5361 746f7368| `.4.;`.../Satosh 00000060
|693a302e 31382e30 2f09ce08 0001|     i:0.18.0/.....   00000070
```

#### The "rpc" subcommand:
This subcommand enables the user to talk to a bitcoin node via JSON RPC
(only password auth implemented for now)
```
# ~/.cargo/bin/btc-cli rpc --node http://localhost:8332 --username user --action get-best-block-hash
Type in the RPC password:

best block hash: 000000000000000000154ed13dc43951141f2d73bb29e6b1cd4c4dfb8ca97418
```

#### The "electrum" subcommand:
This subcommand enables the user to talk to an elecrtrum server. 
NOT IMPLEMENTED YET