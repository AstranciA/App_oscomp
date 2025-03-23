use axhal::mem::VirtAddr;
use xmas_elf::ElfFile;

use crate::elf::ELFInfo;

/// Get the total number of applications.
pub fn get_num_app() -> usize {
    unsafe extern "C" {
        fn _num_app();
    }

    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// Load nth user app at
/// [APP_BASE_ADDRESS + n * APP_SIZE_LIMIT, APP_BASE_ADDRESS + (n+1) * APP_SIZE_LIMIT).
pub fn load_app(idx: usize) -> &'static [u8] {
    unsafe extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    // load apps
    // load app from data section to memory
    let src = unsafe {
        core::slice::from_raw_parts(
            app_start[idx] as *const u8,
            app_start[idx + 1] - app_start[idx],
        )
    };
    src
}

pub fn load_app_elf(idx: usize) -> ElfFile<'static> {
    let app_slice = load_app(idx);
    ElfFile::new(app_slice).unwrap()
}
