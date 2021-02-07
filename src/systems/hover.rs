use crate::spades::{CardEnt, PlayerEnt, Player, CardPosition};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

enum HoverDirection {
    Left,
    Right,
}
#[derive(SystemDesc, Default)]
pub struct HoverSystem{
    key_down: bool,
}

impl<'s> System<'s> for HoverSystem {
    type SystemData = (
        WriteStorage<'s, PlayerEnt>,
        WriteStorage<'s, CardEnt>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut players, mut cards, mut transforms, input): Self::SystemData) {
        let dir: HoverDirection = {
            if input.action_is_down("left").unwrap_or(false) {
                HoverDirection::Left
            } else if input.action_is_down("right").unwrap_or(false) {
                HoverDirection::Right
            } else {
                self.key_down = false;
                return
            }
        };

        if self.key_down {
            return
        }
        self.key_down = true;

        for player in (&mut players).join() {
            if let Player::North = player.player {
                let prev: Option<u8> = player.hover_position;
                player.hover_position = match prev {
                    None => Some(0),
                    Some(pos) => match dir {
                        HoverDirection::Right => Some((pos + 1) % player.num_cards),
                        HoverDirection::Left => Some((pos + player.num_cards - 1) % player.num_cards),
                    }
                };

                for (card, transform) in (&mut cards, &mut transforms).join() {
                    if let Player::North = card.player {
                        if let CardPosition::Hand(pos) = card.position {
                            if Some(pos) == prev {
                                transform.move_down(20.0);
                            }

                            if Some(pos) == player.hover_position {
                                transform.move_up(20.0);
                            }
                        }
                    }
                }
            }
        }


    }
}
