
use crate::expression::Expression;

// Define a struct for TypeChecker
pub struct TypeChecker;

// Define types
#[derive(Debug, PartialEq)]
pub enum Type {
    None,
    Num,
    Bool,
}

// Define error
#[derive(Debug, PartialEq)]
pub struct TypeError {
    expected: Type,
    found: Type,
}

impl TypeChecker {
    pub fn visit(&self, expr: &Expression) -> Result<Type, TypeError> {
        match expr {
            Expression::None => Ok(Type::None),
            Expression::True | Expression::False => Ok(Type::Bool),
            Expression::Num(_) => Ok(Type::Num),
            Expression::Neg(e) => {
                let t = self.visit(e)?;
                if t == Type::Num {
                    Ok(Type::Num)
                } else {
                    Err(TypeError { expected: Type::Num, found: t })
                }
            }
            Expression::Add(e1, e2) => {
                let t1 = self.visit(e1)?;
                if t1 != Type::Num {
                    return Err(TypeError { expected: Type::Num, found: t1 });
                }
                let t2 = self.visit(e2)?;
                if t2 != Type::Num {
                    return Err(TypeError { expected: Type::Num, found: t2 });
                }
                Ok(Type::Num)
            }
            Expression::Or(e1, e2) => {
                let t1 = self.visit(e1)?;
                if t1 != Type::Bool {
                    return Err(TypeError { expected: Type::Bool, found: t1 });
                }
                let t2 = self.visit(e2)?;
                if t2 != Type::Bool {
                    return Err(TypeError { expected: Type::Bool, found: t2 });
                }
                Ok(Type::Bool)
            }
            Expression::Eq(e1, e2) => {
                let t1 = self.visit(e1)?;
                let t2 = self.visit(e2)?;
                if t1 == t2 {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError { expected: t1, found: t2 })
                }
            }
            Expression::If(e1, e2, e3) => {
                let t1 = self.visit(e1)?;
                let cond = self.visit(e2)?;
                if cond != Type::Bool {
                    return Err(TypeError { expected: Type::Bool, found: cond });
                }
                let t3 = self.visit(e3)?;
                if t1 == t3 {
                    Ok(t1)
                } else {
                    Err(TypeError { expected: t1, found: t3 })
                }
            }
            Expression::TEq(e1, e2, e3) => {
                let t1 = self.visit(e1)?;
                let t2 = self.visit(e2)?;
                let t3 = self.visit(e3)?;

                if t1 == t2 || t1 == t3 || t2 == t3 {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError { expected: t1, found: t2 })
                }
            }
        }
    }
}