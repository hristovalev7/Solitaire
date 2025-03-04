#![allow(non_snake_case)]

use crate::card::card::Card;
use crate::card::pile::identifier::PileIdentifier;
use ggez::graphics::Canvas;
use crate::card::pile::identifier::PileIdentifier::{Final1, Final2, Final3, Final4, Pile1, Pile2, Pile3, Pile4, Pile5, Pile6, Pile7};

pub const CARD_OFFSET: f32 = 12.5;
/// An array of all pile identifiers
pub const PILE_IDENTIFIERS: [PileIdentifier; 7] = [Pile1, Pile2, Pile3, Pile4, Pile5, Pile6, Pile7];
/// An array of all final identifiers
pub const FINAL_IDENTIFIERS: [PileIdentifier; 4] = [Final1, Final2, Final3, Final4];

#[derive(Clone)]
pub struct Pile {
    /// The x coordinate of the pile
    pub x: f32,
    /// The y coordinate of the pile
    pub y: f32,
    /// The pile identifier
    pub identifier: PileIdentifier,
    /// The cards in the pile
    pub cards: Vec<Card>,
}

impl Pile {
    /// Creates a new file with the appropriate identifier and x and y
    pub fn new(x: f32, y: f32, identifier: PileIdentifier) -> Pile {
        Pile {
            x,
            y,
            identifier,
            cards: Vec::new(),
        }
    }

    /// Adds a card to the pile WITH offset (Changes x, y, initialX, initialY and initialPile of the card)
    pub fn addCard(&mut self, card: Card) {
        self.cards.push(card);
        let index = self.cards.len() - 1;
        self.cards[index].x = self.x;
        self.cards[index].y = self.y + index as f32 * CARD_OFFSET;
        self.cards[index].initialX = self.x;
        self.cards[index].initialY = self.y + index as f32 * CARD_OFFSET;
        self.cards[index].initialPile = self.identifier.clone();
    }

    /// Adds a card to the pile WITHOUT offset (Changes x, y, initialX, initialY and initialPile of the card)
    pub fn addNoOffset(&mut self, mut card: Card) {
        card.x = self.x;
        card.y = self.y;
        card.initialX = self.x;
        card.initialY = self.y;
        card.initialPile = self.identifier.clone();
        self.cards.push(card);
    }

    /// Adds a card to grabbedCards WITH offset (Changes x and y of the card but does NOT change initialX, initialY and initialPile)
    pub fn addToGrabbed(&mut self, card: Card) {
        self.cards.push(card);
        let index = self.cards.len() - 1;
        self.cards[index].x = self.x;
        self.cards[index].y = self.y + (self.cards.len() - 1) as f32 * CARD_OFFSET;
    }

    /// Removes the top card of the pile adn returns Option<Card>
    pub fn removeCard(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Removes all cards from the pile and returns a vector containing them
    pub fn removeCards(&mut self) -> Vec<Card> {
        let mut cards = Vec::new();
        for _ in self.cards.clone() {
            cards.push(self.cards.pop().unwrap());
        }
        cards
    }

    /// Returns a reference the top card of the pile
    pub fn getTopCard(&self) -> Option<&Card> {
        self.cards.last()
    }

    /// Returns a reference to the card with a given index
    pub fn getCardByIndex(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    /// Flips the top card
    pub fn flipTopCard(&mut self) {
        if self.size() != 0 {
            let index = self.cards.clone().len() - 1;
            self.cards[index].flip();
        }
    }

    /// Draws a pile on the given canvas
    pub fn drawPile(&self, canvas: &mut Canvas) {
        for card in &self.cards {
            card.drawCard(canvas);
        }
    }

    /// Returns the number of cards in the pile
    pub fn size(&self) -> usize {
        self.cards.len()
    }

    /// Checks if the pile is empty
    pub fn empty(&self) -> bool {
        self.cards.is_empty()
    }
}
