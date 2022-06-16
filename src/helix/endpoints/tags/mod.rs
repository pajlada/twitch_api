//! Helix endpoints regarding tags
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient, tags::GetAllStreamTagsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetAllStreamTagsRequest::builder().build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```

use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod get_all_stream_tags;

#[doc(inline)]
pub use get_all_stream_tags::{GetAllStreamTagsRequest, Tag};

/// Language code, formatted as 2 letter language by ISO 639-1, a dash (`-`) and 2 letter region by ISO 3166-1
///
/// i.e
/// `en-us`
/// `bg-bg`
/// etc etc
#[aliri_braid::braid(serde)]
pub struct TagLanguage;

/// Tag is auto-generated or not.
#[derive(Clone, Debug, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(from = "bool")]
#[serde(into = "bool")]
pub enum AutoGenerated {
    /// Was auto-generated
    True,
    /// Was not auto-generated
    False,
}

impl From<bool> for AutoGenerated {
    fn from(v: bool) -> Self {
        match v {
            true => AutoGenerated::True,
            false => AutoGenerated::False,
        }
    }
}

impl From<AutoGenerated> for bool {
    fn from(v: AutoGenerated) -> Self {
        match v {
            AutoGenerated::True => true,
            AutoGenerated::False => false,
        }
    }
}
/// A stream tag as defined by Twitch.
#[derive(PartialEq, Deserialize, Debug, Clone)]
pub struct TwitchTag {
    /// ID of the tag.
    #[serde(alias = "tag_id")]
    pub id: types::TagId,
    /// true if the tag is auto-generated; otherwise, false . An auto-generated tag is one automatically applied by Twitch (e.g., a language tag based on the broadcaster’s settings); these cannot be added or removed by the user.
    pub is_auto: AutoGenerated,
    /// All localized names of the tag.
    pub localization_names: HashMap<TagLanguage, String>,
    /// All localized descriptions of the tag.
    pub localization_descriptions: HashMap<TagLanguage, String>,
}
