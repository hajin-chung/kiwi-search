use core::fmt;
use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Document {
    pub id: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenizedDocument {
    pub doc: Document,
    pub token_len: usize,
}

impl fmt::Display for TokenizedDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenizedDocument {{\n")?;
        write!(f, "\tdoc: {:?}\n", self.doc)?;
        write!(f, "\ttoken_len: {:?}\n", self.token_len)?;
        write!(f, "}}\n")
    }
}
