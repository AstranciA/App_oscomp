//#![cfg_attr(feature = "axstd", no_std)]
//#![cfg_attr(feature = "axstd", no_main)]
#![no_std]
#![no_main]

use core::arch::global_asm;

#[macro_use]
extern crate axstd;
#[macro_use]
extern crate axlog;
#[macro_use]
extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::ToString;
use alloc::sync::Arc;
use axerrno::AxResult;
use axhal::{
    arch::{TrapFrame, UspaceContext},
    mem::VirtAddr,
    trap::{SYSCALL, register_trap_handler},
};
use axmm::{AddrSpace, kernel_aspace};
use axmono::{loader::load_app_from_disk, mm::load_elf_to_mem};
use axstd::println;
use axsync::Mutex;
use axsyscall::syscall_handler;

//#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
#[unsafe(no_mangle)]
fn main() {
    // file_type=jsonc to enable IDE format and comment
    let TESTCASES = include!("./testcase_list.jsonc");

    /*
     *    let read_dir = axfs::api::read_dir("/").unwrap();
     *    for entry in read_dir {
     *        let entry = entry.unwrap();
     *        println!("entry: {:?}", entry);
     *
     *        if !entry.file_type().is_file() {
     *            continue;
     *        }
     *        run_testcase(entry.path().as_str());
     *    }
     */

    for &t in TESTCASES.iter() {
        println!("running testcase: {t}");
        run_testcase(t);
    }
}

fn run_testcase(app_path: &str) -> isize {
    let (entry_vaddr, user_stack_base, uspace) = load_elf_to_mem(
        load_app_from_disk(app_path).unwrap(),
        Some(&[app_path.into()]),
        None,
    )
    .unwrap();
    debug!(
        "app_entry: {:?}, app_stack: {:?}, app_aspace: {:?}",
        entry_vaddr,
        user_stack_base,
        uspace,
    );

    let uctx = UspaceContext::new(entry_vaddr.into(), user_stack_base, 2333);

    let user_task = axmono::task::spawn_user_task(app_path, Arc::new(Mutex::new(uspace)), uctx);

    axtask::spawn_task_by_ref(user_task.clone());

    let exit_code = user_task.join().unwrap();
    info!("app exit with code: {:?}", exit_code);
    exit_code as isize
}

#[register_trap_handler(SYSCALL)]
fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> Option<isize> {
    let args = [
        tf.arg0(),
        tf.arg1(),
        tf.arg2(),
        tf.arg3(),
        tf.arg4(),
        tf.arg5(),
    ];

    debug!("Syscall: {:?}, args: {:?}", syscall_num, args);
    let result = match syscall_handler(syscall_num, args) {
        Ok(result) => Some(result.into()),
        Err(e) => None,
    };

    result
}
