//#![cfg_attr(feature = "axstd", no_std)]
//#![cfg_attr(feature = "axstd", no_main)]
#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)]
#![feature(naked_functions)]

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

    TestCaseBuilder::shell("/ts/musl")
        .script("/testrun.sh")
        .run();
    TestCaseBuilder::shell("/ts/glibc/basic")
        .script("/glibc_test.sh")
        .run();


    info!("All tests completed");
}
