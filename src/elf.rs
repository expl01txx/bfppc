use object::elf::{ELFOSABI_SYSV, EM_X86_64, ET_EXEC, FileFlags, PF_R, PF_X, PT_LOAD};
use object::write::elf::{FileHeader, ProgramHeader, Writer};

pub fn generate_elf(code: &[u8], load_addr: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut writer = Writer::new(object::Endianness::default(), true, &mut buf);

    writer.reserve_file_header();
    writer.reserve_program_headers(1);

    let code_offset = writer.reserve(code.len() as u64, 1);
    let entry = load_addr + code_offset;

    let ehdr = FileHeader {
        os_abi: ELFOSABI_SYSV,
        abi_version: 0,
        e_type: ET_EXEC,
        e_machine: EM_X86_64,
        e_entry: entry,
        e_flags: FileFlags::default(),
    };
    writer.write_file_header(&ehdr).unwrap();

    let total_size = code_offset + code.len() as u64;

    let phdr = ProgramHeader {
        p_type:   PT_LOAD,
        p_flags:  PF_R | PF_X,
        p_offset: 0,
        p_vaddr:  load_addr,
        p_paddr:  load_addr,
        p_filesz: total_size,
        p_memsz:  total_size,
        p_align:  0x1000,
    };
    writer.write_program_header(&phdr);

    writer.write(code);

    buf
}