//! Declarative annotations for `fatal` or `jfyi` error variants.
//!
//! Expand `#[fatal]` annotations on `enum` variants into
//! two additional `enum`s that can be converted back, or
//! the original split into two. Determination of fatality
//! can also be forwarded to an inner error that implements
//! the `trait Fatality`.
//!
//! Stands on the shoulders of `thiserror`.
//!
//! Note: At this time crate `fatality` also has to import `thiserror`
//! as part of the manifest until
//! <https://github.com/dtolnay/thiserror/issues/167>
//! is resolved.

pub use fatality_proc_macro::fatality;
pub use thiserror;

/// Determine the fatality of an error.
pub trait Fatality: std::error::Error + std::fmt::Debug {
    /// Returns `true` if the error variant is _fatal_
    /// or `false` if it is more of a informational error.
    fn is_fatal(&self) -> bool;
}

/// Allows to split an error into two types - a fatal
/// and a informational enum error type, that can be further consumed.
pub trait Split: std::error::Error + std::fmt::Debug {
    type Jfyi: std::error::Error + Send + Sync + 'static;
    type Fatal: std::error::Error + Send + Sync + 'static;

    /// Split the error into it's fatal and non-fatal variants.
    ///
    /// `Ok(jfyi)` contains a enum representing all non-fatal variants, `Err(fatal)`
    /// contains all fatal variants.
    ///
    /// Attention: If the type is splitable, it must _not_ use any `forward`ed finality
    /// evaluations, or it must be splitable up the point where no more `forward` annotations
    /// were used.
    fn split(self) -> std::result::Result<Self::Jfyi, Self::Fatal>;
}

/// Converts a flat, yet `splitable` error into a nested `Result<Result<_,Jfyi>, Fatal>`
/// error type.
pub trait Nested<T, E: Split>
where
    Self: Sized,
{
    /// Convert into a nested error rather than a flat one, commonly for direct handling.
    fn into_nested(
        self,
    ) -> std::result::Result<std::result::Result<T, <E as Split>::Jfyi>, <E as Split>::Fatal>;
}

impl<T, E: Split> Nested<T, E> for std::result::Result<T, E> {
    fn into_nested(
        self,
    ) -> std::result::Result<std::result::Result<T, <E as Split>::Jfyi>, <E as Split>::Fatal> {
        match self {
            Ok(t) => Ok(Ok(t)),
            Err(e) => match e.split() {
                Ok(jfyi) => Ok(Err(jfyi)),
                Err(fatal) => Err(fatal),
            },
        }
    }
}

#[cfg(test)]
mod tests;
