//! 文档暂缺


pub trait Logger {
    // add code here
}



#[macro_export]
macro_rules! log {
    (init $msg:stmt, $args*) => ()
    (info $msg:stmt, $args*) => ()
}