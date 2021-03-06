// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License").
// You may not use this file except in compliance with the License.
// A copy of the License is located at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// or in the "license" file accompanying this file. This file is distributed
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing
// permissions and limitations under the License.

use std::collections::HashMap;

use license::TextData;

#[derive(Serialize, Deserialize)]
pub(crate) struct LicenseEntry {
    pub original: TextData,
    pub aliases: Vec<String>,
    pub headers: Vec<TextData>,
    pub alternates: Vec<TextData>,
}

/// A representation of a collection of known licenses.
///
/// This struct is generally what you want to start with if you're looking to
/// match text against a database of licenses. Load a cache from disk using
/// `from_cache`, then use the `analyze` function to determine what a text most
/// closely matches.
///
/// # Examples
///
/// ```rust,ignore
/// # // FIXME: this should not be ignored, but there's a bug in nightly:
/// # // https://github.com/rust-lang/rust/issues/48890
/// # use std::fs::File;
/// # use std::error::Error;
/// use askalono::{Store, TextData};
///
/// # fn main() -> Result<(), Box<Error>> {
/// let store = Store::from_cache(File::open("askalono-cache.bin.gz")?)?;
/// let result = store.analyze(&TextData::from("what's this"))?;
/// # Ok(())
/// # }
/// ```
#[derive(Default, Serialize, Deserialize)]
pub struct Store {
    pub(super) licenses: HashMap<String, LicenseEntry>,
}

impl LicenseEntry {
    pub fn new(original: TextData) -> LicenseEntry {
        LicenseEntry {
            original,
            aliases: Vec::new(),
            alternates: Vec::new(),
            headers: Vec::new(),
        }
    }
}

impl Store {
    /// Create a new `Store`.
    ///
    /// More often, you probably want to use `from_cache` instead of creating
    /// an empty store.
    pub fn new() -> Store {
        Store {
            licenses: HashMap::new(),
        }
    }
}
