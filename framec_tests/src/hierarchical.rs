type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "hierarchical.rs"));

impl Hierarchical {
    pub fn entered(&mut self, state: String) {
        self.entry_log.push(state);
    }
    pub fn left(&mut self, state: String) {
        self.exit_log.push(state);
    }
    pub fn transition_hook(&mut self, _current: HierarchicalState, _next: HierarchicalState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // Parent entry event handlers are currently not being called on child state entry
    #[test]
    #[ignore]
    fn hierarchical_entry_calls() {
        let mut sm = Hierarchical::new();
        sm.entry_log.clear();
        sm.a();
        assert_eq!(sm.entry_log, vec!["S", "S0"]);
    }

    // Parent exit event handlers are currently not being called on exit from child state
    #[test]
    #[ignore]
    fn hierarchical_exit_calls() {
        let mut sm = Hierarchical::new();
        sm.a();
        sm.exit_log.clear();
        sm.c();
        assert_eq!(sm.exit_log, vec!["S", "S0"]);
    }

    #[test]
    fn hierarchical_current_state() {
        let mut sm = Hierarchical::new();
        assert_eq!(sm.state, HierarchicalState::I);
        sm.a();
        assert_eq!(sm.state, HierarchicalState::S0);
        sm.b();
        assert_eq!(sm.state, HierarchicalState::S1);
        sm.c();
        assert_eq!(sm.state, HierarchicalState::I);
    }
}
