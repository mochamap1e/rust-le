use colored::Colorize;
use chrono::Local;
use ureq::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct ResponseBody {
    solution: String
}

fn main() {
    let now = Local::now();
    let url = format!("https://www.nytimes.com/svc/wordle/v2/{}.json", now.format("%Y-%m-%d").to_string());
    
    let response = get(url)
        .call()
        .unwrap()
        .body_mut()
        .read_json::<ResponseBody>()
        .unwrap();

    let mut guesses = [""; 5];

    let solution = response.solution;
    let guess = "cospe";

    for (index, (guess_char, solution_char)) in guess.chars().zip(solution.chars()).enumerate() {
        if guess_char == solution_char {
            guesses[index] = "green";
        }

        if !solution.contains(guess_char) {
            guesses[index] = "gray";
        }
    }

    println!("{:?}", guesses);
}
