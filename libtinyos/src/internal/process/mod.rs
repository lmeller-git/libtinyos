use tinyos_abi::types::SysErrCode;

// TODO
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProcessError {
    Sys(SysErrCode),
}
