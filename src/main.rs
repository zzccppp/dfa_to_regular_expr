use dfa::DFA;

pub mod dfa;
pub mod regular_exp;

fn main() {
    let dfa = DFA::new_from_file(&"dfa.input".to_string());
    println!("{:?}", dfa);
}
