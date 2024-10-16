//! ARM Firmware Framework for ARMv8-A Profile

#![doc(html_root_url = "https://docs.rs/ffa/latest")]
#![cfg_attr(not(test), no_std)]

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

impl From<FfaError> for i32 {
    fn from(value: FfaError) -> i32 {
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
            FfaError::UnknownError => i32::MIN,
        }
    }
}

impl From<i32> for FfaError {
    fn from(value: i32) -> FfaError {
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

#[repr(u32)]
pub enum FfaFunctionId {
    FfaError = 0x84000060,
    FfaSuccess32 = 0x84000061,
    FfaSuccess64 = 0xc4000061,
    FfaInterrupt = 0x84000062,
    FfaVersion = 0x84000063,
    FfaFeatures = 0x84000064,
    FfaRxRelease = 0x84000065,
    FfaRxTxMap32 = 0x84000066,
    FfaRxTxMap64 = 0xc4000066,
    FfaRxtxUnmap = 0x84000067,
    FfaPartitionInfoGet = 0x84000068,
    FfaIdGet = 0x84000069,
    FfaMsgWait = 0x8400006b,
    FfaMsgYield = 0x8400006c,
    FfaMsgRun = 0x8400006d,
    FfaMsgSend = 0x8400006e,
    FfaMsgSendDirectReq32 = 0x8400006f,
    FfaMsgSendDirectReq64 = 0xc400006f,
    FfaMsgSendDirectResp32 = 0x84000070,
    FfaMsgSendDirectResp64 = 0xc4000070,
    FfaMsgPoll = 0x8400006a,
    FfaMemDonate32 = 0x84000071,
    FfaMemDonate64 = 0xc4000071,
    FfaMemLend32 = 0x84000072,
    FfaMemLend64 = 0xc4000072,
    FfaMemShare32 = 0x84000073,
    FfaMemShare64 = 0xc4000073,
    FfaMemRetrieveReq32 = 0x84000074,
    FfaMemRetrieveReq64 = 0xc4000074,
    FfaMemRetrieveResp = 0x84000075,
    FfaMemRelinquish = 0x84000076,
    FfaMemReclaim = 0x84000077,
    FfaMemFragRx = 0x8400007a,
    FfaMemFragTx = 0x8400007b,
    FfaMemPermGet = 0x84000088,
    FfaMemPermSet = 0x84000089,
    FfaConsoleLog32 = 0x8400008a,
    FfaConsoleLog64 = 0xc400008a,
}

pub struct Ffa;

impl Ffa {
    const FFA_VERSION_MAJOR: u32 = 1;
    const FFA_VERSION_MINOR: u32 = 0;
    pub fn new() -> Self {
        Ffa {}
    }

    pub fn version(&self) -> Result<u32> {
        let params = FfaParams {
            x0: FfaFunctionId::FfaVersion as u32,
            x1: Self::FFA_VERSION_MAJOR << 16 | Self::FFA_VERSION_MINOR << 0,
            ..Default::default()
        };

        let result = self.svc(params);

        if result & (1 << 31) == 0 {
            Ok(result)
        } else {
            Err(Into::<FfaError>::into(result as i32))
        }
    }

    fn svc(&self, params: FfaParams) -> u32 {
        ffa_svc(
            params.x0, params.x1, params.x2, params.x3, params.x4, params.x5, params.x6, params.x7,
        )
    }
}

pub struct FfaParams {
    pub x0: u32,
    pub x1: u32,
    pub x2: u32,
    pub x3: u32,
    pub x4: u32,
    pub x5: u32,
    pub x6: u32,
    pub x7: u32,
}

impl Default for FfaParams {
    fn default() -> Self {
        Self {
            x0: 0,
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
        }
    }
}

/// Supervisor Call
#[inline(always)]
fn ffa_svc(_x0: u32, _x1: u32, _x2: u32, _x3: u32, _x4: u32, _x5: u32, _x6: u32, _x7: u32) -> u32 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let result = 0u32;
        core::arch::asm!("svc #0", "mov x0, {result}", result = out(reg) _, options(nomem, nostack));
        result
    }

    #[cfg(not(target_arch = "aarch64"))]
    unimplemented!()
}

// pub fn console_log(input: &[u8]) {
//     for (i, arg) in input.chunks(8).map(|c| {
//         let mut buf = [0u8; 8];
//         let len = 8.min(c.len());
//         buf[..len].copy_from_slice(&c[..len]);
//         u32::from_le_bytes(buf)
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
//         u32::from_le_bytes(buf)
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
