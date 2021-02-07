use crate::spades::{CardEnt, PlayerEnt, Player, CardPosition, ARENA_HEIGHT, ARENA_WIDTH, CARD_WIDTH, CARD_HEIGHT};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc, Default)]
pub struct PlayCardSystem{
    key_down: bool,
}

impl<'s> System<'s> for PlayCardSystem {
    type SystemData = (
        WriteStorage<'s, PlayerEnt>,
        WriteStorage<'s, CardEnt>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut players, mut cards, mut transforms, input): Self::SystemData) {
        if !input.action_is_down("select").unwrap_or(false) {
            self.key_down = false;
            return
        }
        if self.key_down {
            return
        }
        self.key_down = true;

        for player in (&mut players).join() {
            if let Player::North = player.player {
                for (card, transform) in (&mut cards, &mut transforms).join() {
                    if let Player::North = card.player {
                        if let CardPosition::Hand(pos) = card.position {
                            if let Some(play_pos) = player.hover_position {
                                if pos == play_pos {
                                    transform.set_translation_y(ARENA_HEIGHT * 0.5 - CARD_HEIGHT);
                                    transform.set_translation_x(ARENA_WIDTH * 0.5);
                                    card.position = CardPosition::Play;
                                } else if pos > play_pos {
                                    card.position = CardPosition::Hand(pos - 1);
                                    transform.move_left(20.0);
                                }
                            }
                        }
                    }
                }
                player.hover_position = None;
            }
        }
    }
}
