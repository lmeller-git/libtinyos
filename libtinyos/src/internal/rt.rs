use conquer_once::spin::OnceCell;

use crate::{
    internal::os::{EnvVars, ProcessArgs},
    process::ProcessError,
    syscalls,
};

static RUNTIME: OnceCell<RuntimeData> = OnceCell::uninit();

pub(crate) fn runtime<'a>() -> &'a RuntimeData {
    RUNTIME.get().expect(
        "Runtime data may only be used if the runtime is used. Consider linking against libtinyos::_start.",
    )
}

pub(crate) struct RuntimeData {
    args: Option<ProcessArgs>,
    env: Option<EnvVars>,
}

impl RuntimeData {
    pub fn env(&self) -> Option<&EnvVars> {
        self.env.as_ref()
    }

    pub fn args(&self) -> Option<&ProcessArgs> {
        self.args.as_ref()
    }
}

impl RuntimeData {
    fn new(argc: usize, argv: *const u8, envc: usize, envp: *const u8) -> Self {
        Self {
            args: ProcessArgs::new(argv, argc).ok(),
            env: EnvVars::new(envp, envc).ok(),
        }
    }
}

// SAFETY: RuntimeData is just a collection of (ptrs to) data which should never be mutated (for now) (env for example maybe should be mutable)
unsafe impl Sync for RuntimeData {}
unsafe impl Send for RuntimeData {}

#[unsafe(no_mangle)]
pub extern "C" fn _start(argc: usize, argv: *const u8, envc: usize, envp: *const u8) -> ! {
    RUNTIME.init_once(|| RuntimeData::new(argc, argv, envc, envp));

    unsafe { main() }.unwrap();

    unsafe { syscalls::exit(0) }
}

unsafe extern "Rust" {
    fn main() -> Result<(), ProcessError>;
}
