use bevy::prelude::*;


pub type BlocPosition = IVec3;
#[derive(Component, Debug)]
pub struct Bloc {
    pub pos: BlocPosition,
    pub id: Option<Entity>,
}
impl Bloc {
    pub fn new(pos: BlocPosition, id: Option<Entity>) -> Self {
        Self { 
            pos,
            id,
        }
    }
}