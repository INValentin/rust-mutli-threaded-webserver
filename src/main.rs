use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use mt_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let first_line = buf_reader.lines().next().unwrap().unwrap();

    let url = first_line
        .split_whitespace()
        .find(|s| s.starts_with('/'))
        .unwrap();

    let verb = first_line.split_whitespace().next().unwrap();

    println!("VERB: {verb}, URL: {url}");

    // Handling routes

    let (status_text, content) = match url {
        "/" => ("HTTP/1.1 200 OK", "<h1>Welcome home!</h1>"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));

            ("HTTP/1.1 200 OK", "<h1>Sleeping...</h1>")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "<h1>404 Page Not Found</h1>"),
    };

    let response = format!("{status_text}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
