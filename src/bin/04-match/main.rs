fn main() {
    // match_control_flow();
    acessing_enum_states();
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

#[allow(dead_code)]
fn acessing_enum_states() {
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
