use std::process;

fn main() {
    if let Err(e) = make_choice::run(){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
