use std::{collections::HashSet, ops::Range};

use creature_builder::{builder::node::CreatureMorphologyGraph, CreatureId};
use data_structure_utils::graphs::directed::{DirectedGraph, NodeID};
use rand::{rngs::ThreadRng, Rng};
use rand_distr::Normal;

use self::{node::{MutateNodeParams, MutateNode, RandomNodeParams}, edge::{MutateEdgeParams, MutateEdge, RandomEdgeParams}};

pub mod node;
pub mod edge;
pub mod expr;


pub struct MutateFieldParams {
    /// The frequency at which this field is changed
    pub f: f32,
    /// The distribution to sample when chosing a new value
    pub d: Normal<f32>,
    /// The range, if any, to clamp the result
    pub range: Option<Range<f32>>,
}

impl MutateFieldParams {
    pub fn new(freq: f32, mean: f32, std_dev: f32) -> Result<Self, rand_distr::NormalError> {
        Ok(Self {
            f: freq,
            d: Normal::new(mean, std_dev)?,
            range: None,
        })
    }
    pub fn in_range(mut self, range: Range<f32>) -> Self {
        self.range = Some(range);
        self
    }
    pub fn set_scale(&mut self, inv_scale: f32) {
        self.f *= inv_scale;
    }
    pub fn sample(&self, rng: &mut ThreadRng) -> f32 {
        match &self.range {
            Some(range) => rng.sample(self.d).clamp(range.start, range.end),
            None => rng.sample(self.d)
        }
    }
    pub fn mutate(&self, rng: &mut ThreadRng, old: f32) -> f32 {
        match &self.range {
            Some(range) => (rng.sample(self.d) + old).clamp(range.start, range.end),
            None => rng.sample(self.d) + old
        }
    }
    pub fn change(&self, rng: &mut ThreadRng) -> bool {
        rng.gen_bool(self.f as f64)
    }
    pub fn change_scaled(&self, rng: &mut ThreadRng, scale: f32) -> bool {
        rng.gen_bool((self.f / scale) as f64)
    }
}


pub struct RandomMorphologyParams {
    pub rand_node: RandomNodeParams,
    pub rand_edge: RandomEdgeParams,
    pub nodes: Range<usize>,
    pub edges: Range<usize>,
}

impl RandomMorphologyParams {
    pub fn build_morph(&self, rng: &mut ThreadRng, creature: CreatureId) -> CreatureMorphologyGraph {
        let mut graph = DirectedGraph::new();

        for _ in 0..rng.gen_range(self.nodes.clone()) {
            graph.add_node(self.rand_node.build_node(rng));
        }

        let n_nodes = graph.nodes.len();
        let node_ids: Vec<_> = graph.nodes.keys().map(|v| *v).collect();

        for _ in 0..rng.gen_range(self.edges.clone()) {
            graph.add_edge(self.rand_edge.build_edge(rng), node_ids[rng.gen_range(0..n_nodes)], node_ids[rng.gen_range(0..n_nodes)]);
        }

        let mut connected_nodes = HashSet::new();
        for edge in graph.edges.values() {
            connected_nodes.insert(edge.from);
            connected_nodes.insert(edge.to);
        }
        for node in graph.nodes.keys().map(|v| *v).collect::<Vec<NodeID>>().iter() {
            if !connected_nodes.contains(node) {
                graph.remove_node(*node);
            }
        }
        
        CreatureMorphologyGraph {
            graph,
            creature
        }
    }
}

impl Default for RandomMorphologyParams {
    fn default() -> Self {
        Self {
            rand_node: RandomNodeParams::default(),
            rand_edge: RandomEdgeParams::default(),
            nodes: 3..6,
            edges: 2..4
        }
    }
}


pub struct MutateMorphologyParams {
    pub node: MutateNodeParams,
    pub edge: MutateEdgeParams,
    pub rand_node: RandomNodeParams,
    pub rand_edge: RandomEdgeParams,
    /// The frequency at which edges choose a new node to point to
    pub edge_change_freq: f32,
    /// The frequency at which edges are deleted
    pub edge_del_freq: f32,
    /// The frequency at which edges are added
    pub edge_add_freq: f32,
    /// The inverse scale at which the sizes of creatures reduce the frequency of mutations
    pub size_inv_scale: f32,
}

impl MutateMorphologyParams {
    pub fn set_scale(&mut self, inv_scale: f32) {
        self.node.set_scale(inv_scale);
        self.edge.set_scale(inv_scale);
        self.edge_change_freq *= inv_scale;
        self.edge_del_freq *= inv_scale;
        self.edge_add_freq *= inv_scale;
    }
}

impl Default for MutateMorphologyParams {
    fn default() -> Self {
        Self {
            node: MutateNodeParams {
                density: MutateFieldParams::new(0.05, 0.0, 0.1).unwrap(),
                friction: MutateFieldParams::new(0.05, 0.0, 0.1).unwrap().in_range(0.1..0.9),
                restitution: MutateFieldParams::new(0.05, 0.0, 0.1).unwrap().in_range(0.1..0.9),
                recursive: MutateFieldParams::new(0.05, 0.0, 0.75).unwrap(),
                terminal_freq: 0.05,
            },
            edge: MutateEdgeParams {
                placement_face_freq: 0.05,
                placement_pos: MutateFieldParams::new(0.1, 0.0, 0.05).unwrap().in_range(-1.0..1.0),
                placement_rot: MutateFieldParams::new(0.1, 0.0, 0.1).unwrap(),
                placement_scale: MutateFieldParams::new(0.1, 0.0, 0.075).unwrap().in_range(0.05..20.0),
                limit_axes: MutateFieldParams::new(0.2, 0.0, 0.03).unwrap(),
            },
            rand_node: RandomNodeParams::default(),
            rand_edge: RandomEdgeParams::default(),
            edge_change_freq: 0.1,
            edge_del_freq: 0.1,
            edge_add_freq: 0.1,
            size_inv_scale: 1.0,
        }
    }
}


pub struct MutateMorphology<'a> {
    pub morph: &'a mut CreatureMorphologyGraph,
    pub rng: &'a mut ThreadRng,
    pub params: &'a mut MutateMorphologyParams,
}

impl<'a> MutateMorphology<'a> {
    pub fn new(morph: &'a mut CreatureMorphologyGraph, rng: &'a mut ThreadRng, params: &'a mut MutateMorphologyParams) -> Self {
        Self { morph, rng, params }
    }

    pub fn inner(&'a self) -> &'a CreatureMorphologyGraph {
        self.morph
    }
    pub fn into_inner(self) -> &'a CreatureMorphologyGraph {
        self.morph
    }

    pub fn mutate(&mut self) {
        let scale = self.params.size_inv_scale * self.morph.nodes_len() as f32;
        if scale != 0.0 { self.params.set_scale(1.0 / scale) };
        
        // Step 1: each node's internal parameters can mutate
        for node in self.morph.nodes_mut() {
            let mut mutate = MutateNode::new(&mut node.data, &mut self.rng, &self.params.node);
            mutate.mutate();
        }

        // Step 2: add a new random node
        self.morph.add_node(self.params.rand_node.build_node(self.rng));

        // Step 3: each edge's internal parameters can mutate
        let n_nodes = self.morph.nodes_len();
        let node_ids = self.morph.node_ids();
        for edge in self.morph.edges_mut() {
            let mut mutate = MutateEdge::new(&mut edge.data, &mut self.rng, &self.params.edge);
            mutate.mutate();
            if self.rng.gen_bool(self.params.edge_change_freq as f64) {
                edge.from = node_ids[self.rng.gen_range(0..n_nodes)];
            }
            if self.rng.gen_bool(self.params.edge_change_freq as f64) {
                edge.to = node_ids[self.rng.gen_range(0..n_nodes)];
            }
        }

        // Step 4: add and remove random edges
        for edge in self.morph.edge_ids() {
            if self.morph.edges_len() > 1 && self.rng.gen_bool(self.params.edge_del_freq as f64 / self.morph.edges_len() as f64) {
                self.morph.remove_edge(edge);
            }
        }
        let n_nodes = self.morph.nodes_len();
        let node_ids = self.morph.node_ids();
        for node in self.morph.node_ids() {
            if self.rng.gen_bool(self.params.edge_add_freq as f64 / self.morph.nodes_len() as f64) {
                self.morph.add_edge(self.params.rand_edge.build_edge(self.rng), node, node_ids[self.rng.gen_range(0..n_nodes)]);
            }
        }

        // Step 5: garbage collection
        let mut connected_nodes = HashSet::new();
        for edge in self.morph.edges() {
            connected_nodes.insert(edge.from);
            connected_nodes.insert(edge.to);
        }
        for node in self.morph.node_ids() {
            if !connected_nodes.contains(&node) {
                self.morph.remove_node(node);
            }
        }

        if scale != 0.0 { self.params.set_scale(scale) };
    }
}

impl<'a> Into<&'a CreatureMorphologyGraph> for MutateMorphology<'a> {
    fn into(self) -> &'a CreatureMorphologyGraph {
        self.into_inner()
    }
}
