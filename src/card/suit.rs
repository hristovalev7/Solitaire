#![allow(non_snake_case)]

use crate::card::color::Color;

#[derive(Clone, PartialEq, Debug)]
/// Indicates the suit of the card
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Suit {
    /// Returns the color of a suit
    pub fn getColor(&self) -> Color {
        match self {
            Suit::Heart | Suit::Diamond => Color::Red,
            Suit::Club | Suit::Spade => Color::Black,
        }
    }
}
