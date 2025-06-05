
#[derive(Debug)]
pub enum Expression {
    None,
    True,
    False,
    Num(i32),
    Neg(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    TEq(Box<Expression>, Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::None => "None".to_string(),
            Expression::True => "true".to_string(),
            Expression::False => "false".to_string(),
            Expression::Num(n) => n.to_string(),
            Expression::Neg(e) => format!("-({})", e.to_string()),
            Expression::Add(e1, e2) => format!("({} + {})", e1.to_string(), e2.to_string()),
            Expression::Or(e1, e2) => format!("({} || {})", e1.to_string(), e2.to_string()),
            Expression::Eq(e1, e2) => format!("({} = {})", e1.to_string(), e2.to_string()),
            Expression::If(e1, e2, e3) => format!(
                "({} if {} else {})",
                e1.to_string(),
                e2.to_string(),
                e3.to_string()
            ),
            Expression::TEq(e1, e2, e3) => format!(
                "({} # {} # {})",
                e1.to_string(),
                e2.to_string(),
                e3.to_string()
            ),
        }
    }
}