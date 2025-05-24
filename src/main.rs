//#![cfg_attr(feature = "axstd", no_std)]
//#![cfg_attr(feature = "axstd", no_main)]
#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)]

extern crate axstd;
#[macro_use]
extern crate axlog;
#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use axhal::arch::UspaceContext;
use axmono::{loader::load_elf_from_disk, mm::load_elf_to_mem};
use axsync::Mutex;

//#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
#[unsafe(no_mangle)]
fn main() {
    //run_testcode("libctest", "musl");
    /*
     *run_testcase(
     *    "/riscv/musl/busybox",
     *    "/riscv/musl",
     *    Some(&[
     *        "/riscv/musl/busybox".into(),
     *        "ash".into(),
     *        "/riscv/musl/libc-bench".into(),
     *    ]),
     *);
     */
    run_testcase(
        "/riscv/musl/runtest.exe",
        "/riscv/musl",
        Some(&[
            "runtest.exe".into(),
            "-w".into(),
            "entry-static.exe".into(),
            "pthread_cancel_points".into(),
        ]),
    );
}

fn run_testcode(name: &str, c_lib: &str) {
    let shell = "/riscv/musl/busybox";
    let pwd = format!("/riscv/{}", c_lib);
    let testcode = format!("/riscv/{}/{}_testcode.sh", c_lib, name);
    run_testcase(
        shell,
        pwd.as_str(),
        Some(&[shell.into(), "ash".into(), testcode.into()]),
    );
}

fn run_testcase(app_path: &str, pwd: &str, args: Option<&[String]>) -> isize {
    let (entry_vaddr, user_stack_base, tp, uspace) =
        load_elf_to_mem(load_elf_from_disk(app_path).unwrap(), args, None).unwrap();
    debug!(
        "app_entry: {:?}, app_stack: {:?}, app_aspace: {:?}",
        entry_vaddr, user_stack_base, uspace,
    );

    let mut uctx = UspaceContext::new(entry_vaddr.into(), user_stack_base, 2333);
    if let Some(tp) = tp {
        uctx.set_tp(tp.as_usize());
    }
    let user_task = axmono::task::spawn_user_task(
        app_path,
        Arc::new(Mutex::new(uspace)),
        uctx,
        pwd.into(),
        true,
    );

    axtask::spawn_task_by_ref(user_task.clone());

    let exit_code = user_task.join().unwrap();
    info!("app exit with code: {:?}", exit_code);
    exit_code as isize
}
