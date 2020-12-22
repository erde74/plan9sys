pub use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong};

macro_rules! sys_call {
    ($func_name:ident , $num: ident) => {
        pub fn $func_name() -> isize {
            unsafe {
                let eax: isize;
                asm!("mov rcx, r10");
                asm!("mov rax, {}", const($num));
                asm!("syscall",out("eax") eax);
                eax
            }
        }
    };

    ($func_name:ident , $num: ident , $t1: ty) => {
        pub fn $func_name(_arg1: $t1) -> isize{
            unsafe {
                let eax: isize;
                asm!("mov rcx, r10");
                asm!("mov rax, {}", const($num));
                asm!("syscall",out("eax") eax);
                eax
            }
        }
    };

    ($func_name:ident , $num: ident , $t1: ty , $t2: ty) => {
        pub fn $func_name(_arg1: $t1, _arg2: $t2) -> isize{
            unsafe {
                let eax: isize;
                asm!("mov rcx, r10");
                asm!("mov rax, {}", const($num));
                asm!("syscall",out("eax") eax);
                eax
            }
        }
    };

    ($func_name:ident , $num: ident , $t1: ty , $t2: ty , $t3: ty) => {
        pub fn $func_name(_arg1: $t1, _arg2: $t2, _arg3: $t3) -> isize{
            unsafe {
                let eax: isize;
                asm!("mov rcx, r10");
                asm!("mov rax, {}", const($num));
                asm!("syscall",out("eax") eax);
                eax
            }
        }
    };

    ($func_name:ident , $num: ident , $t1: ty , $t2: ty , $t3: ty, $ty4: ty) => {
        pub fn $func_name(_arg1: $t1, _arg2: $t2, _arg3: $t3, _arg4: $ty4) -> isize{
            unsafe {
                let eax: isize;
                asm!("mov rcx, r10");
                asm!("mov rax, {}", const($num));
                asm!("syscall",out("eax") eax);
                eax
            }
        }
    };

    ($func_name:ident , $num: ident , $t1: ty , $t2: ty , $t3: ty, $ty4: ty, $ty5: ty) => {
        pub fn $func_name(_arg1: $t1, _arg2: $t2, _arg3: $t3, _arg4: $ty4, _argv5: $ty5) -> isize{
            unsafe {
                let eax: isize;
                asm!("mov rcx, r10");
                asm!("mov rax, {}", const($num));
                asm!("syscall",out("eax") eax);
                eax
            }
        }
    };
}

const SYS_SYSR0: c_int = 0;
const SYS_BIND: c_int = 2;
const SYS_CHDIR: c_int = 3;
const SYS_CLOSE: c_int = 4;
const SYS_DUP: c_int = 5;
const SYS_ALARM: c_int = 6;
const SYS_EXEC: c_int = 7;
const SYS_EXITS: c_int = 8;
const SYS_FAUTH: c_int = 10;
const SYS_SEGBRK: c_int = 12;
const SYS_OPEN: c_int = 14;
const SYS_OSEEK: c_int = 16;
const SYS_SLEEP: c_int = 17;
const SYS_RFORK: c_int = 19;
const SYS_PIPE: c_int = 21;
const SYS_CREATE: c_int = 22;
const SYS_FD2PATH: c_int = 23;
const SYS_BRK_: c_int = 24;
const SYS_REMOVE: c_int = 25;
const SYS_NOTIFY: c_int = 28;
const SYS_NOTED: c_int = 29;
const SYS_SEGATTACH: c_int = 30;
const SYS_SEGDETACH: c_int = 31;
const SYS_SEGFREE: c_int = 32;
const SYS_SEGFLUSH: c_int = 33;
const SYS_RENDEZVOUS: c_int = 34;
const SYS_UNMOUNT: c_int = 35;
const SYS_SEMACQUIRE: c_int = 37;
const SYS_SEMRELEASE: c_int = 38;
const SYS_SEEK: c_int = 39;
const SYS_FVERSION: c_int = 40;
const SYS_ERRSTR: c_int = 41;
const SYS_STAT: c_int = 42;
const SYS_FSTAT: c_int = 43;
const SYS_WSTAT: c_int = 44;
const SYS_FWSTAT: c_int = 45;
const SYS_MOUNT: c_int = 46;
const SYS_AWAIT: c_int = 47;
const SYS_PREAD: c_int = 50;
const SYS_PWRITE: c_int = 51;
const SYS_TSEMACQUIRE: c_int = 52;
const SYS_NSEC: c_int = 53;

sys_call!(r0, SYS_SYSR0);
sys_call!(bind, SYS_BIND, *const c_char, *const c_char, c_int);
sys_call!(chdir, SYS_CHDIR, *const c_char);
sys_call!(close, SYS_CLOSE, c_int);
sys_call!(dup, SYS_DUP, c_int, c_int);
sys_call!(alarm, SYS_ALARM, u64);
sys_call!(exec, SYS_EXEC, *const c_char, *const *const c_char);
sys_call!(exits, SYS_EXITS, *const c_char);
sys_call!(fauth, SYS_FAUTH, c_int, *const c_char);
sys_call!(segbrk, SYS_SEGBRK, c_int, c_int);
sys_call!(open, SYS_OPEN, *const c_char, c_int);
sys_call!(oseek, SYS_OSEEK, c_int, i64, c_int);
sys_call!(sleep, SYS_SLEEP, i64);
sys_call!(rfork, SYS_RFORK, c_int);
sys_call!(pipe, SYS_PIPE, *const c_int);
sys_call!(create, SYS_CREATE, *const c_char, c_int, u32);
sys_call!(fd2path, SYS_FD2PATH, c_int, *const c_char, c_int);
sys_call!(brk, SYS_BRK_, *const c_void);
sys_call!(remove, SYS_REMOVE, *const c_char);
sys_call!(
    notify,
    SYS_NOTIFY,
    Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *mut c_char)>
);
sys_call!(noted, SYS_NOTED, c_int);
sys_call!(
    segattach,
    SYS_SEGATTACH,
    c_int,
    *const c_char,
    *const c_void,
    c_ulong
);
sys_call!(segdetach, SYS_SEGDETACH, *const c_void);
sys_call!(segfree, SYS_SEGFREE, *const c_void, c_ulong);
sys_call!(segflush, SYS_SEGFLUSH, *const c_void, c_ulong);
sys_call!(rendezvous, SYS_RENDEZVOUS, *const c_void, *const c_void);
sys_call!(unmount, SYS_UNMOUNT, *const c_char, *const c_char);
sys_call!(semacquire, SYS_SEMACQUIRE, *const c_long, c_int);
sys_call!(semrelease, SYS_SEMRELEASE, *const c_long, c_int);
sys_call!(seek, SYS_SEEK, c_int, c_longlong, c_int);
sys_call!(fversion, SYS_FVERSION, c_int, c_int, *const char, c_int);
sys_call!(errstr, SYS_ERRSTR, *const c_char, c_uint);
sys_call!(stat, SYS_STAT, *const c_char, *const c_uchar, c_int);
sys_call!(fstat, SYS_FSTAT, c_int, *const c_uchar, c_int);
sys_call!(wstat, SYS_WSTAT, *const c_char, *const c_uchar, c_int);
sys_call!(fwstat, SYS_FWSTAT, c_int, *const c_uchar, c_int);
sys_call!(
    mount,
    SYS_MOUNT,
    c_int,
    c_int,
    *const c_char,
    c_int,
    *const c_char
);
sys_call!(sys_await, SYS_AWAIT, *const c_char, c_int);
sys_call!(pread, SYS_PREAD, c_int, *const c_void, c_long, c_longlong);
sys_call!(pwrite, SYS_PWRITE, c_int, *const c_void, c_long, c_longlong);
sys_call!(tsemacquire, SYS_TSEMACQUIRE, *const c_long, c_longlong);
sys_call!(nsec, SYS_NSEC);
