use super::{super::effector::{JointContext, JointContextElement}, value::ExprValue};


#[derive(Clone)]
pub enum ExprNode {
    Value(JointContextElement),
    Constant(ExprValue),
    Add(Box<ExprNode>, Box<ExprNode>),
    Sub(Box<ExprNode>, Box<ExprNode>),
    Mul(Box<ExprNode>, Box<ExprNode>),
    Div(Box<ExprNode>, Box<ExprNode>),
}


impl ExprNode {
    pub fn visit(node: Box<ExprNode>, context: &JointContext) -> ExprValue {
        match *node {
            ExprNode::Value(element) => ExprValue(context[element]),
            ExprNode::Constant(val) => val,
            ExprNode::Add(expr_1, expr_2) => Self::visit(expr_1, context) + Self::visit(expr_2, context),
            ExprNode::Sub(expr_1, expr_2) => Self::visit(expr_1, context) - Self::visit(expr_2, context),
            ExprNode::Mul(expr_1, expr_2) => Self::visit(expr_1, context) * Self::visit(expr_2, context),
            ExprNode::Div(expr_1, expr_2) => Self::visit(expr_1, context) / Self::visit(expr_2, context),
        }
    }
}