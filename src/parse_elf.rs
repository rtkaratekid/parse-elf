#![allow(dead_code)]
#![allow(unused)]

use std::{fmt, fs::File, io::Read, mem::size_of, os::raw::c_char, os::unix::io::AsRawFd};

//libc types
use libc::{off_t, SEEK_SET};

// libc functions
use libc::lseek;

use anyhow::{anyhow, Context, Result};

use crate::elf::*;

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

    // I'm sure this cast doesn't work at all like I expect it to because I
    // don't think Vec<u8> has the same profile as what I'd expect to read
    // from the elf header, I need a size for that vec! Maybe I should just move
    // to a slice instead.
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
