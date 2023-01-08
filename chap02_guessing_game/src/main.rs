use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");

    // generate random number
    let start = 1;
    let end = 100;
    let num_rng = start..=end;

    println!(
        "Generating the secret random number between {} ~ {}...",
        start, end
    );
    let secret_number = rand::thread_rng().gen_range(num_rng);

    // guess
    let mut num_turn = 1;
    loop {
        // input user guess
        println!("\n[turn {}] Please input your guess.", num_turn);
        print!("> ");
        io::stdout().flush().unwrap();
        let guess: String = input();

        // str to int
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(g) => g,
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        };
        println!("Your guessed: {}", guess);

        // check user guess
        let is_correct = guess_number(secret_number, guess);
        if is_correct {
            println!("You win on turn {}!", num_turn);
            break;
        }

        // increment turn
        num_turn += 1;
        continue;
    }
}

fn input() -> String {
    let mut guess = String::new(); // note: シャドーイング

    match io::stdin().read_line(&mut guess) {
        Ok(_) => {
            println!("User input: \"{}\"", guess.trim());
        }
        Err(error) => {
            println!("error: {error}");
        }
    }

    return guess;
}

fn guess_number(secret_number: u32, guess: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            return false;
        }
        Ordering::Greater => {
            println!("Too big!");
            return false;
        }
        Ordering::Equal => {
            return true;
        }
    }
}
