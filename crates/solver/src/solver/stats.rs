use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct IterationStats {
    pub iteration: usize,
    pub root_ev_oop: f64,
    pub avg_abs_regret: f64,
    /// Sum of active OOP hand reach weights at root for this iteration.
    pub oop_reach_mass: f64,
    /// Sum of active IP hand reach weights at root for this iteration.
    pub ip_reach_mass: f64,
}
