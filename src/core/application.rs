




pub trait Application {
    // add code here
    fn run(arg: [String]) -> i32;

    #[allow(unused_variables)]
    fn on_signal(signal: i32) {
        unimplemented!();
    }

    fn on_exit() {
        unimplemented!();
    }
}
