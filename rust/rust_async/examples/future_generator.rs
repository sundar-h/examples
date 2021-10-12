use std::{io::{self, BufRead}, net::TcpStream, ops::Generator};

use futures::Future;

fn main() {
}

async fn print_lines() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let tcp = TcpStream::connect(&addr);
    let io = BufRead::new(tcp);

    for line in io.lines() {
        println!("{}", line);
    }

    Ok(())
}

fn print_lines_generated() -> impl Future<Item = (), Error = io::Error> {
    GeneratorToFuture (|| {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let tcp = {
        let mut future = TcpStream::connect(&addr);
        loop {
            match future.poll() {
                Ok(Async::Ready(e)) => break Ok(e),
                Ok(Async::NotReady) => yield,
                Err(e) => break Err(e),
            }
        }
    };

    let io = BufRead::new(tcp);

    // let mut stream 
    })
}