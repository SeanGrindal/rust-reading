use std::env;
use std::fs;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Dollar,
}

enum UsState {
    Alaska,
    Texas,
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    if_let_testing();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Alaska => {
                    println!("Alaska!!!!");
                },
                UsState::Texas => {
                    println!("Yeeeehaaaaaa")
                }
            }

            25
        },
        Coin::Dollar => 1,
    }
}

fn plus_one_options(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(i) => Some(i),
    } 
}

fn if_let_testing() {
    let set_max: Option<i32> = Some(32);
    if let Some(max) = set_max {
        println!("Maximun set to {}", max);
    } 
}