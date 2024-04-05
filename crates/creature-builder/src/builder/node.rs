use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;
use bevy_rapier3d::dynamics::{GenericJointBuilder, JointAxesMask, JointAxis};
use data_structure_utils::{
    graphs::directed::{
        DirectedGraph, DirectedGraphEdge, DirectedGraphNode, DirectedGraphParameters, DirectedGraphResult, EdgeData, EdgeID, NodeData,
        NodeID,
    },
    stack::Stack,
};

use crate::{
    builder::placement::LimbRelativePlacement, effector::CreatureJointEffectors, joint::CreatureJointBuilder, limb::CreatureLimbBundle,
    CreatureId,
};


#[derive(Debug, Clone)]
pub struct LimbConnection {
    pub placement: LimbRelativePlacement,
    pub locked_axes: JointAxesMask,
    /// Ordered: [X, Y, Z, AngX, AngY, AngZ]
    pub limit_axes: [[f32; 2]; 6],
    pub effectors: CreatureJointEffectors,
}

impl EdgeData for LimbConnection {}


#[derive(Debug, Clone)]
pub struct LimbNode {
    pub name: Option<String>,
    pub density: f32,
    pub friction: f32,
    pub restitution: f32,
    pub terminal_only: bool,
    pub recursive_limit: usize,
}

impl NodeData<LimbConnection, BuildResult, BuildParameters> for LimbNode {
    fn evaluate(
        &self,
        result: &mut BuildResult,
        params: &BuildParameters,
        id: NodeID,
        from_node: Option<&Self>,
        from_edge: Option<&LimbConnection>,
        from_node_id: NodeID,
        _from_edge_id: EdgeID,
    ) -> bool {
        match (from_node, from_edge) {
            (Some(_prev_node), Some(edge)) => {
                let is_terminal = if let Some(recursive_limit) = result.recursive_limits.get(&id) { *recursive_limit == 0 } else { false };
                if is_terminal && !self.terminal_only {
                    return false;
                };
                let should_spawn = is_terminal == self.terminal_only;

                // ! Issue here
                let Some(prev_transform) = result.transforms.get(&from_node_id).map(|x| x.peek().unwrap()) else {
                    error!("Error: couldn't find previous transform when evaluating creature morphology graph. Probably because terminal only was set to true");
                    info!("id: {:?}", id);
                    info!("From node id: {:?}", from_node_id);
                    info!("Terminal only: {:?}", self.terminal_only);
                    panic!()
                };
                let limb_position = edge.placement.create_transform(prev_transform);
                let prev_limb_id = result.node_limb_ids.get(&from_node_id).unwrap().peek().unwrap();
                let cur_limb_id = result.current_limb_id;
                if should_spawn {
                    result.limb_build_queue.push_back((
                        CreatureLimbBundle::new()
                            .with_transform(limb_position.transform.with_scale(Vec3::ONE))
                            .with_name(match self.name.clone() {
                                Some(name) => name,
                                None => "()".to_string(),
                            })
                            .with_size(limb_position.transform.scale)
                            .with_density(self.density)
                            .with_friction(self.friction)
                            .with_restitution(self.restitution),
                        cur_limb_id,
                    ));
                    result.joint_build_queue.push_back((
                        CreatureJointBuilder::new()
                            .with_generic_joint(
                                GenericJointBuilder::new(edge.locked_axes)
                                    .limits(JointAxis::X, edge.limit_axes[0])
                                    .limits(JointAxis::Y, edge.limit_axes[1])
                                    .limits(JointAxis::Z, edge.limit_axes[2])
                                    .limits(JointAxis::AngX, edge.limit_axes[3])
                                    .limits(JointAxis::AngY, edge.limit_axes[4])
                                    .limits(JointAxis::AngZ, edge.limit_axes[5])
                                    .local_anchor1(limb_position.parent_local_anchor)
                                    .local_anchor2(limb_position.local_anchor)
                                    .local_basis1(prev_transform.rotation.inverse() * limb_position.transform.rotation)
                                    .build(),
                            )
                            .with_effectors(edge.effectors.clone()),
                        cur_limb_id,
                        prev_limb_id,
                    ));
                }

                if !self.terminal_only && should_spawn {
                    match result.transforms.get_mut(&id) {
                        Some(history) => history.push(limb_position.transform),
                        None => {
                            let mut history = Stack::new();
                            history.push(limb_position.transform);
                            result.transforms.insert(id, history);
                        },
                    }
                    match result.node_limb_ids.get_mut(&id) {
                        Some(history) => history.push(cur_limb_id),
                        None => {
                            let mut history = Stack::new();
                            history.push(cur_limb_id);
                            result.node_limb_ids.insert(id, history);
                        },
                    }
                }

                result.current_limb_id += 1;

                if is_terminal {
                    return false;
                };

                match result.recursive_limits.get_mut(&id) {
                    Some(recursive_limit) => {
                        *recursive_limit -= 1;
                    },
                    None => {
                        result.recursive_limits.insert(id, self.recursive_limit.max(1) - 1);
                    },
                }
            },
            _ => {
                let cur_limb_id = result.current_limb_id;
                result.limb_build_queue.push_back((
                    CreatureLimbBundle::new()
                        .with_transform(params.root_transform)
                        .with_name(match self.name.clone() {
                            Some(name) => name,
                            None => "()".to_string(),
                        })
                        .with_size(params.root_transform.scale),
                    cur_limb_id,
                ));

                result.current_limb_id += 1;

                match result.transforms.get_mut(&id) {
                    Some(history) => history.push(params.root_transform.with_scale(params.root_transform.scale)),
                    None => {
                        let mut history = Stack::new();
                        history.push(params.root_transform.with_scale(params.root_transform.scale));
                        result.transforms.insert(id, history);
                    },
                };
                match result.node_limb_ids.get_mut(&id) {
                    Some(history) => history.push(cur_limb_id),
                    None => {
                        let mut history = Stack::new();
                        history.push(cur_limb_id);
                        result.node_limb_ids.insert(id, history);
                    },
                };

                result.recursive_limits.insert(id, self.recursive_limit.max(1) - 1);
            },
        };
        true
    }

    fn on_leave(&self, result: &mut BuildResult, _params: &BuildParameters, id: NodeID) {
        if let Some(history) = result.transforms.get_mut(&id) {
            history.pop();
        }
        if let Some(history) = result.node_limb_ids.get_mut(&id) {
            history.pop();
        }
    }
}


#[derive(Resource, Debug, Clone)]
pub struct BuildResult {
    pub limb_build_queue: VecDeque<(CreatureLimbBundle, usize)>,
    pub joint_build_queue: VecDeque<(CreatureJointBuilder, usize, usize)>,
    recursive_limits: HashMap<NodeID, usize>,
    transforms: HashMap<NodeID, Stack<Transform>>,
    node_limb_ids: HashMap<NodeID, Stack<usize>>,
    current_limb_id: usize,
    creature_id: CreatureId,
}

impl DirectedGraphResult for BuildResult {
    fn initial() -> Self {
        Self {
            limb_build_queue: VecDeque::new(),
            joint_build_queue: VecDeque::new(),
            recursive_limits: HashMap::new(),
            transforms: HashMap::new(),
            current_limb_id: 0,
            node_limb_ids: HashMap::new(),
            creature_id: CreatureId(0),
        }
    }
}

impl BuildResult {
    pub fn align_to_ground(&mut self) {
        let lowest_limb = self
            .limb_build_queue
            .iter()
            .min_by(|a, b| {
                a.0.transform
                    .translation
                    .y
                    .partial_cmp(&b.0.transform.translation.y)
                    .expect("Could not compare transform y values, NAN likely in limb_build_queue")
            })
            .expect("limb_build_queue empty")
            .0
            .transform;

        let min_y = lowest_limb.translation.y - Vec3::Y.dot(lowest_limb.rotation * lowest_limb.scale).abs() - 1.0;
        self.limb_build_queue.iter_mut().for_each(|x| x.0.transform.translation.y -= min_y);
    }

    pub fn build(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
        let mut entity_ids = HashMap::new();
        let limb_count = self.limb_build_queue.len();
        while let Some(limb) = self.limb_build_queue.pop_front() {
            let id = commands
                .spawn(
                    limb.0
                        .with_color(Color::rgba(1.0, 1.0, 1.0, 0.8))
                        .with_creature(self.creature_id)
                        .with_limb_count(limb_count)
                        .finish(meshes, materials),
                )
                .id();
            entity_ids.insert(limb.1, id);
        }

        while let Some(joint) = self.joint_build_queue.pop_front() {
            let parent = entity_ids.get(&joint.2).unwrap();
            commands
                .entity(*entity_ids.get(&joint.1).unwrap())
                .insert(joint.0.with_parent(*parent).with_creature(self.creature_id).finish());
        }
    }
}


#[derive(Debug, Clone)]
pub struct BuildParameters {
    pub root_transform: Transform,
}

impl DirectedGraphParameters for BuildParameters {}


#[derive(Debug, Clone)]
pub struct CreatureMorphologyGraph {
    pub graph: DirectedGraph<LimbNode, LimbConnection, BuildResult, BuildParameters>,
    pub root: Transform,
    pub creature: CreatureId,
}

impl CreatureMorphologyGraph {
    pub fn new(creature: CreatureId) -> Self {
        Self { graph: DirectedGraph::new(), root: Transform::IDENTITY, creature }
    }

    pub fn add_node(&mut self, node: LimbNode) -> NodeID {
        self.graph.add_node(node)
    }

    pub fn add_edge(&mut self, edge: LimbConnection, from: NodeID, to: NodeID) -> Option<EdgeID> {
        self.graph.add_edge(edge, from, to)
    }

    pub fn remove_node(&mut self, node: NodeID) -> Option<LimbNode> {
        self.graph.remove_node(node)
    }

    pub fn remove_edge(&mut self, edge: EdgeID) -> Option<LimbConnection> {
        self.graph.remove_edge(edge)
    }

    pub fn set_root(&mut self, id: NodeID) {
        self.graph.set_root(id)
    }

    pub fn nodes_len(&self) -> usize {
        self.graph.nodes.len()
    }

    pub fn edges_len(&self) -> usize {
        self.graph.edges.len()
    }

    pub fn nodes(&self) -> Vec<&DirectedGraphNode<LimbNode, LimbConnection, BuildResult, BuildParameters>> {
        self.graph.nodes.values().collect()
    }

    pub fn edges(&self) -> Vec<&DirectedGraphEdge<LimbConnection>> {
        self.graph.edges.values().collect()
    }

    pub fn nodes_mut(&mut self) -> Vec<&mut DirectedGraphNode<LimbNode, LimbConnection, BuildResult, BuildParameters>> {
        self.graph.nodes.values_mut().collect()
    }

    pub fn edges_mut(&mut self) -> Vec<&mut DirectedGraphEdge<LimbConnection>> {
        self.graph.edges.values_mut().collect()
    }

    pub fn node_ids(&self) -> Vec<NodeID> {
        self.graph.nodes.keys().copied().collect()
    }

    pub fn edge_ids(&self) -> Vec<EdgeID> {
        self.graph.edges.keys().copied().collect()
    }

    pub fn nodes_map(&self) -> &HashMap<NodeID, DirectedGraphNode<LimbNode, LimbConnection, BuildResult, BuildParameters>> {
        &self.graph.nodes
    }

    pub fn edges_map(&self) -> &HashMap<EdgeID, DirectedGraphEdge<LimbConnection>> {
        &self.graph.edges
    }

    pub fn evaluate(&self) -> BuildResult {
        let mut res = self.graph.evaluate(BuildParameters { root_transform: self.root });
        res.creature_id = self.creature;
        res
    }
}
