#[macro_use]
extern crate undefined;

// use undefined::core::application;

#[derive(Debug)]
struct MyApp;

impl undefined::core::applaction::Applaction for MyApp {
    fn run(arg: [String]) -> i32 {
        unimplemented!();
    }
} 


fn main() {

}