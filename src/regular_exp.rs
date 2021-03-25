const EPSILON_CHAR: char = 'ε';
const EMPTY_SET_CHAR: char = '∅';

#[derive(Debug, Clone)]
pub enum ReExpr {
    Epsilon,
    Value(String),
    Or((Box<ReExpr>, Box<ReExpr>)),
    Star(Box<ReExpr>),
    Connect((Box<ReExpr>, Box<ReExpr>)),
    Null,
}

impl ReExpr {
    pub fn to_string(&self) -> String {
        match self {
            ReExpr::Epsilon => String::from(EPSILON_CHAR),
            ReExpr::Value(v) => String::from(v),
            ReExpr::Or((lhs, rhs)) => {
                format!("({}+{})", lhs.to_string(), rhs.to_string())
            }
            ReExpr::Star(star) => {
                format!("({})*", star.to_string())
            }
            ReExpr::Connect((lhs, rhs)) => {
                format!("({}{})", lhs.to_string(), rhs.to_string())
            }
            ReExpr::Null => String::from(EMPTY_SET_CHAR),
        }
    }

    pub fn or(&mut self, other: Self) {
        *self = Self::Or((Box::new(self.clone()), Box::new(other)));
    }

    pub fn star(&mut self) {
        *self = Self::Star(Box::new(self.clone()));
    }

    pub fn connect(&mut self, other: Self) {
        *self = Self::Connect((Box::new(self.clone()), Box::new(other)));
    }

    pub fn simplify(&mut self) {
        match self {
            ReExpr::Epsilon => {
                return;
            }
            ReExpr::Value(v) => {
                return;
            }
            ReExpr::Or((lhs, rhs)) => {
                //fixit: impl this
            }
            ReExpr::Star(_) => {}
            ReExpr::Connect(_) => {}
            ReExpr::Null => {}
        }
    }
}
