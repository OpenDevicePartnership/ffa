use crate::FfaError;

use super::{ffa_smc, FfaFunctionId, FfaParams, Result};

impl From<&FfaMemory> for FfaParams {
    fn from(msg: &FfaMemory) -> Self {
        FfaParams {
            x0: msg._function_id,
            x1: msg._total_length,
            x2: msg._frag_length,
            x3: msg._tx_address,
            x4: msg._page_count,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct FfaMemory {
    _function_id: u64,
    _total_length: u64,
    _frag_length: u64,
    _tx_address: u64,
    _page_count: u64,
    rx_buffer: u64,
    tx_buffer: u64,
}

impl FfaMemory {
    pub fn new() -> Self {
        Self::default()
    }

    fn exec(&self) -> Result<FfaParams> {
        let params: FfaParams = self.into();
        let result = ffa_smc(params);
        let err = result.x2 as i64;
        let function_id = FfaFunctionId::try_from(result.x0).map_err(|_| FfaError::UnknownError)?;

        match function_id {
            FfaFunctionId::FfaSuccess32 | FfaFunctionId::FfaMemRetrieveResp => Ok(result),
            FfaFunctionId::FfaError => Err(err.into()),
            _ => panic!("Unknown error"),
        }
    }

    pub fn set_rxtx_buffers(&mut self, rx_buffer: u64, tx_buffer: u64) {
        self.rx_buffer = rx_buffer;
        self.tx_buffer = tx_buffer;
    }

    pub fn retrieve_req(&mut self, _address: u64, _length: u64) -> Result<FfaParams> {
        // It is expected our TX buffer is already populated

        // Populate FFA request and send it
        self._function_id = FfaFunctionId::FfaMemRetrieveReq.into();
        self._total_length = 0x40;
        self._frag_length = 0x40;
        self._tx_address = 0;
        self._page_count = 0;

        self.exec()
    }
}
