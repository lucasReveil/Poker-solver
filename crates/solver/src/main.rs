use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use poker_solver::{
    cards::Board,
    io::{export_json, load_config, StrategyExport},
    ranges::{expand_range, parse_range},
    solver::{solve, SolveInput},
    tree::compile_tree,
};

#[derive(Debug, Parser)]
#[command(author, version, about = "Heads-up NLHE postflop CFR solver MVP")]
struct Args {
    #[arg(long)]
    config: PathBuf,
    #[arg(long)]
    out: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let cfg = load_config(&args.config)?;

    let board: Board = cfg
        .board
        .parse()
        .map_err(|e: String| anyhow::anyhow!("invalid board: {e}"))?;
    let oop_range = expand_range(&parse_range(&cfg.oop_range)?).context("invalid oop range")?;
    let ip_range = expand_range(&parse_range(&cfg.ip_range)?).context("invalid ip range")?;

    let oop_range = oop_range.filter_board(&board);
    let ip_range = ip_range.filter_board(&board);

    let tree = compile_tree(&cfg.tree).map_err(|e| anyhow::anyhow!("invalid action tree: {e}"))?;
    let result = solve(SolveInput {
        game: cfg.game,
        board,
        oop_range,
        ip_range,
        tree,
        iterations: cfg.iterations,
    });

    let last = result.stats.last().cloned();
    if let Some(stat) = last {
        println!(
            "done iterations={} root_ev_oop={:.6} avg_abs_regret={:.6}",
            stat.iteration, stat.root_ev_oop, stat.avg_abs_regret
        );
    }

    if let Some(path) = args.out {
        export_json(
            &path,
            &StrategyExport {
                stats: result.stats,
                regret_count: result.tables.regrets.len(),
            },
        )?;
        println!("wrote {}", path.display());
    }

    Ok(())
}
