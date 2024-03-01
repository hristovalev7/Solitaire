#![allow(non_snake_case)]

mod cards;
mod assets;

use ggez::*;
use std::{env, path};
use std::io;
use ggez::conf::{Conf, WindowMode};
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{Color, Sampler};
use rand::prelude::*;

use crate::assets::Assets;
use crate::cards::*;
use crate::cards::PileIdentifier::*;
use crate::Difficulty::{Easy, Hard};

/// The x coordinate of pile 1
const PILE1_X: f32 = 85.0;
/// The x coordinate of pile 2
const PILE2_X: f32 = 260.0;
/// The x coordinate of pile 3
const PILE3_X: f32 = 435.0;
/// The x coordinate of pile 4
const PILE4_X: f32 = 610.0;
/// The x coordinate of pile 5
const PILE5_X: f32 = 785.0;
/// The x coordinate of pile 6
const PILE6_X: f32 = 960.0;
/// The x coordinate of pile 7
const PILE7_X: f32 = 1135.0;
/// The y coordinate of the piles from pile1 to pile7
const PILE_Y: f32 = 300.0;
/// The x coordinate of the deck
const DECK_X: f32 = 85.0;
/// The y coordinate of the deck
const DECK_Y: f32 = 30.0;
/// The x coordinate of the discard pile
const DISCARD_X: f32 = 185.0;
/// The y coordinate of the discard pile
const DISCARD_Y: f32 = 30.0;
/// The x coordinate of final1
const FINAL1_X: f32 = 435.0;
/// The x coordinate of final2
const FINAL2_X: f32 = 610.0;
/// The x coordinate of final3
const FINAL3_X: f32 = 785.0;
/// The x coordinate of final4
const FINAL4_X: f32 = 960.0;
/// The y coordinate of the piles from final1 to final4
const FINAL_Y: f32 = 30.0;
/// The card width
const CARD_WIDTH: f32 = 76.0;
/// The card height
const CARD_HEIGHT: f32 = 110.0;
// An array of all pile identifiers
const PILE_IDENTIFIERS: [PileIdentifier; 7] = [Pile1, Pile2, Pile3, Pile4, Pile5, Pile6, Pile7];
// An array of all final identifiers
const FINAL_IDENTIFIERS: [PileIdentifier; 4] = [Final1, Final2, Final3, Final4];

#[derive(PartialEq)]
enum Difficulty {
    Easy,
    Hard,
}

struct State {
    /// Indicates if the game is over
    gameOver: bool,
    /// All resources
    assets: Assets,
    /// The first pile (left -> right)
    pile1: Pile,
    /// The second pile (left -> right)
    pile2: Pile,
    /// The third pile (left -> right)
    pile3: Pile,
    /// The fourth pile (left -> right)
    pile4: Pile,
    /// The fifth pile (left -> right)
    pile5: Pile,
    /// The sixth pile (left -> right)
    pile6: Pile,
    /// The seventh pile (left -> right)
    pile7: Pile,
    /// The deck
    deck: Pile,
    /// The discard pile
    discard: Pile,
    /// The first final pile (left -> right)
    final1: Pile,
    /// The second final pile (left -> right)
    final2: Pile,
    /// The third final pile (left -> right)
    final3: Pile,
    /// The fourth final pile (left -> right)
    final4: Pile,
    /// Indicates if the mouse button is pressed
    mouseDown: bool,
    /// Stores the "grabbed" cards
    grabbedCards: Pile,
    /// Indicates the game difficulty
    difficulty: Difficulty,
}

impl State {
    /// Creates a new State
    fn new(ctx: &mut Context, difficulty: Difficulty) -> GameResult<State> {
        let assets = Assets::new(ctx)?;
        let deck = Pile::new(DECK_X, DECK_Y, Deck);
        let discard = Pile::new(DISCARD_X, DISCARD_Y, Discard);

        let final1 = Pile::new(FINAL1_X, FINAL_Y, Final1);
        let final2 = Pile::new(FINAL2_X, FINAL_Y, Final2);
        let final3 = Pile::new(FINAL3_X, FINAL_Y, Final3);
        let final4 = Pile::new(FINAL4_X, FINAL_Y, Final4);

        let pile1 = Pile::new(PILE1_X, PILE_Y, Pile1);
        let pile2 = Pile::new(PILE2_X, PILE_Y, Pile2);
        let pile3 = Pile::new(PILE3_X, PILE_Y, Pile3);
        let pile4 = Pile::new(PILE4_X, PILE_Y, Pile4);
        let pile5 = Pile::new(PILE5_X, PILE_Y, Pile5);
        let pile6 = Pile::new(PILE6_X, PILE_Y, Pile6);
        let pile7 = Pile::new(PILE7_X, PILE_Y, Pile7);

        let mut state = State {
            gameOver: false,
            assets,
            pile1,
            pile2,
            pile3,
            pile4,
            pile5,
            pile6,
            pile7,
            deck,
            discard,
            final1,
            final2,
            final3,
            final4,
            mouseDown: false,
            grabbedCards: Pile::new(0.0, 0.0, GrabbedCards),
            difficulty,
        };

        state.newDeck(ctx);
        state.dealCards();

        Ok(state)
    }

    /// Generates a new deck with 52 cards and then shuffles it
    fn newDeck(&mut self, ctx: &mut Context) {
        let mut deck: Vec<Card> = Vec::new();

        for suit in vec![Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
            for rank in vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] {
                let card: Card = Card::new(ctx, suit.clone(), rank.clone());
                deck.push(card);
            }
        }

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        for card in deck {
            self.deck.addNoOffset(card);
        }
    }

    /// "Deals" the cards from a generated deck to the piles and the rest remain in the deck
    fn dealCards(&mut self) {
        for i in 0..7 {
            for j in 0..i + 1 {
                let mut card = self.deck.removeCard();
                if j == i {
                    card.as_mut().unwrap().isFaceUp = true;
                }
                match i {
                    0 => self.pile1.addCard(card.unwrap()),
                    1 => self.pile2.addCard(card.unwrap()),
                    2 => self.pile3.addCard(card.unwrap()),
                    3 => self.pile4.addCard(card.unwrap()),
                    4 => self.pile5.addCard(card.unwrap()),
                    5 => self.pile6.addCard(card.unwrap()),
                    6 => self.pile7.addCard(card.unwrap()),
                    _ => panic!("Invalid pile"),
                }
            }
        }
    }

    /// Checks for win conditions
    fn checkForWin(&mut self) {
        self.gameOver = self.final1.size() == 13 && self.final2.size() == 13 && self.final3.size() == 13 && self.final4.size() == 13;
    }

    /// Identifies a pile by a given PileIdentifier and returns a reference to the pile
    fn identifyPile(&self, identifier: PileIdentifier) -> &Pile {
        match identifier {
            Pile1 => &self.pile1,
            Pile2 => &self.pile2,
            Pile3 => &self.pile3,
            Pile4 => &self.pile4,
            Pile5 => &self.pile5,
            Pile6 => &self.pile6,
            Pile7 => &self.pile7,
            Final1 => &self.final1,
            Final2 => &self.final2,
            Final3 => &self.final3,
            Final4 => &self.final4,
            Deck => &self.deck,
            Discard => &self.discard,
            GrabbedCards => &self.grabbedCards
        }
    }

    /// Identifies a pile by a given PileIdentifier and returns a mutable reference to the pile
    fn identifyPileMut(&mut self, identifier: PileIdentifier) -> &mut Pile {
        match identifier {
            Pile1 => &mut self.pile1,
            Pile2 => &mut self.pile2,
            Pile3 => &mut self.pile3,
            Pile4 => &mut self.pile4,
            Pile5 => &mut self.pile5,
            Pile6 => &mut self.pile6,
            Pile7 => &mut self.pile7,
            Final1 => &mut self.final1,
            Final2 => &mut self.final2,
            Final3 => &mut self.final3,
            Final4 => &mut self.final4,
            Deck => &mut self.deck,
            Discard => &mut self.discard,
            GrabbedCards => &mut self.grabbedCards
        }
    }

    /// Checks if the mouse is over a card with coordinates x and y
    fn mouseOver(&self, ctx: &mut Context, x: f32, y: f32) -> bool {
        let mouseX = ctx.mouse.position().x;
        let mouseY = ctx.mouse.position().y;
        return mouseX > x && mouseX < x + CARD_WIDTH && mouseY > y && mouseY < y + CARD_HEIGHT;
    }

    /// Checks if the mouse is over a pile but not over the top card
    fn mouseIsOverPile(&self, ctx: &mut Context, pile: Pile) -> bool {
        let mouseX = ctx.mouse.position().x;
        let mouseY = ctx.mouse.position().y;
        return mouseX > pile.x && mouseX < pile.x + CARD_WIDTH && mouseY > pile.y && mouseY < pile.y + (pile.size() as i32 - 1) as f32 * CARD_OFFSET;
    }

    /// A helper function for canBeStacked
    fn stackableCheckForFinal(&self, card: Card, pile: Pile) -> bool {
        let topCard = pile.getTopCard();
        return (topCard.is_none() && card.rank == Rank::Ace) || (topCard.is_some() && card.suit == topCard.unwrap().suit && card.rank as i32 == topCard.unwrap().clone().rank as i32 + 1);
    }

    /// A helper function for canBeStacked
    fn stackableCheckForPile(&self, card: Card, pile: Pile) -> bool {
        let topCard = pile.getTopCard();
        return (topCard.is_none() && card.rank == Rank::King) || (topCard.is_some() && card.suit.getColor() != topCard.unwrap().suit.getColor() && card.rank as i32 == topCard.unwrap().clone().rank as i32 - 1);
    }

    /// A function that checks if a given card can be placed on a given pile
    fn canBeStacked(&self, card: Card, receiverIdentifier: PileIdentifier) -> bool {
        match receiverIdentifier {
            Final1 | Final2 | Final3 | Final4 => self.stackableCheckForFinal(card, self.identifyPile(receiverIdentifier).clone()),
            Pile1 | Pile2 | Pile3 | Pile4 | Pile5 | Pile6 | Pile7 => self.stackableCheckForPile(card, self.identifyPile(receiverIdentifier).clone()),
            _ => false,
        }
    }

    /// A function that removes all cards from the discard pile, flips them and then returns them in the deck pile
    fn emptyDiscard(&mut self) {
        let mut cards = self.discard.removeCards();
        for card in cards.iter_mut() {
            card.isFaceUp = false;
            self.deck.addNoOffset(card.clone());
        }
    }

    /// A function that handles card discarding.
    fn discardCard(&mut self) {
        if self.difficulty == Easy {
            let mut card = self.deck.removeCard();
            card.as_mut().unwrap().isFaceUp = true;
            self.discard.addNoOffset(card.clone().unwrap());
        } else {
            let mut iterations: usize = 0;

            while iterations < 3 && !self.deck.empty() {
                let mut card = self.deck.removeCard();
                card.as_mut().unwrap().isFaceUp = true;
                self.discard.addNoOffset(card.clone().unwrap());
                iterations = iterations + 1;
            }
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.gameOver {
            return Ok(());
        }
        self.checkForWin();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(0, 128, 0));

        //Fixes blurry sprites
        let sampler = Sampler::nearest_clamp();
        canvas.set_sampler(sampler);

        self.deck.drawPile(&mut canvas);
        self.discard.drawPile(&mut canvas);

        canvas.draw(&self.assets.frame, graphics::DrawParam::default().dest([FINAL1_X, FINAL_Y]));
        canvas.draw(&self.assets.frame, graphics::DrawParam::default().dest([FINAL2_X, FINAL_Y]));
        canvas.draw(&self.assets.frame, graphics::DrawParam::default().dest([FINAL3_X, FINAL_Y]));
        canvas.draw(&self.assets.frame, graphics::DrawParam::default().dest([FINAL4_X, FINAL_Y]));

        self.final1.drawPile(&mut canvas);
        self.final2.drawPile(&mut canvas);
        self.final3.drawPile(&mut canvas);
        self.final4.drawPile(&mut canvas);

        self.pile1.drawPile(&mut canvas);
        self.pile2.drawPile(&mut canvas);
        self.pile3.drawPile(&mut canvas);
        self.pile4.drawPile(&mut canvas);
        self.pile5.drawPile(&mut canvas);
        self.pile6.drawPile(&mut canvas);
        self.pile7.drawPile(&mut canvas);

        // Prints grabbedCards
        let mut offset = 0.0;
        for card in self.grabbedCards.cards.iter().rev() {
            canvas.draw(&card.face, graphics::DrawParam::default().dest([card.x, card.y + offset]));
            offset += CARD_OFFSET;
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult<()> {
        self.mouseDown = true;

        if self.mouseOver(ctx, DECK_X, DECK_Y) && self.grabbedCards.empty() {
            if !self.deck.empty() {
                self.discardCard();
            } else {
                self.emptyDiscard();
            }
            return Ok(());
        }

        for identifier in PILE_IDENTIFIERS {
            let currentPile = self.identifyPile(identifier.clone());
            if !currentPile.empty() && self.mouseOver(ctx, currentPile.x, currentPile.y + currentPile.size() as f32 * CARD_OFFSET) && !currentPile.getTopCard().unwrap().isFaceUp {
                self.identifyPileMut(identifier.clone()).flipTopCard();
                break;
            }
        }

        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult<()> {
        if self.mouseDown && !self.grabbedCards.empty() {
            for pileIdentifier in PILE_IDENTIFIERS {
                let currentPile = self.identifyPile(pileIdentifier.clone()).clone();
                let pileSize = currentPile.size() as f32;
                let lastCard = self.grabbedCards.cards.last();
                if lastCard.is_some() && self.mouseOver(ctx, currentPile.x, PILE_Y + pileSize * CARD_OFFSET) && self.canBeStacked(lastCard.unwrap().clone(), pileIdentifier.clone()) {
                    for card in self.grabbedCards.cards.clone().iter().rev()
                    {
                        let pile = self.identifyPileMut(pileIdentifier.clone());
                        pile.addCard(card.clone());
                    }
                    self.grabbedCards.cards.clear();
                    break;
                }
            }
            for finalIdentifier in FINAL_IDENTIFIERS {
                let lastCard = self.grabbedCards.cards.last();
                if lastCard.is_some() && self.canBeStacked(lastCard.unwrap().clone(), finalIdentifier.clone()) {
                    for card in self.grabbedCards.cards.clone().iter().rev() {
                        let pile = self.identifyPileMut(finalIdentifier.clone());
                        pile.addNoOffset(card.clone());
                    }

                    self.grabbedCards.cards.clear();
                    break;
                }
            }
            for card in self.grabbedCards.cards.clone().iter().rev() {
                let initialPile = self.identifyPileMut(card.clone().initialPile);
                match card.initialPile {
                    Discard | Final1 | Final2 | Final3 | Final4 => initialPile.addNoOffset(card.clone()),
                    _ => initialPile.addCard(card.clone()),
                }
            }
            self.grabbedCards.cards.clear();
        }

        self.mouseDown = false;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> GameResult<()> {
        if self.mouseDown && self.grabbedCards.empty() {
            for pileIdentifier in PILE_IDENTIFIERS {
                let pile = self.identifyPile(pileIdentifier.clone()).clone();
                let topCard = pile.getTopCard();
                let mouseIsOverCurrentPile = self.mouseIsOverPile(ctx, pile.clone());
                let mouseOverTopCard = topCard.is_some() && self.mouseOver(ctx, topCard.unwrap().x, topCard.unwrap().y);

                if topCard.is_some() && (mouseIsOverCurrentPile || mouseOverTopCard) {
                    if mouseOverTopCard {
                        let pile = self.identifyPileMut(pileIdentifier.clone());
                        let removedCard = pile.removeCard().unwrap();
                        self.grabbedCards.addToGrabbed(removedCard.clone());
                    } else {
                        let cardIndex = ((ctx.mouse.position().y - PILE_Y) / CARD_OFFSET) as usize;
                        let pile = self.identifyPile(pileIdentifier.clone()).clone();
                        let card = pile.getCardByIndex(cardIndex).clone();

                        if card.is_some() && card.unwrap().isFaceUp {
                            let pileSize = pile.size();
                            for _ in cardIndex..pileSize {
                                let pile = self.identifyPileMut(pileIdentifier.clone());
                                let card = pile.removeCard();
                                self.grabbedCards.addToGrabbed(card.unwrap());
                            }
                        }
                    }
                    break;
                }
            }
            for finalIdentifier in FINAL_IDENTIFIERS {
                let currentPile = self.identifyPile(finalIdentifier.clone()).clone();
                if currentPile.getTopCard().is_some() && self.mouseOver(ctx, currentPile.x, currentPile.y) {
                    let pile = self.identifyPileMut(finalIdentifier.clone());
                    let card = pile.removeCard().unwrap();
                    self.grabbedCards.addToGrabbed(card);
                    break;
                }
            }
            if self.discard.getTopCard().is_some() && self.mouseOver(ctx, DISCARD_X, DISCARD_Y) {
                self.grabbedCards.addToGrabbed(self.discard.removeCard().unwrap());
            }
        }
        for card in self.grabbedCards.cards.iter_mut() {
            //Местене на картите: https://github.com/ggez/ggez/blob/master/examples/input_test.rs
            card.x = x - 38.0; // CARD_WIDTH / 2.0
            card.y = y - 55.0; // CARD_HEIGHT / 2.0
        }
        Ok(())
    }
}

fn difficultyHandler() -> Difficulty {
    let mut string = String::new();
    while string != "Hard" && string != "hard" && string != "Easy" && string != "easy" {
        string.clear();
        println!("Difficulty: ");
        io::stdin().read_line(&mut string).unwrap();
        if string.contains('\n') {
            string.pop();
        }
        if string.contains('\r') {
            string.pop();
        }

    }
    let difficulty: Difficulty =
        match string.as_str() {
            "Hard" | "hard" => Hard,
            _ => Easy
        };
    return difficulty;
}

// Source за повечето неща от main: https://github.com/AndrewRadev/rust-shooter/blob/main/src/main.rs
fn main() {
    let difficulty = difficultyHandler();

    let conf = Conf::new().window_mode(WindowMode {
        width: 1300.0,
        height: 700.0,
        ..Default::default()
    });

    let (mut ctx, event_loop) = ContextBuilder::new("Solitaire", "Hristo").default_conf(conf.clone()).build().unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    let state = State::new(&mut ctx, difficulty).unwrap();
    ctx.gfx.set_window_title("Solitaire");

    event::run(ctx, event_loop, state);
}

#[cfg(test)]
mod tests
{
    use rand::*;
    use rand::prelude::*;
    use crate::cards::Rank::*;
    use crate::cards::Suit::*;
    use crate::PileIdentifier::*;
    use crate::cards::{TestPile, TestCard, Rank, CARD_OFFSET};
    use crate::{DECK_X, DECK_Y, FINAL1_X, FINAL_Y, PILE1_X, PILE_Y};

    fn stackableCheckForTestPile(card: TestCard, pile: TestPile) -> bool {
        let topCard = pile.getTopCard();
        return (topCard.is_none() && card.rank == Rank::King) || (topCard.is_some() && card.suit.getColor() != topCard.unwrap().suit.getColor() && card.rank as i32 == topCard.unwrap().clone().rank as i32 - 1);
    }

    fn stackableCheckForFinal(card: TestCard, pile: TestPile) -> bool {
        let topCard = pile.getTopCard();
        return (topCard.is_none() && card.rank == Rank::Ace) || (topCard.is_some() && card.suit == topCard.unwrap().suit && card.rank as i32 == topCard.unwrap().clone().rank as i32 + 1);
    }

    fn newDeck() -> TestPile {
        let mut result = TestPile::new(DECK_X, DECK_Y, Deck);
        let mut deck: Vec<TestCard> = Vec::new();

        for suit in vec![Heart, Diamond, Club, Spade] {
            for rank in vec![Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King] {
                let card: TestCard = TestCard::new(suit.clone(), rank.clone());
                deck.push(card);
            }
        }

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        for card in deck {
            result.addNoOffset(card);
        }
        return result;
    }

    #[test]
    fn pileStackingSuccessful() {
        let inPile = TestCard::new(Heart, King);
        let mut pile = TestPile::new(0.0, 0.0, Pile1);
        pile.addCard(inPile);

        let clubQueen = TestCard::new(Club, Queen);
        let spadeQueen = TestCard::new(Spade, Queen);

        // Should be stackable as the Queens are black and the difference between a Queen AND a King is exactly 1
        assert!(stackableCheckForTestPile(clubQueen, pile.clone()));
        assert!(stackableCheckForTestPile(spadeQueen, pile.clone()));
    }

    #[test]
    fn pileStackingUnsuccessful() {
        let card = TestCard::new(Heart, King);
        let mut pile = TestPile::new(0.0, 0.0, Pile1);
        pile.addCard(card);

        let clubKing = TestCard::new(Club, King);
        let spadeKing = TestCard::new(Spade, King);
        let diamondKing = TestCard::new(Diamond, King);

        let heartQueen = TestCard::new(Heart, Queen);
        let diamondQueen = TestCard::new(Diamond, Queen);

        let clubJack = TestCard::new(Club, Jack);
        let spadeJack = TestCard::new(Spade, Jack);

        // Shouldn't be stackable as the cards are the same rank
        assert!(!stackableCheckForTestPile(clubKing, pile.clone()));
        assert!(!stackableCheckForTestPile(spadeKing, pile.clone()));

        // Shouldn't be stackable as the Queen and the King are the same color
        assert!(!stackableCheckForTestPile(diamondQueen, pile.clone()));
        assert!(!stackableCheckForTestPile(heartQueen, pile.clone()));

        // Shouldn't be stackable as the cards have the same rank AND color
        assert!(!stackableCheckForTestPile(diamondKing, pile.clone()));

        // Shouldn't be stackable as the difference between a Jack and a King is 2 (must be 1)
        assert!(!stackableCheckForTestPile(clubJack, pile.clone()));
        assert!(!stackableCheckForTestPile(spadeJack, pile));
    }

    #[test]
    fn emptyPileStackingSuccessful() {
        let pile = TestPile::new(0.0, 0.0, Pile1);

        let diamondKing = TestCard::new(Diamond, King);
        let heartKing = TestCard::new(Heart, King);
        let spadeKing = TestCard::new(Spade, King);
        let clubKing = TestCard::new(Club, King);

        // Should be stackable as Kings can be put only on empty piles
        assert!(stackableCheckForTestPile(diamondKing, pile.clone()));
        assert!(stackableCheckForTestPile(heartKing, pile.clone()));
        assert!(stackableCheckForTestPile(spadeKing, pile.clone()));
        assert!(stackableCheckForTestPile(clubKing, pile));
    }

    #[test]
    fn emptyPileStackingUnsuccessful() {
        let pile = TestPile::new(0.0, 0.0, Pile1);

        // The user shouldn't be able to put a card other than a King on an empty pile
        for suit in vec![Heart, Diamond, Club, Spade] {
            for rank in vec![Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen] {
                let card = TestCard::new(suit.clone(), rank.clone());
                assert!(!stackableCheckForTestPile(card, pile.clone()));
            }
        }
    }

    #[test]
    fn finalStackingSuccessful() {
        let mut pile = TestPile::new(0.0, 0.0, Final1);
        let spadeAce = TestCard::new(Spade, Ace);
        pile.addCard(spadeAce);

        let spadeTwo = TestCard::new(Spade, Two);

        // Should be stackable as the cards are of the same suit AND their rank difference is 1
        assert!(stackableCheckForFinal(spadeTwo, pile));
    }

    #[test]
    fn finalStackingUnsuccessful() {
        let mut pile = TestPile::new(0.0, 0.0, Final1);
        let spadeAce = TestCard::new(Spade, Ace);
        pile.addCard(spadeAce);

        let clubTwo = TestCard::new(Club, Two);
        let diamondTwo = TestCard::new(Diamond, Two);
        let heartTwo = TestCard::new(Heart, Two);

        // Shouldn't be stackable as the cards in a given final pile must be the same suit
        assert!(!stackableCheckForFinal(clubTwo, pile.clone()));
        assert!(!stackableCheckForFinal(diamondTwo, pile.clone()));
        assert!(!stackableCheckForFinal(heartTwo, pile));
    }

    #[test]
    fn emptyFinalStackingSuccessful() {
        let pile = TestPile::new(0.0, 0.0, Final1);

        let clubAce = TestCard::new(Club, Ace);
        let spadeAce = TestCard::new(Spade, Ace);
        let diamondAce = TestCard::new(Diamond, Ace);
        let heartAce = TestCard::new(Heart, Ace);

        // Should be stackable as all aces can be placed on empty final piles
        assert!(stackableCheckForFinal(clubAce, pile.clone()));
        assert!(stackableCheckForFinal(spadeAce, pile.clone()));
        assert!(stackableCheckForFinal(diamondAce, pile.clone()));
        assert!(stackableCheckForFinal(heartAce, pile));
    }

    #[test]
    fn emptyFinalStackingUnsuccessful() {
        let pile = TestPile::new(0.0, 0.0, Final1);

        // Shouldn't be stackable as only Aces can be placed on an empty final pile
        for suit in vec![Heart, Diamond, Club, Spade] {
            for rank in vec![Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King] {
                let card = TestCard::new(suit.clone(), rank.clone());
                assert!(!stackableCheckForFinal(card, pile.clone()));
            }
        }
    }

    #[test]
    fn pileOffset() {
        let mut pile = TestPile::new(PILE1_X, PILE_Y, Pile1);

        let diamondKing = TestCard::new(Diamond, King);
        pile.addCard(diamondKing);
        let topCard = pile.getTopCard().unwrap();
        assert_eq!(topCard.y, pile.y);

        let clubQueen = TestCard::new(Club, Queen);
        pile.addCard(clubQueen);
        let topCard = pile.getTopCard().unwrap();
        assert_eq!(topCard.y, pile.y + CARD_OFFSET);

        let heartJack = TestCard::new(Heart, Jack);
        pile.addCard(heartJack);
        let topCard = pile.getTopCard().unwrap();
        assert_eq!(topCard.y, pile.y + CARD_OFFSET * 2.0);
    }

    #[test]
    fn finalOffset() {
        let mut pile = TestPile::new(FINAL1_X, FINAL_Y, Final1);

        let clubAce = TestCard::new(Club, Ace);
        pile.addNoOffset(clubAce);
        let topCard = pile.getTopCard().unwrap();
        assert_eq!(topCard.y, pile.y);

        let clubTwo = TestCard::new(Club, Two);
        pile.addNoOffset(clubTwo);
        let topCard = pile.getTopCard().unwrap();
        assert_eq!(topCard.y, pile.y);

        let clubThree = TestCard::new(Club, Three);
        pile.addNoOffset(clubThree);
        let topCard = pile.getTopCard().unwrap();
        assert_eq!(topCard.y, pile.y);
    }

    #[test]
    fn uniqueDeck() {
        let deck1 = newDeck();
        let deck2 = newDeck();
        assert_ne!(deck1, deck2);
    }
}