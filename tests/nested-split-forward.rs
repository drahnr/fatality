// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use assert_matches::assert_matches;
use fatality::{fatality, Fatality, Split};

#[fatality(splitable)]
enum Inner {
    #[fatal]
    #[error("That's it.")]
    GameOver,

    #[error("Chuckle")]
    ChuckleOn,
}

#[fatality(splitable)]
enum Kaboom {
    #[fatal(forward)]
    #[error(transparent)]
    Iffy(Inner),

    #[error("Bobo")]
    Bobo,
}

fn game_over() -> Result<(), Kaboom> {
    Err(Kaboom::Iffy(Inner::GameOver))
}

fn laughable() -> Result<(), Kaboom> {
    Err(Kaboom::Iffy(Inner::ChuckleOn))
}

#[test]
fn main() {
    assert!(game_over().unwrap_err().is_fatal());
    assert_matches!(
        game_over().unwrap_err().split(),
        Err(FatalKaboom::Iffy(Inner::GameOver))
    );

    assert!(!laughable().unwrap_err().is_fatal());
    assert_matches!(
        laughable().unwrap_err().split(),
        Ok(JfyiKaboom::Iffy(Inner::ChuckleOn))
    );
}
