use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCALHOST: &str = "127.0.0.1:5858";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(Duration::from_millis(100));
}

// TODO: Organizar os loops e  partes longas em funções
// TODO: enviar para o cliente a mensagem ao invé s de imprimir no server
fn main() {
    let server = TcpListener::bind(LOCALHOST).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx,rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to save client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("invalid message");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("failed to send messago from tx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
    
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
    }
}
