use super::{ffa_smc, FfaError, FfaFunctionId, FfaParams};

impl From<&FfaRxTxMsg> for FfaParams {
    fn from(msg: &FfaRxTxMsg) -> Self {
        FfaParams {
            x0: msg.function_id,
            x1: msg.x1,
            x2: msg.x2,
            x3: msg.x3,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct FfaRxTxMsg {
    function_id: u64,
    x1: u64,
    x2: u64,
    x3: u64,
}

impl FfaRxTxMsg {
    pub fn new() -> Self {
        Self::default()
    }

    fn exec(&self) -> FfaError {
        let params: FfaParams = self.into();

        let result = ffa_smc(params);

        let err = result.x2 as i64;

        match FfaFunctionId::try_from(result.x0).unwrap() {
            FfaFunctionId::FfaSuccess32 => FfaError::Ok,
            FfaFunctionId::FfaError => err.into(),
            _ => panic!("Unknown error"),
        }
    }

    pub fn map(&mut self, tx_addr: u64, rx_addr: u64, page_count: u32) -> FfaError {
        self.function_id = FfaFunctionId::FfaRxTxMap.into();
        self.x1 = tx_addr;
        self.x2 = rx_addr;
        self.x3 = page_count as u64;

        self.exec()
    }

    pub fn unmap(&mut self, vm_id: u16) -> FfaError {
        self.function_id = FfaFunctionId::FfaRxTxUnmap.into();
        self.x1 = (vm_id as u64) << 16;
        self.x2 = 0;
        self.x3 = 0;

        self.exec()
    }
}
