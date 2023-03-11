use ggez::graphics;
use ggez::Context;
use ggez::graphics::Canvas;

pub(crate) const CARD_OFFSET: f32 = 12.5;

#[derive(PartialEq)]
/// The color of the suit of a card
pub enum Color {
    Red,
    Black,
}

#[derive(Clone, PartialEq, Debug)]
/// Indicates the type of the pile
pub enum PileIdentifier {
    Deck,
    Discard,
    Final1,
    Final2,
    Final3,
    Final4,
    Pile1,
    Pile2,
    Pile3,
    Pile4,
    Pile5,
    Pile6,
    Pile7,
    GrabbedCards,
}

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

#[derive(Clone, PartialEq, Debug)]
/// Indicates the rank of the card Ace, Two, ...,King
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

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
            face: graphics::Image::from_path(ctx, generateCardPath(suit.clone(), rank.clone())).unwrap(),
            back: graphics::Image::from_path(ctx, "/back.png").unwrap(),
            isFaceUp: false,
            suit,
            rank,
        }
    }

    /// Draws the card on a given canvas
    pub fn drawCard(&self, canvas: &mut Canvas) {
        if self.isFaceUp {
            canvas.draw(&self.face, graphics::DrawParam::default().dest([self.x, self.y]));
        } else {
            canvas.draw(&self.back, graphics::DrawParam::default().dest([self.x, self.y]));
        }
    }

    /// Flips the card
    pub fn flip(&mut self) {
        self.isFaceUp = !self.isFaceUp;
    }
}

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
        return cards;
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

/// Returns the path to the image of the card by using the given suit and rank
pub fn generateCardPath(suit: Suit, rank: Rank) -> String {
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
    return pair.to_string();
}


// Should be used ONLY for testing!!
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TestCard {
    pub suit: Suit,
    pub rank: Rank,
    pub x: f32,
    pub y: f32,
    pub initialX: f32,
    pub initialY: f32,
    pub initialPile: PileIdentifier,
}

impl TestCard {
    pub fn new(suit: Suit, rank: Rank) -> TestCard {
        TestCard {
            suit,
            rank,
            x: 0.0,
            y: 0.0,
            initialX: 0.0,
            initialY: 0.0,
            initialPile: PileIdentifier::Deck,
        }
    }
}

// Should be used ONLY for testing!!
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TestPile {
    pub x: f32,
    pub y: f32,
    pub identifier: PileIdentifier,
    pub cards: Vec<TestCard>,
}

impl TestPile {
    /// Creates a new file with the appropriate identifier and x and y
    pub fn new(x: f32, y: f32, identifier: PileIdentifier) -> TestPile {
        TestPile {
            x,
            y,
            identifier,
            cards: Vec::new(),
        }
    }
    pub fn getTopCard(&self) -> Option<&TestCard> {
        self.cards.last()
    }

    /// Adds a card to the pile WITH offset (Changes x, y, initialX, initialY and initialPile of the card)
    pub fn addCard(&mut self, card: TestCard) {
        self.cards.push(card);
        let index = self.cards.len() - 1;
        self.cards[index].x = self.x;
        self.cards[index].y = self.y + index as f32 * CARD_OFFSET;
        self.cards[index].initialX = self.x;
        self.cards[index].initialY = self.y + index as f32 * CARD_OFFSET;
        self.cards[index].initialPile = self.identifier.clone();
    }

    /// Adds a card to the pile WITHOUT offset (Changes x, y, initialX, initialY and initialPile of the card)
    pub fn addNoOffset(&mut self, mut card: TestCard) {
        card.x = self.x;
        card.y = self.y;
        card.initialX = self.x;
        card.initialY = self.y;
        card.initialPile = self.identifier.clone();
        self.cards.push(card);
    }
}