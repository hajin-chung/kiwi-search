use std::collections::HashMap;

use anyhow::Result;
use kiwi_rs::{Kiwi, Token};
use serde_json::Value;

use crate::index::SearchIndex;
use crate::tokenizer::should_keep_token;

pub struct SearchEngine {
    tokenizer: Kiwi,
    index: SearchIndex,
}

impl SearchEngine {
    pub fn new(tokenizer: Kiwi, index: SearchIndex) -> Self {
        Self { tokenizer, index }
    }

    pub fn search(&self, query: &str) -> Result<Vec<(f32, Value)>> {
        if self.index.corpus.is_empty() || self.index.avgdl == 0.0 {
            return Ok(Vec::new());
        }

        let query_tokens: Vec<Token> = self
            .tokenizer
            .tokenize(query)?
            .into_iter()
            .filter(should_keep_token)
            .collect();

        let mut scores: HashMap<usize, f32> = HashMap::new();
        let total_docs = self.index.corpus.len() as f32;

        for token in query_tokens {
            let Some(postings) = self.index.inverted_index.get(&token.form) else {
                continue;
            };

            let df = postings.len() as f32;
            let idf = (1.0 + (total_docs - df + 0.5) / (df + 0.5)).ln();

            for posting in postings {
                let doc = &self.index.corpus[posting.doc_idx];

                let tf = posting.term_frequency as f32;
                let doc_len = doc.token_len as f32;

                let tf_sat = (tf * (self.index.k1 + 1.0))
                    / (tf
                        + self.index.k1
                            * (1.0 - self.index.b + self.index.b * doc_len / self.index.avgdl));

                *scores.entry(posting.doc_idx).or_insert(0.0) += idf * tf_sat;
            }
        }

        let mut results: Vec<(f32, Value)> = scores
            .into_iter()
            .map(|(doc_idx, score)| {
                let doc = self.index.corpus[doc_idx].source.clone();
                (score, doc)
            })
            .collect();

        results.sort_by(|a, b| b.0.total_cmp(&a.0));

        Ok(results)
    }
}
