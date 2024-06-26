use super::*;
use assert_matches::assert_matches;
use quote::quote;

fn run_test(attr: TokenStream, input: TokenStream, expected: TokenStream) {
    let output = fatality2(attr, input);
    let output = output.to_string();
    println!(
        r##">>>>>>>>>>>>>>>>>>>
{}
>>>>>>>>>>>>>>>>>>>"##,
        output.as_str()
    );
    assert_eq!(output, expected.to_string(),);
}

mod component {
    use super::*;

    #[test]
    fn parse_attr_blank() {
        let input = TokenStream::new();
        let result = syn::parse2::<Attr>(input);
        assert_matches!(result, Ok(_));
    }

    #[test]
    fn parse_attr_splitable() {
        let input = quote! { splitable }.into();
        let result = syn::parse2::<Attr>(input);
        assert_matches!(result, Ok(_));
    }

    #[test]
    fn parse_attr_resmode_forward() {
        let input = quote! { forward }.into();
        let result = syn::parse2::<ResolutionMode>(input).unwrap();
        assert_matches!(result, ResolutionMode::Forward(..));
    }

    #[test]
    fn parse_attr_err() {
        let input = quote! { xyz }.into();
        let result = syn::parse2::<Attr>(input);
        assert_matches!(result, Err(_));
    }

    #[test]
    fn parse_full_attr() {
        let tokens = quote! { #[fatal(forward)] };
        let mut input = syn::parse::Parser::parse2(syn::Attribute::parse_outer, tokens).unwrap();
        let attr = input.pop().unwrap();
        let result = attr.parse_args::<ResolutionMode>();
        assert_matches!(result, Ok(ResolutionMode::Forward(..)));
    }
}

mod basic {
    use super::*;

    #[test]
    fn visibility_pub_crate_is_retained() {
        run_test(
            TokenStream::new(),
            quote! {
                pub(crate) enum Q {
                    #[fatal]
                    #[error(transparent)]
                    V(I),
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                pub(crate) enum Q {
                    #[error(transparent)]
                    V(I),
                }


                impl crate::Fatality for Q {
                    fn is_fatal(&self) -> bool {
                        match self {
                            Self::V(..) => true,
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn transparent_fatal_implicit() {
        run_test(
            TokenStream::new(),
            quote! {
                enum Q {
                    #[fatal]
                    #[error(transparent)]
                    V(I),
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                enum Q {
                    #[error(transparent)]
                    V(I),
                }


                impl crate::Fatality for Q {
                    fn is_fatal(&self) -> bool {
                        match self {
                            Self::V(..) => true,
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn transparent_fatal_fwd() {
        run_test(
            TokenStream::new(),
            quote! {
                enum Q {
                    #[fatal(forward)]
                    #[error(transparent)]
                    V(I),
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]

                enum Q {
                    #[error(transparent)]
                    V(I),
                }


                impl crate::Fatality for Q {
                    fn is_fatal(&self) -> bool {
                        match self {
                            Self::V(ref arg_0, ..) => <_ as crate::Fatality>::is_fatal(arg_0),
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn transparent_fatal_true() {
        run_test(
            TokenStream::new(),
            quote! {
                enum Q {
                    #[fatal(true)]
                    #[error(transparent)]
                    V(I),
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                enum Q {
                    #[error(transparent)]
                    V(I),
                }


                impl crate::Fatality for Q {
                    fn is_fatal(&self) -> bool {
                        match self {
                            Self::V(..) => true,
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn source_fatal() {
        run_test(
            TokenStream::new(),
            quote! {
                enum Q {
                    #[fatal(forward)]
                    #[error("DDDDDDDDDDDD")]
                    V(first, #[source] I),
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]

                enum Q {
                    #[error("DDDDDDDDDDDD")]
                    V(first, #[source] I),
                }


                impl crate::Fatality for Q {
                    fn is_fatal(&self) -> bool {
                        match self {
                            Self::V(_, ref arg_1, ..) => <_ as crate::Fatality>::is_fatal(arg_1),
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn full() {
        run_test(
            TokenStream::new(),
            quote! {
                enum Kaboom {
                    #[fatal(forward)]
                    #[error(transparent)]
                    // only one arg, that's ok, the first will be used
                    A(X),

                    #[fatal(forward)]
                    #[error("Bar")]
                    B(#[source] Y),

                    #[fatal(forward)]
                    #[error("zzzZZzZ")]
                    C {#[source] z: Z },

                    #[error("What?")]
                    What,


                    #[fatal]
                    #[error(transparent)]
                    O(P),
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                enum Kaboom {
                    #[error(transparent)]
                    A(X),
                    #[error("Bar")]
                    B(#[source] Y),
                    #[error("zzzZZzZ")]
                    C {#[source] z: Z },
                    #[error("What?")]
                    What,
                    #[error(transparent)]
                    O(P),
                }

                impl crate::Fatality for Kaboom {
                    fn is_fatal(&self) -> bool {
                        match self {
                            Self::A(ref arg_0, ..) => <_ as crate::Fatality>::is_fatal(arg_0),
                            Self::B(ref arg_0, ..) => <_ as crate::Fatality>::is_fatal(arg_0),
                            Self::C{ref z, ..} => <_ as crate::Fatality>::is_fatal(z),
                            Self::What => false,
                            Self::O(..) => true,
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn strukt_01_forward() {
        run_test(
            quote! {},
            quote! {
                #[fatal(forward)]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }

                impl crate :: Fatality for X {
                    fn is_fatal (& self) -> bool {
                        crate::Fatality::is_fatal(&self.inner)
                    }
                }
            },
        );
    }

    #[test]
    fn strukt_02_explicit_fatal() {
        run_test(
            quote! {},
            quote! {
                #[fatal(true)]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }

                impl crate :: Fatality for X {
                    fn is_fatal (& self) -> bool {
                        true
                    }
                }
            },
        );
    }

    #[test]
    fn strukt_03_implicit_fatal() {
        run_test(
            quote! {},
            quote! {
                #[fatal]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }

                impl crate :: Fatality for X {
                    fn is_fatal (& self) -> bool {
                        true
                    }
                }
            },
        );
    }
    #[test]
    fn strukt_03_explicit_jfyi() {
        run_test(
            quote! {},
            quote! {
                #[fatal(false)]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                #[error("Mission abort. Maybe?")]
                pub struct X {
                    #[source]
                    inner: InnerError,
                }

                impl crate :: Fatality for X {
                    fn is_fatal (& self) -> bool {
                        false
                    }
                }
            },
        );
    }
}

mod splitable {
    use super::*;

    #[test]
    fn simple() {
        run_test(
            quote! {
                splitable
            },
            quote! {
                enum Kaboom {
                    #[error("Eh?")]
                    Eh,

                    #[fatal]
                    #[error("Explosion")]
                    Explosion,
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                enum Kaboom {
                    #[error("Eh?")]
                    Eh,
                    #[error("Explosion")]
                    Explosion,
                }

                impl crate::Fatality for Kaboom {
                    fn is_fatal(&self) -> bool {
                        match self {
                            Self::Eh => false,
                            Self::Explosion => true,
                        }
                    }
                }

                impl ::std::convert::From<FatalKaboom> for Kaboom {
                    fn from(fatal: FatalKaboom) -> Self {
                        match fatal {
                            FatalKaboom::Explosion => Self::Explosion,
                        }
                    }
                }

                impl ::std::convert::From<JfyiKaboom> for Kaboom {
                    fn from(jfyi: JfyiKaboom) -> Self {
                        match jfyi {
                            JfyiKaboom::Eh => Self::Eh,
                        }
                    }
                }


                #[derive(crate::thiserror::Error, Debug)]
                enum FatalKaboom {
                    #[error("Explosion")]
                    Explosion
                }

                #[derive(crate::thiserror::Error, Debug)]
                enum JfyiKaboom {
                    #[error("Eh?")]
                    Eh
                }

                impl crate::Split for Kaboom {
                    type Fatal = FatalKaboom;
                    type Jfyi = JfyiKaboom;

                    fn split(self) -> ::std::result::Result<Self::Jfyi, Self::Fatal> {
                        match self {
                            // Fatal
                            Self::Explosion => Err(FatalKaboom::Explosion),
                            // JFYI
                            Self::Eh => Ok(JfyiKaboom::Eh),
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn strukt_cannot_split() {
        run_test(
            quote! {
                splitable
            },
            quote! {
                #[fatal]
                #[error("Cancelled")]
                pub struct X;
            },
            quote! {
                #[fatal]
                #[error("Cancelled")]
                pub struct X;
                ::core::compile_error! { "Cannot use `splitable` on a `struct`" }
            },
        );
    }

    #[test]
    fn regression() {
        run_test(
            quote! {
                splitable
            },
            quote! {
                pub enum X {
                    #[fatal]
                    #[error("Cancelled")]
                    Inner(Foo),
                }
            },
            quote! {
                #[derive(crate::thiserror::Error, Debug)]
                pub enum X {
                    #[error("Cancelled")]
                    Inner(Foo),
                }

                impl crate :: Fatality for X {
                    fn is_fatal (& self) -> bool {
                        match self {
                            Self :: Inner (..) => true ,
                        }
                    }
                }

                impl :: std :: convert :: From < FatalX > for X {
                    fn from (fatal : FatalX) -> Self {
                        match fatal {
                            FatalX :: Inner(arg_0) => Self :: Inner(arg_0),
                        }
                    }
                }

                impl :: std :: convert :: From < JfyiX > for X {
                    fn from (jfyi : JfyiX) -> Self {
                        match jfyi {

                        }
                    }
                }

                # [derive (crate :: thiserror :: Error , Debug)]
                pub enum FatalX {
                    #[error("Cancelled")]
                    Inner (Foo) }
                    #[derive (crate :: thiserror :: Error , Debug)]
                    pub enum JfyiX { }
                    impl crate :: Split for X {
                        type Fatal = FatalX ;
                        type Jfyi = JfyiX ;
                        fn split (self) -> :: std :: result :: Result < Self :: Jfyi , Self :: Fatal > {
                            match self {
                                Self::Inner(arg_0) => Err (FatalX :: Inner(arg_0)) ,
                            }
                        }
                    }
            },
        );
    }
}
