//#![cfg_attr(feature = "axstd", no_std)]
//#![cfg_attr(feature = "axstd", no_main)]
#![feature(new_range_api)]
#![no_std]
#![no_main]

use core::arch::global_asm;

#[macro_use]
extern crate axstd;
#[macro_use]
extern crate axlog;
#[macro_use]
extern crate alloc;

mod loader;

use alloc::borrow::Cow;
use alloc::string::ToString;
use alloc::sync::Arc;
use axerrno::AxResult;
use axhal::arch::TrapFrame;
use axhal::trap::{SYSCALL, register_trap_handler};
use axhal::{arch::UspaceContext, mem::VirtAddr};
use axmm::{AddrSpace, kernel_aspace};
use axstd::println;
use axsync::Mutex;
use axsyscall::syscall_handler;
use loader::load_app_from_disk;
//use mm::load_user_app;
use axmono::mm::load_elf_to_mem;

//global_asm!(include_str!("../link_apps.S"));

//#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
#[unsafe(no_mangle)]
fn main() {
    println!("Hello, world!");
    let TESTCASES = include!("./testcase_list");
    /*
     *let read_dir = axfs::api::read_dir("/").unwrap();
     *for entry in read_dir {
     *    let entry = entry.unwrap();
     *    println!("entry: {:?}", entry);
     *    if !entry.file_type().is_file() {
     *        continue;
     *    }
     *    run_testcase(entry.path().as_str());
     *}
     */
    run_testcase("/hello_world");
    run_testcase("/hello_world");
    //run_testcase("/hello_world");
    /*
     *for &t in TESTCASES.iter() {
     *    run_testcase(t);
     *    return;
     *}
     */
    //run_testcase_all();
}

fn run_testcase(app_path: &str) -> isize {
    let (entry_vaddr, ustack_top, uspace) =
        load_elf_to_mem(load_app_from_disk(app_path), Some(&[app_path.into()]), None).unwrap();
    debug!(
        "app_entry: {:?}, app_stack: {:?}, app_aspace: {:?}",
        entry_vaddr, ustack_top, uspace
    );
    let uctx = UspaceContext::new(entry_vaddr.into(), ustack_top, 2333);
    let user_task = axmono::task::spawn_user_task(app_path, Arc::new(Mutex::new(uspace)), uctx);

    let exit_code = user_task.join().unwrap();
    info!("app exit with code: {:?}", exit_code);
    exit_code as isize
}

#[register_trap_handler(SYSCALL)]
fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
    let args = [
        tf.arg0(),
        tf.arg1(),
        tf.arg2(),
        tf.arg3(),
        tf.arg4(),
        tf.arg5(),
    ];
    trace!("Syscall: {:?}, args: {:?}", syscall_num, args);
    let result = syscall_handler(syscall_num, args);
    //let result = unsafe { catch_unwind(syscall_handler, (syscall_num, args), |a, b| -1) }as isize;
    //let result = Ok(result);
    let result: isize = result.into();

    trace!("syscall_handler result: {:}", result);
    result
}

/// If the target architecture requires it, the kernel portion of the address
/// space will be copied to the user address space.
/// TODO: unsafe. using trampoline instead
fn copy_from_kernel(aspace: &mut AddrSpace) -> AxResult {
    if !cfg!(target_arch = "aarch64") && !cfg!(target_arch = "loongarch64") {
        // ARMv8 (aarch64) and LoongArch64 use separate page tables for user space
        // (aarch64: TTBR0_EL1, LoongArch64: PGDL), so there is no need to copy the
        // kernel portion to the user page table.
        aspace.copy_mappings_from(&kernel_aspace().lock())?;
    }

    Ok(())
}
