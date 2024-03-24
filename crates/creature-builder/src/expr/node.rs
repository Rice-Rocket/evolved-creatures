use random_derive::RandField;

use crate::{
    effector::{CreatureContext, CreatureContextElement},
    expr::value::ExprValue,
};


#[derive(Clone, Debug, RandField)]
pub enum ExprUnaryOp {
    Sign,
    Abs,
    Sin,
    Cos,
    Log,
    Exp,
    Sigmoid,
}

#[derive(Clone, Debug, RandField)]
pub enum ExprBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    GreaterThan,
    Min,
    Max,
    Atan,
}

#[derive(Clone, Debug, RandField)]
pub enum ExprTernaryOp {
    IfElse,
    Lerp,
}


#[derive(Clone, Debug)]
pub enum ExprNode {
    Value(CreatureContextElement),
    Constant(ExprValue),
    UnaryOp(ExprUnaryOp, Box<ExprNode>),
    BinaryOp(ExprBinaryOp, Box<ExprNode>, Box<ExprNode>),
    TernaryOp(ExprTernaryOp, Box<ExprNode>, Box<ExprNode>, Box<ExprNode>),
}


impl ExprNode {
    pub fn visit(node: &ExprNode, ctx: &CreatureContext) -> ExprValue {
        use ExprBinaryOp::*;
        use ExprTernaryOp::*;
        use ExprUnaryOp::*;

        let res = match node.clone() {
            ExprNode::Value(element) => ctx.index(element).map(ExprValue),
            ExprNode::Constant(val) => Some(val),

            ExprNode::UnaryOp(Sign, a) => Self::visit(&a, ctx).signum(),
            ExprNode::UnaryOp(Abs, a) => Self::visit(&a, ctx).abs(),
            ExprNode::UnaryOp(Sin, a) => Self::visit(&a, ctx).sin(),
            ExprNode::UnaryOp(Cos, a) => Self::visit(&a, ctx).cos(),
            ExprNode::UnaryOp(Log, a) => Self::visit(&a, ctx).ln(),
            ExprNode::UnaryOp(Exp, a) => Self::visit(&a, ctx).exp(),
            ExprNode::UnaryOp(Sigmoid, a) => Self::visit(&a, ctx).sigmoid(),

            ExprNode::BinaryOp(Add, a, b) => Self::visit(&a, ctx) + Self::visit(&b, ctx),
            ExprNode::BinaryOp(Sub, a, b) => Self::visit(&a, ctx) - Self::visit(&b, ctx),
            ExprNode::BinaryOp(Mul, a, b) => Self::visit(&a, ctx) * Self::visit(&b, ctx),
            ExprNode::BinaryOp(Div, a, b) => Self::visit(&a, ctx) / Self::visit(&b, ctx),
            ExprNode::BinaryOp(Mod, a, b) => Self::visit(&a, ctx).modulo(Self::visit(&b, ctx)),
            ExprNode::BinaryOp(GreaterThan, a, b) => Self::visit(&a, ctx).gt(Self::visit(&b, ctx)),
            ExprNode::BinaryOp(Min, a, b) => Self::visit(&a, ctx).min(Self::visit(&b, ctx)),
            ExprNode::BinaryOp(Max, a, b) => Self::visit(&a, ctx).max(Self::visit(&b, ctx)),
            ExprNode::BinaryOp(Atan, a, b) => Self::visit(&a, ctx).atan(Self::visit(&b, ctx)),

            ExprNode::TernaryOp(IfElse, a, b, c) => Self::visit(&a, ctx).if_else(Self::visit(&b, ctx), Self::visit(&c, ctx)),
            ExprNode::TernaryOp(Lerp, a, b, t) => Self::visit(&a, ctx).lerp(Self::visit(&b, ctx), Self::visit(&t, ctx)),
        };

        match res {
            Some(v) => v,
            None => ExprValue(0.0),
        }
    }
}
