use super::{ffa_smc, FfaError, FfaFunctionId, FfaParams, Result};

#[derive(Default)]
pub struct FfaFeatures {
    _id: u64,
    _properties: u64,
    _interface_properties: u64,
}

impl FfaFeatures {
    pub fn new(_id: u64, _properties: u64) -> Self {
        Self {
            _id,
            _properties,
            _interface_properties: 0,
        }
    }

    pub(crate) fn exec(self) -> Result<Self> {
        let params = FfaParams {
            x0: FfaFunctionId::FfaFeatures.into(),
            x1: self.id(),
            ..Default::default()
        };

        let result = ffa_smc(params);

        if result.x0 == FfaFunctionId::FfaError.into() {
            Err(Into::<FfaError>::into(result.x0 as i64))
        } else {
            Ok(Self {
                _id: result.x0,
                _properties: result.x1,
                _interface_properties: result.x2,
            })
        }
    }

    pub fn id(&self) -> u64 {
        self._id
    }

    pub fn properties(&self) -> u64 {
        self._properties
    }

    pub fn interface_properties(&self) -> u64 {
        self._interface_properties
    }

    pub fn is_feature_id(&self) -> bool {
        self._id & (1 << 31) == 0
    }

    pub fn is_function_id(&self) -> bool {
        self._id & (1 << 31) != 0
    }
}
