use std::ops::Range;

use bevy_rapier3d::dynamics::JointAxis;
use creature_builder::{
    builder::placement::LimbAttachFace,
    effector::{CreatureContextElement, JointContextElement},
    expr::{
        node::{ExprBinaryOp, ExprNode, ExprTernaryOp, ExprUnaryOp},
        value::ExprValue,
        Expr,
    },
};
use rand::{rngs::ThreadRng, Rng};

use super::MutateFieldParams;


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
            2 => JointContextElement::JointAxis {
                axis: match rng.gen_range(0usize..6) {
                    0 => JointAxis::X,
                    1 => JointAxis::Y,
                    2 => JointAxis::Z,
                    3 => JointAxis::AngX,
                    4 => JointAxis::AngY,
                    5 => JointAxis::AngZ,
                    _ => unreachable!(),
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn build_single(&self, rng: &mut ThreadRng) -> Box<ExprNode> {
        Box::new(self.build(rng, self.max_depth + 1))
    }

    fn build(&self, rng: &mut ThreadRng, depth: usize) -> ExprNode {
        const NODE_WEIGHT: usize = 5;

        let range_min = if depth > self.max_depth { NODE_WEIGHT } else { 0usize };
        let value_weight = self.value_weight + NODE_WEIGHT;
        let const_weight = value_weight + self.const_weight;

        let r = rng.gen_range(range_min..const_weight);
        if r < NODE_WEIGHT {
            match r {
                0 | 1 => ExprNode::UnaryOp(ExprUnaryOp::rand_field(rng), Box::new(self.build(rng, depth + 1))),
                2 | 3 => ExprNode::BinaryOp(
                    ExprBinaryOp::rand_field(rng),
                    Box::new(self.build(rng, depth + 1)),
                    Box::new(self.build(rng, depth + 1)),
                ),
                4 => ExprNode::TernaryOp(
                    ExprTernaryOp::rand_field(rng),
                    Box::new(self.build(rng, depth + 1)),
                    Box::new(self.build(rng, depth + 1)),
                    Box::new(self.build(rng, depth + 1)),
                ),
                _ => unreachable!(),
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
        Self { value_weight: 2, const_weight: 2, const_range: -10.0..10.0, max_depth: 3, joint_count: 1 }
    }
}


pub struct MutateExprParams {
    pub op_change_freq: f32,
    pub change_type_freq: f32,
    pub value_change_freq: f32,
    pub value_change_type_freq: f32,
    pub constant: MutateFieldParams,
    pub new_expr: RandomExprParams,
}

impl MutateExprParams {
    pub fn set_scale(&mut self, inv_scale: f32) {
        self.op_change_freq *= inv_scale;
        self.change_type_freq *= inv_scale;
        self.value_change_freq *= inv_scale;
        self.value_change_type_freq *= inv_scale;
    }
}

impl Default for MutateExprParams {
    fn default() -> Self {
        Self {
            op_change_freq: 0.25,
            change_type_freq: 0.1,
            value_change_freq: 0.2,
            value_change_type_freq: 0.1,
            constant: MutateFieldParams::new(0.1, 0.0, 0.25).unwrap(),
            new_expr: RandomExprParams::default(),
        }
    }
}


pub struct MutateExpr<'a> {
    rng: &'a mut ThreadRng,
    expr: &'a mut Expr,
    params: &'a mut MutateExprParams,
}

impl<'a> MutateExpr<'a> {
    pub fn new(expr: &'a mut Expr, rng: &'a mut ThreadRng, params: &'a mut MutateExprParams) -> Self {
        Self { expr, rng, params }
    }

    pub fn with_joint_count(self, count: usize) -> Self {
        self.params.new_expr.joint_count = count;
        self
    }

    pub fn set_joint_count(&mut self, count: usize) {
        self.params.new_expr.joint_count = count;
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

    fn mutate_element(&mut self, element: &JointContextElement) -> JointContextElement {
        match element {
            JointContextElement::ParentContact { .. } => {
                JointContextElement::ParentContact { face: LimbAttachFace::rand_field(&mut self.rng) }
            },
            JointContextElement::ChildContact { .. } => {
                JointContextElement::ChildContact { face: LimbAttachFace::rand_field(&mut self.rng) }
            },
            JointContextElement::JointAxis { .. } => JointContextElement::JointAxis {
                axis: match self.rng.gen_range(0..6) {
                    0 => JointAxis::X,
                    1 => JointAxis::Y,
                    2 => JointAxis::Z,
                    3 => JointAxis::AngX,
                    4 => JointAxis::AngY,
                    5 => JointAxis::AngZ,
                    _ => unreachable!(),
                },
            },
        }
    }

    fn random_element(&mut self) -> JointContextElement {
        match self.rng.gen_range(0..3) {
            0 => JointContextElement::ParentContact { face: LimbAttachFace::rand_field(&mut self.rng) },
            1 => JointContextElement::ChildContact { face: LimbAttachFace::rand_field(&mut self.rng) },
            _ => JointContextElement::JointAxis {
                axis: match self.rng.gen_range(0..6) {
                    0 => JointAxis::X,
                    1 => JointAxis::Y,
                    2 => JointAxis::Z,
                    3 => JointAxis::AngX,
                    4 => JointAxis::AngY,
                    5 => JointAxis::AngZ,
                    _ => unreachable!(),
                },
            },
        }
    }

    fn random_joint_index(&mut self) -> usize {
        self.rng.gen_range(0usize..self.params.new_expr.joint_count)
    }

    fn mutate_node(&mut self, node: &Box<ExprNode>) -> Box<ExprNode> {
        let change_type = self.rng.gen_bool(self.params.change_type_freq as f64);

        if let ExprNode::Value(value) = node.as_ref() {
            if self.rng.gen_bool(self.params.value_change_freq as f64) {
                if self.rng.gen_bool(self.params.value_change_type_freq as f64) {
                    return Box::new(ExprNode::Value(match value {
                        CreatureContextElement::LocalJoint { element } => match self.rng.gen_range(0..2) {
                            0 => CreatureContextElement::GlobalJoint { element: element.clone(), joint: self.random_joint_index() },
                            _ => CreatureContextElement::Time,
                        },
                        CreatureContextElement::GlobalJoint { element, .. } => match self.rng.gen_range(0..2) {
                            0 => CreatureContextElement::LocalJoint { element: element.clone() },
                            _ => CreatureContextElement::Time,
                        },
                        CreatureContextElement::Time => match self.rng.gen_range(0..2) {
                            0 => CreatureContextElement::LocalJoint { element: self.random_element() },
                            _ => CreatureContextElement::GlobalJoint { element: self.random_element(), joint: self.random_joint_index() },
                        },
                    }));
                } else {
                    return Box::new(ExprNode::Value(match value {
                        CreatureContextElement::LocalJoint { element } => {
                            CreatureContextElement::LocalJoint { element: self.mutate_element(element) }
                        },
                        CreatureContextElement::GlobalJoint { element, .. } => {
                            CreatureContextElement::GlobalJoint { element: self.mutate_element(element), joint: self.random_joint_index() }
                        },
                        CreatureContextElement::Time => CreatureContextElement::Time,
                    }));
                }
            }
        }

        if let ExprNode::Constant(v) = node.as_ref() {
            if self.params.constant.change(&mut self.rng) {
                return Box::new(ExprNode::Constant(ExprValue(self.params.constant.mutate(&mut self.rng, v.0))));
            }
        }

        if let ExprNode::UnaryOp(op, x) = node.as_ref() {
            let inner = self.mutate_node(x);
            if change_type {
                if self.rng.gen_bool(0.5) {
                    return Box::new(ExprNode::BinaryOp(
                        ExprBinaryOp::rand_field(&mut self.rng),
                        inner,
                        self.params.new_expr.build_single(&mut self.rng),
                    ));
                } else {
                    return Box::new(ExprNode::TernaryOp(
                        ExprTernaryOp::rand_field(&mut self.rng),
                        inner,
                        self.params.new_expr.build_single(&mut self.rng),
                        self.params.new_expr.build_single(&mut self.rng),
                    ));
                }
            }
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(&mut self.rng), inner));
            }
            return Box::new(ExprNode::UnaryOp(op.clone(), inner));
        }
        if let ExprNode::BinaryOp(op, a, b) = node.as_ref() {
            let inner_a = self.mutate_node(a);
            let inner_b = self.mutate_node(b);
            if change_type {
                if self.rng.gen_bool(0.5) {
                    return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(&mut self.rng), inner_a));
                } else {
                    return Box::new(ExprNode::TernaryOp(
                        ExprTernaryOp::rand_field(&mut self.rng),
                        inner_a,
                        inner_b,
                        self.params.new_expr.build_single(&mut self.rng),
                    ));
                }
            }
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::BinaryOp(ExprBinaryOp::rand_field(&mut self.rng), inner_a, inner_b));
            }
            return Box::new(ExprNode::BinaryOp(op.clone(), inner_a, inner_b));
        }
        if let ExprNode::TernaryOp(op, a, b, c) = node.as_ref() {
            let inner_a = self.mutate_node(a);
            let inner_b = self.mutate_node(b);
            let inner_c = self.mutate_node(c);
            if change_type {
                if self.rng.gen_bool(0.5) {
                    return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(&mut self.rng), inner_a));
                } else {
                    return Box::new(ExprNode::BinaryOp(ExprBinaryOp::rand_field(&mut self.rng), inner_a, inner_b));
                }
            }
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::TernaryOp(ExprTernaryOp::rand_field(&mut self.rng), inner_a, inner_b, inner_c));
            }
            return Box::new(ExprNode::TernaryOp(op.clone(), inner_a, inner_b, inner_c));
        }

        node.clone()
    }
}

impl<'a> Into<&'a Expr> for MutateExpr<'a> {
    fn into(self) -> &'a Expr {
        self.into_inner()
    }
}
