use std::fs;

use base64::prelude::*;
use clap::Parser;
use serde_json::Value;
use colored::Colorize;

struct Token {
    header: String,
    payload: String,
    signature: String,
}

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    /// parse from arg. (Ex: jwtparser "eyJhbGciOiJIUzI1NiIsInR5c...")
    data: Option<String>,
    /// parses from file. (Ex: jwtparser --file file.txt)
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    // parses cli args
    let cli = Cli::parse();

    let mut contents = "".to_owned();
    let mut file_path = "".to_owned();
    
    // reads the data arg
    if let Some(data) = cli.data.as_deref() {
        contents = data.to_owned();
    // reads the file arg
    } else if let Some(file) = cli.file.as_deref() {
        file_path = file.to_owned();
    } else {
        panic!("you should inform the token via arg directly or via file")
    }

    if contents.is_empty() {
        // reads from file
        let read_result = fs::read_to_string(&file_path);

        contents = match read_result {
            Ok(res) => res,
            Err(err) => panic!("Could not read file {}, err: {:?}", file_path, err)
        }
    }

    let res = parse_jwt(contents);
    
    let header_msg = format!("Header: {}", res.header);
    let payload_msg = format!("Payload: {}", res.payload);
    let signature_msg = format!("Signature: {}", res.signature);

    println!("{},\n{},\n{}", header_msg.blue(), payload_msg.green(), signature_msg.yellow());
}

fn parse_jwt(data: String) -> Token {
    let splitted: Vec<&str> = data.split(".").collect();

    if splitted.len() != 3 {
        panic!("Failed to parse token, err: this is not a JWT token");
    }

    // TODO: proper error handling for serde_json::from_str
    let header: Value = serde_json::from_str(
        decode(splitted[0]).as_str()
    ).unwrap();
    let payload: Value = serde_json::from_str(
        decode(splitted[1]).as_str()
    ).unwrap();

    Token {
        header: serde_json::to_string_pretty(&header).unwrap(),
        payload: serde_json::to_string_pretty(&payload).unwrap(),
        // TODO: check signature
        signature: splitted[2].to_owned()
    }
}

fn decode(b64_data: &str) -> String {
    let wrapped = BASE64_URL_SAFE_NO_PAD.decode(b64_data);

    let decoded = match wrapped {
        Ok(res) => {
            String::from_utf8(res).unwrap()
        },
        Err(err) => {
            panic!("Failed to decode token: {:?}", err)
        }
    };

    decoded
}
