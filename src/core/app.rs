//! 文档暂缺

pub trait Application {
    /// 引擎初始化完毕后调用
    fn on_start(&self);

    /// 响应进程间的信号
    /// Returns: 是否捕获信号
    #[allow(unused_variables)]
    fn on_signal(&self, signal: i32) -> bool {
        false
    }

    fn on_exit(&self) {
        // do nothing
    }
}












#[macro_export]
macro_rules! init_app {
    ($app:ty) => {{
        let inter_main = fn <T>(app: T) 
            where T: Application
        {
            use super::initialize;
            initialize::init();
            app.on_start();
        };
        inter_main();
    }}
}

