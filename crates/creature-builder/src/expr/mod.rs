pub mod node;
pub mod value;

use node::ExprNode;


pub struct Expr {
    pub root: ExprNode,
}