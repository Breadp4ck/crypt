use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Vernam cipher
    Vernam {
        #[command(subcommand)]
        commands: VernamCommands,
    },
    /// Rc4 cipher
    Rc4 {
        #[command(subcommand)]
        commands: Rc4Commands,
    },
}

#[derive(Debug, Subcommand)]
enum VernamCommands {
    Encrypt {
        input: PathBuf,
        key: PathBuf,
        output: PathBuf,
    },
    Decrypt {
        input: PathBuf,
        key: PathBuf,
        output: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum Rc4Commands {
    Encrypt {
        input: PathBuf,
        key: String,
        output: PathBuf,
    },
    Decrypt {
        input: PathBuf,
        key: String,
        output: PathBuf,
    },
}

fn vernam_cipher(input_file: &PathBuf, key_file: &PathBuf, output_file: &PathBuf) {
    let mut input = File::open(input_file).expect("Unable to open input file");
    let mut key = File::open(key_file).expect("Unable to open key file");
    let mut output = File::create(output_file).expect("Unable to create output file");

    let mut input_buffer = Vec::new();
    let mut key_buffer = Vec::new();

    input
        .read_to_end(&mut input_buffer)
        .expect("Unable to read input file");
    key.read_to_end(&mut key_buffer)
        .expect("Unable to read key file");

    let encrypted: Vec<u8> = input_buffer
        .iter()
        .zip(key_buffer.iter())
        .map(|(&x, &k)| x ^ k)
        .collect();

    output.write_all(&encrypted).expect("Unable to write data");
}

struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    fn new(key: &[u8]) -> Self {
        let mut s = [0u8; 256];
        for (i, elem) in s.iter_mut().enumerate() {
            *elem = i as u8;
        }

        let mut j = 0;
        for i in 0..256 {
            j = (j + s[i] as usize + key[i % key.len()] as usize) & 0xFF;
            s.swap(i, j as usize);
        }

        Rc4 { s, i: 0, j: 0 }
    }

    fn process(&mut self, data: &[u8], out: &mut [u8]) {
        for (k, &byte) in data.iter().enumerate() {
            self.i = (self.i + 1) & 0xFF;
            self.j = ((self.j as usize + self.s[self.i as usize] as usize) & 0xFF) as u8;
            self.s.swap(self.i as usize, self.j as usize);

            let t = self.s[((self.s[self.i as usize] as usize + self.s[self.j as usize] as usize)
                & 0xFF) as usize];
            out[k] = byte ^ t;
        }
    }
}

fn rc4_cipher(input_file: &PathBuf, key: &[u8], output_file: &PathBuf) {
    let mut input = File::open(input_file).expect("Unable to open input file");
    let mut output = File::create(output_file).expect("Unable to create output file");

    let mut input_buffer = Vec::new();
    input
        .read_to_end(&mut input_buffer)
        .expect("Unable to read input file");

    let mut cipher = Rc4::new(key);
    let mut output_buffer = vec![0; input_buffer.len()];
    cipher.process(&input_buffer, &mut output_buffer);

    output
        .write_all(&output_buffer)
        .expect("Unable to write data");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Vernam { commands } => match commands {
            VernamCommands::Encrypt { input, key, output } => {
                vernam_cipher(&input, &key, &output);
            }
            VernamCommands::Decrypt { input, key, output } => {
                vernam_cipher(&input, &key, &output);
            }
        },
        Commands::Rc4 { commands } => match commands {
            Rc4Commands::Encrypt { input, key, output } => {
                rc4_cipher(&input, &key.as_bytes(), &output);
            }
            Rc4Commands::Decrypt { input, key, output } => {
                rc4_cipher(&input, &key.as_bytes(), &output);
            }
        },
    }
}
