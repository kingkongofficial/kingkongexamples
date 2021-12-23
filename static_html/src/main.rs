use kingkong::prelude::*;

routes! {
    GET "/" => |_| Response::from_asset("index.html");
    // GET "/" => |_| "Hello World";
}

fn main() {
    asset_dir!("../assets");
    run!().unwrap();
}