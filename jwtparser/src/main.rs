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
    let splitted: Vec<&str> = data.split('.').collect();

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

    match wrapped {
        Ok(res) => {
            String::from_utf8(res).unwrap()
        },
        Err(err) => {
            panic!("Failed to decode token: {:?}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, parse_jwt, Token};

    #[test]
    fn should_decode_the_base64_and_return_as_string() {
        let want = "hello world\n";
        let value: &str = "aGVsbG8gd29ybGQK";
        let got = decode(value);

        assert_eq!(want, got);
    }

    #[test]
    #[should_panic(expected = "Failed to decode token: InvalidByte(6, 32)")]
    fn should_fails_if_is_not_base64_and_return_failed_msg() {
        let want = "should fail";

        decode(want);
    }

    #[test]
    #[should_panic(expected = "Failed to parse token, err: this is not a JWT token")]
    fn should_fails_if_value_is_not_a_jwt_encoded() {
        parse_jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string());
    }

    #[test]
    fn should_return_a_token_when_value_is_a_well_formatted_jwt() {
        let header = "{\n  \"alg\": \"HS256\",\n  \"typ\": \"JWT\"\n}".to_string();
        let payload = "{\n  \"iat\": 1516239022,\n  \"name\": \"John Doe\",\n  \"sub\": \"1234567890\"\n}".to_string();
        
        let want = Token{
            header: header,
            payload: payload,
            signature: "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c".to_string(),
        };
        
        let data = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let got = parse_jwt(data.to_string());

        assert_eq!(want.header, got.header);
        assert_eq!(want.payload, got.payload);
        assert_eq!(want.signature, got.signature);
    }
}