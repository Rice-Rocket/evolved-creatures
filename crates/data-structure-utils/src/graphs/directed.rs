use std::{collections::HashMap, marker::PhantomData};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct NodeID(pub usize);
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct EdgeID(pub usize);


pub trait NodeData<E: EdgeData, R: DirectedGraphResult, P: DirectedGraphParameters> {
    fn evaluate(
        &self,
        result: &mut R,
        params: &P,
        id: NodeID,
        from_node: Option<&Self>,
        from_edge: Option<&E>,
        from_node_id: NodeID,
        from_edge_id: EdgeID,
    ) -> bool;
    fn on_leave(&self, result: &mut R, params: &P, id: NodeID);
}

#[derive(Debug)]
pub struct DirectedGraphNode<N: NodeData<E, R, P>, E: EdgeData, R: DirectedGraphResult, P: DirectedGraphParameters> {
    pub outs: Vec<EdgeID>,
    pub data: N,
    phantom: PhantomData<E>,
    phantom2: PhantomData<R>,
    phantom3: PhantomData<P>,
}

impl<N: NodeData<E, R, P>, E: EdgeData, R: DirectedGraphResult, P: DirectedGraphParameters> DirectedGraphNode<N, E, R, P> {
    pub fn new(data: N) -> DirectedGraphNode<N, E, R, P> {
        Self { outs: Vec::new(), data, phantom: PhantomData, phantom2: PhantomData, phantom3: PhantomData }
    }

    pub fn evaluate(
        &self,
        result: &mut R,
        params: &P,
        id: NodeID,
        from_node: Option<&DirectedGraphNode<N, E, R, P>>,
        from_edge: Option<&DirectedGraphEdge<E>>,
        from_node_id: NodeID,
        from_edge_id: EdgeID,
    ) -> bool {
        self.data.evaluate(
            result,
            params,
            id,
            match from_node {
                Some(node) => Some(&node.data),
                None => None,
            },
            match from_edge {
                Some(edge) => Some(&edge.data),
                None => None,
            },
            from_node_id,
            from_edge_id,
        )
    }

    pub fn on_leave(&self, result: &mut R, params: &P, id: NodeID) {
        self.data.on_leave(result, params, id);
    }
}


pub trait EdgeData {}

#[derive(Debug)]
pub struct DirectedGraphEdge<E: EdgeData> {
    pub from: NodeID,
    pub to: NodeID,
    pub data: E,
}


pub trait DirectedGraphResult {
    fn initial() -> Self;
}


pub trait DirectedGraphParameters {}


#[derive(Debug)]
pub struct DirectedGraph<N: NodeData<E, R, P>, E: EdgeData, R: DirectedGraphResult, P: DirectedGraphParameters> {
    root_node: Option<NodeID>,
    pub nodes: HashMap<NodeID, DirectedGraphNode<N, E, R, P>>,
    pub edges: HashMap<EdgeID, DirectedGraphEdge<E>>,
    cur_id: usize,
    phantom: PhantomData<R>,
    phantom2: PhantomData<P>,
}

impl<N: NodeData<E, R, P>, E: EdgeData, R: DirectedGraphResult, P: DirectedGraphParameters> Default for DirectedGraph<N, E, R, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: NodeData<E, R, P>, E: EdgeData, R: DirectedGraphResult, P: DirectedGraphParameters> DirectedGraph<N, E, R, P> {
    pub fn new() -> Self {
        Self { root_node: None, nodes: HashMap::new(), edges: HashMap::new(), cur_id: 0, phantom: PhantomData, phantom2: PhantomData }
    }

    /// Adds a node with user-defined `NodeData` to the graph and returns its
    /// `NodeID`
    pub fn add_node(&mut self, node: N) -> NodeID {
        self.cur_id += 1;
        self.nodes.insert(NodeID(self.cur_id), DirectedGraphNode::new(node));
        NodeID(self.cur_id)
    }

    /// Adds an edge with user-defined `EdgeData` to the graph and returns its
    /// `EdgeID`
    pub fn add_edge(&mut self, edge: E, from: NodeID, to: NodeID) -> Option<EdgeID> {
        self.cur_id += 1;
        let id = self.cur_id;
        self.get_node(to)?;

        if let Some(from_node) = self.get_node_mut(from) {
            from_node.outs.push(EdgeID(id));
        }

        self.edges.insert(EdgeID(self.cur_id), DirectedGraphEdge { from, to, data: edge });
        Some(EdgeID(self.cur_id))
    }

    /// Removes a node from the graph given its `NodeID` and returns the
    /// `NodeData` associated with that node if it exists
    pub fn remove_node(&mut self, id: NodeID) -> Option<N> {
        match self.nodes.remove(&id) {
            Some(node) => Some(node.data),
            None => None,
        }
    }

    /// Removes an edge from the graph given its `EdgeID` and returns the
    /// `EdgeData` associated with that node if it exists
    pub fn remove_edge(&mut self, id: EdgeID) -> Option<E> {
        match self.edges.remove(&id) {
            Some(edge) => Some(edge.data),
            None => None,
        }
    }

    /// Sets the root node of the graph given its `NodeID`
    pub fn set_root(&mut self, id: NodeID) {
        self.root_node = Some(id);
    }

    pub fn get_root(&self) -> Option<NodeID> {
        self.root_node
    }

    /// Borrows a node given its `NodeID`
    pub fn get_node(&self, id: NodeID) -> Option<&DirectedGraphNode<N, E, R, P>> {
        self.nodes.get(&id)
    }

    /// Mutably borrows a node given its `NodeID`
    pub fn get_node_mut(&mut self, id: NodeID) -> Option<&mut DirectedGraphNode<N, E, R, P>> {
        self.nodes.get_mut(&id)
    }

    /// Borrows an edge given its `EdgeID`
    pub fn get_edge(&self, id: EdgeID) -> Option<&DirectedGraphEdge<E>> {
        self.edges.get(&id)
    }

    /// Mutably borrows an edge given its `EdgeID`
    pub fn get_edge_mut(&mut self, id: EdgeID) -> Option<&mut DirectedGraphEdge<E>> {
        self.edges.get_mut(&id)
    }

    /// Borrows a node given its `NodeID` without checking if it exists
    pub fn get_node_unchecked(&self, id: NodeID) -> &DirectedGraphNode<N, E, R, P> {
        self.nodes.get(&id).unwrap()
    }

    /// Borrows an edge given its `EdgeID` without checking if it exists
    pub fn get_edge_unchecked(&self, id: EdgeID) -> &DirectedGraphEdge<E> {
        self.edges.get(&id).unwrap()
    }

    /// Mutably borrows a node given its `NodeID` without checking if it exists
    pub fn get_node_mut_unchecked(&mut self, id: NodeID) -> &mut DirectedGraphNode<N, E, R, P> {
        self.nodes.get_mut(&id).unwrap()
    }

    /// Mutably borrows an edge given its `EdgeID` without checking if it exists
    pub fn get_edge_mut_unchecked(&mut self, id: EdgeID) -> &mut DirectedGraphEdge<E> {
        self.edges.get_mut(&id).unwrap()
    }

    fn eval(&self, result: &mut R, params: &P, cur_id: NodeID) {
        let cur_node = self.get_node_unchecked(cur_id);

        for edge_id in cur_node.outs.iter() {
            let Some(edge) = self.get_edge(*edge_id) else { continue };
            let Some(next_node) = self.get_node(edge.to) else { continue };

            let should_continue = next_node.evaluate(result, params, edge.to, Some(cur_node), Some(edge), cur_id, *edge_id);
            if !should_continue {
                continue;
            };
            self.eval(result, params, edge.to);
            next_node.on_leave(result, params, edge.to);
        }
    }

    /// Evaluates the graph, returning the user-defined `DirectedGraphResult`
    pub fn evaluate(&self, params: P) -> R {
        let mut result = R::initial();
        let Some(root_id) = self.root_node else { panic!("No root node set for directed graph") };

        if let Some(root_node) = self.get_node(root_id) {
            root_node.evaluate(&mut result, &params, root_id, None, None, NodeID(0), EdgeID(0));
            self.eval(&mut result, &params, root_id);
            root_node.on_leave(&mut result, &params, root_id);
        };

        result
    }
}
