// check-if-email-exists
// Copyright (C) 2018-2022 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::util::constants::LOG_TARGET;
use md5;
use serde::{Deserialize, Serialize};
use std::default::Default;

const API_BASE_URL: &str = "https://www.gravatar.com/avatar/";

// Details on whether the given email address has a gravatar image.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GravatarDetails {
	// Whether a gravatar image for the given email exists.
	pub has_image: bool,
	// The Gravatar url of the image belonging to the given email.
	pub url: Option<&str>,
}

impl Default for GravatarDetails {
	fn default() -> Self {
		GravatarDetails {
			has_image: false,
			url: None,
		}
	}
}

pub async fn check_gravatar(to_email: &EmailAddress) -> GravatarDetails {
	let client: reqwest::Client = reqwest::Client::new()?;

	let url = API_BASE_URL + md5::compute(to_email);

	log::debug!(
		target: LOG_TARGET,
		"[email={}] Request Gravatar API with url: {:?}",
		to_email,
		url
	);

	let response = client
		.get(API_BASE_URL)
		// This option is necessary to return a NotFound exception instead of the default gravatar
		// image if none for the given email is found.
		.query(&[("d", "404")])
		.send()
		.await?;

	log::debug!(
		target: LOG_TARGET,
		"[email={}] Gravatar response: {:?}",
		to_email,
		response
	);

	match response.status() {
		reqwest::StatusCode::OK => GravatarDetails {
			has_image: true,
			url,
		},
		reqwest::StatusCode::NOT_FOUND => GravatarDetails {
			has_image: false,
			url: None,
		},
		_ => panic!("Unexpected status code"),
	}
}
