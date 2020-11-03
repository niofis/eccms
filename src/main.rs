use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum Command {
    Ehlo(String),
    Helo,
    Mail,
    Rcpt,
    Data,
    Rset,
    Noop,
    Quit,
    Vrfy,
}

#[derive(Debug)]
struct MailExchange {
    client_name: String,
}

fn get_command(line: &String) -> Option<Command> {
    let parts: Vec<&str> = line.split(' ').collect();
    match &parts.get(0)?.to_lowercase()[..] {
        "ehlo" => Some(Command::Ehlo(parts.get(1)?.to_string())),
        "helo" => Some(Command::Helo),
        _ => Some(Command::Noop),
    }
}

fn handle_client(mut stream: TcpStream) {
    stream
        .write("220 Eccentric Mail Server Ready\r\n".as_bytes())
        .unwrap();
    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut mail_exchange = MailExchange {
        client_name: String::new(),
    };

    loop {
        match reader.read_line(&mut buffer) {
            Ok(total) => {
                if total == 0 {
                    return;
                }
                let command = get_command(&buffer).unwrap();
                println!("Line: {} Command: {:?}", buffer, command);
                match command {
                    Command::Ehlo(client_name) => {
                        mail_exchange.client_name = client_name;
                        stream
                            .write("500 Command not recognized\r\n".as_bytes())
                            .unwrap()
                    }
                    Command::Helo => stream
                        .write("500 Command not recognized\r\n".as_bytes())
                        .unwrap(),
                    _ => stream
                        .write("500 Command not recognized\r\n".as_bytes())
                        .unwrap(),
                };
                println!("MailExchange: {:?}", mail_exchange);
            }
            Err(err) => panic!(err),
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:2525")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
