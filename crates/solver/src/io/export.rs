use std::{fs, path::Path};

use anyhow::Context;
use serde::Serialize;

use crate::solver::IterationStats;

#[derive(Debug, Serialize)]
pub struct StrategyExport {
    pub stats: Vec<IterationStats>,
    pub regret_count: usize,
}

pub fn export_json(path: &Path, data: &StrategyExport) -> anyhow::Result<()> {
    let raw = serde_json::to_string_pretty(data).context("serialize export")?;
    fs::write(path, raw).with_context(|| format!("write {path:?}"))?;
    Ok(())
}
