
#[derive(Debug, Clone)]
pub enum ReExpr{
    Epsilon,
    Value(String),
    Or(Box<ReExpr>),
    Star(Box<ReExpr>),
    Connect((Box<ReExpr>, Box<ReExpr>)),
}