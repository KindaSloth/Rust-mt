use std::{thread::{self, JoinHandle}, sync::{mpsc::{channel, Receiver}, Mutex, Arc}, time::Duration, net::{TcpListener, TcpStream}, io::Write, fs};

fn spawn_threads<T>(n: u8, receiver: Arc<Mutex<Receiver<T>>>, handlers: &mut Vec<JoinHandle<()>>) 
    where
        T: FnOnce() + Send + 'static
{
    for id in 0..n {
        let receiver_clone = Arc::clone(&receiver);

        let handler = thread::spawn(move ||
            loop {
                let job = receiver_clone.lock().unwrap().recv().unwrap();
    
                println!("Thread: {id} received a job");
    
                job();
            }
        );
    
        handlers.push(handler);
    }
}

fn handle_connection(mut stream: TcpStream) {
    thread::sleep(Duration::from_secs(5));

    let contents = fs::read_to_string("./src/test.html").unwrap();
    let length = contents.len();

    let response =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let mut handlers = vec![];

    let (sender, receiver) = channel();

    spawn_threads(5, Arc::new(Mutex::new(receiver)), &mut handlers);

    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        sender.send(|| {
            handle_connection(stream);
        }).unwrap();
    }
}
