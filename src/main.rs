use rand::{self, random};
use std::{collections::HashMap, io};

fn main() {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: usize = choice
        .trim()
        .parse::<usize>()
        .map_err(|_| "Parse Error")
        .and_then(|n| {
            if n < 3 {
                Ok(n)
            } else {
                Err("Number must be less than 3")
            }
        })
        .expect("Not a valid number");
    let available_choices = vec!["rock", "paper", "scissor"];

    let user_choice = available_choices[choice];
    println!("user choice: {user_choice}");

    let winning_choices: HashMap<_, _> =
        [("rock", "scissor"), ("scissor", "paper"), ("paper", "rock")]
            .into_iter()
            .collect();

    let rand_number: u32 = random();
    let comp_choice_num = rand_number % 3;

    let comp_choice = available_choices[comp_choice_num as usize];
    println!("comp choice: {comp_choice}");

    if winning_choices.get(user_choice).unwrap() == &comp_choice {
        println!("User won");
    } else if user_choice == comp_choice {
        println!("Draw");
    } else {
        println!("Comp won")
    }
}
