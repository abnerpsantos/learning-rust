#[allow(dead_code)]
pub fn match_control_flow() {
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

#[allow(dead_code)]
pub fn acessing_enum_states() {
    println!("With match statements we can acess data inside enums...");

    #[allow(dead_code)]
    #[derive(Debug)]
    enum States {
        SaoPaulo(StateInfo),
        RioDeJaneiro(StateInfo),
    }

    #[derive(Debug)]
    struct StateInfo {
        capital: String,
        size_in_millions: u32,
    }

    fn get_state_info(state: States) {
        match state {
            States::RioDeJaneiro(val) => {
                println!("{:#?}", val)
            }
            States::SaoPaulo(val) => {
                println!("{:#?}", val)
            }
        }
    }

    let sao_paulo_object: States = States::SaoPaulo(StateInfo {
        capital: String::from("São Paulo"),
        size_in_millions: 11,
    });
    let rio_de_janeiro_object: States = States::RioDeJaneiro(StateInfo {
        capital: String::from("Rio de Janeiro"),
        size_in_millions: 6,
    });

    get_state_info(sao_paulo_object);
    get_state_info(rio_de_janeiro_object);
}

#[allow(dead_code)]
pub fn about_option_enum() {
    println!("Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value.");
    println!("If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.");

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    println!("{:#?}{:#?}{:#?}", five, six, none)
}

#[allow(dead_code)]
pub fn using_if_let() {
    println!("If you want to match only one pattern while ignore the rest you can use if let, it's less verbose and easier to read");

    /*
    Unless doing this:
      let config_max = Some(3u8);
      match config_max {
          Some(max) => println!("The maximum is configured to be {}", max),
          _ => println!("None"),
      }
      do this:
     */
    let config_max: Option<u8> = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
}
