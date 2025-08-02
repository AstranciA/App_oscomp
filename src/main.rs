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

extern crate axsyscall;

mod testcase;
use axmono::{fs::init_fs, task::init_proc};
use testcase::*;

#[unsafe(no_mangle)]
fn main() {
    axmono::init();
    // 初始化测试环境
    //mount_testsuite();

    init_fs();
    TestCaseBuilder::shell("/").run();
    //oscomp_test();

    //run_testcode("ltp", "musl");
    /*
     *TestCaseBuilder::new("/ts/musl/entry-static.exe", "/ts/musl")
     *    .arg("fscanf")
     *    .run();
     */
    /*
     *    TestCaseBuilder::new("/ts/musl/runtest.exe", "/ts/musl")
     *        .args(&["-w", "/ts/musl/entry-static.exe", "pthread_cancel"])
     *
     *        .run();
     */
    /*
     *    TestCaseBuilder::new("/ts/musl/runtest.exe", "/ts/musl")
     *        .args(&["-w", "/ts/musl/entry-static.exe", "pthread_cond_smasher"])
     *
     *        .run();
     */
    //TestCaseBuilder::shell("/ts/musl").script("./iozone -t 1 -i 0 -i 1 -r 1k -s 1m").run();
    //run_testcode("libcbench", "glibc");

    info!("All tests completed");
}

fn oscomp_test() {
    TestCaseBuilder::shell("/ts/musl")
        .script("/testrun.sh")
        .run();
    TestCaseBuilder::shell("/ts/glibc")
        .script("/testrun_glibc.sh")
        .run();
}
