// src/testcase.rs

#![no_std]
#![no_main]
#![feature(stmt_expr_attributes)]

use core::arch::naked_asm;

use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

use axfs::api::{create_dir, read_dir};
use axfs::{DISKS, Disk, ROOT_DIR};
use axhal::arch::UspaceContext;
use axlog::{debug, info, warn};
use axmono::init_proc;
use axmono::{
    copy_from_kernel,
    loader::load_elf_from_disk,
    mm::{load_elf_to_mem, new_user_aspace_empty},
};
use axsync::Mutex;
use lazyinit::LazyInit;

/// 测试用例构建器（Builder模式实现）
pub struct TestCaseBuilder {
    app_path: String,
    pwd: String,
    args: Vec<String>,
    env: Vec<String>,
}

impl TestCaseBuilder {
    /// 创建新 Builder（通用程序）
    pub fn new(app_path: &str, pwd: &str) -> Self {
        Self {
            app_path: app_path.to_string(),
            pwd: pwd.to_string(),
            args: vec![app_path.to_string()], // 默认包含程序名作为 argv[0]
            env: vec!["PATH=/usr/bin/".to_string()], // 默认环境变量
        }
    }

    /// 创建 shell 测试用例（使用 busybox ash）
    pub fn shell(pwd: &str) -> Self {
        //Self::new("/usr/bin/busybox", pwd).arg("sh") // 默认使用 busybox 的 ash
        Self::new("/ts/musl/busybox", pwd).arg("sh") // 默认使用 busybox 的 ash
    }

    /// 添加单个命令行参数
    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// 添加多个命令行参数
    pub fn args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    /// 设置要执行的 shell 脚本内容（自动添加 -c 参数）
    pub fn script(mut self, script: &str) -> Self {
        self.args.extend(["-c".into(), script.into()]);
        self
    }

    /// 设置环境变量
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env.push(format!("{}={}", key, value));
        self
    }

    /// 执行测试用例
    pub fn run(self) -> isize {
        run_testcase_inner(&self.app_path, &self.pwd, &self.args, &self.env)
    }
}

/// 内部测试执行函数
pub(crate) fn run_testcase_inner(
    app_path: &str,
    pwd: &str,
    args: &[String],
    env: &[String],
) -> isize {
    let mut uspace = new_user_aspace_empty()
        .and_then(|mut it| {
            copy_from_kernel(&mut it)?;
            Ok(it)
        })
        .expect("Failed to create user address space");

    let (entry_vaddr, user_stack_base, tp) = load_elf_to_mem(
        load_elf_from_disk(app_path).unwrap(),
        &mut uspace,
        Some(args),
        Some(env),
    )
    .unwrap();

    debug!(
        "app_entry: {:#x}, app_stack: {:#x}",
        entry_vaddr, user_stack_base
    );

    let mut uctx = UspaceContext::new(entry_vaddr.into(), user_stack_base, 0);
    if let Some(tp) = tp {
        uctx.set_tp(tp.as_usize());
    }

    let user_task = axmono::task::spawn_user_task(
        app_path,
        Arc::new(Mutex::new(uspace)),
        uctx,
        pwd.into(),
        Some(init_proc()),
    );

    axtask::spawn_task_by_ref(user_task.clone());
    user_task.join().unwrap() as isize
}

/// 测试套件挂载点
const TS_MOUNTPOINT: &str = "/ts";

/// 挂载测试文件系统
pub(crate) fn mount_testsuite() {
    let (ts_devname, ts_disk) = DISKS.lock().pop_first().expect("no testcase disk");
    create_dir(TS_MOUNTPOINT);

    static TS_FS: LazyInit<Arc<axfs::fs::lwext4_rust::Ext4FileSystem<Disk>>> = LazyInit::new();
    TS_FS.init_once(Arc::new(axfs::fs::lwext4_rust::Ext4FileSystem::new(
        ts_disk,
        "disk",
        &format!("{}/", TS_MOUNTPOINT), // 需要以/结尾
    )));

    ROOT_DIR.mount(TS_MOUNTPOINT, TS_FS.clone()).unwrap();
}

/// 运行特定测试代码
pub(crate) fn run_testcode(name: &str, c_lib: &str) -> isize {
    let testcode = format!("./{}_testcode.sh", name);
    TestCaseBuilder::shell(&format!("/ts/{}", c_lib))
        .arg(&testcode)
        .run()
}
