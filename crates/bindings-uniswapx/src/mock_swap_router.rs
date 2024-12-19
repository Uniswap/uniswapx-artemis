pub use mock_swap_router::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod mock_swap_router {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("wethAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SWAP_RATE_GRANULARITY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SWAP_RATE_GRANULARITY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WETH9"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH9"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exactInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exactInput"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ExactInputParams"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("results"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSwapRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSwapRate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapRate"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKSWAPROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@Ra'\x10_U4\x80\x15a\0\x14W_\x80\xFD[P`@Qa\x0Fp8\x03\x80a\x0Fp\x839\x81\x01`@\x81\x90Ra\x003\x91a\0DV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0qV[_` \x82\x84\x03\x12\x15a\0TW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0jW_\x80\xFD[\x93\x92PPPV[`\x80Qa\x0E\xE8a\0\x88_9_`x\x01Ra\x0E\xE8_\xF3\xFE`\x80`@R`\x046\x10a\0cW_5`\xE0\x1C\x80cwr\xE9\x04\x11a\0AW\x80cwr\xE9\x04\x14a\x01\x06W\x80c\xB8X\x18?\x14a\x01\x1BW\x80c\xF4\xCD\xE4i\x14a\x01:W_\x80\xFD[\x80cJ\xA4\xA4\xFC\x14a\0gW\x80cZ\xE4\x01\xDC\x14a\0\xC4W\x80ci\x85\x18\xE5\x14a\0\xE4W[_\x80\xFD[4\x80\x15a\0rW_\x80\xFD[Pa\0\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD7a\0\xD26`\x04a\n|V[a\x01ZV[`@Qa\0\xBB\x91\x90a\x0B_V[4\x80\x15a\0\xEFW_\x80\xFD[Pa\0\xF8_T\x81V[`@Q\x90\x81R` \x01a\0\xBBV[4\x80\x15a\x01\x11W_\x80\xFD[Pa\0\xF8a'\x10\x81V[4\x80\x15a\x01&W_\x80\xFD[Pa\0\xF8a\x0156`\x04a\x0B\xDFV[a\x02\xC5V[4\x80\x15a\x01EW_\x80\xFD[Pa\x01Xa\x01T6`\x04a\x0C\x1DV[_UV[\0[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01uWa\x01ua\x0C4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xA8W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\x93W\x90P[P\x90P_[\x82\x81\x10\x15a\x02\xBDW_\x800\x86\x86\x85\x81\x81\x10a\x01\xCAWa\x01\xCAa\x0CaV[\x90P` \x02\x81\x01\x90a\x01\xDC\x91\x90a\x0C\x8EV[`@Qa\x01\xEA\x92\x91\x90a\x0C\xF6V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x02\"W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02'V[``\x91P[P\x91P\x91P\x81a\x02\x95W`D\x81Q\x10\x15a\x02?W_\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02Y\x91\x90a\r\x05V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\x8C\x91\x90a\r\xCBV[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x84\x81Q\x81\x10a\x02\xA8Wa\x02\xA8a\x0CaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x01\xADV[P\x93\x92PPPV[_\x80a\x02\xD1\x83\x80a\x0C\x8EV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RP\x93\x94Pa\x03\x12\x92P\x84\x91Pa\x040\x90PV[PP\x90P[a\x03 \x82a\x04kV[\x15a\x035Wa\x03.\x82a\x04\xA4V[\x91Pa\x03\x17V[_a\x03?\x83a\x040V[P\x91PPa'\x10_T\x86`@\x015a\x03W\x91\x90a\x0E\nV[a\x03a\x91\x90a\x0E!V[\x93P\x84``\x015\x84\x10\x15a\x03\xD1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FToo little received\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\x03\xF7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x1630`@\x89\x015a\x04\xDBV[a\x04(a\x04\n`@\x87\x01` \x88\x01a\x0EYV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x86a\x05\xC9V[PPP\x91\x90PV[_\x80\x80a\x04=\x84\x82a\x06\x9AV[\x92Pa\x04J\x84`\x14a\x07\x9DV[\x90Pa\x04ba\x04[`\x03`\x14a\x0E\x8CV[\x85\x90a\x06\x9AV[\x91P\x91\x93\x90\x92PV[_a\x04x`\x03`\x14a\x0E\x8CV[`\x14a\x04\x85`\x03\x82a\x0E\x8CV[a\x04\x8F\x91\x90a\x0E\x8CV[a\x04\x99\x91\x90a\x0E\x8CV[\x82Q\x10\x15\x90P\x91\x90PV[``a\x04\xD5a\x04\xB5`\x03`\x14a\x0E\x8CV[a\x04\xC1`\x03`\x14a\x0E\x8CV[\x84Qa\x04\xCD\x91\x90a\x0E\x9FV[\x84\x91\x90a\x08\x90V[\x92\x91PPV[_`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` _`d\x83_\x8AZ\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\x05\xC2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[PPPPPV[_`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\x06\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[PPPPV[_\x81a\x06\xA7\x81`\x14a\x0E\x8CV[\x10\x15a\x07\x0FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FtoAddress_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\x07\x1A\x82`\x14a\x0E\x8CV[\x83Q\x10\x15a\x07\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FtoAddress_outOfBounds\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[P\x01` \x01Ql\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90V[_\x81a\x07\xAA\x81`\x03a\x0E\x8CV[\x10\x15a\x08\x12W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FtoUint24_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\x08\x1D\x82`\x03a\x0E\x8CV[\x83Q\x10\x15a\x08\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FtoUint24_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[P\x01`\x03\x01Q\x90V[``\x81a\x08\x9E\x81`\x1Fa\x0E\x8CV[\x10\x15a\t\x06W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[\x82a\t\x11\x83\x82a\x0E\x8CV[\x10\x15a\tyW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\t\x83\x82\x84a\x0E\x8CV[\x84Q\x10\x15a\t\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Fslice_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[``\x82\x15\x80\x15a\n\x0BW`@Q\x91P_\x82R` \x82\x01`@Ra\nsV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\nDW\x80Q\x83R` \x92\x83\x01\x92\x01a\n,V[PP\x85\x84R`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16`@RP[P\x94\x93PPPPV[_\x80_`@\x84\x86\x03\x12\x15a\n\x8EW_\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xACW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\n\xBFW_\x80\xFD[\x815\x81\x81\x11\x15a\n\xCDW_\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\n\xE1W_\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[_[\x83\x81\x10\x15a\x0B\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\xF6V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x0B-\x81` \x86\x01` \x86\x01a\n\xF4V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[_` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a\x0B\xD2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra\x0B\xC0\x85\x83Qa\x0B\x16V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x0B\x86V[P\x92\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a\x0B\xEFW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x05W_\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x0C\x16W_\x80\xFD[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0C-W_\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x0C\xC1W_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\xDBW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0C\xEFW_\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\r\x15W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r,W_\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\r?W_\x80\xFD[\x81Q\x81\x81\x11\x15a\rQWa\rQa\x0C4V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\r\x97Wa\r\x97a\x0C4V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\r\xAFW_\x80\xFD[a\r\xC0\x83` \x83\x01` \x88\x01a\n\xF4V[\x97\x96PPPPPPPV[` \x81R_a\x0C\x16` \x83\x01\x84a\x0B\x16V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xD5Wa\x04\xD5a\r\xDDV[_\x82a\x0ETW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x0EiW_\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\x16W_\x80\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\xD5Wa\x04\xD5a\r\xDDV[\x81\x81\x03\x81\x81\x11\x15a\x04\xD5Wa\x04\xD5a\r\xDDV\xFE\xA2dipfsX\"\x12 \xD4\xAC\x81?\n\xFE\xD8\x15\x14\"\x0E\x0F\x02\xEF\xCC\xF5L\x0C\x89\xFCI\xCF\xED\x17Q\xFF\x82\x92\xDD\x11k\xB8dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKSWAPROUTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0cW_5`\xE0\x1C\x80cwr\xE9\x04\x11a\0AW\x80cwr\xE9\x04\x14a\x01\x06W\x80c\xB8X\x18?\x14a\x01\x1BW\x80c\xF4\xCD\xE4i\x14a\x01:W_\x80\xFD[\x80cJ\xA4\xA4\xFC\x14a\0gW\x80cZ\xE4\x01\xDC\x14a\0\xC4W\x80ci\x85\x18\xE5\x14a\0\xE4W[_\x80\xFD[4\x80\x15a\0rW_\x80\xFD[Pa\0\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD7a\0\xD26`\x04a\n|V[a\x01ZV[`@Qa\0\xBB\x91\x90a\x0B_V[4\x80\x15a\0\xEFW_\x80\xFD[Pa\0\xF8_T\x81V[`@Q\x90\x81R` \x01a\0\xBBV[4\x80\x15a\x01\x11W_\x80\xFD[Pa\0\xF8a'\x10\x81V[4\x80\x15a\x01&W_\x80\xFD[Pa\0\xF8a\x0156`\x04a\x0B\xDFV[a\x02\xC5V[4\x80\x15a\x01EW_\x80\xFD[Pa\x01Xa\x01T6`\x04a\x0C\x1DV[_UV[\0[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01uWa\x01ua\x0C4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xA8W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\x93W\x90P[P\x90P_[\x82\x81\x10\x15a\x02\xBDW_\x800\x86\x86\x85\x81\x81\x10a\x01\xCAWa\x01\xCAa\x0CaV[\x90P` \x02\x81\x01\x90a\x01\xDC\x91\x90a\x0C\x8EV[`@Qa\x01\xEA\x92\x91\x90a\x0C\xF6V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x02\"W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02'V[``\x91P[P\x91P\x91P\x81a\x02\x95W`D\x81Q\x10\x15a\x02?W_\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02Y\x91\x90a\r\x05V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\x8C\x91\x90a\r\xCBV[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x84\x81Q\x81\x10a\x02\xA8Wa\x02\xA8a\x0CaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x01\xADV[P\x93\x92PPPV[_\x80a\x02\xD1\x83\x80a\x0C\x8EV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RP\x93\x94Pa\x03\x12\x92P\x84\x91Pa\x040\x90PV[PP\x90P[a\x03 \x82a\x04kV[\x15a\x035Wa\x03.\x82a\x04\xA4V[\x91Pa\x03\x17V[_a\x03?\x83a\x040V[P\x91PPa'\x10_T\x86`@\x015a\x03W\x91\x90a\x0E\nV[a\x03a\x91\x90a\x0E!V[\x93P\x84``\x015\x84\x10\x15a\x03\xD1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FToo little received\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\x03\xF7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x1630`@\x89\x015a\x04\xDBV[a\x04(a\x04\n`@\x87\x01` \x88\x01a\x0EYV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x86a\x05\xC9V[PPP\x91\x90PV[_\x80\x80a\x04=\x84\x82a\x06\x9AV[\x92Pa\x04J\x84`\x14a\x07\x9DV[\x90Pa\x04ba\x04[`\x03`\x14a\x0E\x8CV[\x85\x90a\x06\x9AV[\x91P\x91\x93\x90\x92PV[_a\x04x`\x03`\x14a\x0E\x8CV[`\x14a\x04\x85`\x03\x82a\x0E\x8CV[a\x04\x8F\x91\x90a\x0E\x8CV[a\x04\x99\x91\x90a\x0E\x8CV[\x82Q\x10\x15\x90P\x91\x90PV[``a\x04\xD5a\x04\xB5`\x03`\x14a\x0E\x8CV[a\x04\xC1`\x03`\x14a\x0E\x8CV[\x84Qa\x04\xCD\x91\x90a\x0E\x9FV[\x84\x91\x90a\x08\x90V[\x92\x91PPV[_`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` _`d\x83_\x8AZ\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\x05\xC2W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[PPPPPV[_`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91PP\x80a\x06\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[PPPPV[_\x81a\x06\xA7\x81`\x14a\x0E\x8CV[\x10\x15a\x07\x0FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FtoAddress_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\x07\x1A\x82`\x14a\x0E\x8CV[\x83Q\x10\x15a\x07\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FtoAddress_outOfBounds\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[P\x01` \x01Ql\x01\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04\x90V[_\x81a\x07\xAA\x81`\x03a\x0E\x8CV[\x10\x15a\x08\x12W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7FtoUint24_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\x08\x1D\x82`\x03a\x0E\x8CV[\x83Q\x10\x15a\x08\x87W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FtoUint24_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[P\x01`\x03\x01Q\x90V[``\x81a\x08\x9E\x81`\x1Fa\x0E\x8CV[\x10\x15a\t\x06W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[\x82a\t\x11\x83\x82a\x0E\x8CV[\x10\x15a\tyW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7Fslice_overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[a\t\x83\x82\x84a\x0E\x8CV[\x84Q\x10\x15a\t\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Fslice_outOfBounds\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x8CV[``\x82\x15\x80\x15a\n\x0BW`@Q\x91P_\x82R` \x82\x01`@Ra\nsV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\nDW\x80Q\x83R` \x92\x83\x01\x92\x01a\n,V[PP\x85\x84R`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16`@RP[P\x94\x93PPPPV[_\x80_`@\x84\x86\x03\x12\x15a\n\x8EW_\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xACW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\n\xBFW_\x80\xFD[\x815\x81\x81\x11\x15a\n\xCDW_\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\n\xE1W_\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[_[\x83\x81\x10\x15a\x0B\x0EW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\xF6V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x0B-\x81` \x86\x01` \x86\x01a\n\xF4V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[_` \x80\x83\x01` \x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a\x0B\xD2W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x88\x86\x03\x01\x84Ra\x0B\xC0\x85\x83Qa\x0B\x16V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x0B\x86V[P\x92\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a\x0B\xEFW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x05W_\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x0C\x16W_\x80\xFD[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0C-W_\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x0C\xC1W_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\xDBW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0C\xEFW_\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\r\x15W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r,W_\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\r?W_\x80\xFD[\x81Q\x81\x81\x11\x15a\rQWa\rQa\x0C4V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\r\x97Wa\r\x97a\x0C4V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\r\xAFW_\x80\xFD[a\r\xC0\x83` \x83\x01` \x88\x01a\n\xF4V[\x97\x96PPPPPPPV[` \x81R_a\x0C\x16` \x83\x01\x84a\x0B\x16V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xD5Wa\x04\xD5a\r\xDDV[_\x82a\x0ETW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x0EiW_\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\x16W_\x80\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\xD5Wa\x04\xD5a\r\xDDV[\x81\x81\x03\x81\x81\x11\x15a\x04\xD5Wa\x04\xD5a\r\xDDV\xFE\xA2dipfsX\"\x12 \xD4\xAC\x81?\n\xFE\xD8\x15\x14\"\x0E\x0F\x02\xEF\xCC\xF5L\x0C\x89\xFCI\xCF\xED\x17Q\xFF\x82\x92\xDD\x11k\xB8dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKSWAPROUTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockSwapRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockSwapRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockSwapRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockSwapRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockSwapRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockSwapRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockSwapRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKSWAPROUTER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOCKSWAPROUTER_ABI.clone(),
                MOCKSWAPROUTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `SWAP_RATE_GRANULARITY` (0x7772e904) function
        pub fn swap_rate_granularity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 114, 233, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInput` (0xb858183f) function
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 88, 24, 63], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0x5ae401dc) function
        pub fn multicall(
            &self,
            p0: ::ethers::core::types::U256,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([90, 228, 1, 220], (p0, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSwapRate` (0xf4cde469) function
        pub fn set_swap_rate(
            &self,
            new_rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 205, 228, 105], new_rate)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapRate` (0x698518e5) function
        pub fn swap_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([105, 133, 24, 229], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockSwapRouter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `SWAP_RATE_GRANULARITY` function with signature `SWAP_RATE_GRANULARITY()` and selector `0x7772e904`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "SWAP_RATE_GRANULARITY", abi = "SWAP_RATE_GRANULARITY()")]
    pub struct SwapRateGranularityCall;
    ///Container type for all input parameters for the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
    ///Container type for all input parameters for the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256))` and selector `0xb858183f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exactInput", abi = "exactInput((bytes,address,uint256,uint256))")]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(uint256,bytes[])` and selector `0x5ae401dc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "multicall", abi = "multicall(uint256,bytes[])")]
    pub struct MulticallCall {
        pub p0: ::ethers::core::types::U256,
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `setSwapRate` function with signature `setSwapRate(uint256)` and selector `0xf4cde469`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setSwapRate", abi = "setSwapRate(uint256)")]
    pub struct SetSwapRateCall {
        pub new_rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapRate` function with signature `swapRate()` and selector `0x698518e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "swapRate", abi = "swapRate()")]
    pub struct SwapRateCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MockSwapRouterCalls {
        SwapRateGranularity(SwapRateGranularityCall),
        Weth9(Weth9Call),
        ExactInput(ExactInputCall),
        Multicall(MulticallCall),
        SetSwapRate(SetSwapRateCall),
        SwapRate(SwapRateCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockSwapRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SwapRateGranularityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapRateGranularity(decoded));
            }
            if let Ok(decoded) = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded) = <ExactInputCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExactInput(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <SetSwapRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSwapRate(decoded));
            }
            if let Ok(decoded) = <SwapRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapRate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockSwapRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SwapRateGranularity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSwapRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockSwapRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SwapRateGranularity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSwapRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapRate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SwapRateGranularityCall> for MockSwapRouterCalls {
        fn from(value: SwapRateGranularityCall) -> Self {
            Self::SwapRateGranularity(value)
        }
    }
    impl ::core::convert::From<Weth9Call> for MockSwapRouterCalls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<ExactInputCall> for MockSwapRouterCalls {
        fn from(value: ExactInputCall) -> Self {
            Self::ExactInput(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for MockSwapRouterCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<SetSwapRateCall> for MockSwapRouterCalls {
        fn from(value: SetSwapRateCall) -> Self {
            Self::SetSwapRate(value)
        }
    }
    impl ::core::convert::From<SwapRateCall> for MockSwapRouterCalls {
        fn from(value: SwapRateCall) -> Self {
            Self::SwapRate(value)
        }
    }
    ///Container type for all return fields from the `SWAP_RATE_GRANULARITY` function with signature `SWAP_RATE_GRANULARITY()` and selector `0x7772e904`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapRateGranularityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Weth9Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256))` and selector `0xb858183f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExactInputReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `multicall` function with signature `multicall(uint256,bytes[])` and selector `0x5ae401dc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `swapRate` function with signature `swapRate()` and selector `0x698518e5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapRateReturn(pub ::ethers::core::types::U256);
}
