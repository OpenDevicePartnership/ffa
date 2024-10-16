//! ARM Firmware Framework for ARMv8-A Profile

#![doc(html_root_url = "https://docs.rs/ffa/latest")]
#![cfg_attr(not(test), no_std)]

macro_rules! ffa_version {
    ($major:expr, $minor:expr) => {
        $major << 16 | $minor << 0
    };
}

pub type Result<T> = core::result::Result<T, FfaError>;

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
    FfaSuccess32,
    FfaSuccess64,
    FfaInterrupt,
    FfaVersion,
    FfaFeatures,
    FfaRxRelease,
    FfaRxTxMap32,
    FfaRxTxMap64,
    FfaRxtxUnmap,
    FfaPartitionInfoGet,
    FfaIdGet,
    FfaMsgWait,
    FfaMsgYield,
    FfaMsgRun,
    FfaMsgSend,
    FfaMsgSendDirectReq32,
    FfaMsgSendDirectReq64,
    FfaMsgSendDirectResp32,
    FfaMsgSendDirectResp64,
    FfaMsgPoll,
    FfaMemDonate32,
    FfaMemDonate64,
    FfaMemLend32,
    FfaMemLend64,
    FfaMemShare32,
    FfaMemShare64,
    FfaMemRetrieveReq32,
    FfaMemRetrieveReq64,
    FfaMemRetrieveResp,
    FfaMemRelinquish,
    FfaMemReclaim,
    FfaMemFragRx,
    FfaMemFragTx,
    FfaMemPermGet,
    FfaMemPermSet,
    FfaConsoleLog32,
    FfaConsoleLog64,
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
            FfaFunctionId::FfaRxTxMap32 => 0x84000066,
            FfaFunctionId::FfaRxTxMap64 => 0xc4000066,
            FfaFunctionId::FfaRxtxUnmap => 0x84000067,
            FfaFunctionId::FfaPartitionInfoGet => 0x84000068,
            FfaFunctionId::FfaIdGet => 0x84000069,
            FfaFunctionId::FfaMsgWait => 0x8400006b,
            FfaFunctionId::FfaMsgYield => 0x8400006c,
            FfaFunctionId::FfaMsgRun => 0x8400006d,
            FfaFunctionId::FfaMsgSend => 0x8400006e,
            FfaFunctionId::FfaMsgSendDirectReq32 => 0x8400006f,
            FfaFunctionId::FfaMsgSendDirectReq64 => 0xc400006f,
            FfaFunctionId::FfaMsgSendDirectResp32 => 0x84000070,
            FfaFunctionId::FfaMsgSendDirectResp64 => 0xc4000070,
            FfaFunctionId::FfaMsgPoll => 0x8400006a,
            FfaFunctionId::FfaMemDonate32 => 0x84000071,
            FfaFunctionId::FfaMemDonate64 => 0xc4000071,
            FfaFunctionId::FfaMemLend32 => 0x84000072,
            FfaFunctionId::FfaMemLend64 => 0xc4000072,
            FfaFunctionId::FfaMemShare32 => 0x84000073,
            FfaFunctionId::FfaMemShare64 => 0xc4000073,
            FfaFunctionId::FfaMemRetrieveReq32 => 0x84000074,
            FfaFunctionId::FfaMemRetrieveReq64 => 0xc4000074,
            FfaFunctionId::FfaMemRetrieveResp => 0x84000075,
            FfaFunctionId::FfaMemRelinquish => 0x84000076,
            FfaFunctionId::FfaMemReclaim => 0x84000077,
            FfaFunctionId::FfaMemFragRx => 0x8400007a,
            FfaFunctionId::FfaMemFragTx => 0x8400007b,
            FfaFunctionId::FfaMemPermGet => 0x84000088,
            FfaFunctionId::FfaMemPermSet => 0x84000089,
            FfaFunctionId::FfaConsoleLog32 => 0x8400008a,
            FfaFunctionId::FfaConsoleLog64 => 0xc400008a,
        }
    }
}

#[derive(Default)]
pub struct Ffa;

impl Ffa {
    const FFA_VERSION_MAJOR: u64 = 1;
    const FFA_VERSION_MINOR: u64 = 0;

    pub fn new() -> Self {
        Ffa {}
    }

    pub fn version(&self) -> Result<u64> {
        let params = FfaParams {
            x0: FfaFunctionId::FfaVersion.into(),
            x1: ffa_version!(Self::FFA_VERSION_MAJOR, Self::FFA_VERSION_MINOR),
            ..Default::default()
        };

        let result = self.svc(params);

        // Checking for BIT 31 is enough here due to sign extension.
        if result.x0 & (1 << 31) == 0 {
            Ok(result.x0)
        } else {
            Err(Into::<FfaError>::into(result.x0 as i64))
        }
    }

    fn svc(&self, params: FfaParams) -> FfaParams {
        let mut result = FfaParams::default();

        ffa_svc(
            params.x0,
            params.x1,
            params.x2,
            params.x3,
            params.x4,
            params.x5,
            params.x6,
            params.x7,
            &mut result,
        );

        result
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
}

/// Supervisor Call
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn ffa_svc(
    _x0: u64,
    _x1: u64,
    _x2: u64,
    _x3: u64,
    _x4: u64,
    _x5: u64,
    _x6: u64,
    _x7: u64,
    _result: &mut FfaParams,
) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!(
            "svc #0",
            inout("x0") _x0 => _result.x0,
            inout("x1") _x1 => _result.x1,
            inout("x2") _x2 => _result.x2,
            inout("x3") _x3 => _result.x3,
            inout("x4") _x4 => _result.x4,
            inout("x5") _x5 => _result.x5,
            inout("x6") _x6 => _result.x6,
            inout("x7") _x7 => _result.x7,
            options(nomem, nostack)
        );
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

// pub fn console_log(input: &[u8]) {
//     for (i, arg) in input.chunks(8).map(|c| {
//         let mut buf = [0u8; 8];
//         let len = 8.min(c.len());
//         buf[..len].copy_from_slice(&c[..len]);
//         u64::from_le_bytes(buf)
//     }).enumerate() {
//         println!("Arg{}: {:016x}", i, arg);
//     }
// }
//
// fn main() {
//     let input = "testing this little thing. Will it work? What happens, therefore, when the input is very, very long? Will it be able to print everything?".as_bytes();
//     let encoded = input.chunks(8).map(|c| {
//         let mut buf = [0u8; 8];
//         let len = 8.min(c.len());
//         buf[..len].copy_from_slice(&c[..len]);
//         u64::from_le_bytes(buf)
//     }).collect::<Vec<_>>();
//     let decoded = encoded.iter().flat_map(|c| c.to_le_bytes()).map(|c| char::from(c)).collect::<Vec<_>>();
//     println!(r##"
//     input:   {:?}
//     encoded: {:016x?}
//     decoded: {:?}
//     "##, input, encoded, decoded);
//     for (i, chunk) in input.chunks(48).enumerate() {
//         println!("Handling chunk {i}");
//         console_log(chunk);
//     }
// }
