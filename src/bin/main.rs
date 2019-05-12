extern crate clap;
extern crate hexdump;

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
        .about("A Bitcoin command line tool that interacts with a full node via the p2p protocol")
        .subcommand(
            SubCommand::with_name("xpub")
            .about("commands dealing with xpub")
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
//            .arg(Arg::with_name("path")
//                .long("path")
//                .value_name("PATH")
//                .help("the path"))
        )
        .subcommand(
            SubCommand::with_name("network")
                .about("commands to talk to a Bitcoin node")
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
        .get_matches();


    if let Some(matches) = matches.subcommand_matches("xpub") {
        let action = matches.value_of("action").unwrap();
        if action == "generate-from-seed" {
            let secp = Secp256k1::new();
            let seed = hex_decode(matches.value_of("seed").unwrap()).unwrap();

            //TODO: support the "network" from command line
            let mut sk = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).unwrap();
            let mut pk = ExtendedPubKey::from_private(&secp, &sk);

            let path = DerivationPath::from_str("m").unwrap();
            //TODO: take path from the command line
            let master_xpub = &pk.derive_pub(&secp, &path).unwrap().to_string()[..];
            println!("{}", action);
            println!("{}", &master_xpub);

        }
        else{
            println!("Unknown action: {}", action)
        }
    }

    if let Some(matches) = matches.subcommand_matches("network") {
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


}