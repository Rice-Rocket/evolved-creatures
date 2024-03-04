use creature_builder::builder::node::LimbNode;
use rand::{rngs::ThreadRng, Rng};

use super::MutateFieldParams;


pub struct MutateNodeParams {
    pub density: MutateFieldParams,
    pub recursive: MutateFieldParams,
    pub terminal_freq: f32,
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
        if self.params.density.change(&mut self.rng) {
            self.node.density += self.params.density.sample(&mut self.rng);
        }; 
        if self.params.recursive.change(&mut self.rng) {
            self.node.recursive_limit = (self.node.recursive_limit as isize + self.params.recursive.sample(&mut self.rng) as isize).max(0) as usize
        };
        if self.rng.gen_bool(self.params.terminal_freq as f64) {
            self.node.terminal_only = !self.node.terminal_only
        };
    }
}

impl<'a> Into<&'a LimbNode> for MutateNode<'a> {
    fn into(self) -> &'a LimbNode {
        self.into_inner()
    }
}
