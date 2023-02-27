use std::mem;

// Six types of regular expressions
#[derive(PartialEq, Eq, Clone)]
pub enum REType {
    Phi {},
    Eps {},
    Char {val: char},
    Alt {left: Box<REType>, right: Box<REType>},
    Conc {left: Box<REType>, right: Box<REType>},
    Star {val: Box<REType>}
}

impl REType {

    // Check if REType results in phi
    fn is_phi(&self) -> bool {
        return match self {
            REType::Phi {} => {
                true
            }
            REType::Alt { left, right } => {
                left.is_phi() && right.is_phi()
            }
            REType::Conc { left, right } => {
                left.is_phi() || right.is_phi()
            }
            _ => {
                false
            }
        }
    }

    // Create String from regular expression
    pub(crate) fn pretty(&self) -> String {
        return match self {
            REType::Phi {} => {
                "phi".to_string()
            }

            REType::Eps {} => {
                "eps".to_string()
            }

            REType::Char { val } => {
                val.to_string()
            }

            REType::Alt { left, right } => {
                "(".to_string() + &left.pretty() + &"|".to_string()
                    + &right.pretty() + &")".to_string()
            }

            REType::Conc { left, right } => {
                "(".to_string() + &left.pretty() + &right.pretty() + &")".to_string()
            }

            REType::Star { val } => {
                val.pretty() + &"*".to_string()
            }
        }
    }

    // Simplify regular expression
    pub(crate) fn simp(&self) -> REType {
        match self {
            REType::Phi {} => {
                *Box::new(REType::Phi {})
            }
            REType::Eps {} => {
                *Box::new(REType::Eps {})
            }
            REType::Char {val} => {
                *Box::new(REType::Char {val: *val})
            }
            REType::Alt {left, right} => {
                if left.is_phi() {
                    right.simp()
                } else if right.is_phi() {
                    left.simp()
                } else if left.pretty() == right.pretty() {
                    left.simp()
                } else {
                    *Box::new(REType::Alt {left: Box::new(left.simp()),
                        right: Box::new(right.simp())})
                }
            }
            REType::Conc {left, right} => {
                // mem::discriminant is needed to compare enums while ignoring their given values
                if left.is_phi() || right.is_phi() {
                    *Box::new(REType::Phi {})
                } else if mem::discriminant(&REType::Eps {}) == mem::discriminant(&left) {
                    right.simp()
                } else if mem::discriminant(&REType::Eps {}) == mem::discriminant(&right) {
                    left.simp()
                } else {
                    *Box::new(REType::Conc {left: Box::new(left.simp()),
                        right: Box::new(right.simp())})
                }
            }
            REType::Star {val} => {
                // mem::discriminant is needed to compare enums while ignoring their given values
                // but a value has to assigned for creation
                if val.is_phi() {
                    *Box::new(REType::Eps {})
                } else if mem::discriminant(&REType::Star { val: Box::new(REType::Eps {}) }) == mem::discriminant(val) {
                    val.simp()
                } else {
                    *Box::new(REType::Star {val: Box::new(val.simp())})
                }
            }
        }
    }
}