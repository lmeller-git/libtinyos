use bitflags::bitflags;

mod funcs;
pub use funcs::*;

#[repr(u64)]
pub enum SysCallDispatch {
    Open = 0,
    Close = 1,
    Read = 2,
    Write = 3,
    Yield = 4,
    Exit = 5,
    Kill = 6,
    Mmap = 7,
    Munmap = 8,
    Clone = 9,
    Wait = 10,
    Machine = 11,
    GetPid = 12,
}

pub type FileDescriptor = u32;

pub const STDIN_FILENO: FileDescriptor = 0;
pub const STDOUT_FILENO: FileDescriptor = 1;
pub const STDERR_FILENO: FileDescriptor = 2;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpenOptions: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const APPEND = 1 << 2;
        const TRUNCATE = 1 << 3;
        const CREATE = 1 << 4;
        const CREATE_DIR = 1 << 5;
        const CREATE_ALL = 1 << 6;
        const CREATE_LINK = 1 << 7;
        const NO_FOLLOW_LINK = 1 << 8;
    }
}

impl OpenOptions {
    pub fn with_read(self) -> Self {
        self | Self::READ
    }

    pub fn with_write(self) -> Self {
        self | Self::WRITE
    }

    pub fn with_no_follow_symlink(self) -> Self {
        self | Self::NO_FOLLOW_LINK
    }

    pub fn with_truncate(self) -> Self {
        self | Self::TRUNCATE
    }

    pub fn with_append(self) -> Self {
        self | Self::APPEND
    }
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self::READ
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UnlinkOptions: u32 {
        const FORCE = 1 << 0;
        const RECURSIVE = 1 << 1;
        const NO_PRESERVE_ROOT = 1 << 2;
    }
}

impl UnlinkOptions {
    fn with_force(self) -> Self {
        self | Self::FORCE
    }

    fn with_rmdir(self) -> Self {
        self | Self::RECURSIVE
    }
}

impl Default for UnlinkOptions {
    fn default() -> Self {
        Self::empty()
    }
}

pub type SysResult<T> = Result<T, SysResultCode>;

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysResultCode {
    Fail = -1,
    Success = 0,
}

impl TryFrom<i64> for SysResultCode {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            -1 => Ok(Self::Fail),
            _ => Err(value),
        }
    }
}

pub fn as_res(r1: u64, r2: i64) -> SysResult<u64> {
    // in case of success r2 is guaranteed to be == SysResultCode::Success
    if r2 != SysResultCode::Success as i64 {
        return Err(r2.try_into().unwrap_or(SysResultCode::Fail));
    }
    Ok(r1)
}

// copied from https://github.com/rust-osdev/x86_64/blob/master/src/structures/paging/page_table.rs, as this is used in tinyOS kernel
bitflags! {
    /// Possible flags for a page table entry.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct PageTableFlags: u64 {
        /// Specifies whether the mapped frame or page table is loaded in memory.
        const PRESENT =         1;
        /// Controls whether writes to the mapped frames are allowed.
        ///
        /// If this bit is unset in a level 1 page table entry, the mapped frame is read-only.
        /// If this bit is unset in a higher level page table entry the complete range of mapped
        /// pages is read-only.
        const WRITABLE =        1 << 1;
        /// Controls whether accesses from userspace (i.e. ring 3) are permitted.
        const USER_ACCESSIBLE = 1 << 2;
        /// If this bit is set, a “write-through” policy is used for the cache, else a “write-back”
        /// policy is used.
        const WRITE_THROUGH =   1 << 3;
        /// Disables caching for the pointed entry is cacheable.
        const NO_CACHE =        1 << 4;
        /// Set by the CPU when the mapped frame or page table is accessed.
        const ACCESSED =        1 << 5;
        /// Set by the CPU on a write to the mapped frame.
        const DIRTY =           1 << 6;
        /// Specifies that the entry maps a huge frame instead of a page table. Only allowed in
        /// P2 or P3 tables.
        const HUGE_PAGE =       1 << 7;
        /// Indicates that the mapping is present in all address spaces, so it isn't flushed from
        /// the TLB on an address space switch.
        const GLOBAL =          1 << 8;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_9 =           1 << 9;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_10 =          1 << 10;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_11 =          1 << 11;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_52 =          1 << 52;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_53 =          1 << 53;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_54 =          1 << 54;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_55 =          1 << 55;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_56 =          1 << 56;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_57 =          1 << 57;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_58 =          1 << 58;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_59 =          1 << 59;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_60 =          1 << 60;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_61 =          1 << 61;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_62 =          1 << 62;
        /// Forbid code execution from the mapped frames.
        ///
        /// Can be only used when the no-execute page protection feature is enabled in the EFER
        /// register.
        const NO_EXECUTE =      1 << 63;
    }
}

#[macro_export]
macro_rules! syscall {
    ($rax:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r9:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                in("r9") $r9,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r9:expr, $r8:expr) => {{
        let ret: u64;
        let ret2: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                in("r9") $r9,
                in("r8") $r8,
                lateout("rax") ret,
                lateout("rdx") ret2
            );

        $crate::syscalls::as_res(ret, ret2 as i64)
    }};
}
