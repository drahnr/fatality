use super::*;

#[derive(Debug, thiserror::Error)]
#[error("X")]
struct X;

impl Fatality for X {
    fn is_fatal(&self) -> bool {
        false
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Y")]
struct Y;

impl Fatality for Y {
    fn is_fatal(&self) -> bool {
        true
    }
}

#[fatality]
enum Acc {
    #[error("0")]
    Zero,

    #[error("X={0}")]
    A(#[source] X),

    #[fatal]
    #[error(transparent)]
    B(Y),

    #[fatal(forward)]
    #[error("X={0}")]
    Aaaaa(#[source] X),

    #[fatal(forward)]
    #[error(transparent)]
    Bbbbbb(Y),
}

#[test]
fn all_in_one() {
    assert_eq!(false, Fatality::is_fatal(&Acc::A(X)));
    assert_eq!(true, Fatality::is_fatal(&Acc::B(Y)));
    assert_eq!(false, Fatality::is_fatal(&Acc::Aaaaa(X)));
    assert_eq!(true, Fatality::is_fatal(&Acc::Bbbbbb(Y)));
    assert_eq!(false, Fatality::is_fatal(&Acc::Zero));
}
