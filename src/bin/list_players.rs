use clap::{App, Arg};
fn main() {
    let matches = App::new("echor")
        .arg(
            Arg::with_name("text")
                .required(false)
                .min_values(0)
                .default_value("")
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    
    if text[0] == "" {
        print!("Player 1: N/A\nPlayer 2: N/A");
    } else if text.len() == 1 {
        print!("Player 1: {}\nPlayer 2: N/A", text[0]);
    } else {
        print!("Player 1: {}\nPlayer 2: {}", text[0], text[1]);
    }
}
