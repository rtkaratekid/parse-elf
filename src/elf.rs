#![allow(dead_code)]
#![allow(unused)]

use std::os::raw::c_char;

/* 32-bit ELF base types. */
pub type Elf32Addr = u32; //	Elf32_Addr;
pub type Elf32Half = u16; //   typedef u16	Elf32Half;
pub type Elf32Off = u32; // typedef u32	Elf32Off;
pub type Elf32Sword = i32; // typedef s32	Elf32Sword;
pub type Elf32Word = u32; //  typedef u32	Elf32_Word;

/* 64-bit ELF base types. */
pub type Elf64Addr = u64; //	Elf64_Addr;
pub type Elf64Half = u16; //	Elf64Half;
pub type Elf64SHalf = i16; //	Elf64SHalf;
pub type Elf64Off = u64; //	Elf64Off;
pub type Elf64Sword = i32; //Elf64Sword;
pub type Elf64Word = u32; //	Elf64_Word;
pub type Elf64Xword = u64; // typedef u64	Elf64_Xword;
pub type Elf64Sxword = i64; // typedef s64	Elf64_Sxword;

pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI: usize = 7;
pub const EI_PAD: usize = 8;

pub const ELFMAG0: u8 = 0x7f;
pub const ELFMAG1: u8 = 'E' as c_char as u8;
pub const ELFMAG2: u8 = 'L' as c_char as u8;
pub const ELFMAG3: u8 = 'F' as c_char as u8;
// const ELFMAG    "\177EL
// const SELFMAG   4

pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;
pub const ELFCLASSNUM: u8 = 3;

pub const ELFDATANONE: u8 = 0;
pub const ELFDATA2LSB: u8 = 1;
pub const ELFDATA2MSB: u8 = 2;

    // const EV_NONE   0   /*
    // const EV_CURRENT  1
    // const EV_NUM    2

pub const ELFOSABI_NONE: u8 = 0;
pub const ELFOSABI_LINUX: u8 = 3;

pub struct Elf32Ehdr {
    pub e_ident: Vec<u8>, // unsigned char	e_ident[EI_NIDENT];
    // pub e_ident: u8 [EI_NIDENT], // something like that?
    pub e_type: Elf32Half,
    pub e_machine: Elf32Half,
    pub e_version: Elf32Word,
    pub e_entry: Elf32Addr,
    pub e_phoff: Elf32Off,
    pub e_shoff: Elf32Off,
    pub e_flags: Elf32Word,
    pub e_ehsize: Elf32Half,
    pub e_phentsize: Elf32Half,
    pub e_phnum: Elf32Half,
    pub e_shentsize: Elf32Half,
    pub e_shnum: Elf32Half,
    pub e_shstrndx: Elf32Half,
}

pub struct Elf64Ehdr {
    pub e_ident: Vec<u8>, /* ELF "magic number" */
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

pub struct Elf32Shdr {
    pub sh_name: Elf32Word,
    pub sh_type: Elf32Word,
    pub sh_flags: Elf32Word,
    pub sh_addr: Elf32Addr,
    pub sh_offset: Elf32Off,
    pub sh_size: Elf32Word,
    pub sh_link: Elf32Word,
    pub sh_info: Elf32Word,
    pub sh_addralignr: Elf32Word,
    pub sh_entsize: Elf32Word,
}

pub struct Elf64Shdr {
    pub sh_name: Elf64Word,       /* Section name, index in string tbl */
    pub sh_type: Elf64Word,       /* Type of section */
    pub sh_flags: Elf64Word,      /* Miscellaneous section attributes */
    pub sh_addr: Elf64Addr,       /* Section virtual addr at execution */
    pub sh_offset: Elf64Off,      /* Section file offset */
    pub sh_size: Elf64Word,       /* Size of section in bytes */
    pub sh_link: Elf64Word,       /* Index of another section */
    pub sh_info: Elf64Word,       /* Additional section information */
    pub sh_addralignr: Elf64Word, /* Section alignment */
    pub sh_entsize: Elf64Word,    /* Entry size if section holds table */
}
