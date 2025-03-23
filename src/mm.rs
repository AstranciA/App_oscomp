use alloc::string::ToString;
use axerrno::AxResult;
use axhal::{
    mem::VirtAddr,
    paging::MappingFlags,
    trap::{PAGE_FAULT, register_trap_handler},
};
use axmm::AddrSpace;
use axstd::os::arceos::modules::axconfig;

use crate::{
    config::{self},
    copy_from_kernel,
    elf::ELFInfo,
    loader,
};

fn new_user_aspace_empty() -> AxResult<AddrSpace> {
    AddrSpace::new_empty(
        VirtAddr::from_usize(config::USER_SPACE_BASE),
        config::USER_SPACE_SIZE,
    )
}

/// load app to memory
/// # Returns
/// - The first return value is the entry point of the user app.
/// - The second return value is the top of the user stack.
/// - The third return value is the address space of the user app.
pub fn load_user_app(app_name: &str, app_id: usize) -> AxResult<(VirtAddr, VirtAddr, AddrSpace)> {
    let mut uspace = new_user_aspace_empty()
        .and_then(|mut it| {
            copy_from_kernel(&mut it)?;
            Ok(it)
        })
        .expect("Failed ot create user address space");
    let (entry, ustack_pointer) = map_elf_sections(app_name, app_id, &mut uspace)?;
    Ok((entry, ustack_pointer, uspace))
}

pub fn map_elf_sections(
    app_name: &str,
    app_id: usize,
    uspace: &mut AddrSpace,
) -> Result<(VirtAddr, VirtAddr), axerrno::AxError> {
    //let elf_info = loader::load_elf(app_name, uspace.base());
    let mut elf_info = ELFInfo::new(loader::load_app_elf(app_id), uspace.base());
    for segement in elf_info.segments.iter() {
        debug!(
            "Mapping ELF segment: [{:#x?}, {:#x?}) flags: {:#x?}",
            segement.start_va,
            segement.start_va + segement.size,
            segement.flags
        );
        uspace.map_alloc(segement.start_va, segement.size, segement.flags, true)?;

        if segement.data.is_empty() {
            continue;
        }

        uspace.write(segement.start_va + segement.offset, segement.data)?;
        // TDOO: flush the I-cache
    }

    // The user stack is divided into two parts:
    // `ustack_start` -> `ustack_pointer`: It is the stack space that users actually read and write.
    // `ustack_pointer` -> `ustack_end`: It is the space that contains the arguments, environment variables and auxv passed to the app.
    //  When the app starts running, the stack pointer points to `ustack_pointer`.
    let ustack_end = VirtAddr::from_usize(config::USER_STACK_TOP);
    let ustack_size = config::USER_STACK_SIZE;
    let ustack_start = ustack_end - ustack_size;
    debug!(
        "Mapping user stack: {:#x?} -> {:#x?}",
        ustack_start, ustack_end
    );
    // FIXME: Add more arguments and environment variables
    let stack_data = kernel_elf_parser::app_stack_region(
        &[app_name.to_string()],
        &[],
        elf_info.auxv.as_mut_slice(),
        ustack_start,
        ustack_size,
    );
    uspace.map_alloc(
        ustack_start,
        ustack_size,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        true,
    )?;

    uspace.write(ustack_start, stack_data.as_slice())?;
    //Ok((elf_info.entry, VirtAddr::from_ptr_of(stack_data.as_ptr())))
    Ok((elf_info.entry, ustack_end))
}

#[register_trap_handler(PAGE_FAULT)]
fn handle_page_fault(vaddr: VirtAddr, access_flags: MappingFlags, is_user: bool) -> bool {
    debug!(
        "Page fault at {:#x?}, flags: {:#x?}, is_user: {:?}",
        vaddr, access_flags, is_user
    );
    todo!();
    true
}
