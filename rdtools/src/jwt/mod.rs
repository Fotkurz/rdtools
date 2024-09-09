use std::fs;

use base64::prelude::*;
use colored::{ColoredString, Colorize};
use serde_json::Value;

#[cfg(test)]
mod tests;

/// Defines the JWT token structure
struct Token {
    header: Value,
    payload: Value,
    signature: String,
    is_valid: bool,
}

/// Defines the jwt tool and its fields
pub struct JwtTool {
    pub data: String,
    pub file: String,
    pub secret: String,
}

impl JwtTool {
    /// Runs the cli command
    pub fn run(&self) {
        let mut contents = self.data.clone();

        if self.data.is_empty() {
            // reads from file
            let read_result = fs::read_to_string(self.file.clone());

            contents = match read_result {
                Ok(res) => res,
                Err(err) => panic!("Could not read file {}, err: {:?}", self.file, err),
            }
        }

        let mut token = JwtTool::parse_jwt(contents);

        if !self.secret.is_empty() {
            token.is_valid = JwtTool::validate(
                &token.signature,
                &self.secret,
                &token.header.get("alg").unwrap().to_string(),
            );
        }

        if self.secret.is_empty() {
            JwtTool::print_final(token, false);
        } else {
            JwtTool::print_final(token, true)
        }
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
            header,
            payload,
            // TODO: check signature
            signature: splitted[2].to_owned(),
            is_valid: false,
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

    fn validate(signature: &String, secret: &String, alg: &String) -> bool {
        true
    }

    fn print_final(token: Token, print_valid: bool) {
        let mut msg = format!(
            "{},\n{},\n{}",
            format!(
                "Header: {}",
                serde_json::to_string_pretty(&token.header).unwrap()
            )
            .blue(),
            format!(
                "Payload: {}",
                serde_json::to_string_pretty(&token.payload).unwrap()
            )
            .green(),
            format!("Signature: {}", token.signature).yellow()
        );

        if print_valid {
            let mut is_valid_msg = format!("Is valid: {}", token.is_valid).red();
            if token.is_valid {
                is_valid_msg = is_valid_msg.green();
            }

            msg = format!("{},\n{}", msg, is_valid_msg);
        }

        println!("{}", msg);
    }
}
