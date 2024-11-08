use uuid::Uuid;

use super::{ffa_smc, FfaDirectMsg, FfaError, FfaFunctionId, FfaParams, Result};

#[derive(Default)]
pub struct FfaMsg {
    _msg: FfaDirectMsg,
}

// Determined by the data that can fit in X4-x17
const FFA_DIRECT_MAX_PACKET_SIZE: usize = 112;

impl FfaMsg {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn function_id(&self) -> u32 {
        self._msg._function_id
    }
    pub fn source_id(&self) -> u16 {
        self._msg._source_id
    }
    pub fn destination_id(&self) -> u16 {
        self._msg._destination_id
    }
    pub fn uuid(&self) -> Uuid {
        self._msg._uuid
    }

    pub fn args64(&self) -> [u64; 14] {
        self._msg._args64
    }

    pub fn extract_u8_at_index(&self, idx: usize) -> u8 {
        // Make sure we access within range
        if idx > FFA_DIRECT_MAX_PACKET_SIZE {
            return 0;
        }

        let u64_index = idx / 8;
        let byte_index = idx % 8;
        (self._msg._args64[u64_index] >> (byte_index * 8) & 0xFF) as u8
    }

    pub(crate) fn exec(&self, msg: &FfaDirectMsg) -> Result<Self> {
        let params: FfaParams = msg.into();

        let result = ffa_smc(params);

        let id = FfaFunctionId::from(result.x0);

        match id {
            FfaFunctionId::FfaMsgSendDirectReq | FfaFunctionId::FfaMsgSendDirectReq2 => Ok(Self {
                _msg: result.into(),
            }),
            FfaFunctionId::FfaError => Err(FfaError::InvalidParameters),
            _ => panic!("Unknown FfaFunctionId"),
        }
    }
}
