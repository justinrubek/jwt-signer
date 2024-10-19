use rsa::{pkcs1::EncodeRsaPublicKey, pkcs8::EncodePrivateKey};
use tokio::io::AsyncWriteExt;
use tracing::info;

#[derive(clap::Args, Debug)]
pub(crate) struct KeyCommand {
    #[clap(subcommand)]
    pub command: KeyCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum KeyCommands {
    /// generate a new keypair
    Generate,
}

impl KeyCommand {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            KeyCommands::Generate => {
                info!("Generating keypair");

                let mut secret_file = tokio::fs::File::create("secret-rsa.pem").await?;
                let mut public_file = tokio::fs::File::create("public-rsa.pem").await?;

                create_rsa_keypair(&mut secret_file, &mut public_file).await?;
            }
        }

        Ok(())
    }
}

async fn create_rsa_keypair(
    secret_file: &mut tokio::fs::File,
    public_file: &mut tokio::fs::File,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let private_key = rsa::RsaPrivateKey::new(&mut rng, 2048)?;
    let public_key = private_key.to_public_key();
    info!("Public key: {:?}", public_key);

    let secret_pem = private_key.to_pkcs8_pem(Default::default())?;
    let public_pem = public_key.to_pkcs1_pem(Default::default())?;

    info!("writing keys");
    secret_file.write_all(secret_pem.as_bytes()).await?;
    public_file.write_all(public_pem.as_bytes()).await?;

    Ok(())
}
