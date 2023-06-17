use crate::ast::expressions::Expression;

struct ProtoType {
    name: String,
    args: Vec<Expression>,
}

struct Function {
    proto: ProtoType,
    body: Expression,
}
