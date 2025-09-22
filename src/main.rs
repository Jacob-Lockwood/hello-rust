use ferris_says::say; // from the previous step
use std::io::{self, stdin, stdout, BufWriter, BufRead};
mod test_mod;

fn main() -> io::Result<()> {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let sum = test_mod::add(4, 8);
    let sumstr = i32::to_string(&sum);
    let sumwidth = sumstr.chars().count();
    say(&sumstr, sumwidth, &mut writer).unwrap();

    let mut buffer = String::new();
    let stdin = stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer)?;
    let inwidth = buffer.chars().count();
    say(&buffer, inwidth, &mut writer).unwrap();
    Ok(())
}