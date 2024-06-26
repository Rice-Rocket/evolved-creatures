use std::ops::Range;

use creature_builder::builder::node::LimbNode;
use rand::{rngs::ThreadRng, Rng};

use super::MutateFieldParams;


#[derive(Clone)]
pub struct RandomNodeParams {
    pub density: Range<f32>,
    pub friction: Range<f32>,
    pub restitution: Range<f32>,
    pub terminal_freq: f32,
    pub recursive_limit: Range<usize>,
}

impl RandomNodeParams {
    pub fn build_node(&self, rng: &mut ThreadRng) -> LimbNode {
        LimbNode {
            name: None,
            density: rng.gen_range(self.density.clone()),
            friction: rng.gen_range(self.friction.clone()),
            restitution: rng.gen_range(self.restitution.clone()),
            terminal_only: rng.gen_bool(self.terminal_freq as f64),
            recursive_limit: rng.gen_range(self.recursive_limit.clone()),
        }
    }
}

impl Default for RandomNodeParams {
    fn default() -> Self {
        // Currently, terminal only is not support and will crash the program.
        // TODO: Fix this
        Self { density: 0.5..3.0, friction: 0.5..3.0, restitution: 0.1..0.9, terminal_freq: 0.0 /* 0.2 */, recursive_limit: 1..6 }
    }
}


#[derive(Clone)]
pub struct MutateNodeParams {
    pub density: MutateFieldParams,
    pub friction: MutateFieldParams,
    pub restitution: MutateFieldParams,
    pub recursive: MutateFieldParams,
    pub terminal_freq: f32,
}

impl MutateNodeParams {
    pub fn set_scale(&mut self, inv_scale: f32) {
        self.density.set_scale(inv_scale);
        self.friction.set_scale(inv_scale);
        self.restitution.set_scale(inv_scale);
        self.recursive.set_scale(inv_scale);
        self.terminal_freq *= inv_scale;
    }
}


pub struct MutateNode<'a> {
    rng: &'a mut ThreadRng,
    node: &'a mut LimbNode,
    params: &'a MutateNodeParams,
}

impl<'a> MutateNode<'a> {
    pub fn new(node: &'a mut LimbNode, rng: &'a mut ThreadRng, params: &'a MutateNodeParams) -> Self {
        Self { node, rng, params }
    }

    pub fn inner(&'a self) -> &'a LimbNode {
        self.node
    }

    pub fn into_inner(self) -> &'a LimbNode {
        self.node
    }

    pub fn mutate(&mut self) {
        if self.params.density.change(self.rng) {
            self.node.density = self.params.density.mutate(self.rng, self.node.density);
        };
        if self.params.friction.change(self.rng) {
            self.node.friction = self.params.friction.mutate(self.rng, self.node.friction);
        };
        if self.params.restitution.change(self.rng) {
            self.node.restitution = self.params.restitution.mutate(self.rng, self.node.restitution);
        };
        if self.params.recursive.change(self.rng) {
            self.node.recursive_limit =
                (self.node.recursive_limit as isize + self.params.recursive.sample(self.rng) as isize).max(1) as usize
        };
        if self.rng.gen_bool(self.params.terminal_freq as f64) {
            self.node.terminal_only = !self.node.terminal_only
        };
    }
}

impl<'a> From<MutateNode<'a>> for &'a LimbNode {
    fn from(val: MutateNode<'a>) -> Self {
        val.into_inner()
    }
}
