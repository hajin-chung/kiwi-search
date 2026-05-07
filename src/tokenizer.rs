use anyhow::Result;
use kiwi_rs::{Kiwi, KiwiConfig, Token};

pub fn create_kiwi() -> Result<Kiwi> {
    let config = KiwiConfig::default()
        .with_library_path("/home/hajin/.local/kiwi/lib/libkiwi.so")
        .with_model_path("/home/hajin/.local/kiwi/models/cong/base");
    Ok(Kiwi::from_config(config)?)
}

pub fn should_keep_token(tok: &Token) -> bool {
    matches!(
        tok.tag.as_str(),
        "NNG" | "NNP" | "SL" | "SN" | "VV" | "VA" | "MAG" | "XR"
    )
}
