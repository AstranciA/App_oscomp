//#![cfg_attr(feature = "axstd", no_std)]
//#![cfg_attr(feature = "axstd", no_main)]
#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)]

use axfs::api::{read, read_dir};

extern crate axstd;
#[macro_use]
extern crate axlog;
#[macro_use]
extern crate alloc;

mod testcase;
use axmono::{fs::init_fs, task::init_proc};
use testcase::*;

#[unsafe(no_mangle)]
fn main() {
    axmono::init();
    // 初始化测试环境
    mount_testsuite();

    init_fs();
    info!("All tests completed");
}
