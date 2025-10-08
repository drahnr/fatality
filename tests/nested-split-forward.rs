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

#[fatality(splitable)]
#[error(transparent)]
struct TransparentStructWrapper {
    #[from]
    source: Kaboom,
}

#[fatality(splitable)]
#[error("Struct wrapper")]
struct StructWrapper {
    source: Kaboom,
    other_field: (),
}

impl From<Kaboom> for StructWrapper {
    fn from(source: Kaboom) -> Self {
        Self {
            source,
            other_field: (),
        }
    }
}

#[fatality(splitable)]
#[error(transparent)]
struct TransparentTupleStructWrapper(Kaboom);

#[fatality(splitable)]
#[error("Tuple struct wrapper")]
struct TupleStructWrapper(#[source] Kaboom, ());

impl From<Kaboom> for TupleStructWrapper {
    fn from(source: Kaboom) -> Self {
        Self(source, ())
    }
}

#[fatality]
#[error(transparent)]
#[fatal(forward)]
struct ForwardWrapper(Kaboom);

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

    assert!(TransparentStructWrapper::from(game_over().unwrap_err()).is_fatal());
    assert_matches!(
        TransparentStructWrapper::from(game_over().unwrap_err()).split(),
        Err(FatalTransparentStructWrapper {
            source: FatalKaboom::Iffy(Inner::GameOver)
        })
    );

    assert!(!TransparentStructWrapper::from(laughable().unwrap_err()).is_fatal());
    assert_matches!(
        TransparentStructWrapper::from(laughable().unwrap_err()).split(),
        Ok(JfyiTransparentStructWrapper {
            source: JfyiKaboom::Iffy(Inner::ChuckleOn)
        })
    );

    assert!(StructWrapper::from(game_over().unwrap_err()).is_fatal());
    assert_matches!(
        StructWrapper::from(game_over().unwrap_err()).split(),
        Err(FatalStructWrapper {
            source: FatalKaboom::Iffy(Inner::GameOver),
            other_field: (),
        })
    );

    assert!(!StructWrapper::from(laughable().unwrap_err()).is_fatal());
    assert_matches!(
        StructWrapper::from(laughable().unwrap_err()).split(),
        Ok(JfyiStructWrapper {
            source: JfyiKaboom::Iffy(Inner::ChuckleOn),
            other_field: (),
        })
    );

    assert!(TransparentTupleStructWrapper(game_over().unwrap_err()).is_fatal());
    assert_matches!(
        TransparentTupleStructWrapper(game_over().unwrap_err()).split(),
        Err(FatalTransparentTupleStructWrapper(FatalKaboom::Iffy(
            Inner::GameOver
        )))
    );

    assert!(!TransparentTupleStructWrapper(laughable().unwrap_err()).is_fatal());
    assert_matches!(
        TransparentTupleStructWrapper(laughable().unwrap_err()).split(),
        Ok(JfyiTransparentTupleStructWrapper(JfyiKaboom::Iffy(
            Inner::ChuckleOn
        )))
    );

    assert!(TupleStructWrapper::from(game_over().unwrap_err()).is_fatal());
    assert_matches!(
        TupleStructWrapper::from(game_over().unwrap_err()).split(),
        Err(FatalTupleStructWrapper(
            FatalKaboom::Iffy(Inner::GameOver),
            ()
        ))
    );

    assert!(!TupleStructWrapper::from(laughable().unwrap_err()).is_fatal());
    assert_matches!(
        TupleStructWrapper::from(laughable().unwrap_err()).split(),
        Ok(JfyiTupleStructWrapper(
            JfyiKaboom::Iffy(Inner::ChuckleOn),
            ()
        ))
    );

    assert!(ForwardWrapper(game_over().unwrap_err()).is_fatal());
    assert!(!ForwardWrapper(laughable().unwrap_err()).is_fatal());
}
