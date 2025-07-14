pub mod funcs;

#[macro_export]
macro_rules! syscall {
    ($rax:expr) => {{
        let ret: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                lateout("rax") ret,
            );

        ret
    }};

    ($rax:expr, $rdi:expr) => {{
        let ret: u64;
            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                lateout("rax") ret,
            );

        ret
    }};

    ($rax:expr, $rdi:expr, $rsi:expr) => {{
        let ret: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                lateout("rax") ret,
            );

        ret
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr) => {{
        let ret: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                lateout("rax") ret,
            );

        ret
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr) => {{
        let ret: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                lateout("rax") ret,
            );

        ret
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r9:expr) => {{
        let ret: u64;

            core::arch::asm!(
                "int 0x80",
                in("rax") $rax,
                in("rdi") $rdi,
                in("rsi") $rsi,
                in("rdx") $rdx,
                in("r10") $r10,
                in("r9") $r9,
                lateout("rax") ret,
            );

        ret
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r9:expr, $r8:expr) => {{
        let ret: u64;

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
            );

        ret
    }};
}
