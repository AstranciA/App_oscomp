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
    mount_testsuite();

    init_fs();
    //TestCaseBuilder::busybox("/").arg("--install").run();
    //TestCaseBuilder::shell("/ts/musl/ltp/testcases/bin").script("/test_ltp.sh").run();
    //TestCaseBuilder::shell("/ts/musl/ltp/testcases/bin").script("/usr/bin/busybox ls /proc/").run();
    //TestCaseBuilder::shell("/ts/musl/ltp/testcases/bin").script("/usr/bin/busybox cat /proc/4/stat").run();
    //run_testcode("copy-file-range", "musl");
    //run_testcode("interrupts", "musl");
    //run_testcode("splice", "musl");
    //run_testcode("ltp", "musl");
    //TestCaseBuilder::new("/ts/musl/ltp/testcases/bin/abort01", "/ts/musl").run();
    //oscomp_test();

    // Should init once to init coreutils
    //TestCaseBuilder::busybox("/").arg("--install").run();

    //TestCaseBuilder::shell("/").run();
    //TestCaseBuilder::shell("/ts/musl").script("zcat /proc/config.gz").run();
    //TestCaseBuilder::new("/ts/musl/ltp/testcases/bin/cgroup_fj_proc", "/ts/musl").run();
    //TestCaseBuilder::new("/ts/musl/ltp/testcases/bin/rt_sigsuspend01", "/ts/musl/ltp/testcases/bin").run();

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
    TestCaseBuilder::busybox("/").arg("--install").run();
    TestCaseBuilder::shell("/ts/musl")
        .script("/testrun.sh")
        .run();
    TestCaseBuilder::shell("/ts/glibc")
        .script("/testrun_glibc.sh")
        .run();
    TestCaseBuilder::shell("/ts/musl/ltp/testcases/bin").script("/test_ltp.sh").run();
}
