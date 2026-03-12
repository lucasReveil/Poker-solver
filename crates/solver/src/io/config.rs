use std::{fs, path::Path};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{game::SolverGame, tree::ActionTreeConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub board: String,
    pub oop_range: String,
    pub ip_range: String,
    pub game: SolverGame,
    pub tree: ActionTreeConfig,
    pub iterations: usize,
}

pub fn load_config(path: &Path) -> anyhow::Result<CliConfig> {
    let raw = fs::read_to_string(path).with_context(|| format!("reading {path:?}"))?;
    let cfg = serde_json::from_str(&raw).context("parsing json config")?;
    Ok(cfg)
}
