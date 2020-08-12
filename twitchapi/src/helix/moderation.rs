//! Endpoints regarding moderation

use crate::helix;
#[doc(inline)]
pub use get_banned_events::{GetBannedEvents, GetBannedEventsRequest};
#[doc(inline)]
pub use get_banned_users::{GetBannedUsers, GetBannedUsersRequest};
#[doc(inline)]
pub use get_moderators::{GetModerators, GetModeratorsRequest};
#[doc(inline)]
pub use get_moderators_events::{GetModeratorsEvents, GetModeratorsEventsRequest};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Returns all moderators in a channel.
/// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
pub mod get_moderators {
    use super::*;

    // FIXME: Twitch Docs is borked here, mentions query param user_id
    // user_id	no	string	Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
    // Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
    // Maximum: 100
    /// Query Parameters for [Get Moderators](super::get_moderators)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    pub struct GetModeratorsRequest {
        /// Provided `broadcaster_id` must match the `user_id` in the auth token.
        pub broadcaster_id: String,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Moderators](super::get_moderators)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    pub struct GetModerators {
        /// User ID of moderator
        ///
        /// Twitch says: `User ID of a user who has been banned.` but this seems wrong.
        user_id: String,
        /// Display name of moderator
        ///
        /// Twitch says: `Display name of a user who has been banned.` but this seems wrong.
        user_name: String,
    }

    impl helix::Request for GetModeratorsRequest {
        type Response = GetModerators;

        const PATH: &'static str = "moderation/moderators";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
    }

    impl helix::RequestGet for GetModeratorsRequest {}

    impl helix::Paginated for GetModeratorsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}

/// Returns a list of moderators or users added and removed as moderators from a channel.
/// [`get-moderators-events`](https://dev.twitch.tv/docs/api/reference#get-moderators-events)
pub mod get_moderators_events {
    use super::*;
    use std::collections::HashMap;

    /// Query Parameters for [Get Moderators Events](super::get_moderators_events)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    pub struct GetModeratorsEventsRequest {
        /// Provided `broadcaster_id` must match the `user_id` in the auth token.
        pub broadcaster_id: String,
        // FIXME: Twitch docs sucks...
        /// Filters the results and only returns a status object for users who are moderators in this channel and have a matching user_id.
        /// Format: Repeated Query Parameter, eg. /moderation/Moderators?broadcaster_id=1&user_id=2&user_id=3
        /// Maximum: 100
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<String>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Moderators Events](super::get_moderators_events)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    pub struct GetModeratorsEvents {
        /// Event ID
        pub id: String,
        // FIXME: Twitch docs sucks...
        /// Displays `moderation.moderator.ban` or `moderation.moderator.unban`
        pub event_type: String,
        /// RFC3339 formatted timestamp for events.
        pub event_timestamp: String,
        /// Returns the version of the endpoint.
        pub version: String,
        /// Returns `broadcaster_id`, `broadcaster_name`, `user_id`, `user_name`, and `expires_at`.
        pub event_data: HashMap<String, String>,
    }

    impl helix::Request for GetModeratorsEventsRequest {
        type Response = GetModeratorsEvents;

        const PATH: &'static str = "moderation/moderators/events";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];

        fn query(&self) -> Result<String, serde_urlencoded::ser::Error> {
            let mut qs = Vec::new();
            qs.push(("broadcaster_id", self.broadcaster_id.clone()));
            //: Option<helix::Cursor>,
            if let Some(ref after) = self.after {
                qs.push(("after", after.clone()))
            }
            let mut s = serde_urlencoded::to_string(qs)?;
            if !s.is_empty() && !self.user_id.is_empty() {
                s.push_str("&")
            }
            s.push_str(&helix::repeat_query("user_id", self.user_id.as_slice()));
            Ok(s)
        }
    }

    impl helix::RequestGet for GetModeratorsEventsRequest {}

    impl helix::Paginated for GetModeratorsEventsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}

/// Returns all banned and timed-out users in a channel.
/// [`get-banned-users`](https://dev.twitch.tv/docs/api/reference#get-banned-users)
pub mod get_banned_users {
    use super::*;

    /// Query Parameters for [Get Banned Users](super::get_banned_users)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    pub struct GetBannedUsersRequest {
        /// Provided `broadcaster_id` must match the `user_id` in the auth token.
        pub broadcaster_id: String,
        /// Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
        /// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
        /// Maximum: 100
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<String>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Banned Users](super::get_banned_users)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    pub struct GetBannedUsers {
        /// User ID of a user who has been banned.
        pub user_id: String,
        /// Display name of a user who has been banned.
        pub user_name: String,
        /// RFC3339 formatted timestamp for timeouts; empty string for bans.
        pub expires_at: Option<String>,
    }

    impl helix::Request for GetBannedUsersRequest {
        type Response = GetBannedUsers;

        const PATH: &'static str = "moderation/banned";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];

        fn query(&self) -> Result<String, serde_urlencoded::ser::Error> {
            let mut qs = Vec::new();
            qs.push(("broadcaster_id", self.broadcaster_id.clone()));
            //: Option<helix::Cursor>,
            if let Some(ref after) = self.after {
                qs.push(("after", after.clone()))
            }
            let mut s = serde_urlencoded::to_string(qs)?;
            if !s.is_empty() && !self.user_id.is_empty() {
                s.push_str("&")
            }
            s.push_str(&helix::repeat_query("user_id", self.user_id.as_slice()));
            Ok(s)
        }
    }

    impl helix::RequestGet for GetBannedUsersRequest {}

    impl helix::Paginated for GetBannedUsersRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}

/// Returns all banned and timed-out users in a channel.
/// [`get-banned-events`](https://dev.twitch.tv/docs/api/reference#get-banned-events)
pub mod get_banned_events {
    use super::*;
    use std::collections::HashMap;

    /// Query Parameters for [Get Banned Events](super::get_banned_events)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    pub struct GetBannedEventsRequest {
        /// Provided `broadcaster_id` must match the `user_id` in the auth token.
        pub broadcaster_id: String,
        /// Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
        /// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
        /// Maximum: 100
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<String>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. This applies only to queries specifying broadcaster_id or game_id. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<usize>,
    }

    /// Return Values for [Get Banned Events](super::get_banned_events)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    pub struct GetBannedEvents {
        /// Event ID
        pub id: String,
        /// Displays `moderation.user.ban` or `moderation.user.unban`
        pub event_type: String,
        /// RFC3339 formatted timestamp for events.
        pub event_timestamp: String,
        /// Returns the version of the endpoint.
        pub version: String,
        /// Returns `broadcaster_id`, `broadcaster_name`, `user_id`, `user_name`, and `expires_at`.
        pub event_data: HashMap<String, String>,
    }

    impl helix::Request for GetBannedEventsRequest {
        type Response = GetBannedEvents;

        const PATH: &'static str = "moderation/banned/events";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];

        fn query(&self) -> Result<String, serde_urlencoded::ser::Error> {
            let mut qs = Vec::new();
            qs.push(("broadcaster_id", self.broadcaster_id.clone()));
            //: Option<helix::Cursor>,
            if let Some(ref after) = self.after {
                qs.push(("after", after.clone()))
            }
            if let Some(ref first) = self.first {
                qs.push(("first", first.to_string()))
            }
            let mut s = serde_urlencoded::to_string(qs)?;
            if !s.is_empty() && !self.user_id.is_empty() {
                s.push_str("&")
            }
            s.push_str(&helix::repeat_query("user_id", self.user_id.as_slice()));
            Ok(s)
        }
    }

    impl helix::RequestGet for GetBannedEventsRequest {}

    impl helix::Paginated for GetBannedEventsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}
