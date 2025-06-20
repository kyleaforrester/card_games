mod board;
mod sword;
mod card;
mod target;
mod prng;
mod cpu;

use std::io::{self, Write};
use board::Board;

fn main() {
    let mut buffer = String::new();
    let mut board = Board::new();

    // What is your name?
    print!("What is your name? ");
    io::stdout().flush();
    io::stdin().read_line(&mut buffer).expect("Could not read from stdin");
    let name = buffer.trim().to_string();

    // How many CPU opponents?
    let cpus: u32 = loop {
        print!("How many CPU opponents? ");
        io::stdout().flush();
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Could not read from stdin");

        match buffer.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 3 {
                    break num;
                }
            },
            Err(e) => {
                println!("{:?}", e);
                continue;
            },
        };
    };

    if name == "CPU" {
        for _ in 0..4 {
            board.add_sword("", false);
        }
    } else {
        // Add players to board
        board.add_sword(&name, true);
        for _ in 0..cpus {
            board.add_sword("", false);
        }
    }

    // Perform the draft
    board.draft();
    
    // Start play
    board.play();
}
