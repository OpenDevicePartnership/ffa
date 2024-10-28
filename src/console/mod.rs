use core::fmt;

use super::{ffa_smc, FfaError, FfaFunctionId, FfaParams, Result};

#[derive(Default)]
pub struct FfaConsole;

impl FfaConsole {
    const FFA_MAX_CHAR_COUNT: usize = 128;

    pub fn new() -> Self {
        Self
    }

    pub(crate) fn exec(&self, bytes: &[u8]) -> Result<()> {
        for chunk in bytes.chunks(Self::FFA_MAX_CHAR_COUNT) {
            self.exec_inner(FfaFunctionId::FfaConsoleLog, chunk)?;
        }

        Ok(())
    }

    fn exec_inner(&self, id: FfaFunctionId, bytes: &[u8]) -> Result<()> {
        let mut iter = bytes.chunks(8).map(|c| {
            let mut buf = [0u8; 8];
            let len = 8.min(c.len());
            buf[..len].copy_from_slice(&c[..len]);
            u64::from_le_bytes(buf)
        });

        let params = FfaParams {
            x0: id.into(),
            x1: bytes.len() as u64,
            x2: iter.next().unwrap_or(0),
            x3: iter.next().unwrap_or(0),
            x4: iter.next().unwrap_or(0),
            x5: iter.next().unwrap_or(0),
            x6: iter.next().unwrap_or(0),
            x7: iter.next().unwrap_or(0),
            x8: iter.next().unwrap_or(0),
            x9: iter.next().unwrap_or(0),
            x10: iter.next().unwrap_or(0),
            x11: iter.next().unwrap_or(0),
            x12: iter.next().unwrap_or(0),
            x13: iter.next().unwrap_or(0),
            x14: iter.next().unwrap_or(0),
            x15: iter.next().unwrap_or(0),
            x16: iter.next().unwrap_or(0),
            x17: iter.next().unwrap_or(0),
        };

        let result = ffa_smc(params);
        let error = FfaError::from(result.x2 as i64);

        match error {
            FfaError::NotSupported | FfaError::InvalidParameters | FfaError::Retry => Err(error),
            FfaError::Ok => Ok(()),
            _ => unreachable!(),
        }
    }
}

impl fmt::Write for FfaConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.exec(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    FfaConsole::new().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::console::_print(format_args_nl!($($arg)*));
    })
}
