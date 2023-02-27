use crate::regular_expressions::REType;

#[derive(Copy, Clone)]
pub struct Transition {
    from:   i32,
    to:     i32,
    char:   char,
}

pub struct NFA {
    transitions:    Vec<Transition>,
    init_state:           i32,
    final_state:         i32,
}

struct TransformWorker {
    name_supply: i32,
}

impl Transition{
    // Check for epsilon transition
    pub fn is_epsilon(&self) -> bool {
        if self.char == '\0' {
            true
        } else {
            false
        }
    }

    // Return to state
    pub fn to_state(&self) -> i32 {
        self.to
    }

    // Check if state can be left with certain char
    pub fn trigger(&self, _from: i32, _char: char) -> bool {
        return self.from == _from && self.char == _char;
    }

    // Ret char
    fn get_char(&self) -> char {
        self.char
    }

    // Representation as sting
    fn to_string(&self) -> String {
        let trans:String;
        if self.is_epsilon() {
            trans = self.from.to_string() + " -----> " + &self.to.to_string();
        } else {
            trans = self.from.to_string() + " --" + &self.char.to_string() + "--> " + &self.to.to_string();
        }
        return trans;
    }
}

impl NFA {

    // Associated function for constructing
    pub fn new(regular_expression: REType) -> Self {
        let transform_worker: TransformWorker = TransformWorker{name_supply: 0};
        let nfa: Box<NFA> = transform_worker.transform(&regular_expression);
        Self {transitions: nfa.get_transitions(), init_state: nfa.init_state, final_state: nfa.final_state }
    }

    // Return transactions
    pub fn get_transitions(&self) -> Vec<Transition> {
        return self.transitions.clone();
    }

    // Return initial state
    pub fn get_initial(&self) -> i32 {
        return self.init_state;
    }

    // Return finale state
    pub fn get_final(&self) -> i32 {
        return self.final_state;
    }

    // Representation of NFA in String format
    pub fn to_string(&self) -> String {
        let mut nfa_string: String = "Sigma = {".to_string();
        let mut helper_char: String = "".to_string();
        let mut helper_state: String = "Q = {".to_string() + &self.init_state.to_string() + ",";
        let mut vector_states: Vec<i32> = vec![];
        let mut helper_transitions: String = "".to_string();
        for transition in &self.transitions {
            let char = transition.get_char();
            let state = transition.to_state();
            if !transition.is_epsilon() && !helper_char.contains(char) {
                helper_char.push_str(&*(char.to_string() + ","))
            }
            if !vector_states.contains(&state) {
                vector_states.push(state);
            }
            helper_transitions.push_str(&*(transition.to_string() + "\n"));
        }
        vector_states.sort();
        for state in vector_states {
            helper_state.push_str(&*(state.to_string() + ","));
        }
        nfa_string.push_str(&*(helper_char.trim_end_matches(',').to_string() + "}\n"
            + helper_state.trim_end_matches(',') + "}\n"
            + "Initialzustand = " + &*self.init_state.to_string() + "\n"
            + "Finalzustand = " + &*self.final_state.to_string() + "\n"
            + "Transitionen:\n" + &*helper_transitions));
        return nfa_string;
    }
}

impl TransformWorker {
    fn init(&mut self){
        self.name_supply = 0;
    }

    fn fresh(&mut self) -> i32 {
        let value = self.name_supply;
        self.name_supply += 1;
        return value;
    }

    fn transform(mut self, regular_expression: &REType) -> Box<NFA> {
        self.init();
        return self.transform_worker(regular_expression);
    }

    fn transform_worker(&mut self, regular_expression: &REType) -> Box<NFA> {
        let mut transitions: Vec<Transition> = vec![];
        let start: i32;
        let stop: i32;

        match regular_expression {
            REType::Phi {} => {
                start = self.fresh();
                stop = self.fresh();
                Box::new(NFA{transitions, init_state:start, final_state:stop})
            }

            REType::Eps {} => {
                start = self.fresh();
                stop = self.fresh();

                let trans: Transition = Transition{from: start, to: stop, char: '\0'};
                transitions.push(trans);
                Box::new(NFA{transitions, init_state: start, final_state: stop})

            }

            REType::Char {val} => {
                start = self.fresh();
                stop = self.fresh();

                let trans: Transition = Transition{from: start, to: stop, char: *val };
                transitions.push(trans);
                Box::new(NFA{transitions, init_state: start, final_state: stop})
            }

            REType::Alt {left, right} => {

                let n1: Box<NFA> = self.transform_worker(left);
                let n2: Box<NFA> = self.transform_worker(right);
                start = self.fresh();
                stop = self.fresh();

                let n1_start = n1.get_initial();
                let n1_stop = n1.get_final();
                let n2_start = n2.get_initial();
                let n2_stop = n2.get_final();

                // let t1 = n1.get_transitions();
                // let t2 = n2.get_transitions();

                transitions.extend(n1.get_transitions());
                transitions.extend(n2.get_transitions());
                transitions.push(Transition{from: start, to: n1_start, char: '\0'});
                transitions.push(Transition{from: start, to: n2_start, char: '\0'});
                transitions.push(Transition{from: n1_stop, to: stop, char: '\0'});
                transitions.push(Transition{from: n2_stop, to: stop, char: '\0'});

                Box::new(NFA { transitions, init_state: start, final_state: stop })
            }

            REType::Conc {left,right} => {

                let n1: Box<NFA> = self.transform_worker(left);
                self.name_supply -= 1;
                let n2: Box<NFA> = self.transform_worker(right);

                let n1_start = n1.get_initial();
                let n2_stop = n2.get_final();

                // let t1 = n1.get_transitions();
                // let t2 = n2.get_transitions();

                transitions.extend(n1.get_transitions());
                transitions.extend(n2.get_transitions());

                Box::new(NFA{transitions, init_state: n1_start, final_state: n2_stop})
            }

            REType::Star {val} => {

                start = self.fresh();
                let n1: Box<NFA> = self.transform_worker(val);
                stop = self.fresh();

                let n1_start = n1.get_initial();
                let n1_stop = n1.get_final();

                transitions.extend(n1.get_transitions());
                transitions.push(Transition{from: start, to: n1_start, char: '\0'});
                transitions.push(Transition{from: start, to: stop, char: '\0'});
                transitions.push(Transition{from: n1_stop, to: n1_start, char: '\0'});
                transitions.push(Transition{from: n1_stop, to: stop, char: '\0'});

                Box::new(NFA{transitions, init_state: start, final_state: stop})
            }
        }
    }
}