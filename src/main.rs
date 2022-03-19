use crossterm::event::{read, Event, KeyModifiers};
use crossterm::style::Stylize;
use crossterm::style::*;
use crossterm::ExecutableCommand;
use std::env;
use std::io::{stdout, Write};
use std::net::TcpStream;

fn init() -> crossterm::Result<()> {
    let mut stdout = stdout();
    stdout.execute(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;

    println!("{}", "-".repeat(80).slow_blink().green().on_black());
    for _ in 1..10 {
        println!(
            "{}{}{}",
            "|".slow_blink().green().on_black(),
            " ".repeat(78).green().on_black(),
            "|".slow_blink().green().on_black()
        );
    }

    println!(
        "{}{}{}{}{}{}",
        "|".slow_blink().green().on_black(),
        " ".repeat(27).green().on_black(),
        "Welcome to the ".green().on_black(),
        "BONE ZONE".bold().red().on_black(),
        " ".repeat(27).green().on_black(),
        "|".slow_blink().green().on_black()
    );
    println!(
        "{}{}{}{}{}",
        "|".slow_blink().green().on_black(),
        " ".repeat(27).green().on_black(),
        "COMMANDS:".green().on_black(),
        " ".repeat(42).green().on_black(),
        "|".slow_blink().green().on_black()
    );
    println!(
        "{}{}{}{}{}",
        "|".slow_blink().green().on_black(),
        " ".repeat(27).green().on_black(),
        "W FORWARD".green().on_black(),
        " ".repeat(42).green().on_black(),
        "|".slow_blink().green().on_black()
    );
    println!(
        "{}{}{}{}{}",
        "|".slow_blink().green().on_black(),
        " ".repeat(27).green().on_black(),
        "A LEFT".green().on_black(),
        " ".repeat(45).green().on_black(),
        "|".slow_blink().green().on_black()
    );
    println!(
        "{}{}{}{}{}",
        "|".slow_blink().green().on_black(),
        " ".repeat(27).green().on_black(),
        "S BACK".green().on_black(),
        " ".repeat(45).green().on_black(),
        "|".slow_blink().green().on_black()
    );
    println!(
        "{}{}{}{}{}",
        "|".slow_blink().green().on_black(),
        " ".repeat(27).green().on_black(),
        "D RIGHT".green().on_black(),
        " ".repeat(44).green().on_black(),
        "|".slow_blink().green().on_black()
    );
    println!(
        "{}{}{}{}{}",
        "|".slow_blink().green().on_black(),
        " ".repeat(27).green().on_black(),
        "Q QUIT".green().on_black(),
        " ".repeat(45).green().on_black(),
        "|".slow_blink().green().on_black()
    );

    for _ in 1..10 {
        println!(
            "{}{}{}",
            "|".slow_blink().green().on_black(),
            " ".repeat(78).green().on_black(),
            "|".slow_blink().green().on_black()
        );
    }

    println!("{}", "-".repeat(80).slow_blink().green().on_black());
    Ok(())
}


fn connection() -> Result<(), Box<dyn std::error::Error>>{

    for argument in env::args() {
        if argument == "--help" || argument == "-h" {
            println!("./robot-cli <ip> <port>");
        }
    }

    let args: Vec<String> = env::args().collect();
    let robot = format!("{}:9999", args[1]);
    let mut stream = TcpStream::connect(&robot)?;

    let q = crossterm::event::KeyCode::Char('q');
    let w = crossterm::event::KeyCode::Char('w');
    let a = crossterm::event::KeyCode::Char('a');
    let s = crossterm::event::KeyCode::Char('s');
    let d = crossterm::event::KeyCode::Char('d');
    let quit = crossterm::event::KeyEvent::new(q, KeyModifiers::NONE);
    let foward = crossterm::event::KeyEvent::new(w, KeyModifiers::NONE);
    let left = crossterm::event::KeyEvent::new(a, KeyModifiers::NONE);
    let back = crossterm::event::KeyEvent::new(s, KeyModifiers::NONE);
    let right = crossterm::event::KeyEvent::new(d, KeyModifiers::NONE);
    loop {
        let keystroke = read()?;
        if keystroke == Event::Key(quit) {
            break;
        } else if keystroke == Event::Key(foward) {
            let buf = "1\n".as_bytes();
            stream.write(buf)?;
            stream.flush()?;
        } else if keystroke == Event::Key(left) {
            let buf = "2\n".as_bytes();
            stream.write(buf)?;
            stream.flush()?;
        } else if keystroke == Event::Key(back) {
            let buf = "3\n".as_bytes();
            stream.write(buf)?;
            stream.flush()?;
        } else if keystroke == Event::Key(right) {
            let buf = "4\n".as_bytes();
            stream.write(buf)?;
            stream.flush()?;
        }
    }
    Ok(())
}


fn main() -> Result<(),Box<dyn std::error::Error>> {
    init()?;
    crossterm::terminal::enable_raw_mode()?;
    let res = connection();
    if let Err(e) = res {
    eprintln!("{:?}",e);
    std::thread::sleep(std::time::Duration::from_secs(3));
    }
    crossterm::terminal::disable_raw_mode()?;
    stdout().execute(ResetColor)?;
    stdout().execute(crossterm::terminal::Clear(
        crossterm::terminal::ClearType::All,
    ))?;
    Ok(())
}
