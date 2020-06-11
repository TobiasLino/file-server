use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::fs::{self, DirBuilder, read_dir};

const LOCALHOST: &str = "127.0.0.1:5858";
const MSG_SIZE: usize = 32;


fn check_path() -> std::io::Result<String> {
    let mut s = String::new();
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        s = s.clone() + &(format!("{:?}\n", dir.path())).to_string();
    }
    Ok(s)
}
fn sleep() {
    thread::sleep(Duration::from_millis(100));
}
// * Já consegui enviar mensagens pela rede, agora preciso que o servidor leia os dados da pasta
// * exclusiva e envie para  o cliente
// TODO: Organizar os loops e  partes longas em funções
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

        loop {
            let mut msg = String::new();
            match check_path() {
                Ok(st) => msg = st,
                Err(_) => println!("Erro na leitura")
            }
            if tx.send(msg).is_err() { break }
        }
    }
}
