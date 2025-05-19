use core::{mem, slice};

use uuid::Uuid;

use super::{ffa_smc, FfaError, FfaFunctionId, FfaParams, Result};

impl From<&FfaMsg> for FfaParams {
    fn from(msg: &FfaMsg) -> Self {
        let (uuid_high, uuid_low) = msg.uuid.as_u64_pair();
        FfaParams {
            x0: msg.function_id.into(),
            x1: ((msg.source_id as u64) << 16) | (msg.destination_id as u64),
            x2: uuid_high.to_be(),
            x3: uuid_low.to_be(),
            x4: msg.args64[0],
            x5: msg.args64[1],
            x6: msg.args64[2],
            x7: msg.args64[3],
            x8: msg.args64[4],
            x9: msg.args64[5],
            x10: msg.args64[6],
            x11: msg.args64[7],
            x12: msg.args64[8],
            x13: msg.args64[9],
            x14: msg.args64[10],
            x15: msg.args64[11],
            x16: msg.args64[12],
            x17: msg.args64[13],
        }
    }
}

impl TryFrom<FfaParams> for FfaMsg {
    type Error = ();

    fn try_from(params: FfaParams) -> core::result::Result<Self, Self::Error> {
        let function_id = FfaFunctionId::try_from(params.x0)?;
        Ok(FfaMsg {
            function_id,
            source_id: (params.x1 >> 16) as u16, // Source in upper 16 bits
            destination_id: params.x1 as u16,    // Destination in lower 16 bits
            uuid: Uuid::from_u64_pair(params.x2.to_be(), params.x3.to_be()),
            args64: [
                params.x4, params.x5, params.x6, params.x7, params.x8, params.x9, params.x10,
                params.x11, params.x12, params.x13, params.x14, params.x15, params.x16, params.x17,
            ],
        })
    }
}

#[derive(Debug)]
pub struct FfaMsg {
    pub function_id: FfaFunctionId,
    pub source_id: u16,
    pub destination_id: u16,
    pub uuid: Uuid,
    pub args64: [u64; 14],
}

impl FfaMsg {
    pub fn extract_u8_at_index(&self, idx: usize) -> u8 {
        // x4-x17 is 112 bytes
        let args: [u8; 112] = unsafe { core::mem::transmute(self.args64) };
        args[idx]
    }

    pub fn struct_to_args64<T>(&mut self, s: &T) {
        let size = mem::size_of::<T>();
        let args_len = self.args64.len();

        unsafe {
            let ptr = s as *const T as *const u8;
            let byte_slice = slice::from_raw_parts(ptr, size);

            for (i, chunk) in byte_slice.chunks(8).enumerate() {
                if i >= args_len {
                    break;
                }
                let mut buffer = [0u8; 8];
                buffer[..chunk.len()].copy_from_slice(chunk);
                self.args64[i] = u64::from_ne_bytes(buffer);
            }
        }
    }

    pub(crate) fn exec(&self) -> Result<Self> {
        let params: FfaParams = self.into();
        let result = ffa_smc(params);

        // Return FfaMsg or error to caller
        result.try_into().map_err(|_| FfaError::UnknownError)
    }
}
