use plan9sys::syscall::bind;

fn main() {
    bind("\0".as_ptr() as *const i8, "\0".as_ptr() as *const i8, 1);
}
