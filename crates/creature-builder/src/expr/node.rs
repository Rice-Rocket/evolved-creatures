use crate::{effector::{CreatureContext, CreatureContextElement}, expr::value::ExprValue};


#[derive(Clone, Debug)]
pub enum ExprNode {
    Value(CreatureContextElement),
    Constant(ExprValue),
    Add(Box<ExprNode>, Box<ExprNode>),
    Sub(Box<ExprNode>, Box<ExprNode>),
    Mul(Box<ExprNode>, Box<ExprNode>),
    Div(Box<ExprNode>, Box<ExprNode>),
    Mod(Box<ExprNode>, Box<ExprNode>),
    GreaterThan(Box<ExprNode>, Box<ExprNode>),
    Sign(Box<ExprNode>),
    Min(Box<ExprNode>, Box<ExprNode>),
    Max(Box<ExprNode>, Box<ExprNode>),
    Abs(Box<ExprNode>),
    IfElse(Box<ExprNode>, Box<ExprNode>, Box<ExprNode>),
    Lerp(Box<ExprNode>, Box<ExprNode>, Box<ExprNode>),
    Sin(Box<ExprNode>),
    Cos(Box<ExprNode>),
    Atan(Box<ExprNode>, Box<ExprNode>),
    Log(Box<ExprNode>),
    Exp(Box<ExprNode>),
    Sigmoid(Box<ExprNode>),
}


impl ExprNode {
    pub fn visit(node: Box<ExprNode>, ctx: &CreatureContext) -> ExprValue {
        let res = match *node {
            ExprNode::Value(element) => {
                if let Some(elem) = ctx.index(element) {
                    Some(ExprValue(elem))
                } else {
                    None
                }
            },
            ExprNode::Constant(val) => Some(val),
            ExprNode::Add(a, b) => Self::visit(a, ctx) + Self::visit(b, ctx),
            ExprNode::Sub(a, b) => Self::visit(a, ctx) - Self::visit(b, ctx),
            ExprNode::Mul(a, b) => Self::visit(a, ctx) * Self::visit(b, ctx),
            ExprNode::Div(a, b) => Self::visit(a, ctx) / Self::visit(b, ctx),
            ExprNode::Mod(a, b) => Self::visit(a, ctx).modulo(Self::visit(b, ctx)),
            ExprNode::GreaterThan(a, b) => Self::visit(a, ctx).gt(Self::visit(b, ctx)),
            ExprNode::Sign(a) => Self::visit(a, ctx).signum(),
            ExprNode::Min(a, b) => Self::visit(a, ctx).min(Self::visit(b, ctx)),
            ExprNode::Max(a, b) => Self::visit(a, ctx).max(Self::visit(b, ctx)),
            ExprNode::Abs(a) => Self::visit(a, ctx).abs(),
            ExprNode::IfElse(a, b, c) => Self::visit(a, ctx).if_else(Self::visit(b, ctx), Self::visit(c, ctx)),
            ExprNode::Lerp(a, b, t) => Self::visit(a, ctx).lerp(Self::visit(b, ctx), Self::visit(t, ctx)),
            ExprNode::Sin(a) => Self::visit(a, ctx).sin(),
            ExprNode::Cos(a) => Self::visit(a, ctx).cos(),
            ExprNode::Atan(a, b) => Self::visit(a, ctx).atan(Self::visit(b, ctx)),
            ExprNode::Log(a) => Self::visit(a, ctx).ln(),
            ExprNode::Exp(a) => Self::visit(a, ctx).exp(),
            ExprNode::Sigmoid(a) => Self::visit(a, ctx).sigmoid()
        };

        match res {
            Some(v) => v,
            None => ExprValue(0.0),
        }
    }
}
