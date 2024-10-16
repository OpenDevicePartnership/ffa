use super::{ffa_svc, FfaError, FfaFunctionId, FfaParams, Result};

macro_rules! ffa_version {
    ($major:expr, $minor:expr) => {
        $major << 16 | $minor << 0
    };
}

#[derive(Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FfaVersion {
    _major: u16,
    _minor: u16,
}

impl FfaVersion {
    const FFA_VERSION_MAJOR: u64 = 1;
    const FFA_VERSION_MINOR: u64 = 3;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn major(&self) -> u16 {
        self._major
    }

    pub fn minor(&self) -> u16 {
        self._minor
    }

    pub(crate) fn exec(self) -> Result<Self> {
        let params = FfaParams {
            x0: FfaFunctionId::FfaVersion.into(),
            x1: ffa_version!(Self::FFA_VERSION_MAJOR, Self::FFA_VERSION_MINOR),
            ..Default::default()
        };

        let result = ffa_svc(params);

        // Specification explicitly calls out checking bit 31
        if result.x0 & (1 << 31) == 0 {
            Ok(Self {
                _major: (result.x0 >> 16) as u16,
                _minor: (result.x0 & 0xffff) as u16,
            })
        } else {
            Err(Into::<FfaError>::into(result.x0 as i64))
        }
    }
}
