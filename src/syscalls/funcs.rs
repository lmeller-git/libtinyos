use crate::syscall;

pub fn exit(status: u64) -> ! {
    unsafe { syscall!(1, status) };
    unreachable!()
}

pub fn kill() {
    todo!()
}

pub fn print() {
    todo!()
}
