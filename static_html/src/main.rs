use kingkong::prelude::*;

routes! {
    GET "/index" => |_| Response::from_asset("index.html");
    GET "/" => |_| "Hello World";
}

fn main() {
    run!().unwrap();
}