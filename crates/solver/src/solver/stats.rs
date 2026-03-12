use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct IterationStats {
    pub iteration: usize,
    pub root_ev_oop: f64,
    pub avg_abs_regret: f64,
}
