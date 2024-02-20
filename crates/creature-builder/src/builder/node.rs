use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::dynamics::{GenericJointBuilder, JointAxesMask, LockedAxes};
use data_structure_utils::{graphs::directed::{NodeData, EdgeData, DirectedGraphResult, DirectedGraph, NodeID, EdgeID, DirectedGraphParameters}, queue::Queue, stack::Stack};

use super::{super::{limb::CreatureLimbBundle, joint::CreatureJointBuilder}, placement::LimbRelativePlacement};


pub struct LimbConnection {
    pub placement: LimbRelativePlacement,
    pub locked_axes: LockedAxes,
    pub limit_axes: [[f32; 2]; 6],
}

impl EdgeData for LimbConnection {}


pub struct LimbNode {
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
        _from_edge_id: EdgeID
    ) -> bool {
        match (from_node, from_edge) {
            (Some(_prev_node), Some(edge)) => {
                let is_terminal = if let Some(recursive_limit) = result.recursive_limits.get(&id) {
                    *recursive_limit == 0
                } else {
                    false
                };
                if is_terminal && !self.terminal_only { return false };
                let should_spawn = is_terminal == self.terminal_only;

                let prev_transform = result.transforms.get(&from_node_id).unwrap().peek().unwrap();
                let limb_position = edge.placement.create_transform(prev_transform);
                let prev_limb_id = result.node_limb_ids.get(&from_node_id).unwrap().peek().unwrap();
                let cur_limb_id = result.current_limb_id;
                if should_spawn {
                    result.limb_build_queue.push(
                        (
                            CreatureLimbBundle::new()
                                .with_transform(limb_position.transform.with_scale(Vec3::ONE))
                                .with_size(limb_position.transform.scale),
                            cur_limb_id,
                        )
                    );
                    result.joint_build_queue.push(
                        (
                            CreatureJointBuilder::new().with_generic_joint(
                                GenericJointBuilder::new(JointAxesMask::all())
                                    .local_anchor1(limb_position.parent_local_anchor)
                                    .local_anchor2(limb_position.local_anchor)
                                    .local_basis1(prev_transform.rotation.inverse() * limb_position.transform.rotation)
                                    .build()
                            ),
                            cur_limb_id, prev_limb_id
                        )
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
                    match result.node_limb_ids.get_mut(&id) {
                        Some(history) => { history.push(cur_limb_id) },
                        None => {
                            let mut history = Stack::new();
                            history.push(cur_limb_id);
                            result.node_limb_ids.insert(id, history);
                        }
                    }
                }

                result.current_limb_id += 1;

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
                let cur_limb_id = result.current_limb_id;
                result.limb_build_queue.push(
                    (
                        CreatureLimbBundle::new()
                            .with_transform(params.root_transform)
                            .with_size(params.root_transform.scale),
                        cur_limb_id
                    )
                );

                result.current_limb_id += 1;
                
                match result.transforms.get_mut(&id) {
                    Some(history) => { history.push(params.root_transform.with_scale(params.root_transform.scale)) },
                    None => {
                        let mut history = Stack::new();
                        history.push(params.root_transform.with_scale(params.root_transform.scale));
                        result.transforms.insert(id, history);
                    }
                };
                match result.node_limb_ids.get_mut(&id) {
                    Some(history) => { history.push(cur_limb_id) },
                    None => {
                        let mut history = Stack::new();
                        history.push(cur_limb_id);
                        result.node_limb_ids.insert(id, history);
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
        if let Some(history) = result.node_limb_ids.get_mut(&id) {
            history.pop();
        }
    }
}


#[derive(Resource)]
pub struct BuildResult {
    pub limb_build_queue: Queue<(CreatureLimbBundle, usize)>,
    pub joint_build_queue: Queue<(CreatureJointBuilder, usize, usize)>,
    recursive_limits: HashMap<NodeID, usize>,
    transforms: HashMap<NodeID, Stack<Transform>>,
    node_limb_ids: HashMap<NodeID, Stack<usize>>,
    current_limb_id: usize,
}

impl DirectedGraphResult for BuildResult {
    fn initial() -> Self {
        Self {
            limb_build_queue: Queue::new(),
            joint_build_queue: Queue::new(),
            recursive_limits: HashMap::new(),
            transforms: HashMap::new(),
            current_limb_id: 0,
            node_limb_ids: HashMap::new(),
        }
    }
}


pub struct BuildParameters {
    pub root_transform: Transform,
}

impl DirectedGraphParameters for BuildParameters {}


pub type CreatureMorphologyGraph = DirectedGraph<LimbNode, LimbConnection, BuildResult, BuildParameters>;