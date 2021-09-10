use rand::Rng;
use std::io;

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

fn main() {
    println!("Rock Paper Scissor");
    loop {
        loop_game();
    }
}

fn loop_game() {
    println!("1: Rock | 2: Paper | 3: Scissor");
    let user_input = get_user_input();
    let computer_input = get_computer_input();
    let result = get_result(user_input, computer_input);
    match result {
        Result::Win => println!("You win!"),
        Result::Lose => println!("You lose!"),
        Result::Draw => println!("Draw!"),
    }
}

fn get_user_input() -> Move {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().parse().expect("Please type a valid move");
    return map_num_to_move(input);
}

fn get_computer_input() -> Move {
    let random_number = rand::thread_rng().gen_range(1..4);
    return map_num_to_move(random_number);
}

fn map_num_to_move(num: u8) -> Move {
    match num {
        1 => Move::Rock,
        2 => Move::Paper,
        3 => Move::Scissors,
        _ => panic!("Invalid number"),
    }
}

fn get_result(user_input: Move, computer_input: Move) -> Result {
    match user_input {
        Move::Rock => match computer_input {
            Move::Rock => Result::Draw,
            Move::Paper => Result::Lose,
            Move::Scissors => Result::Win,
        },
        Move::Paper => match computer_input {
            Move::Rock => Result::Win,
            Move::Paper => Result::Draw,
            Move::Scissors => Result::Lose,
        },
        Move::Scissors => match computer_input {
            Move::Rock => Result::Lose,
            Move::Paper => Result::Win,
            Move::Scissors => Result::Draw,
        },
    }
}
