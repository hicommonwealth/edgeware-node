// Copyright 2018-2020 Commonwealth Labs, Inc.
// This file is part of Edgeware.

// Edgeware is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Edgeware is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate Node CLI

#![warn(missing_docs)]

use sc_cli::VersionInfo;

fn main() -> sc_cli::Result<()> {
	let version = VersionInfo {
		name: "Edgeware Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "edgeware",
		author: "Commonwealth Labs <hello@commonwealth.im>",
		description: "Edgeware node",
		support_url: "https://github.com/hicommonwealth/edgeware-node/issues/new",
		copyright_start_year: 2018,
	};

	edgeware_cli::run(std::env::args(), version)
}
