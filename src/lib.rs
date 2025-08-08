use rand::Rng;
use std::io;
use std::io::Write;
use std::error::Error;
use std::fs::File;


pub fn run() -> Result<(), Box<dyn Error>>{
    println!("Enter number of choices:");
    let n_choices = read_int_input().map_err(|_| "Unable to read number of choices input")?;

    println!("Now tell me how many times in a row do you want me to get a particular choice before I choose it as the final one:");
    let n_times_before_selection = read_int_input().map_err(|_| "Unable to read number of times in a row input")?;

    println!("Now input your choices:");
    let choices = read_choices(n_choices)?;

    let ch = Choice::build(choices, n_choices, n_times_before_selection);

    let _ = main_logic(ch);
    Ok(())
}

#[derive(Clone)]
struct Choice {
    choices: Vec<String>,
    choice: Option<String>,
    n_choices: u32,
    n_times_before_selection: u32,
    n_turn: u32,
    continuity_counter: u32,
}

impl Choice {
    fn build(choices: Vec<String>, n_choices:u32, n_times_before_selection: u32) -> Choice {
        if std::path::Path::new("log.txt").exists() {
            let _ = File::create("log.txt");
        } // this is to truncate the log file
        Choice{
            choices,
            choice: None,
            n_choices,
            n_times_before_selection,
            n_turn:0,
            continuity_counter: 1,
        }
    }

    fn should_we_continue(&self) -> bool {
        self.continuity_counter < self.n_times_before_selection
    }

    fn get_random_choice(&self, rng: &mut rand::rngs::ThreadRng) -> String {
        let choice = self.choices[rng.random_range(0..self.n_choices) as usize].clone();
        choice
    }

// use std::fs::OpenOptions;
//     fn log(&self, choice: &String) -> std::io::Result<()> {
//         let mut file = OpenOptions::new()
//             .append(true)   // allow writing at the end of file
//             .create(true)   // create file if it doesn't exist
//             .open("log.txt")?;

//         writeln!(file, "{}: {}", self.n_turn + 1,choice)?;

//         Ok(())
//     }

    fn check_choice(&mut self, new_choice: String){
        self.n_turn += 1;
        match self.choice.clone() {
            Some(str) => {
                    if new_choice == str {
                        self.continuity_counter += 1;
                    } else {
                        self.choice = Some(new_choice);
                        self.continuity_counter = 1;
                    }
                }
            None => self.choice = Some(new_choice),
        }
    }

    fn print_result(&self) {
        println!(
            "Having made random choices {} times, finally one choice occured {} times in a row!!\nTheoretically it would take an average of {} turns for this to happen\nFinal choice is: {}\n(check log.txt for all choices made)",
            self.n_turn,
            self.n_times_before_selection,
            self.n_choices.pow(self.n_times_before_selection -1),
            self.choice.as_ref().unwrap()
        )
    }
}

fn read_int_input() -> Result<u32, ()> {
    let mut input = String::new();
    for _ in 0..5{
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<u32>(){
            Ok(n) => return Ok(n),
            Err(error) => println!("Failed to read input, please enter valid number: {error}"),
        }
        input.clear();
    }
    Err(())
}

fn read_choices(n:u32) -> Result<Vec<String>, io::Error>{
    (1..=n).map(|i| {
        println!("Enter choice {i}");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }).collect()
}

fn main_logic(mut ch: Choice) -> u32 {
    let mut rng = rand::rng();
    while ch.should_we_continue() {
        let choice = ch.get_random_choice(&mut rng);
        // ch.log(&choice).unwrap_or_else(|_| eprintln!("Failed to save log choice !"));
        ch.check_choice(choice);
    }
    ch.print_result();
    return ch.n_turn

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]  // run with:  cargo test test_run -- --nocapture --ignored
    fn test_run() {

        let n_choices = run();
        println!("{n_choices:?}");
    }

    #[test]
    fn main_logic_test() {
        let choices = vec![String::from("Red"), String::from("Blue"), String::from("Green"), String::from("Violet"), String::from("Indigo"), String::from("Yellow"), String::from("Orange")];
        let len = choices.len() as u32;
        let n_times_before_selection = 4 as u32;
        let ch = Choice::build(choices.clone(), len, n_times_before_selection);
        let mut sum: f64 = 0.0;
        let n = 100_000;

        let start = std::time::Instant::now();

        for _ in 1..=n{
            sum += main_logic(ch.clone()) as f64;
        }

        let duration = start.elapsed();

        eprintln!("Time taken to run: {:?}\nn: {}\nsum: {}\nestimated average:{}\navg:{}",
            duration,
            n, 
            sum, 
            (len.pow(n_times_before_selection)-1)/(len-1), 
            sum/(n as f64)
        );
    }
}