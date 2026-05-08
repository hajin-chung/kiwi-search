use std::io::{self, BufRead, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::engine::SearchEngine;

#[derive(Deserialize)]
struct DaemonRequest {
    id: u64,
    query: String,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct DaemonResponse {
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<Vec<SearchResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct SearchResult {
    score: f32,
    document: Value,
}

pub fn run_daemon(engine: SearchEngine, simple: bool) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());

    for line in stdin.lock().lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let request = if simple {
            Ok(DaemonRequest {
                id: 1,
                query: line,
                limit: Some(100),
            })
        } else {
            serde_json::from_str::<DaemonRequest>(&line)
        };

        let response = match request {
            Ok(request) => handle_daemon_request(&engine, request),
            Err(err) => DaemonResponse {
                id: 0,
                results: None,
                error: Some(format!("invalid request: {err}")),
            },
        };

        if simple {
            serde_json::to_writer_pretty(&mut stdout, &response)?;
        } else {
            serde_json::to_writer(&mut stdout, &response)?;
        }
        writeln!(stdout)?;
        stdout.flush()?;
    }

    Ok(())
}

fn handle_daemon_request(engine: &SearchEngine, request: DaemonRequest) -> DaemonResponse {
    let limit = request.limit.unwrap_or(10);

    match engine.search(&request.query) {
        Ok(results) => {
            let results = results
                .into_iter()
                .take(limit)
                .map(|(score, document)| SearchResult { score, document })
                .collect();

            DaemonResponse {
                id: request.id,
                results: Some(results),
                error: None,
            }
        }

        Err(err) => DaemonResponse {
            id: request.id,
            results: None,
            error: Some(err.to_string()),
        },
    }
}
