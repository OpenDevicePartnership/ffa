use super::{ffa_smc, FfaDirectMsg, FfaError, FfaFunctionId, FfaParams, Result};

#[derive(Default)]
pub struct FfaMsgWait {
    _msg: FfaDirectMsg,
}

impl FfaMsgWait {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn msg(&self) -> FfaDirectMsg {
        self._msg
    }

    pub fn function_id(&self) -> u32 {
        self._msg._function_id
    }

    pub(crate) fn exec(self) -> Result<Self> {
        let params = FfaParams {
            x0: FfaFunctionId::FfaMsgWait.into(),
            ..Default::default()
        };

        let result = ffa_smc(params);

        let id = FfaFunctionId::from(result.x0);

        match id {
            FfaFunctionId::FfaMsgSendDirectReq | FfaFunctionId::FfaMsgSendDirectReq2 => Ok(Self {
                _msg: result.into(),
            }),
            FfaFunctionId::FfaInterrupt => panic!("FfaInterrupt Not implemented"),
            FfaFunctionId::FfaError => Err(FfaError::InvalidParameters),
            _ => unreachable!(),
        }
    }
}
