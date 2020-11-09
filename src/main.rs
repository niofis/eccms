use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::string::String;

#[derive(Debug)]
enum Command {
    Ehlo(String),
    Helo(String),
    Mail(String),
    Rcpt(String),
    Data,
    Rset,
    Noop,
    Quit,
    Vrfy,
    Unsupported,
}

#[derive(Debug)]
struct MailExchange {
    domain: String,
    from: String,
    to: String,
    data: String,
    current_command: Command,
}

fn get_command(line: &String) -> Option<Command> {
    let command = &line[0..4];
    match &command.to_lowercase()[..] {
        "ehlo" => Some(Command::Ehlo(line[5..].to_string())),
        "helo" => Some(Command::Helo(line[5..].to_string())),
        "mail" => Some(Command::Mail(line[5..].to_string())),
        "rcpt" => Some(Command::Rcpt(line[5..].to_string())),
        "data" => Some(Command::Data),
        "noop" => Some(Command::Noop),
        "rset" => Some(Command::Rset),
        "vrfy" => Some(Command::Vrfy),
        "quit" => Some(Command::Quit),
        _ => Some(Command::Unsupported),
    }
}

fn handle_client(mut stream: TcpStream) {
    stream
        .write("220 Eccentric Mail Server Ready\r\n".as_bytes())
        .unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut mail_exchange = MailExchange {
        domain: String::new(),
        from: String::new(),
        to: String::new(),
        data: String::new(),
        current_command: Command::Noop,
    };

    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(total) => {
                if total == 0 {
                    return;
                }
                let command = get_command(&buffer).unwrap();
                println!("Line: {} Command: {:?}", buffer, command);
                match command {
                    Command::Ehlo(domain) => {
                        mail_exchange.domain = domain.to_owned();
                        stream.write("250 OK\r\n".as_bytes()).unwrap();
                    }
                    Command::Helo(domain) => {
                        mail_exchange.domain = domain.to_owned();
                        stream.write("250 OK\r\n".as_bytes()).unwrap();
                    }
                    Command::Mail(from) => {
                        mail_exchange.from = from.to_owned();
                        stream.write("250 OK\r\n".as_bytes()).unwrap();
                    }
                    Command::Rcpt(to) => {
                        mail_exchange.to = to.to_owned();
                        stream.write("250 OK\r\n".as_bytes()).unwrap();
                    }
                    Command::Vrfy => {
                        stream.write("250 OK\r\n".as_bytes()).unwrap();
                    }
                    Command::Data => {
                        stream.write("354\r\n".as_bytes()).unwrap();
                        loop {
                            let mut buffer = String::new();
                            if let Ok(_) = reader.read_line(&mut buffer) {
                                mail_exchange.data = mail_exchange.data + &buffer;
                                println!("data -> {}", buffer);
                                if buffer.ends_with(".\r\n") {
                                    stream.write("250 OK\r\n".as_bytes()).unwrap();
                                    break;
                                }
                            }
                        }
                    }
                    Command::Noop => {
                        stream.write("250 OK\r\n".as_bytes()).unwrap();
                    }
                    Command::Rset => {
                        mail_exchange = MailExchange {
                            domain: String::new(),
                            from: String::new(),
                            to: String::new(),
                            data: String::new(),
                            current_command: Command::Noop,
                        };
                        stream.write("250 OK\r\n".as_bytes()).unwrap();
                    }
                    Command::Quit => {
                        stream.write("221 OK\r\n".as_bytes()).unwrap();
                        println!("MailExchange: {:?}", mail_exchange);
                        return;
                    }
                    Command::Unsupported => {
                        stream
                            .write("500 Command not recognized\r\n".as_bytes())
                            .unwrap();
                    }
                };
                //println!("MailExchange: {:?}", mail_exchange);
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
