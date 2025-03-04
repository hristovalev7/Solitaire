#![allow(non_snake_case)]

use crate::card::pile::identifier::PileIdentifier;
use crate::card::rank::Rank;
use crate::card::suit::Suit;
use ggez::graphics::Canvas;
use ggez::{graphics, Context};

/// The card width
pub const CARD_WIDTH: f32 = 76.0;
/// The card height
pub const CARD_HEIGHT: f32 = 110.0;

#[derive(Clone)]
/// Represents a card
pub struct Card {
    /// The current x coordinate of the card
    pub x: f32,
    /// The current y coordinate of the card
    pub y: f32,
    /// The initial x coordinate (used to return a card to its initial coordinates if stacking was unsuccessful)
    pub initialX: f32,
    /// The initial y coordinate (used to return a card to its initial coordinates if stacking was unsuccessful)
    pub initialY: f32,
    /// The initial pile of the card (used to return a card to its initial pile if stacking was unsuccessful)
    pub initialPile: PileIdentifier,
    /// The face of the card
    pub face: graphics::Image,
    /// The back of the card
    pub back: graphics::Image,
    /// Indicates if the card is face-up or face-down
    pub isFaceUp: bool,
    /// The suit of the card
    pub suit: Suit,
    /// The rank of the card
    pub rank: Rank,
}

impl Card {
    /// Creates a new card from the given suit and rank
    pub fn new(ctx: &mut Context, suit: Suit, rank: Rank) -> Card {
        Card {
            x: 0.0,
            y: 0.0,
            initialX: 0.0,
            initialY: 0.0,
            initialPile: PileIdentifier::Deck,
            face: graphics::Image::from_path(
                ctx,
                Self::generateCardPath(suit.clone(), rank.clone()),
            )
            .unwrap(),
            back: graphics::Image::from_path(ctx, "/back.png").unwrap(),
            isFaceUp: false,
            suit,
            rank,
        }
    }

    /// Draws the card on a given canvas
    pub fn drawCard(&self, canvas: &mut Canvas) {
        if self.isFaceUp {
            canvas.draw(
                &self.face,
                graphics::DrawParam::default().dest([self.x, self.y]),
            );
        } else {
            canvas.draw(
                &self.back,
                graphics::DrawParam::default().dest([self.x, self.y]),
            );
        }
    }

    /// Flips the card
    pub fn flip(&mut self) {
        self.isFaceUp = !self.isFaceUp;
    }

    fn generateCardPath(suit: Suit, rank: Rank) -> String {
        let pair = match (suit, rank) {
            (Suit::Heart, Rank::Ace) => "/ace_of_hearts.png",
            (Suit::Heart, Rank::Two) => "/2_of_hearts.png",
            (Suit::Heart, Rank::Three) => "/3_of_hearts.png",
            (Suit::Heart, Rank::Four) => "/4_of_hearts.png",
            (Suit::Heart, Rank::Five) => "/5_of_hearts.png",
            (Suit::Heart, Rank::Six) => "/6_of_hearts.png",
            (Suit::Heart, Rank::Seven) => "/7_of_hearts.png",
            (Suit::Heart, Rank::Eight) => "/8_of_hearts.png",
            (Suit::Heart, Rank::Nine) => "/9_of_hearts.png",
            (Suit::Heart, Rank::Ten) => "/10_of_hearts.png",
            (Suit::Heart, Rank::Jack) => "/jack_of_hearts.png",
            (Suit::Heart, Rank::Queen) => "/queen_of_hearts.png",
            (Suit::Heart, Rank::King) => "/king_of_hearts.png",
            (Suit::Diamond, Rank::Ace) => "/ace_of_diamonds.png",
            (Suit::Diamond, Rank::Two) => "/2_of_diamonds.png",
            (Suit::Diamond, Rank::Three) => "/3_of_diamonds.png",
            (Suit::Diamond, Rank::Four) => "/4_of_diamonds.png",
            (Suit::Diamond, Rank::Five) => "/5_of_diamonds.png",
            (Suit::Diamond, Rank::Six) => "/6_of_diamonds.png",
            (Suit::Diamond, Rank::Seven) => "/7_of_diamonds.png",
            (Suit::Diamond, Rank::Eight) => "/8_of_diamonds.png",
            (Suit::Diamond, Rank::Nine) => "/9_of_diamonds.png",
            (Suit::Diamond, Rank::Ten) => "/10_of_diamonds.png",
            (Suit::Diamond, Rank::Jack) => "/jack_of_diamonds.png",
            (Suit::Diamond, Rank::Queen) => "/queen_of_diamonds.png",
            (Suit::Diamond, Rank::King) => "/king_of_diamonds.png",
            (Suit::Club, Rank::Ace) => "/ace_of_clubs.png",
            (Suit::Club, Rank::Two) => "/2_of_clubs.png",
            (Suit::Club, Rank::Three) => "/3_of_clubs.png",
            (Suit::Club, Rank::Four) => "/4_of_clubs.png",
            (Suit::Club, Rank::Five) => "/5_of_clubs.png",
            (Suit::Club, Rank::Six) => "/6_of_clubs.png",
            (Suit::Club, Rank::Seven) => "/7_of_clubs.png",
            (Suit::Club, Rank::Eight) => "/8_of_clubs.png",
            (Suit::Club, Rank::Nine) => "/9_of_clubs.png",
            (Suit::Club, Rank::Ten) => "/10_of_clubs.png",
            (Suit::Club, Rank::Jack) => "/jack_of_clubs.png",
            (Suit::Club, Rank::Queen) => "/queen_of_clubs.png",
            (Suit::Club, Rank::King) => "/king_of_clubs.png",
            (Suit::Spade, Rank::Ace) => "/ace_of_spades.png",
            (Suit::Spade, Rank::Two) => "/2_of_spades.png",
            (Suit::Spade, Rank::Three) => "/3_of_spades.png",
            (Suit::Spade, Rank::Four) => "/4_of_spades.png",
            (Suit::Spade, Rank::Five) => "/5_of_spades.png",
            (Suit::Spade, Rank::Six) => "/6_of_spades.png",
            (Suit::Spade, Rank::Seven) => "/7_of_spades.png",
            (Suit::Spade, Rank::Eight) => "/8_of_spades.png",
            (Suit::Spade, Rank::Nine) => "/9_of_spades.png",
            (Suit::Spade, Rank::Ten) => "/10_of_spades.png",
            (Suit::Spade, Rank::Jack) => "/jack_of_spades.png",
            (Suit::Spade, Rank::Queen) => "/queen_of_spades.png",
            (Suit::Spade, Rank::King) => "/king_of_spades.png",
        };
        pair.to_string()
    }
}
