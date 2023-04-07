use std::io::{stdout, BufWriter};
use ferris_says::say;


struct cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path)
    };
    let stdout = stdout();
    let message = String::from("hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();


}
