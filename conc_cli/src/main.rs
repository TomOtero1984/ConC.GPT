use clap::{Parser, Subcommand};
use std::fs;

mod codec;
use codec::encoder;
use codec::decoder;
mod symbols;
use symbols::charset::SYMBOL_CHARSET;
use symbols::map::SymbolMap;


#[derive(Parser)]
#[command(
    name = "conc",
    about = "ConC.GPT - Compress and decompress English using symbolic encoding"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,

        #[arg(short = 'm', long)]
        map_dir: String,
    },
    Decode {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,

        #[arg(short = 'm', long)]
        map_dir: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input, output, map_dir } => {
            println!("Encoding: {} -> {}", input, output);
            let files: Vec<String> = fs::read_dir(map_dir)
                .expect("Failed to read map directory")
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            Some(path.to_string_lossy().into_owned())
                        } else {
                            None
                        }
                    })
                })
                .collect();
            let map_paths = files.iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let map = SymbolMap::from_jsonl_files(&map_paths);
            encoder::encode_file(input.clone(), output.clone(), &map);
        }
        Commands::Decode { input, output, map_dir } => {
            println!("Decoding: {} -> {}", input, output);
            let files: Vec<String> = fs::read_dir(map_dir)
                .expect("Failed to read map directory")
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            Some(path.to_string_lossy().into_owned())
                        } else {
                            None
                        }
                    })
                })
                .collect();

            let map_paths = files.iter().map(|s| s.as_str()).collect::<Vec<_>>();
            let map = SymbolMap::from_jsonl_files(&map_paths);
            decoder::decode_file(input, output, &map);
        }
    }
}
