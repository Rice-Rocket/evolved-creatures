use data_structure_utils::graphs::directed::{NodeData, EdgeData, DirectedGraphResult};


pub struct LimbConnection {

}

impl EdgeData for LimbConnection {}


pub struct LimbNode {

}

impl NodeData<LimbConnection, BuildResult> for LimbNode {
    fn evaluate(&self, result: &mut BuildResult, from_node: Option<&Self>, from_edge: Option<&LimbConnection>) {
        
    }
}


pub struct BuildResult {

}

impl DirectedGraphResult for BuildResult {
    fn initial() -> Self {
        Self {

        }
    }
}