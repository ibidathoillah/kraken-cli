//! Stable integration surface for consumers that embed this crate as a
//! submodule or path dependency.

pub use crate::client::{FuturesClient, SpotClient};
pub use crate::config::{Credentials, FuturesCredentials, SpotCredentials};
pub use crate::errors::{KrakenError, Result as KrakenResult};
pub use crate::output::{CommandOutput, OutputFormat};
pub use crate::{dispatch, normalize_pair, AppContext, Cli, Command};

/// Convenience imports for external consumers.
pub mod prelude {
    pub use super::{
        dispatch, normalize_pair, AppContext, Cli, Command, CommandOutput, Credentials,
        FuturesClient, FuturesCredentials, KrakenError, KrakenResult, OutputFormat, SpotClient,
        SpotCredentials,
    };
}
