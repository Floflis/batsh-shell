#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, main, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn atoll(__nptr: *const libc::c_char) -> libc::c_longlong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type size_t = libc::c_ulong;
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
static mut data: [libc::c_char; 543] =
    [1, 33, -90, 126, -85, -11, 1, 9, 88, -43, 23, 2, 98, 15, -35, -45, -91,
     36, 97, -33, 3, -59, 117, -114, 105, -74, 125, 101, 46, -85, -54, 40,
     -88, 110, -97, 87, 77, 114, -67, -110, 86, -96, -34, -25, 98, 19, 93,
     -124, 38, -107, 47, -59, -67, 125, -117, 26, 24, 18, -96, 2, -76, -92,
     -37, 124, -59, 26, 18, 36, 33, -3, 93, -100, 109, 44, -1, -11, -26, -102,
     -34, -107, -56, -61, 69, 85, -105, -37, -56, -90, 80, -79, -71, -104, 10,
     -113, 38, 118, 75, 88, 103, 3, 23, -94, -76, -27, -125, -102, -114, 88,
     113, -46, 61, 113, -86, 122, 123, -24, -24, -6, 122, 26, 17, 48, -44,
     -97, -104, 69, -88, -90, 106, -91, 100, 8, 79, 45, -82, -97, -34, 104,
     55, -23, -9, 93, 95, 66, -2, 121, 91, 85, 100, 102, 51, -105, 80, -1, 38,
     -105, 122, 45, 55, 66, 106, -16, -1, 36, 40, 107, -83, 62, -6, -78, 6,
     53, -114, 25, 119, -82, 21, -46, 3, 122, 57, 55, 17, -119, 55, 56, 32,
     -79, 101, 88, -12, -48, 73, -13, -12, 113, 95, -94, -80, 89, 84, -74,
     -114, -30, -48, 5, -111, -27, -40, -107, 95, 17, -51, 113, -102, 4, -87,
     -69, -74, 15, 20, -86, -33, 93, -99, -44, -49, -4, 118, 127, 85, -53, 54,
     -28, -82, 6, -22, 64, -20, -62, -42, 76, -45, -93, -67, 110, -88, 103,
     41, 94, 118, 62, 9, 86, -101, -89, 42, 107, -92, -95, -22, -7, 109, 32,
     -34, 28, 39, -56, 92, 20, -117, 50, 96, 94, -42, 29, -51, 127, -123, -9,
     -34, -4, 53, -25, 82, -47, -113, 124, 60, 51, 30, 38, 45, -117, 71, 11,
     -89, 110, -44, 4, -126, 95, 55, -29, -66, 13, 0, -117, -115, -122, -126,
     107, -126, -73, 83, -44, -120, -30, 81, -60, 21, 111, -21, 67, -5, 50,
     78, -93, -95, 34, -89, 36, -126, -34, 7, 64, -20, 8, -53, 121, -114, 78,
     -27, 16, 5, 56, -28, -114, 27, 54, 83, 49, -90, 62, 116, -95, 113, -61,
     69, 18, -27, -19, 54, 103, -52, 61, -88, -72, 70, 115, 50, -44, -63, 23,
     -28, -57, 80, -55, 86, 108, 0, -87, -98, -90, -24, 18, 72, 89, -43, -115,
     108, -69, 123, -94, 35, 71, -32, -53, -1, 38, 63, 50, -32, 102, -23, -80,
     -30, -123, -9, 32, 35, -11, 111, 101, -35, -108, 112, 101, -116, 117,
     -126, 82, -68, -77, 90, -127, 78, -84, 15, -14, 81, -70, 6, -86, -35, 15,
     -2, -35, 124, 98, -77, -48, -68, 30, 38, 23, 54, -126, -4, -111, -48,
     -24, 84, -28, -112, 64, -103, 111, -43, -30, -88, -4, 54, 123, -31, 107,
     -117, 52, -18, 17, 19, 89, 8, 90, -111, 47, 121, 127, 73, -95, -28, 22,
     21, -44, 91, 69, -118, -32, -13, -79, 20, 119, -101, 26, -21, 15, -32,
     26, 20, 83, -63, -38, 99, -84, 116, 42, 17, 67, 106, -78, -3, 1, -22,
     123, -34, -72, -3, -24, 51, -56, -64, -19, 48, 102, 41, 94, -9, 119, 69,
     -89, -45, -110, 10, -123, -19, -119, 85, -15, -47, 45, 83, 96, 0];
/* Define as 1 to call setuid(0) at start of script */
/* Define as 1 to debug execvp calls */
/* Define as 1 to enable ptrace the executable */
/* Define as 1 to disable ptrace/dump the executable */
/* Define as 1 to enable work with busybox */
/* HARDENING */
/* rtc.c */
/* 'Alleged RC4' */
static mut stte: [libc::c_uchar; 256] = [0; 256];
static mut indx: libc::c_uchar = 0;
static mut jndx: libc::c_uchar = 0;
static mut kndx: libc::c_uchar = 0;
/*
 * Reset arc4 stte. 
 */
#[no_mangle]
pub unsafe extern "C" fn stte_0() {
    kndx = 0 as libc::c_int as libc::c_uchar;
    jndx = kndx;
    indx = jndx;
    loop  {
        stte[indx as usize] = indx;
        indx = indx.wrapping_add(1);
        if !(indx != 0) { break ; }
    };
}
/*
 * Set key. Can be used more than once. 
 */
#[no_mangle]
pub unsafe extern "C" fn key(mut str: *mut libc::c_void,
                             mut len: libc::c_int) {
    let mut tmp: libc::c_uchar = 0;
    let mut ptr: *mut libc::c_uchar = str as *mut libc::c_uchar;
    while len > 0 as libc::c_int {
        loop  {
            tmp = stte[indx as usize];
            kndx =
                (kndx as libc::c_int + tmp as libc::c_int) as libc::c_uchar;
            kndx =
                (kndx as libc::c_int +
                     *ptr.offset((indx as libc::c_int % len) as isize) as
                         libc::c_int) as libc::c_uchar;
            stte[indx as usize] = stte[kndx as usize];
            stte[kndx as usize] = tmp;
            indx = indx.wrapping_add(1);
            if !(indx != 0) { break ; }
        }
        ptr = ptr.offset(256 as libc::c_int as isize);
        len -= 256 as libc::c_int
    };
}
/*
 * Crypt data. 
 */
#[no_mangle]
pub unsafe extern "C" fn arc4(mut str: *mut libc::c_void,
                              mut len: libc::c_int) {
    let mut tmp: libc::c_uchar = 0;
    let mut ptr: *mut libc::c_uchar = str as *mut libc::c_uchar;
    while len > 0 as libc::c_int {
        indx = indx.wrapping_add(1);
        tmp = stte[indx as usize];
        jndx = (jndx as libc::c_int + tmp as libc::c_int) as libc::c_uchar;
        stte[indx as usize] = stte[jndx as usize];
        stte[jndx as usize] = tmp;
        tmp =
            (tmp as libc::c_int + stte[indx as usize] as libc::c_int) as
                libc::c_uchar;
        *ptr =
            (*ptr as libc::c_int ^ stte[tmp as usize] as libc::c_int) as
                libc::c_uchar;
        ptr = ptr.offset(1);
        len -= 1
    };
}
/* End of ARC4 */
/* HARDENING */
/*
 * Key with file invariants. 
 */
#[no_mangle]
pub unsafe extern "C" fn key_with_file(mut file: *mut libc::c_char)
 -> libc::c_int {
    let mut statf: [stat; 1] =
        [stat{st_dev: 0,
              st_ino: 0,
              st_nlink: 0,
              st_mode: 0,
              st_uid: 0,
              st_gid: 0,
              __pad0: 0,
              st_rdev: 0,
              st_size: 0,
              st_blksize: 0,
              st_blocks: 0,
              st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
              st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
              st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
              __glibc_reserved: [0; 3],}; 1];
    let mut control: [stat; 1] =
        [stat{st_dev: 0,
              st_ino: 0,
              st_nlink: 0,
              st_mode: 0,
              st_uid: 0,
              st_gid: 0,
              __pad0: 0,
              st_rdev: 0,
              st_size: 0,
              st_blksize: 0,
              st_blocks: 0,
              st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
              st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
              st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
              __glibc_reserved: [0; 3],}; 1];
    if stat(file, statf.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    /* Turn on stable fields */
    memset(control.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[stat; 1]>() as libc::c_ulong);
    (*control.as_mut_ptr()).st_ino = (*statf.as_mut_ptr()).st_ino;
    (*control.as_mut_ptr()).st_dev = (*statf.as_mut_ptr()).st_dev;
    (*control.as_mut_ptr()).st_rdev = (*statf.as_mut_ptr()).st_rdev;
    (*control.as_mut_ptr()).st_uid = (*statf.as_mut_ptr()).st_uid;
    (*control.as_mut_ptr()).st_gid = (*statf.as_mut_ptr()).st_gid;
    (*control.as_mut_ptr()).st_size = (*statf.as_mut_ptr()).st_size;
    (*control.as_mut_ptr()).st_mtim.tv_sec =
        (*statf.as_mut_ptr()).st_mtim.tv_sec;
    (*control.as_mut_ptr()).st_ctim.tv_sec =
        (*statf.as_mut_ptr()).st_ctim.tv_sec;
    key(control.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[stat; 1]>() as libc::c_ulong as libc::c_int);
    return 0 as libc::c_int;
}
/* DEBUGEXEC */
#[no_mangle]
pub unsafe extern "C" fn rmarg(mut argv: *mut *mut libc::c_char,
                               mut arg: *mut libc::c_char) {
    while !argv.is_null() && !(*argv).is_null() && *argv != arg {
        argv = argv.offset(1)
    }
    while !argv.is_null() && !(*argv).is_null() {
        *argv = *argv.offset(1 as libc::c_int as isize);
        argv = argv.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn chkenv(mut argc: libc::c_int) -> libc::c_int {
    let mut buff: [libc::c_char; 512] = [0; 512];
    let mut mask: libc::c_ulong = 0;
    let mut m: libc::c_ulong = 0;
    let mut l: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    extern "C" {
        #[no_mangle]
        static mut environ: *mut *mut libc::c_char;
    }
    mask = getpid() as libc::c_ulong;
    stte_0();
    key(::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> libc::c_int>,
                                *mut libc::c_void>(Some(chkenv as
                                                            unsafe extern "C" fn(_:
                                                                                     libc::c_int)
                                                                ->
                                                                    libc::c_int)),
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                *mut libc::c_void>(Some(chkenv_end as
                                                            unsafe extern "C" fn()
                                                                ->
                                                                    ())).wrapping_offset_from(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                      libc::c_int)
                                                                                                                                 ->
                                                                                                                                     libc::c_int>,
                                                                                                                      *mut libc::c_void>(Some(chkenv
                                                                                                                                                  as
                                                                                                                                                  unsafe extern "C" fn(_:
                                                                                                                                                                           libc::c_int)
                                                                                                                                                      ->
                                                                                                                                                          libc::c_int)))
            as libc::c_long as libc::c_int);
    key(&mut data as *mut [libc::c_char; 543] as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 543]>() as libc::c_ulong as
            libc::c_int);
    key(&mut mask as *mut libc::c_ulong as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as
            libc::c_int);
    arc4(&mut mask as *mut libc::c_ulong as *mut libc::c_void,
         ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as
             libc::c_int);
    sprintf(buff.as_mut_ptr(),
            b"x%lx\x00" as *const u8 as *const libc::c_char, mask);
    string = getenv(buff.as_mut_ptr());
    l = strlen(buff.as_mut_ptr()) as libc::c_int;
    if string.is_null() {
        /* 1st */
        sprintf(&mut *buff.as_mut_ptr().offset(l as isize) as
                    *mut libc::c_char,
                b"=%lu %d\x00" as *const u8 as *const libc::c_char, mask,
                argc);
        putenv(strdup(buff.as_mut_ptr()));
        return 0 as libc::c_int
    }
    c =
        sscanf(string, b"%lu %d%c\x00" as *const u8 as *const libc::c_char,
               &mut m as *mut libc::c_ulong, &mut a as *mut libc::c_int,
               buff.as_mut_ptr());
    if c == 2 as libc::c_int && m == mask {
        /* 3rd */
        rmarg(environ, &mut *string.offset((-l - 1 as libc::c_int) as isize));
        return 1 as libc::c_int + (argc - a)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn chkenv_end() { }
/* HARDENING */
/* !TRACEABLE */
#[no_mangle]
pub unsafe extern "C" fn xsh(mut argc: libc::c_int,
                             mut argv: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    let mut scrpt: *mut libc::c_char = 0 as *mut libc::c_char; /* Reexecute */
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut varg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut me: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    if me.is_null() {
        me = getenv(b"_\x00" as *const u8 as *const libc::c_char)
    }
    if me.is_null() {
        fprintf(stderr,
                b"E: neither argv[0] nor $_ works.\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    ret = chkenv(argc);
    stte_0();
    key(&mut *data.as_mut_ptr().offset(146 as libc::c_int as isize) as
            *mut libc::c_char as *mut libc::c_void, 256 as libc::c_int);
    arc4(&mut *data.as_mut_ptr().offset(21 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 65 as libc::c_int);
    arc4(&mut *data.as_mut_ptr().offset(128 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 1 as libc::c_int);
    if *(&mut *data.as_mut_ptr().offset(128 as libc::c_int as isize) as
             *mut libc::c_char).offset(0 as libc::c_int as isize) as
           libc::c_int != 0 &&
           atoll(&mut *data.as_mut_ptr().offset(128 as libc::c_int as isize))
               < time(0 as *mut time_t) as libc::c_longlong {
        return &mut *data.as_mut_ptr().offset(21 as libc::c_int as isize) as
                   *mut libc::c_char
    }
    arc4(&mut *data.as_mut_ptr().offset(120 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 8 as libc::c_int);
    arc4(&mut *data.as_mut_ptr().offset(428 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 3 as libc::c_int);
    arc4(&mut *data.as_mut_ptr().offset(487 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 15 as libc::c_int);
    arc4(&mut *data.as_mut_ptr().offset(505 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 1 as libc::c_int);
    arc4(&mut *data.as_mut_ptr().offset(461 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 22 as libc::c_int);
    key(&mut *data.as_mut_ptr().offset(461 as libc::c_int as isize) as
            *mut libc::c_char as *mut libc::c_void, 22 as libc::c_int);
    arc4(&mut *data.as_mut_ptr().offset(433 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 22 as libc::c_int);
    if 22 as libc::c_int != 22 as libc::c_int ||
           memcmp(&mut *data.as_mut_ptr().offset(461 as libc::c_int as isize)
                      as *mut libc::c_char as *const libc::c_void,
                  &mut *data.as_mut_ptr().offset(433 as libc::c_int as isize)
                      as *mut libc::c_char as *const libc::c_void,
                  22 as libc::c_int as libc::c_ulong) != 0 {
        return &mut *data.as_mut_ptr().offset(461 as libc::c_int as isize) as
                   *mut libc::c_char
    }
    arc4(&mut *data.as_mut_ptr().offset(97 as libc::c_int as isize) as
             *mut libc::c_char as *mut libc::c_void, 19 as libc::c_int);
    if ret < 0 as libc::c_int {
        return &mut *data.as_mut_ptr().offset(97 as libc::c_int as isize) as
                   *mut libc::c_char
    }
    varg =
        calloc((argc + 10 as libc::c_int) as libc::c_ulong,
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if varg.is_null() { return 0 as *mut libc::c_char }
    if ret != 0 {
        arc4(&mut *data.as_mut_ptr().offset(402 as libc::c_int as isize) as
                 *mut libc::c_char as *mut libc::c_void, 1 as libc::c_int);
        if *(&mut *data.as_mut_ptr().offset(402 as libc::c_int as isize) as
                 *mut libc::c_char).offset(0 as libc::c_int as isize) == 0 &&
               key_with_file(&mut *data.as_mut_ptr().offset(120 as libc::c_int
                                                                as isize)) !=
                   0 {
            return &mut *data.as_mut_ptr().offset(120 as libc::c_int as isize)
                       as *mut libc::c_char
        }
        arc4(&mut *data.as_mut_ptr().offset(20 as libc::c_int as isize) as
                 *mut libc::c_char as *mut libc::c_void, 1 as libc::c_int);
        arc4(&mut *data.as_mut_ptr().offset(511 as libc::c_int as isize) as
                 *mut libc::c_char as *mut libc::c_void, 31 as libc::c_int);
        arc4(&mut *data.as_mut_ptr().offset(407 as libc::c_int as isize) as
                 *mut libc::c_char as *mut libc::c_void, 19 as libc::c_int);
        key(&mut *data.as_mut_ptr().offset(407 as libc::c_int as isize) as
                *mut libc::c_char as *mut libc::c_void, 19 as libc::c_int);
        arc4(&mut *data.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 *mut libc::c_char as *mut libc::c_void, 19 as libc::c_int);
        if 19 as libc::c_int != 19 as libc::c_int ||
               memcmp(&mut *data.as_mut_ptr().offset(407 as libc::c_int as
                                                         isize) as
                          *mut libc::c_char as *const libc::c_void,
                      &mut *data.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
                          *mut libc::c_char as *const libc::c_void,
                      19 as libc::c_int as libc::c_ulong) != 0 {
            return &mut *data.as_mut_ptr().offset(407 as libc::c_int as isize)
                       as *mut libc::c_char
        }
        /* Prepend hide_z spaces to script text to hide it. */
        scrpt =
            malloc((4096 as libc::c_int + 31 as libc::c_int) as libc::c_ulong)
                as *mut libc::c_char; /* My own name at execution */
        if scrpt.is_null() {
            return 0 as *mut libc::c_char
        } /* Options on 1st line of code */
        memset(scrpt as *mut libc::c_void, ' ' as i32,
               4096 as libc::c_int as
                   libc::c_ulong); /* Option introducing inline code */
        memcpy(&mut *scrpt.offset(4096 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut *data.as_mut_ptr().offset(511 as libc::c_int as isize) as
                   *mut libc::c_char as *const libc::c_void,
               31 as libc::c_int as libc::c_ulong); /* The script itself */
    } else if data[487 as libc::c_int as usize] != 0 {
        scrpt =
            malloc(512 as libc::c_int as libc::c_ulong) as
                *mut libc::c_char; /* Option meaning last option */
        if scrpt.is_null() {
            return 0 as *mut libc::c_char
        } /* Args numbering correction */
        sprintf(scrpt,
                &mut *data.as_mut_ptr().offset(487 as libc::c_int as isize) as
                    *mut libc::c_char, me); /* Main run-time arguments */
    } else { scrpt = me } /* NULL terminated array */
    j = 0 as libc::c_int;
    let fresh0 = j;
    j = j + 1;
    let ref mut fresh1 = *varg.offset(fresh0 as isize);
    *fresh1 = *argv.offset(0 as libc::c_int as isize);
    if ret != 0 && data[20 as libc::c_int as usize] as libc::c_int != 0 {
        let fresh2 = j;
        j = j + 1;
        let ref mut fresh3 = *varg.offset(fresh2 as isize);
        *fresh3 =
            &mut *data.as_mut_ptr().offset(20 as libc::c_int as isize) as
                *mut libc::c_char
    }
    if data[428 as libc::c_int as usize] != 0 {
        let fresh4 = j;
        j = j + 1;
        let ref mut fresh5 = *varg.offset(fresh4 as isize);
        *fresh5 =
            &mut *data.as_mut_ptr().offset(428 as libc::c_int as isize) as
                *mut libc::c_char
    }
    let fresh6 = j;
    j = j + 1;
    let ref mut fresh7 = *varg.offset(fresh6 as isize);
    *fresh7 = scrpt;
    if data[505 as libc::c_int as usize] != 0 {
        let fresh8 = j;
        j = j + 1;
        let ref mut fresh9 = *varg.offset(fresh8 as isize);
        *fresh9 =
            &mut *data.as_mut_ptr().offset(505 as libc::c_int as isize) as
                *mut libc::c_char
    }
    i = if ret > 1 as libc::c_int { ret } else { 0 as libc::c_int };
    while i < argc {
        let fresh10 = i;
        i = i + 1;
        let fresh11 = j;
        j = j + 1;
        let ref mut fresh12 = *varg.offset(fresh11 as isize);
        *fresh12 = *argv.offset(fresh10 as isize)
    }
    let ref mut fresh13 = *varg.offset(j as isize);
    *fresh13 = 0 as *mut libc::c_char;
    execvp(&mut *data.as_mut_ptr().offset(120 as libc::c_int as isize),
           varg as *const *mut libc::c_char);
    return &mut *data.as_mut_ptr().offset(120 as libc::c_int as isize) as
               *mut libc::c_char;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let ref mut fresh14 = *argv.offset(1 as libc::c_int as isize);
    *fresh14 = xsh(argc, argv);
    fprintf(stderr, b"%s%s%s: %s\n\x00" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            if *__errno_location() != 0 {
                b": \x00" as *const u8 as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if *__errno_location() != 0 {
                strerror(*__errno_location())
            } else { b"\x00" as *const u8 as *const libc::c_char },
            if !(*argv.offset(1 as libc::c_int as isize)).is_null() {
                *argv.offset(1 as libc::c_int as isize)
            } else { b"<null>\x00" as *const u8 as *const libc::c_char });
    return 1 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
