use gophermap::{GopherMenu,ItemType};
use std::io::{self, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

const HOST: &str = "localhost";
const PORT: u16 = 1234;

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut line = String::new();
    BufReader::new(stream.try_clone()?).read_line(&mut line)?;
    let line = line.trim();

    println!("New request: {}", line);

    let mut menu = GopherMenu::with_write(&stream);

    let menu_link = |text: &str, selector: &str|
        menu.write_entry(ItemType::Directory, text, selector, HOST, PORT);

    match line {
        "/" | "" => {
            menu.info("Hi!")?;
            menu.info("Welcome to my Gopher server!")?;
            menu_link("Tomatoes", "/tomato")?;
            menu.info("Opinion piece about tomatoes")?;
            menu_link("Potatoes", "/potato")?;
            menu.info("Opinion piece about potatoes")?;
            menu_link("Go to unknown link", "/lel")?;
        }
        "/tomato" => {
            menu.info("Tomatoes are not good")?;
            menu_link("Home page", "/")?;
        }
        "/potato" => {
            menu.info("Potatoes are the best")?;
            menu_link("Home page", "/")?;
        }
        x => {
            menu.info(&format!("Unknown link: {}", x))?;
            menu_link("Home page", "/")?;
        }
    };
    menu.end()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;

    for stream in listener.incoming() {
        thread::spawn(move || handle_client(stream?));
    }

    Ok(())
}
