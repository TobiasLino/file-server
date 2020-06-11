use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCALHOST: &str = "127.0.0.1:5858";
const MSG_SIZE: usize = 32;

// * Já consegui enviar mensagens pela rede, agora preciso que o servidor leia os dados da pasta
// * exclusiva e envie para  o cliente
// TODO: Organizar os loops e  partes longas em funções
fn main() {
    println!("Starting...");
    let mut client = TcpStream::connect(LOCALHOST).expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to initiate non blocking");

    let (tx, rx) = mpsc::channel::<String>();
    println!("Connected.");

    println!("\nComandos aceitos:\n\tls - mostra os arquivos\n\texit - Sair.\n");
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok (_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recevied {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    print!("> ");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading failed");
        let msg = buff.trim().to_string();
        if msg == "exit" { break }
        // não é um comando aceitável? pula a última linha
        if msg != "ls" { continue }
        if tx.send(msg).is_err() { break }
    }
    println!("Bye");
}