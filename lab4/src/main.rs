use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Encrypt provided string with key
    Encrypt {
        input: PathBuf,
        key: String,
        output: PathBuf,
    },
    /// Decrypt provided string with key
    Decrypt {
        input: PathBuf,
        key: String,
        output: PathBuf,
    },
}

fn encrypt_file(input: &PathBuf, key: &[u8], output: &PathBuf, iv: &[u8]) -> std::io::Result<()> {
    let data = std::fs::read(input)?;
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(&data);
    std::fs::write(output, &ciphertext)?;
    Ok(())
}

fn decrypt_file(input: &PathBuf, key: &[u8], output: &PathBuf, iv: &[u8]) -> std::io::Result<()> {
    let data = std::fs::read(input)?;
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&data).unwrap();
    std::fs::write(output, &decrypted_ciphertext)?;
    Ok(())
}

fn hash_from_key(key: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&key);

    let result = hasher.finalize();
    let hash: Vec<u8> = result[..16].to_vec();

    hash
}

fn main() {
    let iv = b"unique init vect";

    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, key, output } => {
            encrypt_file(&input, &hash_from_key(key), &output, iv).unwrap();
        }
        Commands::Decrypt { input, key, output } => {
            decrypt_file(&input, &hash_from_key(key), &output, iv).unwrap();
        }
    }
}
