use std::path::PathBuf;

use clap::{Parser, ValueHint};
use env_logger::Env;

use crate::hrboxclient::HrBoxClient;

mod hrboxclient;
mod hrdocumentbox;

/// Pulls every document from the HR document box.
#[derive(Parser)]
#[clap(author, version, about)]
struct CliArguments {
    /// The subdomain of the HR document box, from which to download the documents.
    #[clap(short, long, value_parser, env = "HR_BOX_SUBDOMAIN")]
    subdomain: String,

    /// The username used to authenticate to the document box.
    #[clap(short, long, value_parser, value_hint = ValueHint::Username, env = "HR_BOX_USERNAME")]
    username: String,

    /// The password used to authenticate to the document box.
    #[clap(short, long, value_parser, env = "HR_BOX_PASSWORD")]
    password: String,

    /// The folder where the HR documents will be saved.
    #[clap(
        short,
        long,
        value_parser,
        value_hint = ValueHint::DirPath,
        value_name = "FOLDER",
        default_value = ".",
        env = "HR_BOX_OUTPUT"
    )]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let arguments = CliArguments::parse();

    let hrbox = HrBoxClient::new(&arguments.subdomain)?;
    hrbox.login(&arguments.username, &arguments.password)?;

    let document_box = hrbox.get_all_documents()?;
    for document in document_box.documents {
        hrbox.download_file(&document, &arguments.output)?;
    }

    Ok(())
}
