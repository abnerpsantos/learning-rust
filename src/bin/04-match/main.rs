fn main() {
    match_control_flow();
} 

#[allow(dead_code)]
fn match_control_flow() {
    println!("Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.");
    println!("The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.");
    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

  fn get_coin(coin: Coin) {
    match coin {
        Coin::Dime => println!("Dime"),
        Coin::Nickel => println!("Nickel"),
        Coin::Quarter => println!("Quarter"),
        Coin::Penny => println!("Penny"),
    }
  }

  get_coin(Coin::Dime);
  get_coin(Coin::Nickel);
  get_coin(Coin::Penny);
  get_coin(Coin::Quarter);
}
