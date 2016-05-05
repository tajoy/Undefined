#[macro_use]
extern crate undefined;

use undefined::core::app::Application;

#[derive(Debug)]
#[allow(dead_code)]
struct MyApp;

impl Application for MyApp {
    #[allow(unused_variables)]
    fn on_start() {
        unimplemented!();
    }
}

#[allow(unused_variables)]
fn main() {
    let x:MyApp = MyApp{};
    println!("{:?}", x);
}