use dfa::DFA;
use regular_exp::ReExpr;

pub mod dfa;
pub mod regular_exp;

fn main() {
    let dfa = DFA::new_from_file(&"dfa.input".to_string()).unwrap();
    println!("{:?}", dfa);

    let regx = ReExpr::Or((
        Box::new(ReExpr::Value("1".to_string())),
        Box::new(ReExpr::Connect((
            Box::new(ReExpr::Value("0".to_string())),
            Box::new(ReExpr::Star(Box::new(ReExpr::Value("1".to_string())))),
        ))),
    ));
    println!("{}", regx.to_string());

    let mut a = ReExpr::Value("0".to_string());
    let b = ReExpr::Value("1".to_string());
    a.or(b);
    println!("{:?}", a);

    dfa.construct_regx();
}
