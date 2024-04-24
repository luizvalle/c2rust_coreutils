#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
use ::rusty_echo::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn set_program_name(argv0: *const libc::c_char);
    static mut program_name: *const libc::c_char;
    fn proper_name_lite(
        name_ascii: *const libc::c_char,
        name_utf8: *const libc::c_char,
    ) -> *const libc::c_char;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn close_stdout();
    static mut Version: *const libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub const DEFAULT_ECHO_TO_XPG: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
use std::collections::HashMap;
#[inline]
fn emit_ancillary_info(program: &str) {
    // Define the infomap
    let infomap: HashMap<&str, Option<&str>> = [
        ("[", Some("test invocation")),
        ("coreutils", Some("Multi-call invocation")),
        ("sha224sum", Some("sha2 utilities")),
        ("sha256sum", Some("sha2 utilities")),
        ("sha384sum", Some("sha2 utilities")),
        ("sha512sum", Some("sha2 utilities")),
    ]
    .iter()
    .cloned()
    .collect();

    // Obtain the node from the infomap based on the program
    let node = infomap.get(program).and_then(|&node| node);

    // Print the online help message
    println!(
        "\n{} online help: <{}>",
        "GNU coreutils",
        "https://www.gnu.org/software/coreutils/"
    );

    // Check the LC_MESSAGES locale
    if let Some(lc_messages) = std::env::var("LC_MESSAGES").ok() {
        if !lc_messages.starts_with("en_") {
            println!(
                "{}",
                "Report any translation bugs to <https://translationproject.org/team/>"
            );
        }
    }

    // Define the URL for the program documentation
    let url_program = if program == "[" { "test" } else { program };

    // Print the full documentation URL
    println!(
        "Full documentation <{}{}>",
        "https://www.gnu.org/software/coreutils/",
        url_program
    );

    // Print the local documentation info
    println!(
        "or available locally via: info '(coreutils) {}{}'",
        node.unwrap_or(program),
        if node.is_none() { " invocation" } else { "" }
    );
}
#[inline]
fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) {
    if status == 0 as libc::c_int {} else {
        __assert_fail(
            b"status == 0\0" as *const u8 as *const libc::c_char,
            b"echo.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void usage(int)\0"))
                .as_ptr(),
        );
    }
    'c_6567: {
        if status == 0 as libc::c_int {} else {
            __assert_fail(
                b"status == 0\0" as *const u8 as *const libc::c_char,
                b"echo.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"void usage(int)\0"))
                    .as_ptr(),
            );
        }
    };
    printf(
        gettext(
            b"Usage: %s [SHORT-OPTION]... [STRING]...\n  or:  %s LONG-OPTION\n\0"
                as *const u8 as *const libc::c_char,
        ),
        program_name,
        program_name,
    );
    fputs_unlocked(
        gettext(
            b"Echo the STRING(s) to standard output.\n\n  -n             do not output the trailing newline\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            if DEFAULT_ECHO_TO_XPG as libc::c_int != 0 {
                b"  -e             enable interpretation of backslash escapes (default)\n  -E             disable interpretation of backslash escapes\n\0"
                    as *const u8 as *const libc::c_char
            } else {
                b"  -e             enable interpretation of backslash escapes\n  -E             disable interpretation of backslash escapes (default)\n\0"
                    as *const u8 as *const libc::c_char
            },
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"      --help        display this help and exit\n\0" as *const u8
                as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"      --version     output version information and exit\n\0" as *const u8
                as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"\nIf -e is in effect, the following sequences are recognized:\n\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"  \\\\      backslash\n  \\a      alert (BEL)\n  \\b      backspace\n  \\c      produce no further output\n  \\e      escape\n  \\f      form feed\n  \\n      new line\n  \\r      carriage return\n  \\t      horizontal tab\n  \\v      vertical tab\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    fputs_unlocked(
        gettext(
            b"  \\0NNN   byte with octal value NNN (1 to 3 digits)\n  \\xHH    byte with hexadecimal value HH (1 to 2 digits)\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    printf(
        gettext(
            b"\nYour shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        b"echo\0" as *const u8 as *const libc::c_char,
    );
    fputs_unlocked(
        gettext(
            b"\nConsider using the 'printf' command instead,\nas it avoids problems when outputting option-like strings.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
    emit_ancillary_info("echo");
    exit(status);
}
fn hex_to_bin(c: char) -> u8 {
    match c {
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => c.to_digit(16).unwrap_or(0) as u8,
    }
}
fn main_0(argc: i32, argv: &[String]) -> i32 {
    let display_return = true;
    let posixly_correct = std::env::var("POSIXLY_CORRECT").is_err();
    let allow_options = !posixly_correct || (DEFAULT_ECHO_TO_XPG != 0) && argc > 1 && argv[1] == "-n";

    let mut do_v9: bool = DEFAULT_ECHO_TO_XPG != 0;
    
    // Not translating calls to set_program_name, setlocale, bindtextdomain, textdomain, and atexit
    if allow_options && argc == 2 {
        if argv[1] == "--help" {
            unsafe {
                usage(0);
            }
        } else if argv[1] == "--version" {
            use std::os::unix::ffi::OsStrExt;

            unsafe {
                use std::os::unix::io::AsRawFd;

                // Obtain the raw file descriptor
                let stdout_fd = std::io::stdout().as_raw_fd();

                // Convert the file descriptor to a FILE pointer
                let mut stdout_ptr = stdout_fd as *mut _;

                let version_info = b"GNU coreutils\0" as *const u8;
                let author_name1 = b"Brian Fox\0" as *const u8;
                let author_name2 = b"Chet Ramey\0" as *const u8;

                version_etc(
                    stdout_ptr,
                    program_name,
                    version_info,
                    Version,
                    author_name1,
                    author_name2,
                    None::<&mut std::ffi::OsStr>,
                );
            }

            return 0;
        }
    }

    let mut args_iter = argv.iter().skip(1);
    let mut display_return = true;

    while let Some(arg) = args_iter.next() {
        if arg.starts_with('-') {
            let mut chars_iter = arg.chars().skip(1);
            while let Some(ch) = chars_iter.next() {
                match ch {
                    'e' => do_v9 = true,
                    'E' => do_v9 = false,
                    'n' => display_return = false,
                    _ => {}
                }
            }
        } else if do_v9 || posixly_correct {
            for ch in arg.chars() {
                if ch == '\\' {
                    if let Some(next_char) = args_iter.next() {
                        match next_char.as_str() {
                            "a" => print!("\x07"),
                            "b" => print!("\x08"),
                            "c" => return 0,
                            "e" => print!("\x1B"),
                            "f" => print!("\x0C"),
                            "n" => print!("\n"),
                            "r" => print!("\r"),
                            "t" => print!("\t"),
                            "v" => print!("\x0B"),
                            "x" => {
                                if let Some(hex_digits) = args_iter.next() {
                                    if let Ok(hex_value) = u32::from_str_radix(hex_digits, 16) {
                                        if let Some(unicode_char) = std::char::from_u32(hex_value) {
                                            print!("{}", unicode_char);
                                        }
                                    }
                                }
                            }
                            "0" => {
                                if let Some(octal_digits) = args_iter.next() {
                                    if let Ok(octal_value) = u32::from_str_radix(octal_digits, 8) {
                                        if let Some(unicode_char) = std::char::from_u32(octal_value) {
                                            print!("{}", unicode_char);
                                        }
                                    }
                                }
                            }
                            _ => print!("\\{}", next_char),
                        }
                    }
                } else {
                    print!("{}", ch);
                }
            }
        } else {
            print!("{}", arg);
        }

        if args_iter.len() > 0 {
            print!(" ");
        }
    }

    if display_return {
        println!();
    }

    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    std::process::exit(main_0(args.len() as i32, &args));
}