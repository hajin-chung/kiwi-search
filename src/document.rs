use core::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenizedDocument {
    pub source: Value,
    pub token_len: usize,
}

impl fmt::Display for TokenizedDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenizedDocument {{\n")?;
        write!(f, "\tsource: {:?}\n", self.source)?;
        write!(f, "\ttoken_len: {:?}\n", self.token_len)?;
        write!(f, "}}\n")
    }
}
