use std::ops::Range;

use bevy_rapier3d::dynamics::JointAxis;
use creature_builder::{expr::{Expr, node::{ExprNode, ExprUnaryOp, ExprBinaryOp, ExprTernaryOp}, value::ExprValue}, effector::{CreatureContextElement, JointContextElement}, builder::placement::LimbAttachFace};
use rand::{rngs::ThreadRng, Rng};

pub struct RandomExprParams {
    pub value_weight: usize,
    pub const_weight: usize,
    pub const_range: Range<f32>,
    pub max_depth: usize,
    joint_count: usize,
}

impl RandomExprParams {
    pub fn build_expr(&self, rng: &mut ThreadRng) -> Expr {
        Expr { root: self.build(rng, 0) }
    }
    pub fn with_joint_count(mut self, count: usize) -> Self {
        self.joint_count = count;
        self
    }

    fn random_element(&self, rng: &mut ThreadRng) -> JointContextElement {
        match rng.gen_range(0usize..3) {
            0 => JointContextElement::ParentContact { face: LimbAttachFace::from_index(rng.gen_range(0..6)) },
            1 => JointContextElement::ChildContact { face: LimbAttachFace::from_index(rng.gen_range(0..6)) },
            2 => JointContextElement::JointAxis { axis: match rng.gen_range(0usize..6) {
                0 => JointAxis::X,
                1 => JointAxis::Y,
                2 => JointAxis::Z,
                3 => JointAxis::AngX,
                4 => JointAxis::AngY,
                5 => JointAxis::AngZ,
                _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
    fn build(&self, rng: &mut ThreadRng, depth: usize) -> ExprNode {
        let range_min = if depth > self.max_depth { 18usize } else { 0usize };
        let value_weight = self.value_weight + 18;
        let const_weight = value_weight + self.const_weight;

        let r = rng.gen_range(range_min..const_weight);
        if r < 18 {
            match r {
                0 => ExprNode::UnaryOp(ExprUnaryOp::Sign, Box::new(self.build(rng, depth + 1))),
                1 => ExprNode::UnaryOp(ExprUnaryOp::Abs, Box::new(self.build(rng, depth + 1))),
                2 => ExprNode::UnaryOp(ExprUnaryOp::Sin, Box::new(self.build(rng, depth + 1))),
                3 => ExprNode::UnaryOp(ExprUnaryOp::Cos, Box::new(self.build(rng, depth + 1))),
                4 => ExprNode::UnaryOp(ExprUnaryOp::Log, Box::new(self.build(rng, depth + 1))),
                5 => ExprNode::UnaryOp(ExprUnaryOp::Exp, Box::new(self.build(rng, depth + 1))),
                6 => ExprNode::UnaryOp(ExprUnaryOp::Sigmoid, Box::new(self.build(rng, depth + 1))),

                7 => ExprNode::BinaryOp(ExprBinaryOp::Add, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                8 => ExprNode::BinaryOp(ExprBinaryOp::Sub, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                9 => ExprNode::BinaryOp(ExprBinaryOp::Mul, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                10 => ExprNode::BinaryOp(ExprBinaryOp::Div, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                11 => ExprNode::BinaryOp(ExprBinaryOp::Mod, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                12 => ExprNode::BinaryOp(ExprBinaryOp::GreaterThan, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                13 => ExprNode::BinaryOp(ExprBinaryOp::Min, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                14 => ExprNode::BinaryOp(ExprBinaryOp::Max, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                15 => ExprNode::BinaryOp(ExprBinaryOp::Atan, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),

                16 => ExprNode::TernaryOp(ExprTernaryOp::IfElse, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                17 => ExprNode::TernaryOp(ExprTernaryOp::Lerp, Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1)), Box::new(self.build(rng, depth + 1))),
                _ => unreachable!()
            }
        } else if r < value_weight {
            ExprNode::Value(match rng.gen_range(0usize..3) {
                0 => CreatureContextElement::LocalJoint { element: self.random_element(rng) },
                1 => CreatureContextElement::GlobalJoint { element: self.random_element(rng), joint: rng.gen_range(0..self.joint_count) },
                2 => CreatureContextElement::Time,
                _ => unreachable!(),
            })
        } else {
            ExprNode::Constant(ExprValue(rng.gen_range(self.const_range.clone())))
        }
    }
}

impl Default for RandomExprParams {
    fn default() -> Self {
        Self {
            value_weight: 5,
            const_weight: 5,
            const_range: -10.0..10.0,
            max_depth: 3,
            joint_count: 1,
        }
    }
}


pub struct MutateExprParams {
    pub op_change_freq: f32,
}

impl MutateExprParams {
    pub fn set_scale(&mut self, inv_scale: f32) {
        self.op_change_freq *= inv_scale;
    }
}

impl Default for MutateExprParams {
    fn default() -> Self {
        Self {
            op_change_freq: 0.1,
        }
    }
}


pub struct MutateExpr<'a> {
    rng: &'a mut ThreadRng,
    expr: &'a mut Expr,
    params: &'a MutateExprParams,
}

impl<'a> MutateExpr<'a> {
    pub fn new(expr: &'a mut Expr, rng: &'a mut ThreadRng, params: &'a MutateExprParams) -> Self {
        Self { expr, rng, params }
    }

    pub fn inner(&'a self) -> &'a Expr {
        self.expr
    }
    pub fn into_inner(self) -> &'a Expr {
        self.expr
    }

    pub fn mutate(&mut self) {
        let root = Box::new(self.expr.root.clone());
        self.expr.root = self.mutate_node(&root).as_ref().clone();
    }
    fn mutate_node(&mut self, node: &Box<ExprNode>) -> Box<ExprNode> {
        if let ExprNode::UnaryOp(op, x) = node.as_ref() {
            let inner = self.mutate_node(x);
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(&mut self.rng), inner));
            }
            return Box::new(ExprNode::UnaryOp(op.clone(), inner));
        }
        if let ExprNode::BinaryOp(op, a, b) = node.as_ref() {
            let inner_a = self.mutate_node(a);
            let inner_b = self.mutate_node(b);
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::BinaryOp(ExprBinaryOp::rand_field(&mut self.rng), inner_a, inner_b));
            }
            return Box::new(ExprNode::BinaryOp(op.clone(), inner_a, inner_b));
        }

        node.clone()
    }
}

impl<'a> Into<&'a Expr> for MutateExpr<'a> {
    fn into(self) -> &'a Expr {
        self.into_inner()
    }
}
