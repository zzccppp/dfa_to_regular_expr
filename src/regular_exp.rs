use std::ops::Deref;

const EPSILON_CHAR: char = 'ε';
const EMPTY_SET_CHAR: char = '∅';

#[derive(Debug, Clone, PartialEq)]
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
                lhs.simplify();
                rhs.simplify();

                if lhs == rhs {
                    *self = (**lhs).clone();
                    return;
                }

                if **lhs == Self::Null {
                    *self = (**rhs).clone();
                    return;
                }

                if **rhs == Self::Null {
                    *self = (**lhs).clone();
                    return;
                }

                if let Self::Connect((ll, rr)) = (*rhs).deref() {
                    if ll == lhs {
                        if let Self::Star(_) = rr.deref() {
                            *self = (**rhs).clone();
                        }
                    }
                    return;
                }

                if let Self::Connect((ll, rr)) = (*lhs).deref() {
                    if ll == rhs {
                        if let Self::Star(_) = rr.deref() {
                            *self = (**lhs).clone();
                        }
                    }
                    return;
                }
            }
            ReExpr::Star(ex) => {
                ex.simplify();

                if let Self::Epsilon = (*ex).deref() {
                    *self = Self::Epsilon;
                    return;
                } else if let Self::Or((lhs, rhs)) = (*ex).deref() {
                    //depends on the OR simplify
                    if lhs.deref() == &Self::Epsilon {
                        *self = Self::Star(Box::new(rhs.deref().clone()));
                        return;
                    }
                    if rhs.deref() == &Self::Epsilon {
                        *self = Self::Star(Box::new(lhs.deref().clone()));
                        return;
                    }
                }
            }
            ReExpr::Connect((lhs, rhs)) => {
                lhs.simplify();
                rhs.simplify();

                if let Self::Null = (*lhs).deref() {
                    *self = Self::Null;
                    return;
                }

                if let Self::Null = (*rhs).deref() {
                    *self = Self::Null;
                    return;
                }

                if let Self::Epsilon = (*lhs).deref() {
                    *self = (**rhs).clone();
                    return;
                }

                if let Self::Epsilon = (*rhs).deref() {
                    *self = (**lhs).clone();
                    return;
                }

                // (ep + r) r* = r*

                if let Self::Or((ll, rr)) = (*lhs).deref() {
                    if let Self::Star(r) = (*rhs).deref() {
                        if ll.deref() == &Self::Epsilon {
                            if rr == r {
                                *self = (**rhs).clone();
                            }
                            return;
                        }
                        if rr.deref() == &Self::Epsilon {
                            if ll == r {
                                *self = (**rhs).clone();
                            }
                            return;
                        }
                    }
                }
            }
            ReExpr::Null => {
                return;
            }
        }
    }
}
