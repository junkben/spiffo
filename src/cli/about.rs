/// `spiffo about`
pub fn cmd() {
    print_about_msg()
}

fn print_about_msg() {
    line_break();
    println!("Spiffo is a command-line interface meant to manage a dedicated Project Zomboid");
    println!("multiplayer server with support for Steam Workshop mods, backups, updates, etc.");
    newline();
    println!("Currently, Spiffo is a personal project and is not yet a functioning prototype.");
    println!("Nonetheless, I greatly appreciate the use of my tool all the same.");
    newline();
    println!("Discussions are welcome and encouraged:");
    println!("https://github.com/junkben/spiffo/discussions");
    newline();
    println!("Copyright (c) 2023, junkben");
    line_break();
}

fn line_break() {
    println!("{}", "-".repeat(80));
}

fn newline() {
    println!("");
}
