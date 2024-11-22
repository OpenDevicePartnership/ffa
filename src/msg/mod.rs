use uuid::Uuid;

use super::{ffa_smc, FfaDirectMsg, FfaError, FfaFunctionId, FfaParams, Result};

#[derive(Default)]
pub struct FfaMsg {
    _msg: FfaDirectMsg,
}

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
        // x4-x17 is 112 bytes
        let args: [u8; 112] = unsafe { core::mem::transmute(self._msg._args64) };
        args[idx]
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
