use behavior_evolver::mutate::{
    edge::{MutateEdge, MutateEdgeParams},
    expr::{MutateExpr, MutateExprParams, RandomExprParams},
    node::{MutateNode, MutateNodeParams},
    MutateFieldParams, MutateMorphology, MutateMorphologyParams, RandomMorphologyParams,
};
use bevy::math::{Quat, Vec2, Vec3};
use bevy_rapier3d::dynamics::JointAxesMask;
use creature_builder::{
    builder::{
        node::{LimbConnection, LimbNode},
        placement::{LimbAttachFace, LimbRelativePlacement},
    },
    effector::CreatureJointEffectors,
    CreatureId,
};


#[test]
fn node() -> Result<(), rand_distr::NormalError> {
    let mut rng = rand::thread_rng();
    let mut node = LimbNode { name: None, density: 3.0, friction: 0.3, restitution: 0.0, terminal_only: false, recursive_limit: 2 };
    let params = MutateNodeParams {
        density: MutateFieldParams::new(1.0, 0.0, 0.1)?,
        friction: MutateFieldParams::new(0.1, 0.0, 0.1)?,
        restitution: MutateFieldParams::new(0.1, 0.0, 0.5)?,
        recursive: MutateFieldParams::new(0.5, 0.0, 0.75)?,
        terminal_freq: 0.3,
    };
    let mut mutate = MutateNode::new(&mut node, &mut rng, &params);

    for _ in 0..20 {
        mutate.mutate();
        println!("{:?}", mutate.inner());
    }

    Ok(())
}


#[test]
fn edge() -> Result<(), rand_distr::NormalError> {
    let mut rng = rand::thread_rng();
    let mut edge = LimbConnection {
        placement: LimbRelativePlacement {
            attach_face: LimbAttachFace::PosX,
            attach_position: Vec2::new(0.5, -0.3),
            orientation: Quat::from_rotation_x(0.5),
            scale: Vec3::ONE,
        },
        locked_axes: JointAxesMask::LIN_AXES,
        limit_axes: [[0.5, 0.5]; 6],
        effectors: CreatureJointEffectors::new([None, None, None, None, None, None]),
    };
    let params = MutateEdgeParams {
        placement_face_freq: 0.5,
        placement_pos: MutateFieldParams::new(1.0, 0.0, 0.05)?,
        placement_rot: MutateFieldParams::new(1.0, 0.0, 0.1)?,
        placement_scale: MutateFieldParams::new(1.0, 0.0, 0.075)?,
        limit_axes: MutateFieldParams::new(1.0, 0.0, 0.03)?,
    };

    let mut mutate = MutateEdge::new(&mut edge, &mut rng, &params);

    for _ in 0..20 {
        mutate.mutate();
        println!("{:?}", mutate.inner().limit_axes);
    }

    Ok(())
}


#[test]
fn expr() {
    let mut rng = rand::thread_rng();
    let mut expr = RandomExprParams::default().build_expr(&mut rng);
    let mut params = MutateExprParams::default();

    let mut mutate = MutateExpr::new(&mut expr, &mut rng, &mut params);

    let mut prev_val = 0;
    for _ in 0..1000 {
        mutate.mutate();
        let val = MutateExpr::<'_>::get_expr_size(&Box::new(mutate.expr.root.clone()));
        if prev_val != val {
            println!("{:?}", val)
        }
        prev_val = val;
    }
}


#[test]
fn morph() {
    let mut rng = rand::thread_rng();
    let mut morph = RandomMorphologyParams::default().build_morph(&mut rng, CreatureId(0));
    let mut params = MutateMorphologyParams::default();

    let mut mutate = MutateMorphology::new(&mut morph, &mut rng, &mut params);

    let mut prev = 0;
    for _ in 0..1000 {
        mutate.mutate();
        let val = match mutate.morph.graph.edges.values().last().unwrap().data.effectors.effectors.iter().find(|x| x.is_some()) {
            Some(v) => MutateExpr::<'_>::get_expr_size(&Box::new(v.clone().unwrap().expr.root)),
            None => {
                println!("None!");
                0
            },
        };
        if prev != val {
            println!("{:?}", val)
        }
        prev = val;
    }
}
