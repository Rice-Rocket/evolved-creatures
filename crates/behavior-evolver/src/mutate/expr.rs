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


#[derive(Clone)]
pub struct RandomExprParams {
    pub value_weight: usize,
    pub const_weight: usize,
    pub const_range: Range<f32>,
    pub max_depth: usize,
    pub min_depth: usize,
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
        Box::new(self.build(rng, 0))
    }

    fn build(&self, rng: &mut ThreadRng, depth: usize) -> ExprNode {
        const NODE_WEIGHT: usize = 100;

        let range_min = if depth >= self.max_depth { NODE_WEIGHT } else { 0usize };
        let value_weight = if depth < self.min_depth { 0usize } else { self.value_weight } + NODE_WEIGHT;
        let const_weight = if depth < self.min_depth { 0usize } else { self.const_weight } + value_weight;

        let r = rng.gen_range(range_min..const_weight);
        if r < NODE_WEIGHT {
            match r {
                0..=39 => ExprNode::UnaryOp(ExprUnaryOp::rand_field(rng), Box::new(self.build(rng, depth + 1))),
                40..=79 => ExprNode::BinaryOp(
                    ExprBinaryOp::rand_field(rng),
                    Box::new(self.build(rng, depth + 1)),
                    Box::new(self.build(rng, depth + 1)),
                ),
                80..=99 => ExprNode::TernaryOp(
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
        Self { value_weight: 20, const_weight: 20, const_range: -10.0..10.0, min_depth: 1, max_depth: 3, joint_count: 1 }
    }
}


pub struct MutateExprParams {
    pub op_change_freq: f32,
    pub op_change_type_freq: f32,
    pub value_change_freq: f32,
    pub value_change_type_freq: f32,
    pub op_add_freq: f32,
    pub op_del_freq: f32,
    pub constant: MutateFieldParams,
    pub new_expr: RandomExprParams,
}

impl MutateExprParams {
    pub fn set_scale(&mut self, inv_scale: f32) {
        self.op_change_freq *= inv_scale;
        self.op_change_type_freq *= inv_scale;
        self.value_change_freq *= inv_scale;
        self.value_change_type_freq *= inv_scale;
        self.op_add_freq *= inv_scale;
        self.op_del_freq *= inv_scale;
        self.constant.set_scale(inv_scale);
    }
}

impl Default for MutateExprParams {
    fn default() -> Self {
        Self {
            op_change_freq: 0.2,
            op_change_type_freq: 0.1,
            value_change_freq: 0.2,
            value_change_type_freq: 0.15,
            op_add_freq: 0.03,
            op_del_freq: 0.1,
            constant: MutateFieldParams::new(0.25, 0.0, 0.25).unwrap(),
            new_expr: RandomExprParams { value_weight: 100, const_weight: 100, max_depth: 1, min_depth: 0, ..RandomExprParams::default() },
        }
    }
}


pub struct MutateExpr<'a> {
    rng: &'a mut ThreadRng,
    pub expr: &'a mut Expr,
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
        let size = Self::get_expr_size(&root);
        self.params.set_scale(1.0 / size as f32);
        self.expr.root = self.mutate_node(&root).as_ref().clone();
        self.params.set_scale(size as f32);
    }

    pub fn get_expr_size(node: &ExprNode) -> usize {
        match node {
            ExprNode::Value(_) => 1,
            ExprNode::Constant(_) => 1,
            ExprNode::UnaryOp(_, n) => Self::get_expr_size(n) + 1,
            ExprNode::BinaryOp(_, n1, n2) => Self::get_expr_size(n1) + Self::get_expr_size(n2) + 1,
            ExprNode::TernaryOp(_, n1, n2, n3) => Self::get_expr_size(n1) + Self::get_expr_size(n2) + Self::get_expr_size(n3) + 1,
        }
    }

    fn mutate_element(&mut self, element: &JointContextElement) -> JointContextElement {
        match element {
            JointContextElement::ParentContact { .. } => JointContextElement::ParentContact { face: LimbAttachFace::rand_field(self.rng) },
            JointContextElement::ChildContact { .. } => JointContextElement::ChildContact { face: LimbAttachFace::rand_field(self.rng) },
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
            0 => JointContextElement::ParentContact { face: LimbAttachFace::rand_field(self.rng) },
            1 => JointContextElement::ChildContact { face: LimbAttachFace::rand_field(self.rng) },
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

    fn mutate_node(&mut self, node: &ExprNode) -> Box<ExprNode> {
        let change_type = self.rng.gen_bool(self.params.op_change_type_freq as f64);

        if let ExprNode::Value(value) = node {
            let val = if self.rng.gen_bool(self.params.value_change_freq as f64) {
                if self.rng.gen_bool(self.params.value_change_type_freq as f64) {
                    Box::new(ExprNode::Value(match value {
                        CreatureContextElement::LocalJoint { element } => match self.rng.gen_range(0..2) {
                            0 => CreatureContextElement::GlobalJoint { element: *element, joint: self.random_joint_index() },
                            _ => CreatureContextElement::Time,
                        },
                        CreatureContextElement::GlobalJoint { element, .. } => match self.rng.gen_range(0..2) {
                            0 => CreatureContextElement::LocalJoint { element: *element },
                            _ => CreatureContextElement::Time,
                        },
                        CreatureContextElement::Time => match self.rng.gen_range(0..2) {
                            0 => CreatureContextElement::LocalJoint { element: self.random_element() },
                            _ => CreatureContextElement::GlobalJoint { element: self.random_element(), joint: self.random_joint_index() },
                        },
                    }))
                } else {
                    Box::new(ExprNode::Value(match value {
                        CreatureContextElement::LocalJoint { element } => {
                            CreatureContextElement::LocalJoint { element: self.mutate_element(element) }
                        },
                        CreatureContextElement::GlobalJoint { element, .. } => {
                            CreatureContextElement::GlobalJoint { element: self.mutate_element(element), joint: self.random_joint_index() }
                        },
                        CreatureContextElement::Time => CreatureContextElement::Time,
                    }))
                }
            } else {
                Box::new(ExprNode::Value(value.clone()))
            };

            if self.rng.gen_bool(self.params.op_add_freq as f64) {
                return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(self.rng), val));
            } else {
                return val;
            }
        }

        if let ExprNode::Constant(v) = node {
            let val = if self.params.constant.change(self.rng) {
                Box::new(ExprNode::Constant(ExprValue(self.params.constant.mutate(self.rng, v.0))))
            } else {
                Box::new(ExprNode::Constant(v.clone()))
            };

            if self.rng.gen_bool(self.params.op_add_freq as f64) {
                return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(self.rng), val));
            } else {
                return val;
            }
        }

        if let ExprNode::UnaryOp(op, x) = node {
            let inner = self.mutate_node(x);
            if change_type && self.rng.gen_bool(0.5) {
                return Box::new(ExprNode::BinaryOp(
                    ExprBinaryOp::rand_field(self.rng),
                    inner,
                    self.params.new_expr.build_single(self.rng),
                ));
            }
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(self.rng), inner));
            }
            return Box::new(ExprNode::UnaryOp(op.clone(), inner));
        }
        if let ExprNode::BinaryOp(op, a, b) = node {
            let inner_a = self.mutate_node(a);
            let inner_b = self.mutate_node(b);
            if change_type {
                if self.rng.gen_bool(0.5) {
                    return Box::new(ExprNode::UnaryOp(ExprUnaryOp::rand_field(self.rng), inner_a));
                } else {
                    return Box::new(ExprNode::TernaryOp(
                        ExprTernaryOp::rand_field(self.rng),
                        inner_a,
                        inner_b,
                        self.params.new_expr.build_single(self.rng),
                    ));
                }
            }
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::BinaryOp(ExprBinaryOp::rand_field(self.rng), inner_a, inner_b));
            }
            return Box::new(ExprNode::BinaryOp(op.clone(), inner_a, inner_b));
        }
        if let ExprNode::TernaryOp(op, a, b, c) = node {
            let inner_a = self.mutate_node(a);
            let inner_b = self.mutate_node(b);
            let inner_c = self.mutate_node(c);
            if change_type && self.rng.gen_bool(0.5) {
                return Box::new(ExprNode::BinaryOp(ExprBinaryOp::rand_field(self.rng), inner_a, inner_b));
            }
            if self.rng.gen_bool(self.params.op_change_freq as f64) {
                return Box::new(ExprNode::TernaryOp(ExprTernaryOp::rand_field(self.rng), inner_a, inner_b, inner_c));
            }
            return Box::new(ExprNode::TernaryOp(op.clone(), inner_a, inner_b, inner_c));
        }

        Box::new(node.clone())
    }
}

impl<'a> From<MutateExpr<'a>> for &'a Expr {
    fn from(val: MutateExpr<'a>) -> Self {
        val.into_inner()
    }
}
