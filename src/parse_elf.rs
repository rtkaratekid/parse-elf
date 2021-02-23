#![allow(dead_code)]
#![allow(unused)]
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

fn read_elf_header64(fd: i32, elf_header: *mut Elf64Ehdr) {
    ()
}

/* Functions for x86_64 */

fn is_elf64(eh: Elf64Ehdr) {
    ()
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
