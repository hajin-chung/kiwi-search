mod document;
mod tokenizer;
mod index;
mod engine;

use anyhow::Result;
use clap::{Parser, Subcommand};

use tokenizer::create_kiwi;
use index::{SearchIndex, read_documents};
use engine::SearchEngine;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Build {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },

    Search {
        #[arg(short, long)]
        index: String,

        query: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let kiwi = create_kiwi()?;

    match cli.command {
        Command::Build { input, output } => {
            let documents = read_documents(&input)?;
            let index = SearchIndex::build(&kiwi, documents)?;
            index.save_json(&output)?;

            println!("Index built: {}", output);
        }

        Command::Search { index, query } => {
            let index = SearchIndex::load_json(&index)?;
            let engine = SearchEngine::new(kiwi, index);

            for (score, doc) in engine.search(&query)? {
                println!("score: {:.4} | {} | {}", score, doc.id, doc.content);
            }
        }
    }

    Ok(())
}
