#![allow(dead_code)]
#![allow(unused)]

use std::{fmt, fs::File, io::Read, mem::size_of, os::raw::c_char, os::unix::io::AsRawFd};

//libc types
use libc::{off_t, SEEK_SET};

// libc functions
use libc::lseek;

use anyhow::{anyhow, Context, Result};

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

const EI_MAG0: usize = 0;
const EI_MAG1: usize = 1;
const EI_MAG2: usize = 2;
const EI_MAG3: usize = 3;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
const EI_PAD: usize = 8;

const ELFMAG0: u8 = 0x7f;
const ELFMAG1: u8 = 'E' as c_char as u8;
const ELFMAG2: u8 = 'L' as c_char as u8;
const ELFMAG3: u8 = 'F' as c_char as u8;
// const ELFMAG    "\177EL
// const SELFMAG   4

const ELFCLASSNONE: u8 = 0;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const ELFCLASSNUM: u8 = 3;

const ELFDATANONE: u8 = 0;
const ELFDATA2LSB: u8 = 1;
const ELFDATA2MSB: u8 = 2;

// const EV_NONE   0   /*
// const EV_CURRENT  1
// const EV_NUM    2

const ELFOSABI_NONE: u8 = 0;
const ELFOSABI_LINUX: u8 = 3;

pub struct Elf32Ehdr {
    pub e_ident: Vec<u8>, // unsigned char	e_ident[EI_NIDENT];
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

fn disassemble(fd: i32, eh: Elf32Ehdr, sh_tbl: *mut Elf32Shdr) {
    ()
}

fn disassemble64(fd: i32, eh: Elf32Ehdr, sh_tbl: *mut Elf64Shdr) {
    ()
}

pub fn read_elf_header64(file: &mut File) -> Result<Elf64Ehdr> {
    // assert(lseek(fd, (off_t)0, SEEK_SET) == (off_t)0);
    unsafe {
        if lseek(file.as_raw_fd(), 0 as off_t, SEEK_SET) != 0 as off_t {
            return Err(anyhow!("[!] Could not lseek offset"));
        }
    }

    // assert(read(fd, (void *)elf_header, sizeof(Elf64_Ehdr)) == sizeof(Elf64_Ehdr));
    let mut buffer = [0; size_of::<Elf64Ehdr>()];

    if file.read(&mut buffer)? != size_of::<Elf64Ehdr>() {
        return Err(anyhow!(
            "[!] Read incorrect number of bytes to fetch ELF header"
        ));
    }

    // I'm sure this cast doesn't work at all like I expect it to
    let eh: Elf64Ehdr = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };

    Ok(eh)
}

/* Functions for x86_64 */

fn is_elf64(eh: Elf64Ehdr) -> Result<bool> {

    // check the first four bytes for the magic number
    let magic: Vec<u8> = vec![ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3];
    if eh.e_ident[0..5] == magic {
        println!("[*] We have an ELF file!");
        return Ok(true);
    }

    println!("[!] No ELF magic number, bailing.");
    Ok(false)
}

impl fmt::Display for Elf64Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* Storage capacity class */
        write!(f, "Storage class\t= ");
        match self.e_ident[EI_CLASS as usize] {
            ELFCLASS32 => writeln!(f, "32-bit objects"),
            ELFCLASS64 => writeln!(f, "64-bit objects"),
            _ => writeln!(f, "INVALID CLASS"),
        };

        /* Data Format */
        println!("Data format\t= ");
        match self.e_ident[EI_DATA] {
            ELFDATA2LSB => writeln!(f, "2's complement, little endian"),
            ELFDATA2MSB => writeln!(f, "2's complement, big endian"),
            _ => writeln!(f, "INVALID FORMAT"),
        };

        // TODO: I dont' know where the hell to get these macro values from
        // I don't see them in elf.h, maybe in libelf? libc?
        // found some reference here: https://static.re-ws.pl/makeelf/master/classmakeelf_1_1elfstruct_1_1ELFOSABI.html
        println!("OS ABI\t\t= ");
        match self.e_ident[EI_OSABI] {
            ELFOSABI_SYSV => writeln!(f, "UNIX System V ABI\n"),
            ELFOSABI_HPUX => writeln!(f, "HP-UX"),
            ELFOSABI_NETBSD => writeln!(f, "NetBSD"),
            ELFOSABI_LINUX => writeln!(f, "Linux"),
            ELFOSABI_SOLARIS => writeln!(f, "Sun Solaris"),
            ELFOSABI_AIX => writeln!(f, "IBM AIX"),
            ELFOSABI_IRIX => writeln!(f, "SGI Irix"),
            ELFOSABI_FREEBSD => writeln!(f, "FreeBSD"),
            ELFOSABI_TRU64 => writeln!(f, "Compaq TRU64 UNIX"),
            ELFOSABI_MODESTO => writeln!(f, "Novell Modesto"),
            ELFOSABI_OPENBSD => writeln!(f, "OpenBSD"),
            ELFOSABI_ARM_AEABI => writeln!(f, "ARM EABI"),
            ELFOSABI_ARM => writeln!(f, "ARM"),
            ELFOSABI_STANDALONE => writeln!(f, "Standalone (embedded) app"),
            _ => writeln!(f, "Unknown {}", self.e_ident[EI_OSABI]),
        };

        write!(f, "")
    }
}

fn print_elf_header64(elf_header: Elf64Ehdr) {
    ()
}

fn read_section_header_table64(fd: i32, eh: Elf64Ehdr, sh_table: Vec<Elf64Shdr>) {
    ()
}

fn read_section64(fd: i32, sh: Elf64Shdr) -> String {
    "yay".to_string()
}

fn print_section_headers64(fd: i32, eh: Elf64Ehdr, sh_table: Vec<Elf64Shdr>) {
    ()
}

fn print_symbol_table64(fd: i32, eh: Elf64Ehdr, sh_table: Vec<Elf64Shdr>, symbol_table: u32) {
    ()
}

fn print_symbols64(fd: i32, eh: Elf64Ehdr, sh_table: Vec<Elf64Shdr>) {
    ()
}

fn save_text_section64(fd: i32, eh: Elf64Ehdr, sh_table: Vec<Elf64Shdr>) {
    ()
}

fn read_elf_header(fd: i32, elf_header: *mut Elf32Ehdr) {
    ()
}

/* Functions for 32 bit */

fn is_elf(eh: Elf32Ehdr) -> bool {
    true
}

fn print_elf_header(elf_header: Elf32Ehdr) {
    ()
}

fn read_section_header_table(fd: i32, eh: Elf32Ehdr, sh_table: Vec<Elf32Shdr>) {
    ()
}

fn read_section(fd: i32, sh: Elf32Shdr) -> String {
    "yay".to_string()
}

fn print_section_headers(fd: i32, eh: Elf32Ehdr, sh_table: Vec<Elf32Shdr>) {
    ()
}

fn print_symbol_table(fd: i32, eh: Elf32Ehdr, sh_table: Vec<Elf32Shdr>, symbol_table: u32) {
    ()
}

fn print_symbols(fd: i32, eh: Elf32Ehdr, sh_table: Vec<Elf32Shdr>) {
    ()
}

fn save_text_section(fd: i32, eh: Elf32Ehdr, sh_table: Vec<Elf32Shdr>) {
    ()
}

fn is_64bit(eh: Elf32Ehdr) -> bool {
    true
}
