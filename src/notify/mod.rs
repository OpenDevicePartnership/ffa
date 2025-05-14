use super::{ffa_smc, FfaError, FfaFunctionId, FfaParams, Result};

#[derive(Default)]
pub struct FfaNotify {
    pub function_id: u64,
    pub source_id: u16,
    pub destination_id: u16,
    pub args64: [u64; 16],
}

impl From<FfaParams> for FfaNotify {
    fn from(params: FfaParams) -> FfaNotify {
        FfaNotify {
            function_id: params.x0,              // Function id is in lower 32 bits of x0
            source_id: (params.x1 >> 16) as u16, // Source in upper 16 bits
            destination_id: params.x1 as u16,    // Destination in lower 16 bits
            args64: [
                params.x2, params.x3, params.x4, params.x5, params.x6, params.x7, params.x8,
                params.x9, params.x10, params.x11, params.x12, params.x13, params.x14, params.x15,
                params.x16, params.x17,
            ],
        }
    }
}

impl From<&FfaNotify> for FfaParams {
    fn from(msg: &FfaNotify) -> Self {
        FfaParams {
            x0: msg.function_id,
            x1: ((msg.source_id as u64) << 16) | (msg.destination_id as u64),
            x2: msg.args64[0],
            x3: msg.args64[1],
            x4: msg.args64[2],
            x5: msg.args64[3],
            x6: msg.args64[4],
            x7: msg.args64[5],
            x8: msg.args64[6],
            x9: msg.args64[7],
            x10: msg.args64[8],
            x11: msg.args64[9],
            x12: msg.args64[10],
            x13: msg.args64[11],
            x14: msg.args64[12],
            x15: msg.args64[13],
            x16: msg.args64[14],
            x17: msg.args64[15],
        }
    }
}

impl FfaNotify {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract_u8_at_index(&self, idx: usize) -> u8 {
        // x2-x17 = 128 bytes
        let args: [u8; 128] = unsafe { core::mem::transmute(self.args64) };
        args[idx]
    }

    pub fn exec(&self) -> Result<Self> {
        let params: FfaParams = self.into();

        let result = ffa_smc(params);
        let function_id = FfaFunctionId::try_from(result.x0).map_err(|_| FfaError::UnknownError)?;

        match function_id {
            FfaFunctionId::FfaSuccess32 => Ok(result.into()),
            FfaFunctionId::FfaError => Err(FfaError::InvalidParameters),
            _ => panic!("Unknown FfaFunctionId"),
        }
    }
}
