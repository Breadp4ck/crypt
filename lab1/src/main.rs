use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Attack the encrypted text
    Attack {
        #[command(subcommand)]
        commands: AttackCommands,
    },
    /// Encrypt provided string with key
    Encrypt { input: String, key: i32 },
    /// Decrypt provided string with key
    Decrypt { input: String, key: i32 },
}

#[derive(Debug, Subcommand)]
enum AttackCommands {
    /// Get key with provided open and encrypted text
    OpenText { open: String, encrypted: String },
    /// Get all possible decrypted texts based on provided encrypted text
    CypherText { encrypted: String },
    /// Decrypt text with provided encrypted text and english word dictionary
    CypherTextDyctionary {
        dictionary: PathBuf,
        encrypted: String,
    },
}

fn caesar_cipher(text: &str, shift: i32) -> String {
    let shift = (shift % 26 + 26) % 26; // Normalize shift to be within 0-25
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = first + (c as u8 - first + shift as u8) % 26;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn key_from_open_and_encrypted(open: &str, encrypted: &str) -> u8 {
    let first_open = open.chars().next().unwrap() as u8;
    let first_encrypted = encrypted.chars().next().unwrap() as u8;

    if first_encrypted > first_open {
        first_encrypted - first_open
    } else {
        first_encrypted + 26 - first_open
    }
}

fn count_real_words(input: &str, dict: &Vec<String>) -> usize {
    input
        .split_whitespace()
        .map(|word| {
            let mut result = String::with_capacity(word.len());
            for ch in word.chars() {
                if ch.is_ascii_alphabetic() {
                    result.push(ch);
                }
            }
            result
        })
        .filter(|word| dict.contains(&word))
        .count()
}

fn load_dictionary(path: &PathBuf) -> Vec<String> {
    let content = std::fs::read_to_string(path).unwrap();
    content.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Attack { commands } => match commands {
            AttackCommands::OpenText { open, encrypted } => {
                println!(
                    "Decrypted: {}",
                    key_from_open_and_encrypted(&open, &encrypted)
                );
            }
            AttackCommands::CypherText { encrypted } => {
                for shift in 0..26 {
                    println!(
                        "Key: {}\t Encrypted: {}",
                        shift,
                        caesar_cipher(&encrypted, 26 - shift)
                    );
                }
            }
            AttackCommands::CypherTextDyctionary {
                dictionary,
                encrypted,
            } => {
                let dict = load_dictionary(&dictionary);

                for shift in 0..26 {
                    let result = caesar_cipher(&encrypted, 26 - shift);
                    let matches = count_real_words(&result, &dict);

                    println!(
                        "Key: {}\t Matches: {}\t Encrypted: {}",
                        shift, matches, result
                    );
                }
            }
        },
        Commands::Encrypt { input, key } => {
            println!("Encrypted: {}", caesar_cipher(&input, key));
        }
        Commands::Decrypt { input, key } => {
            println!("Decrypted: {}", caesar_cipher(&input, -key));
        }
    }
}
