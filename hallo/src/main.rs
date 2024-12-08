use rand::seq::SliceRandom;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = [
            "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King", "Ace",
        ];

        let mut cards = vec![];
        for suit in suits.iter() {
            for value in values.iter() {
                cards.push(format!("{} of {}", value, suit));
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num: usize) -> Vec<String> {

        self.cards.split_off(self.cards.len() - num)

        // let mut hand = vec![];
        // for _ in 0..num {
        //     hand.push(self.cards.pop().unwrap());
        // }
        // hand
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let hand = deck.deal(2);
    println!("Here is your hand: {:#?}", hand);

    println!("Here is your deck: {:#?}", deck);

    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // println!("You typed: {}", input);
}
