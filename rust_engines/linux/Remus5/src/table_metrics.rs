//! Worker-local table counters; no shared writes in the recursive search.

#[derive(Clone, Copy, Default)]
pub struct TableMetrics {
    pub probes: u64,
    pub hits: u64,
    pub cutoffs: u64,
    pub stores: u64,
    pub replacements: u64,
}

#[allow(dead_code)]
impl TableMetrics {
    pub fn add(&mut self, other: Self) {
        self.probes += other.probes;
        self.hits += other.hits;
        self.cutoffs += other.cutoffs;
        self.stores += other.stores;
        self.replacements += other.replacements;
    }
}

impl std::fmt::Display for TableMetrics {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "tt_probes={} tt_hits={} tt_cutoffs={} tt_stores={} tt_replacements={} tt_hit_pct={:.2} tt_cutoff_pct={:.2} tt_replacement_pct={:.2}",
            self.probes,
            self.hits,
            self.cutoffs,
            self.stores,
            self.replacements,
            100.0 * self.hits as f64 / self.probes.max(1) as f64,
            100.0 * self.cutoffs as f64 / self.probes.max(1) as f64,
            100.0 * self.replacements as f64 / self.stores.max(1) as f64
        )
    }
}
