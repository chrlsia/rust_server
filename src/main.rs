fn main() {
    // dbg!(&string);
    // dbg!(string_slice);

    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let head = Method::HEAD;
    let connect = Method::CONNECT;
    let options = Method::OPTIONS;
    let trace = Method::TRACE;
    let patch = Method::PATCH;


    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();

}

struct Server{
    addr:String,
}

impl Server {
    fn new(addr: String)->Self{
        Self { addr, }
    }
    
    // server will loop for ever
    // so we do not care if the instance of struct
    // is going to be consumed by run fn
    fn run(self){
        println!("Listening on {}",self.addr);
    }
}

struct Request {
    path: String,
    query_string:Option<String>,
    method: Method,
}

enum Method{
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/