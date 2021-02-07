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


pub enum Player {
    North,
    South,
    East,
    West,
}

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


impl SimpleState for Spades{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        let mut deck: Deck = Deck::new();
        deck.shuffle();
        let hands: HashMap<u8, Hand> = deck.deal(NUM_PLAYERS, CARDS_IN_HAND).unwrap();
        let hand: &Hand = hands.get(&0).unwrap();

        world.register::<CardEnt>();
        world.register::<PlayerEnt>();

        initialize_hand(world, sprite_sheet_handle, hand);
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

fn get_card_location(pos: u8, hand_size: i32) -> (f32, f32) {
    let card_disp: f32 = 20.0;
    let x_disp: f32 = card_disp * ((pos as i32 - hand_size / 2) as f32);
    (ARENA_WIDTH * 0.5 + x_disp, 0.0)
}

fn initialize_hand(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, hand: &Hand) {
    let player = PlayerEnt{ player: Player::North, num_cards: CARDS_IN_HAND, hover_position: None };

    world.create_entity().with(player).build();

    let hand_size: i32 = hand.cards.len() as i32;
    for card in hand.cards.iter().rev() {
        initialize_card(world, sprite_sheet_handle.clone(), card, hand_size, Player::North);
    }
}

fn initialize_card(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, card: &CardInHand, hand_size: i32, player: Player) {
    let mut local_transform = Transform::default();
    let (card_x, card_y) = get_card_location(card.position, hand_size);
    local_transform.set_translation_xyz(card_x, card_y, 0.0);

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
