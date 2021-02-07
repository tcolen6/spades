use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use playing_cards::card::Card;
use playing_cards::hand::{Hand, CardInHand};
use playing_cards::deck::Deck;
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Default)]
pub struct Spades;

pub const ARENA_HEIGHT: f32 = 600.0;
pub const ARENA_WIDTH: f32 = 800.0;
pub const CARDS_IN_SUIT: i32 = 13;
pub const NUM_CARD_BACKS: i32 = 5;
pub const NUM_PLAYERS: u8 = 4;
pub const CARDS_IN_HAND: u8 = 13;
pub const CARD_HEIGHT: f32 = 144.0;
pub const CARD_WIDTH: f32 = 96.0;

#[derive(Copy, Clone, Eq, Hash)]
pub enum Player {
    North,
    East,
    South,
    West,
}

impl Player {
    pub fn next(player: &Self) -> Self {
         match player {
             Player::North => Player::East,
             Player::East => Player::South,
             Player::South => Player::West,
             Player::West => Player::North,
         }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        (*self as u8) == (*other as u8)
    }
}

impl From<Player> for u8 {
    fn from(value: Player) -> Self {
        match value {
            Player::North => 0,
            Player::East => 1,
            Player::South => 2,
            Player::West => 3,
        }
    }
}

impl From<u8> for Player {
    fn from(value: u8) -> Self {
        match value {
            0 => Player::North,
            1 => Player::East,
            2 => Player::South,
            _ => Player::West,
        }
    }
}

impl From<Player> for f32 {
    fn from(value: Player) -> Self {
        u8::from(value) as f32
    }
}

#[derive(Copy, Clone)]
pub enum CardPosition {
    Hand(u8),
    Play
}

pub struct CardEnt {
    pub position: CardPosition,
    pub card: Card,
    pub player: Player,
}

impl Component for CardEnt {
    type Storage = DenseVecStorage<Self>;
}

pub struct PlayerEnt{
    pub player: Player,
    pub num_cards: u8,
    pub hover_position: Option<u8>,
}

impl Component for PlayerEnt {
    type Storage = DenseVecStorage<Self>;
}

pub struct GameState {
    pub to_play: Player,
}


impl SimpleState for Spades{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        let mut deck: Deck = Deck::new();
        deck.shuffle();
        let hands: HashMap<Player, Hand> = deck.deal(vec![Player::North, Player::East, Player::South, Player::West], CARDS_IN_HAND).unwrap();

        initialize_game(world, sprite_sheet_handle, hands);
        initialize_camera(world);
    }
}


fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle: Handle<Texture> = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/card_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/card_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn get_sprite_location(card: Card) -> u8 {
    let suit: i32 = card.suit.into();
    let cards_before: u8 = (suit * CARDS_IN_SUIT + NUM_CARD_BACKS) as u8;
    cards_before + (card.value - 2)
}

fn get_card_location(pos: u8, hand_size: i32, player: Player) -> (f32, f32) {
    let card_disp: f32 = 20.0;
    let card_shift: f32 = (pos as i32 - hand_size / 2) as f32;
    match player {
        Player::North => (ARENA_WIDTH * 0.5 + card_disp * card_shift, 0.0),
        Player::South => (ARENA_WIDTH * 0.5 - card_disp * card_shift, ARENA_HEIGHT),
        Player::East => (0.0, ARENA_HEIGHT * 0.5 - card_disp * card_shift),
        Player::West=> (ARENA_WIDTH, ARENA_HEIGHT * 0.5 + card_disp * card_shift),
    }
}

fn initialize_game(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, players: HashMap<Player, Hand>) {
    world.insert(GameState{ to_play: Player::North });

    for (player, hand) in players {
        initialize_hand(world, sprite_sheet_handle.clone(), &hand, player);
    }
}

fn initialize_hand(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, hand: &Hand, player: Player) {
    let player_ent = PlayerEnt{ player , num_cards: CARDS_IN_HAND, hover_position: None};

    world.create_entity().with(player_ent).build();

    let hand_size: i32 = hand.cards.len() as i32;
    for card in hand.cards.iter().rev() {
        initialize_card(world, sprite_sheet_handle.clone(), card, hand_size, player);
    }
}

fn initialize_card(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, card: &CardInHand, hand_size: i32, player: Player) {
    let mut local_transform = Transform::default();
    let (card_x, card_y) = get_card_location(card.position, hand_size, player);
    local_transform.set_translation_xyz(card_x, card_y, 0.0);
    local_transform.rotate_2d(PI / 2.0 * f32::from(player));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: get_sprite_location(card.card).into(),
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(CardEnt{ position: CardPosition::Hand(card.position), card: card.card, player})
        .with(local_transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}
