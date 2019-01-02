// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

mod account;
mod routing;
#[cfg(test)]
mod tests;
pub mod vault;

pub use self::account::{Account, DEFAULT_MAX_MUTATIONS};
pub use self::routing::{RequestHookFn, Routing};

use crate::routing::XorName;

/// Identifier of immutable data
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ImmutableDataId(pub XorName);

impl ImmutableDataId {
    pub fn name(&self) -> &XorName {
        &self.0
    }
}

/// Identifier of mutable data
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MutableDataId(pub XorName, pub u64);

impl MutableDataId {
    pub fn name(&self) -> &XorName {
        &self.0
    }
}

/// Identifier for a data (immutable or mutable)
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum DataId {
    /// Identifier of immutable data.
    Immutable(ImmutableDataId),
    /// Identifier of mutable data.
    Mutable(MutableDataId),
}

impl DataId {
    /// Create `DataId` for immutable data.
    pub fn immutable(name: XorName) -> Self {
        DataId::Immutable(ImmutableDataId(name))
    }

    /// Create `DataId` for mutable data.
    pub fn mutable(name: XorName, tag: u64) -> Self {
        DataId::Mutable(MutableDataId(name, tag))
    }

    /// Get name of this identifier.
    pub fn name(&self) -> &XorName {
        match *self {
            DataId::Immutable(ref id) => id.name(),
            DataId::Mutable(ref id) => id.name(),
        }
    }
}
