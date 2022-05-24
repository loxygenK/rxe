use atty::Stream::Stdout;

pub fn error(message: impl ToString) {
    println!("{}", with_color(1, message));
}

pub fn with_color(color: u8, message: impl ToString) -> String {
    if atty::is(Stdout) {
        format!("\x1b[38;5;{}m{}\x1b[m", color, message.to_string())
    } else {
        message.to_string()
    }
}