use bevy::prelude::*;



#[derive(Resource)]
pub struct CreatureBuilderConfig {
    pub collision_types: ActiveCollisionTypes,
}

impl Default for CreatureBuilderConfig {
    fn default() -> Self {
        Self {
            collision_types: ActiveCollisionTypes::default()
        }
    }
}


pub struct ActiveCollisionTypes(u8);

bitflags::bitflags! {
    impl ActiveCollisionTypes: u8 {
        const NONE = 0;
        const LIMB_VS_LIMB = 1 << 0;
        const LIMB_VS_GROUND = 1 << 1;
        const ALL = Self::LIMB_VS_LIMB.bits() | Self::LIMB_VS_GROUND.bits();
    }
}

impl Default for ActiveCollisionTypes {
    fn default() -> Self {
        Self::LIMB_VS_GROUND
    }
}