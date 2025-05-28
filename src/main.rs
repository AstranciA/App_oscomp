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
use axfs::DISKS;
use axfs::Disk;
use axfs::ROOT_DIR;
use axfs::api::create_dir;
use axfs::api::read_dir;
use axhal::arch::UspaceContext;
use axmono::{loader::load_elf_from_disk, mm::load_elf_to_mem};
use axsync::Mutex;
use lazyinit::LazyInit;

//#[cfg_attr(feature = "axstd", unsafe(no_mangle))]
#[unsafe(no_mangle)]
fn main() {
    mount_testsuite();

    for d in read_dir("/").unwrap() {
        let d = d.unwrap();
        warn!("{:?}", d.file_name());
    }
    for d in read_dir(TS_MOUNTPOINT_).unwrap() {
        let d = d.unwrap();
        warn!("{:?}", d.file_name());
    }
    //run_testcode("libctest", "musl");
    //run_testcode("lua", "musl");
    run_testcase(
        "/usr/bin/busybox",
        "/ts/riscv/musl",
        Some(&[
            "/usr/bin/busybox".into(),
            "ash".into(),
            "test.sh".into(),
            "date.lua".into(),
        ]),
    );
    /*
     *run_testcase(
     *    "/ts/riscv/musl/runtest.exe",
     *    "/ts/riscv/musl",
     *    Some(&[
     *        "runtest.exe".into(),
     *        "-w".into(),
     *        "entry-static.exe".into(),
     *        "pthread_cancel".into(),
     *    ]),
     *);
     */
}

fn run_testcode(name: &str, c_lib: &str) {
    //let shell = "/usr/bin/busybox";
    let shell = "/usr/bin/busybox";
    let pwd = format!("/ts/riscv/{}", c_lib);
    let testcode = format!("/ts/riscv/{}/{}_testcode.sh", c_lib, name);
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

const TS_MOUNTPOINT_: &str = "/ts/";
const TS_MOUNTPOINT: &str = "/ts";

fn mount_testsuite() {
    let (ts_devname, ts_disk) = DISKS.lock().pop_first().expect("no testcase disk");
    create_dir(TS_MOUNTPOINT);
    static TS_FS: LazyInit<Arc<axfs::fs::lwext4_rust::Ext4FileSystem<Disk>>> = LazyInit::new();
    TS_FS.init_once(Arc::new(axfs::fs::lwext4_rust::Ext4FileSystem::new(
        ts_disk,
        "disk",
        &TS_MOUNTPOINT_,
    )));

    ROOT_DIR.mount(&TS_MOUNTPOINT, TS_FS.clone()).unwrap();
}
