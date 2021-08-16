use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // If the result is OK, it'll unwrap the result into variable
        // otherwise it'll stop the program from executing
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}
