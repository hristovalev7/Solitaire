# Solitaire
A simple Solitaire game built using the [ggez](https://github.com/ggez/ggez) Rust library

# Gameplay demo
![SolitaireDemo](https://user-images.githubusercontent.com/51285393/224511856-577ce0d9-a93c-4a33-a034-75a31a14f81a.gif)

# How to play
## Description
Solitaire is played with a standard deck of 52 cards.
There are 7 *ordinary* piles, 1 deck, 1 discard pile, and 4 *final* piles.
Initially there will be 1 card in the first pile, 2 cards in the second pile, 3 cards in the third pile, etc. and only the top card of each pile will be face-up. 

## Moving the cards
You can move cards to another pile by following these rules:
1. The card you are trying to move needs to be **exactly** 1 rank lower **and** of different color than the card you are moving it onto.
2. If the destination pile is empty, only a King can be put there (There are no limitations regarding the color and the suit of the King in this case).
3. You can also move *subpiles*[^1] if the top card of the subpile can be moved according to the rules described above.

Keep in mind that only **face-up** cards can be moved!

## Goal
The goal is to move all cards to the four *final* piles.
Note that each *final* pile corresponds to a specific suit and that cards need to be arranged in ascending order[^2].

## Compiling
Make sure that you are using cargo 1.66.0 when compiling, otherwise you will only see a green screen!

[^1]: A subpile is any contiguous part of an *ordinary* pile with more than 1 element
[^2]: Ascending order of cards: A, 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K
