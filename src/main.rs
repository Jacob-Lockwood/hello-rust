use std::io::stdin;

fn parse_num(source: &mut String) -> i32 {
    let mut n = 0;
    while source.len() > 0 && "0123456789".contains(source.chars().nth(0).unwrap()) {
        n = 10 * n + "0123456789".find(source.remove(0)).unwrap();
    }
    return n as i32;
}

fn exec(source: &mut String) -> i32 {
    let x = parse_num(source);
    let op = source.remove(0);
    let y = parse_num(source);
    return match op {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        _ => y,
    };
}
fn main() {
    let stdin = stdin();
    println!(
        "welcome!\nenter expressions of the form xFy where x and y are integers,\nx defaults to zero, and F is one of +, -, *.\nenter nothing to quit"
    );
    loop {
        let mut input = String::from("");
        stdin.read_line(&mut input).unwrap();
        input = input.trim_end().to_string();
        if input == "" {
            break;
        }
        let value = exec(&mut input);
        println!("{}", value);
    }
    println!("goodbye!");
}
