// structs are kind of classes from other languages in rust. 
// a variable is a binding in the Rust. 
// in rust a trait is a set of functions 
// program logic in main fn 
// variables or bindings in rust are immutable by default unlike other languages so we make them mutaable by adding mut so we can push values in them. with this we can also reassigns values to them.  
// we can use modules and sub modules in side our code such as mod xyz and then in the fn main we can make use of sub modules. 


use rand::{thread_rng, seq::SliceRandom}; // use is keyword we can use for our code optimization. 

#[derive(Debug)] // derive attribute ->> tells the rust compiler that we want it to add in some additional functions for this particular struct.then we list out inside the derive the Debug trait. 
#[allow(dead_code)] // Added to suppress the warning
struct Deck { // in this struct, cards are list of fields or data that this strcut will wrap-up. Vec is vector that will contain strings.  
    cards: Vec<String>, // Vectors are like arrays that can grow/shrink in size. Rust also has arrays which have fixed lenght. 
}

// inherent implemenetation , this is a return type annotation (->).  this tells the rust compiler exactly what type of value we expect to return from function. 
// in our case new fn we trying to make a brand new deck and return it. in most of cases we Self.  
// Self is kind of like a reference to whatever type is mentioned in the parent implementation block
impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamond"]; // an array of list, as we know the size of it. 
        let values = ["Ace", "Two", "Three"]; // an array of list, as we know the size of it.
        let mut cards = vec![]; // we made them mutable by mut. now we can add new elements to this vector. 

    // nested for loop 
        for suit in suits{
            for value in values {
                let card = format!("{} of {}", value, suit); //formed the card.format macro can be to join together strings in some intelligent fashion. now need to store somewhere. 
                cards.push(card);
           }
       }
      
       // below line taking advantage of feature in rust called implicit return. in this we cant add ; at the end. 
       Deck { cards } // declaring var but in rust they are referred as binding. :? is debug formatter. if we have a line below then we can condence it down. { cards: cards } ->> like this {cards}.

    
   }

   fn shuffle(&mut self) {
    let mut rng = thread_rng(); // to get the copy of it we use mut here too. 
    self.cards.shuffle(rng); // then we call these. 
   } // func to randomize the cards and data. that self is the ref to the deck now in deck we mut so to get the copy of it we use mut with self.

   fn deal(&mut self, num_cards: usize) -> Vec<String> {
    self.cards.split_off( // split off vec helps us in splitting the vec from the point of index we want. but directly passing through it the values returns five values so we can achieve it with below down approach. 
        self.cards.len() - num_cards // by making use of len and minusing it i.e -> num of vectors - num of cards 
    ) // we are relying upon implicit return and ; at the end here so we do not need to put return here.  
   }
}


fn main() { // function in rust. 
    let mut deck = Deck::new();

    deck.shuffle();

    // need to be fixed with error handling. 
    let cards = deck.deal(3);
    
    println!("Here's your hand: {:#?}", cards);
    println!("Here's your deck: {:#?}", deck); // ! with println is macro in rust. in rust strings are formed with double quotes. single quote form char. adding # in {:?} makes the formatting
}