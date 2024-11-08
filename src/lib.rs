//! ARM Firmware Framework for ARMv8-A Profile

#![doc(html_root_url = "https://docs.rs/ffa/latest")]
#![cfg_attr(not(test), no_std)]

use core::{mem, slice};

use console::FfaConsole;
use features::FfaFeatures;
use msg::FfaMsg;
use version::FfaVersion;

#[macro_use]
pub mod console;
pub mod features;
pub mod msg;
pub mod version;

pub type Result<T> = core::result::Result<T, FfaError>;

#[derive(Default)]
pub struct FfaDirectMsg {
    _function_id: u32,
    _source_id: u16,
    _destination_id: u16,
    _uuid: u128,
    _args64: [u64; 14],
}

impl FfaDirectMsg {
    pub fn new(
        function_id: FfaFunctionId,
        source_id: Option<u16>,
        destination_id: Option<u16>,
        uuid: Option<u128>,
        args64: Option<[u64; 14]>,
    ) -> FfaDirectMsg {
        FfaDirectMsg {
            _function_id: <FfaFunctionId as Into<u64>>::into(function_id) as u32,
            _source_id: source_id.unwrap_or(0),
            _destination_id: destination_id.unwrap_or(0),
            _uuid: uuid.unwrap_or(0),
            _args64: args64.unwrap_or([0; 14]),
        }
    }

    pub fn struct_to_args64<T>(&mut self, s: &T) {
        let size = mem::size_of::<T>();
        let args_len = self._args64.len();

        unsafe {
            let ptr = s as *const T as *const u8;
            let byte_slice = slice::from_raw_parts(ptr, size);

            for (i, chunk) in byte_slice.chunks(8).enumerate() {
                if i >= args_len {
                    break;
                }
                let mut buffer = [0u8; 8];
                buffer[..chunk.len()].copy_from_slice(chunk);
                self._args64[i] = u64::from_ne_bytes(buffer);
            }
        }
    }
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
    FfaMsgSendDirectResp2,
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
            FfaFunctionId::FfaMsgSendDirectResp2 => 0xc400008e,
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
            0xc400008e => FfaFunctionId::FfaMsgSendDirectResp2,
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
            _uuid: (params.x2 as u128) << 64 | params.x3 as u128,
            _args64: [
                params.x4, params.x5, params.x6, params.x7, params.x8, params.x9, params.x10,
                params.x11, params.x12, params.x13, params.x14, params.x15, params.x16, params.x17,
            ],
        }
    }
}

impl From<&FfaDirectMsg> for FfaParams {
    fn from(msg: &FfaDirectMsg) -> Self {
        FfaParams {
            x0: msg._function_id as u64,
            x1: ((msg._source_id as u64) << 16) | (msg._destination_id as u64),
            x2: msg._uuid as u64,
            x3: (msg._uuid >> 64) as u64,
            x4: msg._args64[0],
            x5: msg._args64[1],
            x6: msg._args64[2],
            x7: msg._args64[3],
            x8: msg._args64[4],
            x9: msg._args64[5],
            x10: msg._args64[6],
            x11: msg._args64[7],
            x12: msg._args64[8],
            x13: msg._args64[9],
            x14: msg._args64[10],
            x15: msg._args64[11],
            x16: msg._args64[12],
            x17: msg._args64[13],
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

    pub fn features(&self, id: u64, properties: u64) -> Result<FfaFeatures> {
        FfaFeatures::new(id, properties).exec()
    }

    pub fn msg_wait(&self) -> Result<FfaMsg> {
        FfaMsg::new().exec(&FfaDirectMsg::new(
            FfaFunctionId::FfaMsgWait,
            None,
            None,
            None,
            None,
        ))
    }

    pub fn msg_resp(&self, msg: &FfaDirectMsg) -> Result<FfaMsg> {
        FfaMsg::new().exec(msg)
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
