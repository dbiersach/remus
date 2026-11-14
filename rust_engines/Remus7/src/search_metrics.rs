//! Per-worker search accounting. Node costs exclude nested double counting.

#[derive(Clone, Copy)]
pub enum Research {
    Pvs,
    Lmr,
    Aspiration,
}

#[derive(Clone, Copy, Default)]
pub struct SearchMetrics {
    pub nodes: u64,
    pub active_ns: u64,
    pub wait_ns: u64,
    pub jobs: u64,
    pub pvs_researches: u64,
    pub lmr_researches: u64,
    pub aspiration_researches: u64,
    pub research_nodes: u64,
    pub cutoff_discarded_nodes: u64,
    pub stop_discarded_nodes: u64,
    pub cutoff_discarded_tasks: u64,
    pub cutoff_unclaimed_tasks: u64,
}

impl SearchMetrics {
    pub fn add(&mut self, other: Self) {
        self.nodes += other.nodes;
        self.active_ns += other.active_ns;
        self.wait_ns += other.wait_ns;
        self.jobs += other.jobs;
        self.pvs_researches += other.pvs_researches;
        self.lmr_researches += other.lmr_researches;
        self.aspiration_researches += other.aspiration_researches;
        self.research_nodes += other.research_nodes;
        self.cutoff_discarded_nodes += other.cutoff_discarded_nodes;
        self.stop_discarded_nodes += other.stop_discarded_nodes;
        self.cutoff_discarded_tasks += other.cutoff_discarded_tasks;
        self.cutoff_unclaimed_tasks += other.cutoff_unclaimed_tasks;
    }
}

impl std::fmt::Display for SearchMetrics {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "worker_nodes={} worker_active_ms={:.3} worker_wait_ms={:.3} worker_jobs={} pvs_researches={} lmr_researches={} aspiration_researches={} research_nodes={} cutoff_discarded_nodes={} stop_discarded_nodes={} cutoff_discarded_tasks={} cutoff_unclaimed_tasks={}",
            self.nodes,
            self.active_ns as f64 / 1_000_000.0,
            self.wait_ns as f64 / 1_000_000.0,
            self.jobs,
            self.pvs_researches,
            self.lmr_researches,
            self.aspiration_researches,
            self.research_nodes,
            self.cutoff_discarded_nodes,
            self.stop_discarded_nodes,
            self.cutoff_discarded_tasks,
            self.cutoff_unclaimed_tasks
        )
    }
}
