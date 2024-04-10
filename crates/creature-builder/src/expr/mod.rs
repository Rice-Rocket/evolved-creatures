pub mod node;
pub mod value;

use node::ExprNode;
use serde::{Deserialize, Serialize};
use value::ExprValue;

use crate::effector::CreatureContext;


// It is okay to not have the effector be completely local.
// The joint context is allowed to take sensors from other parts of the
// creature. When mutating if a connection is broken (because the sensor from
// the other part of the creature was removed) or a value doesn't exist just
// default it to a constant (like 0) A mutation can change the constant or
// replace it with a new expr
//


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    pub root: ExprNode,
}

impl Expr {
    pub fn evaluate(&self, context: &CreatureContext) -> ExprValue {
        ExprNode::visit(&self.root, context)
    }
}
