//! 文档暂缺


#![crate_name="undefined"]
#![crate_type="dylib"]

// 前期可能变动很大, 所以在后期补文档
// #![deny(missing_docs)]

// 这些也后期补
// #![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
//        html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
//        html_root_url = "https://doc.rust-lang.org/nightly/",
//        html_playground_url = "https://play.rust-lang.org/",
//        issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
//        test(no_crate_inject, attr(deny(warnings))),
//        test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]


#[macro_use]
pub mod core;

#[macro_use]
pub mod graphic;

#[macro_use]
pub mod input;

#[macro_use]
pub mod node;



