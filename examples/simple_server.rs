use gophermap::{GopherEntry, ItemType};
use std::io::{self, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

const HOST: &str = "localhost";
const PORT: u16 = 1234;

const fn info_link(text: &str) -> GopherEntry {
    GopherEntry {
        item_type: ItemType::Info,
        display_string: text,
        selector: "FAKE",
        host: "fake.host",
        port: 1,
    }
}

const fn menu_link<'a>(text: &'a str, target: &'a str) -> GopherEntry<'a> {
    GopherEntry {
        item_type: ItemType::Directory,
        display_string: text,
        selector: target,
        host: HOST,
        port: PORT,
    }
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut line = String::new();
    BufReader::new(stream.try_clone()?).read_line(&mut line)?;
    let line = line.trim();

    println!("New request: {}", line);

    match line {
        "/" | "" => {
            info_link("Hi!").write(&stream)?;
            info_link("Welcome to my Gopher server!").write(&stream)?;
            menu_link("Tomatoes", "/tomato").write(&stream)?;
            info_link("Opinion piece about tomatoes").write(&stream)?;
            menu_link("Potatoes", "/potato").write(&stream)?;
            info_link("Opinion piece about potatoes").write(&stream)?;
            menu_link("Go to unknown link", "/lel").write(&stream)?;
        }
        "/tomato" => {
            info_link("Tomatoes are not good").write(&stream)?;
            menu_link("Home page", "/").write(&stream)?;
        }
        "/potato" => {
            info_link("Potatoes are the best").write(&stream)?;
            menu_link("Home page", "/").write(&stream)?;
        }
        x => {
            info_link(&format!("Unknown link: {}", x)).write(&stream)?;
            menu_link("Home page", "/").write(&stream)?;
        }
    };
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;

    for stream in listener.incoming() {
        thread::spawn(move || handle_client(stream?));
    }

    Ok(())
}
