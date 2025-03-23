use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use axhal::{
    mem::{MemoryAddr, PAGE_SIZE_4K, VirtAddr},
    paging::MappingFlags,
};
use kernel_elf_parser::{AuxvEntry, ELFParser};
use xmas_elf::{
    ElfFile,
    header::{self, Header},
    program::{Flags, ProgramHeader, SegmentData},
};

const USPACE: [usize; 16 * 1024] = [0; 16 * 1024];
const USTACK: [usize; 4 * 1024] = [0; 4 * 1024];

/// The information of a given ELF file
pub struct ELFInfo {
    /// The entry point of the ELF file
    pub entry: VirtAddr,
    /// The segments of the ELF file
    pub segments: Vec<ELFSegment>,
    /// The auxiliary vectors of the ELF file
    pub auxv: [AuxvEntry; 16],
}

impl ELFInfo {
    pub fn new(elf: ElfFile<'static>, uspace_base: VirtAddr) -> Self {
        let elf_header = elf.header;

        // will be checked in parser
        //Self::assert_magic(&elf_header);

        println!("uspace_base:{:x}", uspace_base.as_usize());
        Self::check_arch(&elf_header).unwrap();
        let elf_parser = kernel_elf_parser::ELFParser::new(&elf, 0, None, uspace_base.as_usize()).unwrap();

        let elf_offset = elf_parser.base();
        println!("elf_offset: {:x}", elf_offset);

        let segments = elf
            .program_iter()
            .filter(|ph| ph.get_type() == Ok(xmas_elf::program::Type::Load))
            .map(|ph| {
                let st_va = VirtAddr::from(ph.virtual_addr() as usize) + elf_offset;
                let st_va_align: VirtAddr = st_va.align_down_4k();

                let ed_vaddr_align = VirtAddr::from((ph.virtual_addr() + ph.mem_size()) as usize)
                    .align_up_4k()
                    + elf_offset;

                let ph_flags = ph.flags();
                let flags = ELFSegment::into_to_mapping_flag(ph_flags);

                let size = ed_vaddr_align.as_usize() - st_va_align.as_usize();

                let data: &'static [u8] = match ph.get_data(&elf).unwrap() {
                    SegmentData::Undefined(data) => data,
                    _ => panic!("failed to get ELF segment data"),
                };

                ELFSegment {
                    flags,
                    start_va: st_va_align,
                    size,
                    data,
                    offset: st_va.align_offset_4k()
                }
            })
            .collect();

        info!("{:x}, {:x}", elf.header.pt2.entry_point(), elf_offset);
        ELFInfo {
            entry: VirtAddr::from(elf.header.pt2.entry_point() as usize + elf_offset),
            segments,
            auxv: elf_parser.auxv_vector(PAGE_SIZE_4K),
        }
    }

    pub fn assert_magic(elf_header: &Header) {
        assert_eq!(elf_header.pt1.magic, *b"\x7fELF", "invalid elf!");
    }

    pub fn check_arch(elf_header: &Header) -> Result<(), &'static str> {
        let expect_arch = if cfg!(target_arch = "x86_64") {
            header::Machine::X86_64
        } else if cfg!(target_arch = "aarch64") {
            header::Machine::AArch64
        } else if cfg!(target_arch = "riscv64") {
            header::Machine::RISC_V
        } else {
            return Err("Unsupported architecture!");
        };
        if elf_header.pt2.machine().as_machine() != expect_arch {
            return Err("Invalid ELF arch!");
        }
        Ok(())
    }
}
pub struct ELFSegment {
    pub start_va: VirtAddr,
    pub size: usize,
    pub flags: MappingFlags,
    pub data: &'static [u8],
    pub offset: usize,
}

impl ELFSegment {
    pub fn into_to_mapping_flag(ph_flags: Flags) -> MappingFlags {
        let mut ret = MappingFlags::USER;
        if ph_flags.is_read() {
            ret |= MappingFlags::READ;
        }
        if ph_flags.is_write() {
            ret |= MappingFlags::WRITE;
        }
        if ph_flags.is_execute() {
            ret |= MappingFlags::EXECUTE;
        }
        ret
    }
}
