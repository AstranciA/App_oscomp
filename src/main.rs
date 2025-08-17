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
use core::ptr::read;

use axmono::{fs::init_fs, task::init_proc};
use testcase::*;

#[unsafe(no_mangle)]
fn main() {
    axmono::init();
    // 初始化测试环境
    mount_testsuite();

    init_fs();

    //TestCaseBuilder::shell("/ts/musl").script("/libctest_testcode.sh").run();
    //TestCaseBuilder::shell("/").script("/ts/git-2.46.0/git init").run();
    /*
     *git.clone().args(&["config", "--global", "user.email", "AstrancE"]).run();
     *git.clone().args(&["config", "--global", "AstrancE"]).run();
     */
    //TestCaseBuilder::new("/ts/glibc/copy-file-range-test-1", "/ts/glibc").run();
    //TestCaseBuilder::busybox("/").arg("--install").run();
    //TestCaseBuilder::shell("/ts/musl/ltp/testcases/bin").script("/usr/bin/busybox ls /proc/").run();
    //TestCaseBuilder::shell("/ts/musl/ltp/testcases/bin").script("/usr/bin/busybox cat /proc/4/stat").run();
    //run_testcode("copy-file-range", "glibc");
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
    if axfs::api::read("/ts/musl/test_splice").is_ok() {
        run_testcode("copy-file-range", "musl");
        run_testcode("interrupts", "musl");
        run_testcode("splice", "musl");
        run_testcode("copy-file-range", "glibc");
        run_testcode("interrupts", "glibc");
        run_testcode("splice", "glibc");
    } else {
        TestCaseBuilder::busybox("/").arg("--install").run();
        TestCaseBuilder::shell("/ts/musl")
            .script("/testrun.sh")
            .run();

        #[cfg(target_arch = "loongarch64")]
        {
            TestCaseBuilder::shell("/ts/musl")
                .script("/libctest_testcode.sh")
                .run();

            TestCaseBuilder::shell("/ts/musl")
                .script("./busybox echo \"#### OS COMP TEST GROUP END libctest-musl ####\"")
                .run();
        }

        TestCaseBuilder::shell("/ts/glibc")
            .script("/testrun_glibc.sh")
            .run();
        TestCaseBuilder::shell("/ts/musl/ltp/testcases/bin")
            .script("/test_ltp.sh")
            .run();

        #[cfg(target_arch = "riscv64")]
        TestCaseBuilder::shell("/ts/glibc/ltp/testcases/bin")
            .script("/test_ltp_glibc.sh")
            .run();

        run_testcode("libcbench", "musl"); // will panic
        run_testcode("libcbench", "glibc"); // will panic
    }
}

fn git_test() {
    axfs::api::remove_dir("/ts/test/.git");
    axfs::api::create_dir("/ts/test/.git");
    axfs::api::write("/ts/test/.git/config", "");
    axfs::api::write("/ts/test/.git/HEAD", "ref: refs/heads/master");
    let git = TestCaseBuilder::new("/ts/git-2.46.0/git", "/ts/test")
        .env("GIT_TRACE", "")
        .env("GIT_TRACE_SETUP", "");

    git.clone().arg("init").run();
    axfs::api::write("/ts/test/hello", "world!");
    git.clone().args(&["add", "."]).run();
    git.clone().arg("status").run();
    git.clone().args(&["commit", "-m", "add hello"]).run();
    git.clone().args(&["branch", "new_branch"]).run();
    git.clone().args(&["checkout", "new_branch"]).run();
    axfs::api::write("/ts/test/world", "hello");
    git.clone().args(&["add", "."]).run();
    git.clone().arg("status").run();
    git.clone().args(&["commit", "-m", "add hello"]).run();
    TestCaseBuilder::busybox("/ts/test")
        .arg("cat")
        .arg("world")
        .run();
    git.clone().args(&["checkout", "master"]).run();
    TestCaseBuilder::busybox("/ts/test")
        .arg("cat")
        .arg("hello")
        .run();
}
