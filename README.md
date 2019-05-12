# btc-cli
A Bitcoin command line tool that interacts with a full node via the p2p protocol


**DISCLAIMER: this project is experimental and the code is not  production ready. DO NOT USE IN PRODUCTION!!!!**

#### Usage Examples:
####The "xpub" subcommand" 
This sub-command deals with exposing the different features BIP32 hierarchical deterministic wallets. 
```
➜  btc-cli git:(development) ✗ cargo run xpub --action generate-from-seed --seed="000102030405060708090a0b0c0d0e0a"
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/main xpub --action generate-from-seed --seed=000102030405060708090a0b0c0d0e0a`
xpub661MyMwAqRbcF5SPUhJf73MNzHm1KXQr9q3oBpxTe9z6xt65VCo87sgMpKxMJPpa4jTDzyEncBPYFyHCQZdgzWf6Y5V6G7hhyvpJKWU2Qb5
```

####The "network" subcommand"
This sub-command enables the user to interact with a bitcoin node using the  p2p protocol
```
➜  btc-cli git:(development) ✗ cargo run network --action raw-message --node localhost:8333 --message "f9beb4d976657273696f6e000000000066000000be61b8277f1101000d04000000000000f00f4d5c00000000000000000000000000000000000000000000ffff5bf08c80b4bd0d04000000000000000000000000000000000000000000000000faa99559cc68a1c1102f5361746f7368693a302e31372e312f938c080001"
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/main network --action raw-message --node 'localhost:8333' --message f9beb4d976657273696f6e000000000066000000be61b8277f1101000d04000000000000f00f4d5c00000000000000000000000000000000000000000000ffff5bf08c80b4bd0d04000000000000000000000000000000000000000000000000faa99559cc68a1c1102f5361746f7368693a302e31372e312f938c080001`
|f9beb4d9 76657273 696f6e00 00000000| ....version..... 00000000
|66000000 973bbb3c 7f110100 0d040000| f....;.<........ 00000010
|00000000 4d83d85c 00000000 00000000| ....M..\........ 00000020
|00000000 00000000 00000000 00000000| ................ 00000030
|00000000 00000d04 00000000 00000000| ................ 00000040
|00000000 00000000 00000000 00000000| ................ 00000050
|cb265dc4 6a092416 102f5361 746f7368| .&].j.$../Satosh 00000060
|693a302e 31382e30 2ff8c808 0001|     i:0.18.0/.....   00000070
                                                       0000007e
➜  btc-cli git:(development) ✗

```
