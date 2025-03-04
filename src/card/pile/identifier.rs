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
