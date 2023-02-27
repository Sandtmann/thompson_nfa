use  crate::nfa::NFA;

pub struct FSA {
    current:    Vec<i32>,
    nfa:        NFA,
}

impl FSA {

    // Associated function for constructing
    pub fn new(nfa: NFA) -> Self {
        Self {current: vec![], nfa}
    }

    // Add all states reachable with epsilon transition to self.current
    fn closure(&mut self) {
        let mut more = false;
        for state in self.current.clone() {
            for transition in self.nfa.get_transitions() {
                if transition.trigger(state, '\0') && !self.current.contains(&transition.to_state()){
                    self.current.push(transition.to_state());
                    more = true;
                }
            }
        }
        if more {
            self.closure();
        }
    }

    // Reset FSA
    fn reset(&mut self) {
        self.current.clear();
        self.current.push(self.nfa.get_initial());
        self.closure();
    }

    // Clear self.current and add all states reachable with certain char
    fn step(&mut self, char: char) {
        let mut next_states: Vec<i32> = vec![];
        for state in self.current.clone() {
            for transition in self.nfa.get_transitions() {
                if transition.trigger(state, char) && !next_states.contains(&transition.to_state()){
                    next_states.push(transition.to_state());
                }
            }
        }
        self.current.clear();
        self.current = next_states;
        self.closure();
    }

    // Check if final state
    fn is_final(&self) -> bool {
        if self.current.contains(&self.nfa.get_final()) {
            true
        } else {
            false
        }
    }

    // Test if word is creatable with FSA
    pub fn run(&mut self, word: String) -> bool {
        self.reset();
        let word_as_char = word.chars();
        for char in word_as_char {
            self.step(char);
        }
        self.is_final()
    }
}