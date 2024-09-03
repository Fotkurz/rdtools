use clap::{Parser, Subcommand};

mod jwtparser;

use jwtparser::JwtTool;

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    tool: Option<Tools>,
}

#[derive(Subcommand)]
enum Tools {
    /// Jwt parsing and validating tools.
    ///
    /// You can either inform a token directly or inform a file containing
    /// the token to be decoded.
    ///
    /// Ex:
    ///     rdtools jwt <encoded_jwt>
    Jwt {
        /// A base64 encoded JWT token to decode.
        token: Option<String>,

        /// A file with a base64 encoded JWT token to decode. (e.g rdtools jwt --file data/sample.txt).
        #[arg(short, long)]
        file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.tool {
        Some(Tools::Jwt { token: data, file }) => {
            JwtTool {
                data: match data {
                    Some(v) => v.to_owned(),
                    None => "".to_owned(),
                },
                file: match file {
                    Some(v) => v.to_owned(),
                    None => "".to_owned(),
                },
            }
            .run();
        }
        None => {
            println!("Select at least one of the tools")
        }
    }
}
