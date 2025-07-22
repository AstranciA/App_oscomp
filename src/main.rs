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
use core::ptr;

mod testcase;
use alloc::vec::Vec;
use axfs::{api::read, proc};
use axhal::{
    arch::enable_irqs,
    irq::register_irq_handler,
    mem::{PhysAddr, phys_to_virt},
};
use axmono::{fs::init_fs, task::init_proc};
use axstd::{
    io::{Read, Stdin},
    println,
};
use testcase::*;

#[unsafe(no_mangle)]
fn main() {
    axmono::init();
    // 初始化测试环境
    mount_testsuite();

    //oscomp_test();
    enable_irqs();

    /*
     *for i in 1..12 {
     *    register_irq_handler(i, || {
     *        warn!("plic handler");
     *    });
     *}
     */
    //run_testcode("basic", "musl");
    init_fs();
    //TestCaseBuilder::new("/ts/glibc/basic/execve", "/ts/glibc").run();
    let a = read("/proc/interrupts");
    println!("{a:?}");
    TestCaseBuilder::new("/ts/glibc/basic/execve", "/ts/glibc").run();
    let a = read("/proc/interrupts");
    println!("{a:?}");

    /*
     *TestCaseBuilder::new("/usr/bin/busybox", "/")
     *    .arg("sh")
     *    .run();
     *tmp_test();
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
    /*
     *TestCaseBuilder::shell("/ts/musl/basic")
     *    .script("busybox which ls")
     *    .run();
     */
    /*
     *TestCaseBuilder::shell("/ts/glibc")
     *    .script("/test_busybox2.sh")
     *    .run();
     */
    //run_testcode("busybox", "musl");

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

fn tmp_test() {
    print_plic(0);
    println!("------------------------------------\n");

    let ctrl = axhal::arch::plic::GLOBAL_PLIC_CONTROLLER.lock();
    ctrl.print_plic_status(0);
}

const PLIC_BASE_ADDRESS: usize = phys_to_virt(PhysAddr::from_usize(0x0C000000)).as_usize(); // 示例 PLIC 基地址

const CLAIM_COMPLETE_OFFSET_IN_CONTEXT: usize = 0x0004; // ClaimComplete 在 IrqEnablePerContext 中的偏移 (注意：这里假设 ClaimComplete 紧跟在 EnableBits 之后，如果 EnableBits 是数组，则需要更精确的计算)

// PLIC 内存布局的各个部分的偏移和大小
const INTERRUPT_PRIORITY_OFFSET: usize = 0x000000;
const INTERRUPT_PRIORITY_SIZE: usize = 0x000004 * MAX_IRQ_SOURCES;

const INTERRUPT_PENDING_OFFSET: usize = 0x001000;
const INTERRUPT_PENDING_SIZE: usize = 0x000004 * ((MAX_IRQ_SOURCES + 31) / 32);

// *** 修正这里 ***
// ContextEnables 数组的起始偏移 (相对于 PLIC_BASE_ADDRESS)
const CONTEXT_ENABLES_ARRAY_OFFSET: usize = 0x002000;
// IrqEnablePerContext 结构体的大小
const IRQ_ENABLE_PER_CONTEXT_SIZE: usize = 0x80;

// ContextRegs 数组的起始偏移 (相对于 PLIC_BASE_ADDRESS)
const CONTEXT_REGS_ARRAY_OFFSET: usize = 0x200000;
// ContextSpecificRegs 结构体的大小
const CONTEXT_SPECIFIC_REGS_SIZE: usize = 0x1000;

// ContextSpecificRegs 内部的偏移
const CONTEXT_PRIORITY_THRESHOLD_OFFSET_IN_BLOCK: usize = 0x0000;
const CONTEXT_CLAIM_COMPLETE_OFFSET_IN_BLOCK: usize = 0x0004;

// 假设 MAX_IRQ_SOURCES 是已知的，例如 1024
const MAX_IRQ_SOURCES: usize = 1024;
const U32_BITS: usize = u32::BITS as usize; // Added for consistency with your struct

/// 打印 PLIC MMIO 区域的全部内容
///
/// # Arguments
/// * `context_id` - 要打印的上下文 ID (通常是 mhartid)
fn print_plic(context_id: usize) {
    // 假设 phys_to_virt 函数已经将物理地址转换为虚拟地址
    let plic_base_virt_addr = PLIC_BASE_ADDRESS;
    println!(
        "\n--- PLIC MMIO Dump (Base: 0x{:016x}) ---",
        plic_base_virt_addr
    );

    // 1. 打印中断优先级 (Interrupt Priorities)
    println!("\n[Interrupt Priorities]");
    let priority_base = plic_base_virt_addr.saturating_add(INTERRUPT_PRIORITY_OFFSET);
    let priority_end = priority_base.saturating_add(INTERRUPT_PRIORITY_SIZE);
    let mut current_addr = priority_base;

    // 只打印前 32 个 IRQ 的优先级作为示例 (或者根据需要调整数量)
    for i in 0..32 {
        let irq_addr = priority_base.saturating_add(i * core::mem::size_of::<u32>());
        if irq_addr >= priority_end {
            break;
        }
        let ptr = irq_addr as *const u32;
        let value = unsafe { ptr::read_volatile(ptr) };
        println!("  IRQ {:>3}: Priority = {}", i, value);
    }
    if priority_end > priority_base.saturating_add(32 * core::mem::size_of::<u32>()) {
        println!("  ... (truncated for brevity) ...");
    }

    // 2. 打印中断挂起状态 (Interrupt Pending Bits)
    println!("\n[Interrupt Pending Bits]");
    let pending_base = plic_base_virt_addr.saturating_add(INTERRUPT_PENDING_OFFSET);
    let pending_end = pending_base.saturating_add(INTERRUPT_PENDING_SIZE);
    current_addr = pending_base;

    // 遍历每个 32 位挂起寄存器
    for i in 0..((MAX_IRQ_SOURCES + U32_BITS - 1) / U32_BITS) {
        // 确保遍历所有挂起组
        let reg_addr = pending_base.saturating_add(i * core::mem::size_of::<u32>());
        if reg_addr >= pending_end {
            break;
        } // 防止越界

        let ptr = reg_addr as *const u32;
        let value = unsafe { ptr::read_volatile(ptr) };
        println!("  Pending[{:>2}] (0x{:016x}): 0x{:08x}", i, reg_addr, value);
    }

    // 3. 打印特定上下文的中断使能状态 (Interrupt Enable Bits)
    println!("\n[Interrupt Enable Bits for Context {}]", context_id);
    // *** 修正这里的地址计算 ***
    let context_enable_base_addr_res = plic_base_virt_addr
        .checked_add(CONTEXT_ENABLES_ARRAY_OFFSET) // 从 ContextEnables 数组的起始偏移开始
        .and_then(|addr| addr.checked_add(context_id.checked_mul(IRQ_ENABLE_PER_CONTEXT_SIZE)?)); // 加上上下文索引 * 每个 IrqEnablePerContext 的大小

    if let Some(context_enable_base_addr) = context_enable_base_addr_res {
        let enable_end = context_enable_base_addr.saturating_add(IRQ_ENABLE_PER_CONTEXT_SIZE);
        current_addr = context_enable_base_addr;

        // 遍历每个 32 位使能寄存器 (IrqEnablePerContext 内部的 EnableBits 数组)
        for i in 0..((MAX_IRQ_SOURCES + U32_BITS - 1) / U32_BITS) {
            let reg_addr = context_enable_base_addr.saturating_add(i * core::mem::size_of::<u32>());
            if reg_addr >= enable_end {
                break;
            } // 防止越界

            let ptr = reg_addr as *const u32;
            let value = unsafe { ptr::read_volatile(ptr) };
            println!("  Enable[{:>2}] (0x{:016x}): 0x{:08x}", i, reg_addr, value);
            // 可以选择性地打印哪些 IRQ 被启用
            for bit_idx in 0..U32_BITS {
                if (value >> bit_idx) & 1 != 0 {
                    let irq_num = i * U32_BITS + bit_idx;
                    if irq_num < MAX_IRQ_SOURCES {
                        // println!("    -> IRQ {} enabled", irq_num); // 避免打印过多信息
                    }
                }
            }
        }
    } else {
        println!(
            "  Error: Could not calculate address for Context {} Enable Bits.",
            context_id
        );
    }

    // 4. 打印特定上下文的优先级阈值 (Priority Threshold)
    println!("\n[Priority Threshold for Context {}]", context_id);
    // *** 修正这里的地址计算 ***
    let threshold_addr_res = plic_base_virt_addr
        .checked_add(CONTEXT_REGS_ARRAY_OFFSET) // 从 ContextRegs 数组的起始偏移开始
        .and_then(|addr| addr.checked_add(context_id.checked_mul(CONTEXT_SPECIFIC_REGS_SIZE)?)) // 加上上下文索引 * 每个 ContextSpecificRegs 的大小
        .and_then(|addr| addr.checked_add(CONTEXT_PRIORITY_THRESHOLD_OFFSET_IN_BLOCK)); // 加上 ContextSpecificRegs 内部的偏移

    if let Some(threshold_addr) = threshold_addr_res {
        let ptr = threshold_addr as *const u32;
        let value = unsafe { ptr::read_volatile(ptr) };
        println!("  Threshold (0x{:016x}): {}", threshold_addr, value);
    } else {
        println!(
            "  Error: Could not calculate address for Context {} Priority Threshold.",
            context_id
        );
    }

    // 5. 打印特定上下文的 Claim/Complete 寄存器
    println!("\n[Claim/Complete for Context {}]", context_id);
    // *** 修正这里的地址计算 ***
    let claim_complete_addr_res = plic_base_virt_addr
        .checked_add(CONTEXT_REGS_ARRAY_OFFSET) // 从 ContextRegs 数组的起始偏移开始
        .and_then(|addr| addr.checked_add(context_id.checked_mul(CONTEXT_SPECIFIC_REGS_SIZE)?)) // 加上上下文索引 * 每个 ContextSpecificRegs 的大小
        .and_then(|addr| addr.checked_add(CONTEXT_CLAIM_COMPLETE_OFFSET_IN_BLOCK)); // 加上 ContextSpecificRegs 内部的偏移

    if let Some(claim_complete_addr) = claim_complete_addr_res {
        let ptr = claim_complete_addr as *const u32;
        let value = unsafe { ptr::read_volatile(ptr) };
        println!(
            "  Claim/Complete (0x{:016x}): {}",
            claim_complete_addr, value
        );
        if value != 0 {
            println!("    -> Currently pending IRQ ID: {}", value);
        } else {
            println!("    -> No pending IRQ claimed.");
        }
    } else {
        println!(
            "  Error: Could not calculate address for Context {} Claim/Complete.",
            context_id
        );
    }

    println!("\n--- End of PLIC MMIO Dump ---");
}
