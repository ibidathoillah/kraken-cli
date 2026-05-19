//! Stable integration surface for consumers that embed this crate as a
//! submodule or path dependency.

pub use crate::client::{FuturesClient, SpotClient};
pub use crate::config::{FuturesCredentials, SpotCredentials};
pub use crate::errors::{KrakenError, Result as KrakenResult};
pub use crate::output::OutputFormat;
pub use crate::{dispatch, normalize_pair, AppContext, Cli, Command};

/// Backwards-compatible spot credential alias for external consumers.
pub type Credentials = SpotCredentials;

/// Convenience imports for external consumers.
pub mod prelude {
    pub use super::{
        dispatch, normalize_pair, AppContext, Cli, Command, Credentials, FuturesClient,
        FuturesCredentials, KrakenError, KrakenResult, OutputFormat, SpotClient, SpotCredentials,
    };
}
