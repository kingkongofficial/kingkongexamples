use kingkong::prelude::*;

routes! {
    GET "/" => |_| "Hello Everyone";
}

fn main() {
    kingkong::run!();
}