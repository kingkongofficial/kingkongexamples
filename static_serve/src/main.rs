use kingkong::prelude::*;

routes! {
    GET "/" => |_| "<img src='/kingkong.jpeg'>";
}

fn main() {
    asset_dir!("./assets");
    run!().unwrap();
}