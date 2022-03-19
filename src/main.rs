use clap::{Arg, Command};
use crossterm::event::{read, Event, KeyModifiers};
use crossterm::style::*;
use crossterm::ExecutableCommand;
use std::io::{stdout, Write};
use std::net::{TcpStream, UdpSocket};

fn terminal() -> crossterm::Result<()> {
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

fn tcp_connection(ip: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target = format!("{}:{}", ip.to_string(), port.to_string());
    let mut stream = TcpStream::connect(&target)?;
    terminal()?;
    crossterm::terminal::enable_raw_mode()?;
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

fn udp_connection(ip: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:12345")?;
    let target = format!("{}:{}", ip.to_string(), port.to_string());
    socket.connect(target)?;
    terminal()?;
    crossterm::terminal::enable_raw_mode()?;
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
            socket.send(buf)?;
        } else if keystroke == Event::Key(left) {
            let buf = "2\n".as_bytes();
            socket.send(buf)?;
        } else if keystroke == Event::Key(back) {
            let buf = "3\n".as_bytes();
            socket.send(buf)?;
        } else if keystroke == Event::Key(right) {
            let buf = "4\n".as_bytes();
            socket.send(buf)?;
        }
    }
    Ok(())
}

fn init() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::new("robot-cli")
        .version("1.0.1")
        .author("numonce")
        .about("A simple rust program used to control robots")
        .arg(
            Arg::new("IP")
                .long("ip")
                .short('i')
                .takes_value(true)
                .required(true)
                .help("IP of the target robot."),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .takes_value(true)
                .required(true)
                .help("Port of the target robot"),
        )
        .arg(
            Arg::new("udp")
                .long("udp")
                .short('u')
                .takes_value(false)
                .help("Specifies a UDP connection."),
        )
        .get_matches();

    let ip = args.value_of("IP").unwrap();
    let port = args.value_of("port").unwrap();
    if args.is_present("udp") {
        udp_connection(ip, port)?;
    } else {
        tcp_connection(ip, port)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = init();
    if let Err(e) = res {
        eprintln!("{:?}", e);
    } else {
        crossterm::terminal::disable_raw_mode()?;
        stdout().execute(ResetColor)?;
        stdout().execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
    }
    Ok(())
}
