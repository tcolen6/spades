use crate::spades::{CardEnt, PlayerEnt, Player, CardPosition, ARENA_HEIGHT, ARENA_WIDTH, CARD_WIDTH, CARD_HEIGHT, GameState};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, Entity, Entities, WriteExpect, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc, Default)]
pub struct PlayCardSystem{
    key_down: bool,
}

impl PlayCardSystem {
    fn get_play_location(player: Player) -> (f32, f32) {
        match player {
            Player::North => (ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5 - CARD_HEIGHT),
            Player::South => (ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5 + CARD_HEIGHT),
            Player::East => (ARENA_WIDTH * 0.5 - CARD_HEIGHT, ARENA_HEIGHT * 0.5),
            Player::West => (ARENA_WIDTH * 0.5 + CARD_HEIGHT, ARENA_HEIGHT * 0.5),
        }
    }
}

impl<'s> System<'s> for PlayCardSystem {
    type SystemData = (
        WriteStorage<'s, PlayerEnt>,
        WriteStorage<'s, CardEnt>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        WriteExpect<'s, GameState>,
        Entities<'s>,
    );

    fn run(&mut self, (mut players, mut cards, mut transforms, input, mut game, entities): Self::SystemData) {
        if !input.action_is_down("select").unwrap_or(false) {
            self.key_down = false;
            return
        }
        if self.key_down {
            return
        }
        self.key_down = true;

        let mut played_cards: Vec<Entity> = vec![];
        let mut card_played: bool = false;

        for player in (&mut players).join() {
            if player.player != game.to_play { continue; }

            for (card, transform, ent) in (&mut cards, &mut transforms, &*entities).join() {
                if let CardPosition::Play = &card.position {
                    played_cards.push(ent);
                }
                if card.player != player.player { continue; }

                if let CardPosition::Hand(pos) = card.position {
                    if let Some(play_pos) = player.hover_position {
                        if pos == play_pos {
                            let (card_x, card_y) = Self::get_play_location(card.player);
                            transform.set_translation_y(card_y);
                            transform.set_translation_x(card_x);
                            card.position = CardPosition::Play;
                            card_played = true;
                        } else if pos > play_pos {
                            card.position = CardPosition::Hand(pos - 1);
                            transform.move_left(20.0);
                        }
                    }
                }
            }
            player.hover_position = None;
        }

        if card_played {
            if played_cards.len() >= 4 {
                for card in played_cards.iter() {
                    entities.delete(*card).unwrap();
                }
            }

            game.to_play = Player::next(&game.to_play);
        }
    }
}
