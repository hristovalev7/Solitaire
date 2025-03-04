#![allow(non_snake_case)]

use crate::assets::assets::Assets;
use crate::card::pile::identifier::PileIdentifier;
use crate::card::rank::Rank;
use crate::card::suit::Suit;
use ggez::graphics::{Canvas, Image};
use ggez::graphics;

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
    pub face: Image,
    /// The back of the card
    pub back: Image,
    /// Indicates if the card is face-up or face-down
    pub isFaceUp: bool,
    /// The suit of the card
    pub suit: Suit,
    /// The rank of the card
    pub rank: Rank,
}

impl Card {
    /// Creates a new card from the given suit and rank
    pub fn new(suit: Suit, rank: Rank, assets: Assets) -> Card {
        let back = assets.back.clone();
        Card {
            x: 0.0,
            y: 0.0,
            initialX: 0.0,
            initialY: 0.0,
            initialPile: PileIdentifier::Deck,
            face: Self::getImage(suit.clone(), rank.clone(), assets),
            back,
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

    fn getImage(suit: Suit, rank: Rank, assets: Assets) -> Image {
        match (suit, rank) {
            (Suit::Heart, Rank::Ace) => assets.aceHeart,
            (Suit::Heart, Rank::Two) => assets.twoHeart,
            (Suit::Heart, Rank::Three) => assets.threeHeart,
            (Suit::Heart, Rank::Four) => assets.fourHeart,
            (Suit::Heart, Rank::Five) => assets.fiveHeart,
            (Suit::Heart, Rank::Six) => assets.sixHeart,
            (Suit::Heart, Rank::Seven) => assets.sevenHeart,
            (Suit::Heart, Rank::Eight) => assets.eightHeart,
            (Suit::Heart, Rank::Nine) => assets.nineHeart,
            (Suit::Heart, Rank::Ten) => assets.tenHeart,
            (Suit::Heart, Rank::Jack) => assets.jackHeart,
            (Suit::Heart, Rank::Queen) => assets.queenHeart,
            (Suit::Heart, Rank::King) => assets.kingHeart,
            (Suit::Diamond, Rank::Ace) => assets.aceDiamond,
            (Suit::Diamond, Rank::Two) => assets.twoDiamond,
            (Suit::Diamond, Rank::Three) => assets.threeDiamond,
            (Suit::Diamond, Rank::Four) => assets.fourDiamond,
            (Suit::Diamond, Rank::Five) => assets.fiveDiamond,
            (Suit::Diamond, Rank::Six) => assets.sixDiamond,
            (Suit::Diamond, Rank::Seven) => assets.sevenDiamond,
            (Suit::Diamond, Rank::Eight) => assets.eightDiamond,
            (Suit::Diamond, Rank::Nine) => assets.nineDiamond,
            (Suit::Diamond, Rank::Ten) => assets.tenDiamond,
            (Suit::Diamond, Rank::Jack) => assets.jackDiamond,
            (Suit::Diamond, Rank::Queen) => assets.queenDiamond,
            (Suit::Diamond, Rank::King) => assets.kingDiamond,
            (Suit::Club, Rank::Ace) => assets.aceClub,
            (Suit::Club, Rank::Two) => assets.twoClub,
            (Suit::Club, Rank::Three) => assets.threeClub,
            (Suit::Club, Rank::Four) => assets.fourClub,
            (Suit::Club, Rank::Five) => assets.fiveClub,
            (Suit::Club, Rank::Six) => assets.sixClub,
            (Suit::Club, Rank::Seven) => assets.sevenClub,
            (Suit::Club, Rank::Eight) => assets.eightClub,
            (Suit::Club, Rank::Nine) => assets.nineClub,
            (Suit::Club, Rank::Ten) => assets.tenClub,
            (Suit::Club, Rank::Jack) => assets.jackClub,
            (Suit::Club, Rank::Queen) => assets.queenClub,
            (Suit::Club, Rank::King) => assets.kingClub,
            (Suit::Spade, Rank::Ace) => assets.aceSpade,
            (Suit::Spade, Rank::Two) => assets.twoSpade,
            (Suit::Spade, Rank::Three) => assets.threeSpade,
            (Suit::Spade, Rank::Four) => assets.fourSpade,
            (Suit::Spade, Rank::Five) => assets.fiveSpade,
            (Suit::Spade, Rank::Six) => assets.sixSpade,
            (Suit::Spade, Rank::Seven) => assets.sevenSpade,
            (Suit::Spade, Rank::Eight) => assets.eightSpade,
            (Suit::Spade, Rank::Nine) => assets.nineSpade,
            (Suit::Spade, Rank::Ten) => assets.tenSpade,
            (Suit::Spade, Rank::Jack) => assets.jackSpade,
            (Suit::Spade, Rank::Queen) => assets.queenSpade,
            (Suit::Spade, Rank::King) => assets.kingSpade,
        }
    }
}
