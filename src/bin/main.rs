extern crate clap;
extern crate hexdump;
extern crate bitcoin;
extern crate bitcoincore_rpc;
extern crate rpassword;

use rpassword::read_password;
use bitcoincore_rpc::{Auth, Client, Error, RpcApi};
use bitcoin::consensus::encode::{Decodable, Encodable};
use bitcoin::consensus::encode::{Decoder, Encoder, serialize};
use bitcoin::network::constants::Network;
use bitcoin::network::message::NetworkMessage;
use bitcoin::network::message::RawNetworkMessage;
use bitcoin::network::stream_reader::StreamReader;
use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey;
use clap::{App, Arg, SubCommand};
use hex::decode as hex_decode;
use secp256k1::{self, Secp256k1};
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;

fn main() {
    let matches = App::new("btc-cli")
        .version("0.1.0")
        .author("John Yukon <johnyukon21@protonmail.com")
        .about("A Bitcoin command line tool that interacts with a full node via the p2p protocol,
        the RPC protocol and talks to an electrum server")
        .subcommand(
            SubCommand::with_name("key-management")
            .about("commands dealing with key management")
            .arg(Arg::with_name("action")
                .long("action")
                .value_name("ACTION")
                .required(true)
                .help("the action to perform"))
            .arg(Arg::with_name("seed")
                .long("seed")
                .value_name("SEED")
                .required(true)
                .help("the seed (hex-encoded) to generate the  xpub from"))
            .arg(Arg::with_name("path")
                .long("path")
                .value_name("PATH")
                .help("the derivation path"))
            .arg(Arg::with_name("network")
                .long("network")
                .value_name("NETWORK")
                .help("specify which bitcoin network us being used (bitcoin, testnet, regtest)"))
        )
        .subcommand(
            SubCommand::with_name("p2p")
                .about("commands to talk to a Bitcoin node via the p2p network")
                .arg(Arg::with_name("action")
                    .long("action")
                    .value_name("ACTION")
                    .required(true)
                    .help("the action to perform"))
                .arg(Arg::with_name("node")
                    .long("node")
                    .value_name("NODE")
                    .required(true)
                    .help("the node to connect to eg. 127.0.0.1:8333"))
                .arg(Arg::with_name("message")
                    .long("message")
                    .value_name("MESSAGE")
                    .help("the file containing the hex encoded message to send to the node"))
        )
        .subcommand(
            SubCommand::with_name("rpc")
                .about("commands to talk to a bitcoin node via JSON-RPC")
                .arg(Arg::with_name("action")
                    .long("action")
                    .value_name("ACTION")
                    .required(true)
                    .help("the action to perform"))
                .arg(Arg::with_name("username")
                    .long("username")
                    .value_name("USERNAME")
                    .required(true)
                    .help("rpc username"))
                .arg(Arg::with_name("node")
                    .long("node")
                    .value_name("NODE")
                    .required(true)
                    .help("the node to connect to eg. 127.0.0.1:8333"))
        )
        .subcommand(
            SubCommand::with_name("electrum")
                .about("commands to talk to an electrum server")
                .arg(Arg::with_name("action")
                    .long("action")
                    .value_name("ACTION")
                    .required(true)
                    .help("the action to perform"))
                .arg(Arg::with_name("node")
                    .long("node")
                    .value_name("NODE")
                    .required(true)
                    .help("the electrum server to connect to"))
        )
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("key-management") {
        let action = matches.value_of("action").unwrap();
        if action == "generate-from-seed" {
            let secp = Secp256k1::new();
            let seed = hex_decode(matches.value_of("seed").unwrap()).unwrap();

            let network_string = matches.value_of("network").unwrap();
            let network = (Network::from_str(network_string)).unwrap();
            let mut sk = ExtendedPrivKey::new_master(network, &seed).unwrap();
            let mut pk = ExtendedPubKey::from_private(&secp, &sk);

            let path_string = matches.value_of("path").unwrap();
            let path = DerivationPath::from_str(path_string).unwrap();

            let xpub = &pk.derive_pub(&secp, &path).unwrap().to_string()[..];
            let public_key = &pk.public_key;
            println!("{}", &xpub);
            println!("{}", &public_key);

        }
        else{
            println!("Unknown action: {}", action)
        }
    }

    if let Some(matches) = matches.subcommand_matches("p2p") {
        let action = matches.value_of("action").unwrap();
        if action == "raw-message" {
            let node_end_point = matches.value_of("node").unwrap();
            let mut stream = TcpStream::connect(node_end_point).expect("Couldn't connect to the server...");;
            let input_message =  matches.value_of("message").unwrap();

            let decoded = hex::decode(&input_message).expect("Decoding failed");

            stream.write(&decoded);
            let message = StreamReader::new(&mut stream, None).next_message().unwrap();
            hexdump::hexdump(&serialize(&message));

        }
        else{
            println!("Unknown action: {}", action)
        }
    }

    if let Some(matches) = matches.subcommand_matches("rpc") {
        let action = matches.value_of("action").unwrap();
        if action == "get-best-block-hash" {
            let node_end_point = matches.value_of("node").unwrap();
            let username = matches.value_of("username").unwrap();
            println!("Type in the RPC password: ");
            let password = read_password().unwrap();
            let rpc = Client::new(node_end_point.to_string(), Auth::UserPass(username.to_string(), password)).unwrap();

            //TODO: use reflection to figure out which rpc function to call
            let best_block_hash = rpc.get_best_block_hash().unwrap();
            println!("best block hash: {}", best_block_hash);

        }
        else{
            println!("Unknown action: {}", action)
        }
    }

    if let Some(matches) = matches.subcommand_matches("electrum") {
        println!("not implemented yet")
    }


}