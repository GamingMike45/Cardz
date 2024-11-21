// Define a struct for the Card
#[derive(Debug)]
struct Card {
    value: i32,
    suit: char,
}

// Function to print the hand of cards
fn print_hand(hand: &[Card]) {
    for card in hand {
        println!("Value: {}, Suit: {}", card.value, card.suit);
    }
}

// Function to check if the cards are in ascending order
fn is_ascending(hand: &[Card]) -> bool {
    for i in 1..hand.len() {
        if hand[i].value < hand[i - 1].value {
            return false; // Cards are not in ascending order
        }
    }
    true
}

// Function to check if the cards are in descending order
fn is_descending(hand: &[Card]) -> bool {
    for i in 1..hand.len() {
        if hand[i].value > hand[i - 1].value {
            return false; // Cards are not in descending order
        }
    }
    true
}

// Function to check if all cards have the same suit
fn all_same_suit(hand: &[Card]) -> bool {
    let first_suit = hand[0].suit;
    hand.iter().all(|card| card.suit == first_suit)
}

// Function to calculate the total value of the hand
fn calculate_total_value(hand: &[Card]) -> i32 {
    hand.iter().map(|card| card.value).sum()
}

fn main() {
    // Create the array of card tuples (value, suit)
    let hand = vec![
        Card { value: 11, suit: 'H' },
        Card { value: 2, suit: 'S' },
        Card { value: 3, suit: 'H' },
        Card { value: 4, suit: 'H' },
        Card { value: 5, suit: 'H' },
    ];

    // Print the hand
    println!("Hand of Cards:");
    print_hand(&hand);

    // Assess the properties of the hand
    let ascending = is_ascending(&hand);
    let descending = is_descending(&hand);
    let same_suit = all_same_suit(&hand);
    let total_value = calculate_total_value(&hand);

    // Print the results of the assessment
    println!("\nAssessment Results:");
    println!("Are the cards in ascending order? {}", if ascending { "Yes" } else { "No" });
    println!("Are the cards in descending order? {}", if descending { "Yes" } else { "No" });
    println!("Are all the cards of the same suit? {}", if same_suit { "Yes" } else { "No" });
    println!("Total point value of the hand: {}", total_value);
}
