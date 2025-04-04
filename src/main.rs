fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();

}

struct Server{
    addr:String,
}

impl Server {
    fn new(addr: String)->Server{
        Server { addr, }
    }
    
    // server will loop for ever
    // so we do not care if the instance of struct
    // is going to be consumed by run fn
    fn run(self){

    }
}