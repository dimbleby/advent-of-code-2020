use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Deck {
    cards: VecDeque<usize>,
}

impl Deck {
    fn new(cards: VecDeque<usize>) -> Self {
        Self { cards }
    }

    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(n, c)| (n + 1) * c)
            .sum()
    }

    fn truncated(&self, length: usize) -> Self {
        let new_cards = self.cards.iter().take(length).cloned().collect();
        Self::new(new_cards)
    }
}

impl FromStr for Deck {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .lines()
            .skip(1)
            .map(|line| line.parse())
            .collect::<Result<VecDeque<_>, _>>()?;
        let deck = Deck::new(cards);
        Ok(deck)
    }
}

#[derive(Copy, Clone)]
enum Player {
    Player1,
    Player2,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct GameState {
    player1: Deck,
    player2: Deck,
}

impl GameState {
    fn new(player1: Deck, player2: Deck) -> Self {
        Self { player1, player2 }
    }

    fn winner(&self) -> Option<Player> {
        if self.player1.cards.is_empty() {
            Some(Player::Player2)
        } else if self.player2.cards.is_empty() {
            Some(Player::Player1)
        } else {
            None
        }
    }

    fn player_score(&self, player: Player) -> usize {
        let deck = match player {
            Player::Player1 => &self.player1,
            Player::Player2 => &self.player2,
        };
        deck.score()
    }

    fn play_round(&mut self, recursive: bool) {
        let card1 = self.player1.cards.pop_front().unwrap();
        let card2 = self.player2.cards.pop_front().unwrap();

        let round_winner = if recursive
            && card1 <= self.player1.cards.len()
            && card2 <= self.player2.cards.len()
        {
            let subplayer1 = self.player1.truncated(card1);
            let subplayer2 = self.player2.truncated(card2);
            let mut subgame = Game::new(subplayer1, subplayer2);
            subgame.play(true)
        } else if card1 > card2 {
            Player::Player1
        } else {
            Player::Player2
        };

        match round_winner {
            Player::Player1 => {
                self.player1.cards.push_back(card1);
                self.player1.cards.push_back(card2);
            }
            Player::Player2 => {
                self.player2.cards.push_back(card2);
                self.player2.cards.push_back(card1);
            }
        }
    }
}

struct Game {
    state: GameState,
    history: HashSet<GameState>,
}

impl Game {
    fn new(player1: Deck, player2: Deck) -> Self {
        let state = GameState::new(player1, player2);
        let history = HashSet::new();
        Self { state, history }
    }

    fn player_score(&self, player: Player) -> usize {
        self.state.player_score(player)
    }

    fn play(&mut self, recursive: bool) -> Player {
        loop {
            if !self.history.insert(self.state.clone()) {
                return Player::Player1;
            }

            self.state.play_round(recursive);

            if let Some(player) = self.state.winner() {
                return player;
            }
        }
    }
}

pub(crate) fn day22() {
    let input = std::fs::read_to_string("data/day22.txt").unwrap();
    let mut hands = input.split("\n\n");
    let hand1: Deck = hands.next().unwrap().parse().unwrap();
    let hand2: Deck = hands.next().unwrap().parse().unwrap();

    let mut game = Game::new(hand1.clone(), hand2.clone());
    let winner = game.play(false);
    println!("Part one answer is {}", game.player_score(winner));

    let mut game = Game::new(hand1, hand2);
    let winner = game.play(true);
    println!("Part two answer is {}", game.player_score(winner));
}
