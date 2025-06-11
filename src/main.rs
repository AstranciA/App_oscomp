//#![cfg_attr(feature = "axstd", no_std)]
//#![cfg_attr(feature = "axstd", no_main)]
#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)]

use axfs::api::read_dir;

extern crate axstd;
#[macro_use]
extern crate axlog;
#[macro_use]
extern crate alloc;

mod testcase;
use axmono::task::init_proc;
use testcase::*;

#[unsafe(no_mangle)]
fn main() {
    axmono::init();
    // 初始化测试环境
    mount_testsuite();

    let builder = TestCaseBuilder::new("/ts/musl/basic/dup", "/ts/musl/basic")
        .run();
    let builder = TestCaseBuilder::new("/ts/musl/basic/dup2", "/ts/musl/basic")
        .run();
    let builder = TestCaseBuilder::new("/ts/musl/basic/chdir", "/ts/musl/basic")
        .run();
    let builder = TestCaseBuilder::new("/ts/musl/basic/fork", "/ts/musl/basic")
        .run();
    let builder = TestCaseBuilder::new("/ts/musl/basic/gettimeofday", "/ts/musl/basic")
        .run();
    let builder = TestCaseBuilder::new("/ts/musl/basic/write", "/ts/musl/basic")
        .run();
    let builder = TestCaseBuilder::new("/ts/musl/basic/mmap", "/ts/musl/basic")
        .run();
    let builder = TestCaseBuilder::new("/ts/musl/basic/fstat", "/ts/musl/basic")
        .run();
    // TestCaseBuilder::shell("/")
    //     .script("echo 'Kernel Test Start' && ls /")
    //     .run();
    // 示例2：运行shell命令
    // TestCaseBuilder::shell("/ts/musl")
    //     .script("/test.sh")
    //     .run();
    // // 示例3：运行标准测试套件
    // run_testcode("libctest", "musl");
    // run_testcode("lua", "musl");
    // // 示例4：复杂测试用例
    // let builder = TestCaseBuilder::new("/ts/musl/entry-dynamic.exe", "/ts/musl")
    //     .args(&["vfork"])
    //     .env("DEBUG", "1")
    //     .run();
    // info!("All tests completed");
}
