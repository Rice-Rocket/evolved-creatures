use std::{collections::HashMap, marker::PhantomData};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct NodeID(usize);
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct EdgeID(usize);


pub trait NodeData<E: EdgeData, R: DirectedGraphResult> {
    fn evaluate(&self, result: &mut R, from_node: Option<&Self>, from_edge: Option<&E>);
}

pub struct DirectedGraphNode<N: NodeData<E, R>, E: EdgeData, R: DirectedGraphResult> {
    pub outs: Vec<EdgeID>,
    pub data: N,
    phantom: PhantomData<E>,
    phantom2: PhantomData<R>,
}

impl<N: NodeData<E, R>, E: EdgeData, R: DirectedGraphResult> DirectedGraphNode<N, E, R> {
    pub fn new(data: N) -> DirectedGraphNode<N, E, R> {
        Self { outs: Vec::new(), data, phantom: PhantomData, phantom2: PhantomData }
    }
    pub fn evaluate(&self, result: &mut R, from_node: Option<&DirectedGraphNode<N, E, R>>, from_edge: Option<&DirectedGraphEdge<E>>) {
        self.data.evaluate(
            result, 
            match from_node {
                Some(node) => Some(&node.data),
                None => None
            }, 
            match from_edge {
                Some(edge) => Some(&edge.data),
                None => None
            }
        );
    }
}


pub trait EdgeData {}

pub struct DirectedGraphEdge<E: EdgeData> {
    pub from: NodeID,
    pub to: NodeID,
    pub data: E,
}


pub trait DirectedGraphResult {
    fn initial() -> Self;
}


pub struct DirectedGraph<N: NodeData<E, R>, E: EdgeData, R: DirectedGraphResult> {
    root_node: Option<NodeID>,
    nodes: HashMap<NodeID, DirectedGraphNode<N, E, R>>,
    edges: HashMap<EdgeID, DirectedGraphEdge<E>>,
    cur_id: usize, 
    phantom: PhantomData<R>
}

impl<N: NodeData<E, R>, E: EdgeData, R: DirectedGraphResult> DirectedGraph<N, E, R> {
    pub fn new() -> Self {
        Self {
            root_node: None,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            cur_id: 0,
            phantom: PhantomData,
        }
    }

    /// Adds a node with user-defined `NodeData` to the graph and returns its `NodeID`
    pub fn add_node(&mut self, node: N) -> NodeID {
        self.cur_id += 1;
        self.nodes.insert(NodeID(self.cur_id), DirectedGraphNode::new(node));
        NodeID(self.cur_id)
    }
    /// Adds an edge with user-defined `EdgeData` to the graph and returns its `EdgeID`
    pub fn add_edge(&mut self, edge: E, from: NodeID, to: NodeID) -> Option<EdgeID> {
        self.cur_id += 1;
        let id = self.cur_id;
        if self.get_node(to).is_none() { return None }

        if let Some(from_node) = self.get_node_mut(from) {
            from_node.outs.push(EdgeID(id));
        }

        self.edges.insert(EdgeID(self.cur_id), DirectedGraphEdge { from, to, data: edge });
        Some(EdgeID(self.cur_id))
    }

    /// Removes a node from the graph given its `NodeID` and returns the `NodeData` associated with that node if it exists
    pub fn remove_node(&mut self, id: NodeID) -> Option<N> {
        match self.nodes.remove(&id) {
            Some(node) => Some(node.data),
            None => None
        }
    }
    /// Removes an edge from the graph given its `EdgeID` and returns the `EdgeData` associated with that node if it exists
    pub fn remove_edge(&mut self, id: EdgeID) -> Option<E> {
        match self.edges.remove(&id) {
            Some(edge) => Some(edge.data),
            None => None
        }
    }

    /// Sets the root node of the graph given its `NodeID`
    pub fn set_root(&mut self, id: NodeID) {
        self.root_node = Some(id);
    }

    /// Borrows a node given its `NodeID`
    pub fn get_node(&self, id: NodeID) -> Option<&DirectedGraphNode<N, E, R>> {
        self.nodes.get(&id)
    }
    /// Mutably borrows a node given its `NodeID`
    pub fn get_node_mut(&mut self, id: NodeID) -> Option<&mut DirectedGraphNode<N, E, R>> {
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
    pub fn get_node_unchecked(&self, id: NodeID) -> &DirectedGraphNode<N, E, R> {
        self.nodes.get(&id).unwrap()
    }
    /// Borrows an edge given its `EdgeID` without checking if it exists
    pub fn get_edge_unchecked(&self, id: EdgeID) -> &DirectedGraphEdge<E> {
        self.edges.get(&id).unwrap()
    }
    /// Mutably borrows a node given its `NodeID` without checking if it exists
    pub fn get_node_mut_unchecked(&mut self, id: NodeID) -> &mut DirectedGraphNode<N, E, R> {
        self.nodes.get_mut(&id).unwrap()
    }
    /// Mutably borrows an edge given its `EdgeID` without checking if it exists
    pub fn get_edge_mut_unchecked(&mut self, id: EdgeID) -> &mut DirectedGraphEdge<E> {
        self.edges.get_mut(&id).unwrap()
    }

    fn eval(&self, result: &mut R, cur_id: NodeID) {
        let cur_node = self.get_node_unchecked(cur_id);
        
        for edge_id in cur_node.outs.iter() {
            let Some(edge) = self.get_edge(*edge_id) else { continue };
            let Some(next_node) = self.get_node(edge.to) else { continue };
            
            next_node.evaluate(result, Some(cur_node), Some(edge));
            self.eval(result, edge.to);
        }
    }
    /// Evaluates the graph, returning the user-defined `DirectedGraphResult`
    pub fn evaluate(&self) -> R {
        let mut result = R::initial();
        let Some(root_id) = self.root_node else { panic!("No root node set for directed graph") };

        if let Some(root_node) = self.get_node(root_id) {
            root_node.evaluate(&mut result, None, None);
            self.eval(&mut result, root_id);
        };

        result
    }
}