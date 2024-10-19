use jwt_signer::config::Config;

#[derive(clap::Args, Debug)]
pub(crate) struct ServerCommand {
    #[clap(subcommand)]
    pub command: ServerCommands,

    #[arg(default_value = "0.0.0.0:5000", long, short)]
    pub addr: std::net::SocketAddr,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum ServerCommands {
    /// start the http server
    Http,
}

impl ServerCommand {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        let server = jwt_signer_http::Server::builder()
            .addr(self.addr)
            .jwt_secret(config.secret_key.as_bytes().to_owned())
            .issuer(config.issuer)
            .build()?;

        match self.command {
            ServerCommands::Http => server.run().await?,
        }

        Ok(())
    }
}
