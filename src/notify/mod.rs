use super::{ffa_smc, FfaError, FfaFunctionId, FfaParams, Result};

// Determined by the data that can fit in X2-x17
const FFA_DIRECT_MAX_PACKET_SIZE: usize = 128;

#[derive(Default)]
pub struct FfaNotifyMsg {
    _function_id: u32,
    _source_id: u16,
    _destination_id: u16,
    _args64: [u64; 16],
}

#[derive(Default)]
pub struct FfaNotify {
    _msg: FfaNotifyMsg,
}

impl FfaNotifyMsg {
    pub fn new(
        function_id: FfaFunctionId,
        source_id: u16,
        destination_id: u16,
        args64: [u64; 16],
    ) -> FfaNotifyMsg {
        FfaNotifyMsg {
            _function_id: <FfaFunctionId as Into<u64>>::into(function_id) as u32,
            _source_id: source_id,
            _destination_id: destination_id,
            _args64: args64,
        }
    }
}

impl From<FfaParams> for FfaNotifyMsg {
    fn from(params: FfaParams) -> FfaNotifyMsg {
        FfaNotifyMsg {
            _function_id: params.x0 as u32, // Function id is in lower 32 bits of x0
            _source_id: (params.x1 >> 16) as u16, // Source in upper 16 bits
            _destination_id: params.x1 as u16, // Destination in lower 16 bits
            _args64: [
                params.x2, params.x3, params.x4, params.x5, params.x6, params.x7, params.x8,
                params.x9, params.x10, params.x11, params.x12, params.x13, params.x14, params.x15,
                params.x16, params.x17,
            ],
        }
    }
}

impl From<&FfaNotifyMsg> for FfaParams {
    fn from(msg: &FfaNotifyMsg) -> Self {
        FfaParams {
            x0: msg._function_id as u64,
            x1: ((msg._source_id as u64) << 16) | (msg._destination_id as u64),
            x2: msg._args64[0],
            x3: msg._args64[1],
            x4: msg._args64[2],
            x5: msg._args64[3],
            x6: msg._args64[4],
            x7: msg._args64[5],
            x8: msg._args64[6],
            x9: msg._args64[7],
            x10: msg._args64[8],
            x11: msg._args64[9],
            x12: msg._args64[10],
            x13: msg._args64[11],
            x14: msg._args64[12],
            x15: msg._args64[13],
            x16: msg._args64[14],
            x17: msg._args64[15],
        }
    }
}

impl FfaNotify {
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

    pub fn args64(&self) -> [u64; 16] {
        self._msg._args64
    }

    pub fn status(&self) -> i64 {
        self._msg._args64[0] as i64
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

    pub fn exec(&self, msg: &FfaNotifyMsg) -> Result<Self> {
        let params: FfaParams = msg.into();

        let result = ffa_smc(params);
        let id = FfaFunctionId::from(result.x0);
        match id {
            FfaFunctionId::FfaSuccess32 => Ok(Self {
                _msg: result.into(),
            }),
            FfaFunctionId::FfaError => Err(FfaError::InvalidParameters),
            _ => panic!("Unknown FfaFunctionId"),
        }
    }
}
