mod document;
mod engine;
mod index;
mod tokenizer;

use anyhow::Result;
use clap::{Parser, Subcommand};

use engine::SearchEngine;
use index::{read_documents, SearchIndex};
use tokenizer::create_kiwi;

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

        #[arg(short, long)]
        field: String,
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
        Command::Build {
            input,
            output,
            field,
        } => {
            let documents = read_documents(&input)?;
            let index = SearchIndex::build(&kiwi, documents, field)?;
            index.save_json(&output)?;

            println!("Index built: {}", output);
        }

        Command::Search { index, query } => {
            let index = SearchIndex::load_json(&index)?;
            let engine = SearchEngine::new(kiwi, index);

            let results = engine.search(&query)?;

            let output: Vec<_> = results
                .into_iter()
                .map(|(score, document)| {
                    serde_json::json!({
                        "score": score,
                        "document": document
                    })
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}
