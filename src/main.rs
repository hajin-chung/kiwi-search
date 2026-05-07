mod daemon;
mod document;
mod engine;
mod index;
mod logger;
mod tokenizer;

use anyhow::Result;
use clap::{Parser, Subcommand};

use daemon::run_daemon;
use engine::SearchEngine;
use index::{read_documents, SearchIndex};
use logger::set_debug_log;
use tokenizer::create_kiwi;

#[derive(Parser)]
struct Cli {
    #[arg(long, global = true)]
    debug: bool,

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

    Daemon {
        #[arg(short, long)]
        index: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    set_debug_log(cli.debug);
    debug_log!("kiwi search");

    let kiwi = create_kiwi()?;
    debug_log!("created kiwi tokenizer");

    match cli.command {
        Command::Build {
            input,
            output,
            field,
        } => {
            let documents = read_documents(&input)?;
            debug_log!("loaded documents, count={}", documents.len());

            let index = SearchIndex::build(&kiwi, documents, field)?;
            debug_log!("built index");

            index.save(&output)?;
            debug_log!("saved index to {}", output);

            println!("Index built: {}", output);
        }

        Command::Search { index, query } => {
            let index = SearchIndex::load(&index)?;
            debug_log!("loaded index");

            let engine = SearchEngine::new(kiwi, index);
            let results = engine.search(&query)?;
            debug_log!("searched, results={}", results.len());

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

        Command::Daemon { index } => {
            let index = SearchIndex::load(&index)?;
            debug_log!("loaded index");

            let engine = SearchEngine::new(kiwi, index);

            run_daemon(engine)?;
        }
    }

    Ok(())
}
