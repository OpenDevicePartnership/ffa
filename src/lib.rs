//! ARM Firmware Framework for ARMv8-A Profile

#![doc(html_root_url = "https://docs.rs/ffa/latest")]
#![cfg_attr(not(test), no_std)]

use console::FfaConsole;
use msg_wait::FfaMsgWait;
use version::FfaVersion;

#[macro_use]
pub mod console;
pub mod msg_wait;
pub mod version;

pub type Result<T> = core::result::Result<T, FfaError>;

#[derive(Default, Copy, Clone)]
pub struct FfaDirectMsg {
    _function_id: u32,
    _source_id: u16,
    _destination_id: u16,
    _args64: [u64; 16],
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum FfaError {
    Ok,
    NotSupported,
    InvalidParameters,
    NoMemory,
    Busy,
    Interrupted,
    Denied,
    Retry,
    Aborted,
    UnknownError,
}

impl From<FfaError> for i64 {
    fn from(value: FfaError) -> i64 {
        match value {
            FfaError::Ok => 0,
            FfaError::NotSupported => -1,
            FfaError::InvalidParameters => -2,
            FfaError::NoMemory => -3,
            FfaError::Busy => -4,
            FfaError::Interrupted => -5,
            FfaError::Denied => -6,
            FfaError::Retry => -7,
            FfaError::Aborted => -8,
            FfaError::UnknownError => i64::MIN,
        }
    }
}

impl From<i64> for FfaError {
    fn from(value: i64) -> FfaError {
        match value {
            0 => FfaError::Ok,
            -1 => FfaError::NotSupported,
            -2 => FfaError::InvalidParameters,
            -3 => FfaError::NoMemory,
            -4 => FfaError::Busy,
            -5 => FfaError::Interrupted,
            -6 => FfaError::Denied,
            -7 => FfaError::Retry,
            -8 => FfaError::Aborted,
            _ => FfaError::UnknownError,
        }
    }
}

impl FfaError {
    pub fn into_result(self) -> Result<()> {
        match self {
            FfaError::Ok => Ok(()),
            err => Err(err),
        }
    }
}

pub enum FfaFunctionId {
    FfaError,
    FfaSuccess64,
    FfaSuccess32,
    FfaInterrupt,
    FfaVersion,
    FfaFeatures,
    FfaRxRelease,
    FfaRxTxMap,
    FfaRxtxUnmap,
    FfaPartitionInfoGet,
    FfaIdGet,
    FfaMsgWait,
    FfaMsgYield,
    FfaMsgRun,
    FfaMsgSend,
    FfaMsgSendDirectReq,
    FfaMsgSendDirectResp,
    FfaMsgPoll,
    FfaMemDonate,
    FfaMemLend,
    FfaMemShare,
    FfaMemRetrieveReq,
    FfaMemRetrieveResp,
    FfaMemRelinquish,
    FfaMemReclaim,
    FfaMemFragRx,
    FfaMemFragTx,
    FfaMemPermGet,
    FfaMemPermSet,
    FfaConsoleLog,
    FfaMsgSendDirectReq2,
}

impl From<FfaFunctionId> for u64 {
    fn from(value: FfaFunctionId) -> u64 {
        match value {
            FfaFunctionId::FfaError => 0x84000060,
            FfaFunctionId::FfaSuccess32 => 0x84000061,
            FfaFunctionId::FfaSuccess64 => 0xc4000061,
            FfaFunctionId::FfaInterrupt => 0x84000062,
            FfaFunctionId::FfaVersion => 0x84000063,
            FfaFunctionId::FfaFeatures => 0x84000064,
            FfaFunctionId::FfaRxRelease => 0x84000065,
            FfaFunctionId::FfaRxTxMap => 0xc4000066,
            FfaFunctionId::FfaRxtxUnmap => 0x84000067,
            FfaFunctionId::FfaPartitionInfoGet => 0x84000068,
            FfaFunctionId::FfaIdGet => 0x84000069,
            FfaFunctionId::FfaMsgWait => 0x8400006b,
            FfaFunctionId::FfaMsgYield => 0x8400006c,
            FfaFunctionId::FfaMsgRun => 0x8400006d,
            FfaFunctionId::FfaMsgSend => 0x8400006e,
            FfaFunctionId::FfaMsgSendDirectReq => 0xc400006f,
            FfaFunctionId::FfaMsgSendDirectResp => 0xc4000070,
            FfaFunctionId::FfaMsgPoll => 0x8400006a,
            FfaFunctionId::FfaMemDonate => 0xc4000071,
            FfaFunctionId::FfaMemLend => 0xc4000072,
            FfaFunctionId::FfaMemShare => 0xc4000073,
            FfaFunctionId::FfaMemRetrieveReq => 0xc4000074,
            FfaFunctionId::FfaMemRetrieveResp => 0x84000075,
            FfaFunctionId::FfaMemRelinquish => 0x84000076,
            FfaFunctionId::FfaMemReclaim => 0x84000077,
            FfaFunctionId::FfaMemFragRx => 0x8400007a,
            FfaFunctionId::FfaMemFragTx => 0x8400007b,
            FfaFunctionId::FfaMemPermGet => 0x84000088,
            FfaFunctionId::FfaMemPermSet => 0x84000089,
            FfaFunctionId::FfaConsoleLog => 0xc400008a,
            FfaFunctionId::FfaMsgSendDirectReq2 => 0xc400008d,
        }
    }
}

impl From<u64> for FfaFunctionId {
    fn from(value: u64) -> FfaFunctionId {
        match value {
            0x84000060 => FfaFunctionId::FfaError,
            0x84000061 => FfaFunctionId::FfaSuccess32,
            0xc4000061 => FfaFunctionId::FfaSuccess64,
            0x84000062 => FfaFunctionId::FfaInterrupt,
            0x84000063 => FfaFunctionId::FfaVersion,
            0x84000064 => FfaFunctionId::FfaFeatures,
            0x84000065 => FfaFunctionId::FfaRxRelease,
            0xc4000066 => FfaFunctionId::FfaRxTxMap,
            0x84000067 => FfaFunctionId::FfaRxtxUnmap,
            0x84000068 => FfaFunctionId::FfaPartitionInfoGet,
            0x84000069 => FfaFunctionId::FfaIdGet,
            0x8400006b => FfaFunctionId::FfaMsgWait,
            0x8400006c => FfaFunctionId::FfaMsgYield,
            0x8400006d => FfaFunctionId::FfaMsgRun,
            0x8400006e => FfaFunctionId::FfaMsgSend,
            0xc400006f => FfaFunctionId::FfaMsgSendDirectReq,
            0xc4000070 => FfaFunctionId::FfaMsgSendDirectResp,
            0x8400006a => FfaFunctionId::FfaMsgPoll,
            0xc4000071 => FfaFunctionId::FfaMemDonate,
            0xc4000072 => FfaFunctionId::FfaMemLend,
            0xc4000073 => FfaFunctionId::FfaMemShare,
            0xc4000074 => FfaFunctionId::FfaMemRetrieveReq,
            0x84000075 => FfaFunctionId::FfaMemRetrieveResp,
            0x84000076 => FfaFunctionId::FfaMemRelinquish,
            0x84000077 => FfaFunctionId::FfaMemReclaim,
            0x8400007a => FfaFunctionId::FfaMemFragRx,
            0x8400007b => FfaFunctionId::FfaMemFragTx,
            0x84000088 => FfaFunctionId::FfaMemPermGet,
            0x84000089 => FfaFunctionId::FfaMemPermSet,
            0xc400008a => FfaFunctionId::FfaConsoleLog,
            0xc400008d => FfaFunctionId::FfaMsgSendDirectReq2,
            _ => panic!("Unknown FfaFunctionId value"),
        }
    }
}

impl From<FfaParams> for FfaDirectMsg {
    fn from(params: FfaParams) -> FfaDirectMsg {
        FfaDirectMsg {
            _function_id: params.x0 as u32, // Function id is in lower 32 bits of x0
            _source_id: (params.x1 >> 16) as u16, // Source in upper 16 bits
            _destination_id: params.x1 as u16, // Destination in lower 16 bits
            _args64: [
                params.x2, params.x3, params.x4, params.x5, params.x6, params.x7, params.x8,
                params.x9, params.x10, params.x11, params.x12, params.x13, params.x14, params.x15,
                params.x16, params.x17,
            ],
        }
    }
}

#[derive(Default)]
pub struct Ffa;

impl Ffa {
    pub fn new() -> Self {
        Ffa {}
    }

    pub fn version(&self) -> Result<FfaVersion> {
        FfaVersion::default().exec()
    }

    pub fn console_log(&self, s: &str) -> Result<()> {
        FfaConsole::new().exec(s.as_bytes())
    }

    pub fn msg_wait(&self) -> Result<FfaMsgWait> {
        FfaMsgWait::new().exec()
    }
}

#[derive(Default)]
pub struct FfaParams {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
}

/// Secure Monitor Call
pub(crate) fn ffa_smc(params: FfaParams) -> FfaParams {
    let mut result = FfaParams::default();

    ffa_smc_inner(&params, &mut result);

    result
}

#[inline(always)]
fn ffa_smc_inner(_params: &FfaParams, _result: &mut FfaParams) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "smc #0",
            inout("x0") _params.x0 => _result.x0,
            inout("x1") _params.x1 => _result.x1,
            inout("x2") _params.x2 => _result.x2,
            inout("x3") _params.x3 => _result.x3,
            inout("x4") _params.x4 => _result.x4,
            inout("x5") _params.x5 => _result.x5,
            inout("x6") _params.x6 => _result.x6,
            inout("x7") _params.x7 => _result.x7,
            inout("x8") _params.x8 => _result.x8,
            inout("x9") _params.x9 => _result.x9,
            inout("x10") _params.x10 => _result.x10,
            inout("x11") _params.x11 => _result.x11,
            inout("x12") _params.x12 => _result.x12,
            inout("x13") _params.x13 => _result.x13,
            inout("x14") _params.x14 => _result.x14,
            inout("x15") _params.x15 => _result.x15,
            inout("x16") _params.x16 => _result.x16,
            inout("x17") _params.x17 => _result.x17,
            options(nomem, nostack)
        );
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}
