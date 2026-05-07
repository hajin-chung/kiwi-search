use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::Result;
use kiwi_rs::{Kiwi, Token};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::document::TokenizedDocument;
use crate::tokenizer::should_keep_token;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Posting {
    pub doc_idx: usize,
    pub term_frequency: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchIndex {
    pub search_field: String,
    pub corpus: Vec<TokenizedDocument>,
    pub inverted_index: HashMap<String, Vec<Posting>>,
    pub avgdl: f32,
    pub k1: f32,
    pub b: f32,
}

impl SearchIndex {
    pub fn build(tokenizer: &Kiwi, documents: Vec<Value>, search_field: String) -> Result<Self> {
        let mut corpus = Vec::new();
        let mut inverted_index: HashMap<String, Vec<Posting>> = HashMap::new();
        let mut total_len = 0usize;

        for (doc_idx, doc) in documents.iter().enumerate() {
            let text = doc
                .get(&search_field)
                .and_then(|value| value.as_str())
                .unwrap_or("");

            let tokens: Vec<Token> = tokenizer
                .tokenize(text)?
                .into_iter()
                .filter(should_keep_token)
                .collect();

            let token_len = tokens.len();
            total_len += token_len;

            corpus.push(TokenizedDocument {
                source: doc.clone(),
                token_len,
            });

            let mut term_frequency: HashMap<String, u32> = HashMap::new();
            for token in &tokens {
                *term_frequency.entry(token.form.clone()).or_insert(0) += 1;
            }

            for (token, freq) in term_frequency {
                inverted_index.entry(token).or_default().push(Posting {
                    doc_idx,
                    term_frequency: freq,
                });
            }
        }

        let avgdl = if corpus.is_empty() {
            0.0
        } else {
            total_len as f32 / corpus.len() as f32
        };

        Ok(Self {
            search_field,
            corpus,
            inverted_index,
            avgdl,
            k1: 1.2,
            b: 0.4,
        })
    }

    pub fn save_json(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn load_json(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }
}

pub fn read_documents(path: &str) -> Result<Vec<Value>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
