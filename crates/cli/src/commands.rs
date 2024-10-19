pub(crate) mod key;
pub(crate) mod server;
pub(crate) mod token;
use key::KeyCommand;
use server::ServerCommand;
use token::TokenCommand;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    /// commands for generating keypairs
    Key(KeyCommand),
    /// commands for running the server
    Server(ServerCommand),
    /// commands for tokens
    Token(TokenCommand),
}
