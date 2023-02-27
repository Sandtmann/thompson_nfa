use crate::fsa::FSA;
use crate::nfa::NFA;
use crate::regular_expressions::{REType};

mod regular_expressions;
mod nfa;
mod fsa;

fn main() {

    // eps ((a*)* (phi + b)) ---> (a*b)
    let test = Box::new(REType::Conc {
        left: Box::new(REType::Eps {}) ,
        right: Box::new(REType::Conc {
            left: Box::new(REType::Star {
                val: Box::new(REType::Star {
                    val: Box::new(REType::Char {val : 'a'})
                })
            }),
            right: Box::new(REType::Alt {
                left: Box::new(REType::Phi {}),
                right: Box::new(REType::Char {val : 'b'})
            })
        })
    });
    let words = ["", " ", "aa", "aab", "aaaaaaaab", "aaabbbb", "b", "123", "011", "110", "0011"];

    // Wikipedia example: binary numbers that are multiples of 3
    // let test = Box::new(REType::Star {val: Box::new(
    //     REType::Alt {left: Box::new(REType::Char {val: '0'}), right: Box::new(
    //         REType::Star {val: Box::new(REType::Conc {left: Box::new(
    //             REType::Char {val: '1'}), right: Box::new(REType::Conc {left: Box::new(
    //             REType::Star {val: Box::new(REType::Conc {left: Box::new(
    //                 REType::Char {val: '0'}), right: Box::new(REType::Conc {left: Box::new(
    //                 REType::Star {val: Box::new(REType::Char {val: '1'})}), right: Box::new(
    //                 REType::Conc {left: Box::new(REType::Star {val: Box::new(
    //                     REType::Conc {left: Box::new(
    //                         REType::Char {val: '0'}), right: Box::new(
    //                         REType::Char {val: '0'})})}), right: Box::new(
    //                     REType::Char {val: '0'})})})})}), right: Box::new(
    //             REType::Char {val: '1'})})})})})});
    // let words = ["", " ", "aa", "aab", "0","11", "000", "011", "0011", "0110", "1111"];
      

    // Simplify expression
    let simpl = test.simp();
    let expression_string = simpl.pretty();

    // NFA for expression
    let nfa = NFA::new(simpl);
    let nfa_string = nfa.to_string();

    // Create FSA for testing
    let mut test_fsa = FSA::new(nfa);

    println!("You entered the expression: {expression_string}");
    println!("The NFA is :\n{nfa_string}\n");
    for word in words {
        // Test words
        let result = test_fsa.run(word.to_string());
        println!("{word}\t\t{result}")
    }
}
