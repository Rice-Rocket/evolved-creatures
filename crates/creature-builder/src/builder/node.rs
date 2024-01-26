use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::dynamics::LockedAxes;
use data_structure_utils::{graphs::directed::{NodeData, EdgeData, DirectedGraphResult, DirectedGraph, NodeID, EdgeID, DirectedGraphParameters}, queue::Queue, stack::Stack};

use super::{super::{limb::CreatureLimbBundle, joint::CreatureJointBuilder}, placement::LimbRelativePlacement};


pub struct LimbConnection {
    pub locked_axes: LockedAxes,
    pub limit_axes: [[f32; 2]; 6],
    pub local_anchor: Vec3,
    pub parent_local_anchor: Vec3,
}

impl EdgeData for LimbConnection {}


pub struct LimbNode {
    pub placement: LimbRelativePlacement,
    pub density: f32,
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
        from_edge_id: EdgeID
    ) -> bool {
        match (from_node, from_edge) {
            (Some(prev_node), Some(edge)) => {
                let is_terminal = if let Some(recursive_limit) = result.recursive_limits.get(&id) {
                    *recursive_limit == 0
                } else {
                    false
                };
                if is_terminal && !self.terminal_only { return false };
                let should_spawn = is_terminal == self.terminal_only;

                let prev_transform = result.transforms.get(&from_node_id).unwrap().peek().unwrap();
                let limb_position = self.placement.create_transform(prev_transform);
                if should_spawn {
                    result.limb_build_queue.push(
                        CreatureLimbBundle::new()
                            .with_transform(limb_position.transform.with_scale(Vec3::ONE))
                            .with_size(limb_position.transform.scale)
                    );
                }

                if !self.terminal_only && should_spawn {
                    // update transform history
                    match result.transforms.get_mut(&id) {
                        Some(history) => { history.push(limb_position.transform) },
                        None => {
                            let mut history = Stack::new();
                            history.push(limb_position.transform);
                            result.transforms.insert(id, history);
                        }
                    }
                }

                if is_terminal { return false };

                match result.recursive_limits.get_mut(&id) {
                    Some(recursive_limit) => {
                        *recursive_limit -= 1;
                    },
                    None => {
                        result.recursive_limits.insert(id, self.recursive_limit.max(1) - 1);
                    }
                }
            },
            _ => {
                result.limb_build_queue.push(
                    CreatureLimbBundle::new()
                        .with_transform(params.root_transform)
                        .with_size(self.placement.scale)
                );
                
                match result.transforms.get_mut(&id) {
                    Some(history) => { history.push(params.root_transform) },
                    None => {
                        let mut history = Stack::new();
                        history.push(params.root_transform.with_scale(self.placement.scale));
                        result.transforms.insert(id, history);
                    }
                };

                result.recursive_limits.insert(id, self.recursive_limit.max(1) - 1);
            }
        };
        true
    }
    fn on_leave(&self, result: &mut BuildResult, _params: &BuildParameters, id: NodeID) {
        if let Some(history) = result.transforms.get_mut(&id) {
            history.pop();
        }
    }
}


pub struct BuildResult {
    pub limb_build_queue: Queue<CreatureLimbBundle>,
    pub joint_build_queue: Queue<CreatureJointBuilder>,
    recursive_limits: HashMap<NodeID, usize>,
    transforms: HashMap<NodeID, Stack<Transform>>,
}

impl DirectedGraphResult for BuildResult {
    fn initial() -> Self {
        Self {
            limb_build_queue: Queue::new(),
            joint_build_queue: Queue::new(),
            recursive_limits: HashMap::new(),
            transforms: HashMap::new(),
        }
    }
}


pub struct BuildParameters {
    pub root_transform: Transform,
}

impl DirectedGraphParameters for BuildParameters {}


pub type CreatureMorphologyGraph = DirectedGraph<LimbNode, LimbConnection, BuildResult, BuildParameters>;