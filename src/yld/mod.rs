use super::{ffa_smc, FfaError, FfaFunctionId, FfaParams};

pub struct FfaYield {
    pub function_id: FfaFunctionId,
    pub vcpu_id: u16,
    pub endpoint_id: u16,
    pub timeout_lo: u32,
    pub timeout_hi: u32,
}

impl From<&FfaYield> for FfaParams {
    fn from(msg: &FfaYield) -> Self {
        FfaParams {
            x0: FfaFunctionId::FfaMsgYield.into(),
            x1: ((msg.endpoint_id as u64) << 16) | (msg.vcpu_id as u64),
            x2: msg.timeout_lo as u64,
            x3: msg.timeout_hi as u64,
            ..Default::default()
        }
    }
}

impl FfaYield {
    pub fn new(timeout: u64) -> Self {
        FfaYield {
            function_id: FfaFunctionId::FfaMsgYield.into(),
            vcpu_id: 0,
            endpoint_id: 0,
            timeout_lo: timeout as u32,
            timeout_hi: (timeout >> 32) as u32,
        }
    }

    pub fn exec(&self) -> FfaError {
        let params: FfaParams = self.into();

        let result = ffa_smc(params);

        match result.x0.try_into().unwrap() {
            FfaFunctionId::FfaSuccess32 => FfaError::Ok,
            FfaFunctionId::FfaError => (result.x2 as i64).into(),
            _ => {
                // Panic on debug builds else return error to handler on release
                #[cfg(debug_assertions)]
                panic!("Unknown return from FfaYield: {}", result.x0);
                #[cfg(not(debug_assertions))]
                FfaError::InvalidParameters
            }
        }
    }
}
