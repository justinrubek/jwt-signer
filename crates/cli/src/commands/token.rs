use jwt_signer::config::Config;
use jwt_signer_http::auth::Claims;

#[derive(clap::Args, Debug)]
pub(crate) struct TokenCommand {
    #[clap(subcommand)]
    pub command: TokenCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum TokenCommands {
    /// generate a new keypair
    Generate,
}

impl TokenCommand {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;
        match &self.command {
            TokenCommands::Generate => {
                let encoding_key =
                    jsonwebtoken::EncodingKey::from_rsa_pem(config.secret_key.as_bytes())?;
                let claims = Claims::new(config.issuer);
                let token = claims.encode(&encoding_key).await?;
                println!("{token}");
            }
        }

        Ok(())
    }
}
