use std::fs;

use base64::prelude::*;
use colored::Colorize;
use serde_json::Value;

#[cfg(test)]
mod tests;

/// Defines the JWT token structure
struct Token {
    header: String,
    payload: String,
    signature: String,
}

/// Defines the jwt tool and its fields
pub struct JwtTool {
    pub data: String,
    pub file: String,
}

impl JwtTool {
    /// Runs the cli command
    pub fn run(&self) {
        let mut contents = self.data.clone();

        if self.data.is_empty() {
            // reads from file
            let read_result = fs::read_to_string(&self.file.clone());

            contents = match read_result {
                Ok(res) => res,
                Err(err) => panic!("Could not read file {}, err: {:?}", self.file, err),
            }
        }

        let res = JwtTool::parse_jwt(contents);

        let header_msg = format!("Header: {}", res.header);
        let payload_msg = format!("Payload: {}", res.payload);
        let signature_msg = format!("Signature: {}", res.signature);

        println!(
            "{},\n{},\n{}",
            header_msg.blue(),
            payload_msg.green(),
            signature_msg.yellow()
        );
    }

    fn parse_jwt(data: String) -> Token {
        let splitted: Vec<&str> = data.split('.').collect();

        if splitted.len() != 3 {
            panic!("Failed to parse token, err: this is not a JWT token");
        }

        // TODO: proper error handling for serde_json::from_str
        let header: Value = serde_json::from_str(JwtTool::decode(splitted[0]).as_str()).unwrap();
        let payload: Value = serde_json::from_str(JwtTool::decode(splitted[1]).as_str()).unwrap();

        Token {
            header: serde_json::to_string_pretty(&header).unwrap(),
            payload: serde_json::to_string_pretty(&payload).unwrap(),
            // TODO: check signature
            signature: splitted[2].to_owned(),
        }
    }

    fn decode(b64_data: &str) -> String {
        let wrapped = BASE64_URL_SAFE_NO_PAD.decode(b64_data);

        match wrapped {
            Ok(res) => String::from_utf8(res).unwrap(),
            Err(err) => {
                panic!("Failed to decode token: {:?}", err)
            }
        }
    }
}
