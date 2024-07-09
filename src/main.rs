use std::fs;

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_decrypt, process_encode, process_encrypt,
    process_generate_key, process_genpass, process_sign, process_verify, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }

        SubCommand::Genpass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", String::from_utf8_lossy(&decoded));
            }
        },

        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let valid = process_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{}", valid);
            }
            TextSubCommand::GenerateKey(opts) => {
                let key = process_generate_key(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.text");
                        std::fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
            TextSubCommand::Encrypt(opts) => {
                let encrypted = process_encrypt(&opts.input, &opts.key)?;
                println!("{}", encrypted);
            }
            TextSubCommand::Decrypt(opts) => {
                let decrypted = process_decrypt(&opts.input, &opts.key)?;
                println!("{}", decrypted);
            }
        },
    }
    Ok(())
}
