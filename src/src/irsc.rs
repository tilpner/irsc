use std::io::{
    BufferedReader,
    TcpStream, 
    IoResult
};
use std::str;
use std::ascii::StrAsciiExt;
use std::simd::f32x4;

struct Server {
    host: String,
    port: u16,
    stream: Option<TcpStream>
}

impl Server {

    fn new(host: &str, port: u16) -> Server {
        Server {
            host: String::from_str(host),
            port: port,
            stream: None
        }
    }

    fn connect(&mut self) {
        self.stream = Some(TcpStream::connect(self.host.as_slice(), self.port).unwrap());

        let cs = self.stream.clone();
        spawn(proc() {
            Server::listen(cs);
        });
    }
    
    #[inline]
    fn sendraw(stream: &mut TcpStream, s: &str) {
        stream.write_str(s);
        stream.flush();
    }

    #[inline]
    fn sendrawln(stream: &mut TcpStream, s: &str) {
        Server::sendraw(stream, String::with_capacity(s.len() + 2).append(s).append("\r\n").as_slice());
    }

    fn join(self, channel: &str) {
        self.stream.map(|mut st| Server::sendrawln(&mut st, format!("JOIN {}", channel).as_slice()));
    }

    fn listen(stream: Option<TcpStream>) {
        let mut abort = false;
        let mut stream = match stream {
            Some(s) => s,
            None => return
        };
        if abort {return;}
        let mut reader = BufferedReader::new(stream.clone());
        loop {
            let line = reader.read_line().unwrap();
            //println!("{}", line);
            let mut parts = line.as_slice().split(' ').collect::<Vec<&str>>();
            println!("{}", parts);

            if parts.len() == 0 {
                continue;
            }

            // if message has a prefix
            let prefix = if parts.get(0).chars().next().unwrap() == ':' {
                parts.shift().unwrap()
            } else { "" };

            match (*parts.get(0)).to_ascii_upper().as_slice() {
                "PING" => {
                    Server::sendrawln(&mut stream, format!("PONG {}", parts.get(1)).as_slice());
                    continue;
                }
                _ => {}
            }
            
        }
    }

    
}

fn main() {
    let mut furnet = Server::new("irc.furnet.org", 6667);
    furnet.connect();

    std::io::timer::sleep(5000u64);

    furnet.join("#teenagefurs");

    // create simd vectors
    let x = f32x4(1.0, 2.0, 3.0, 4.0);
    let y = f32x4(4.0, 3.0, 2.0, 1.0);

    // simd product
    let z = x * y;

    // like any struct, the simd vector can be destructured using `let`
    let f32x4(a, b, c, d) = z;

    println!("{}", (a, b, c, d));
}
