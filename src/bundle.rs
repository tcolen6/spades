use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};
use crate::systems::{HoverSystem, PlayCardSystem};

pub struct SpadesBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpadesBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(HoverSystem::default(), "hover_system", &["input_system"]);
        builder.add(PlayCardSystem::default(), "play_card_system", &["input_system"]);
        Ok(())
    }
}
