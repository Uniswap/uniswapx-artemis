pub use mock_exclusivity_lib::*;
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
pub mod mock_exclusivity_lib {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("handleExclusiveOverrideBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "handleExclusiveOverrideBlock",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusivityEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "exclusivityOverrideBps",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumberish"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("handleExclusiveOverrideTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "handleExclusiveOverrideTimestamp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusivityEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "exclusivityOverrideBps",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasFillingRights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasFillingRights"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusivityEnd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentPosition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NoExclusiveOverride"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoExclusiveOverride",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKEXCLUSIVITYLIB_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\nw\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80c\xCA\xFFN\xF6\x14a\0CW\x80c\xDF$\x81\x18\x14a\0lW\x80c\xF8\x81>\xE7\x14a\0\x7FW[_\x80\xFD[a\0Va\0Q6`\x04a\x06\xF1V[a\0\xA2V[`@Qa\0c\x91\x90a\x08\x16V[`@Q\x80\x91\x03\x90\xF3[a\0Va\0z6`\x04a\t?V[a\x01%V[a\0\x92a\0\x8D6`\x04a\t\xA4V[a\x01\xAAV[`@Q\x90\x15\x15\x81R` \x01a\0cV[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\x1C\x85\x85\x85\x85a\x01\xBEV[P\x92\x93\x92PPPV[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\xA0\x86\x86\x86\x86\x86a\x01\xD1V[P\x93\x94\x93PPPPV[_a\x01\xB6\x84\x84\x84a\x01\xE5V[\x94\x93PPPPV[a\x01\xCB\x84\x84\x84\x84Ba\x020V[PPPPV[a\x01\xDE\x85\x85\x85\x85\x85a\x020V[PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15\x80a\x02\x08WP\x82\x82\x11[\x80a\x01\xB6WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x14\x90P\x93\x92PPPV[a\x02;\x84\x84\x83a\x01\xE5V[a\x01\xDEW\x81a\x02vW`@Q\x7F\xB9\xEC\x1E\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x85\x01Q_[\x81Q\x81\x10\x15a\x02\xD0W_\x82\x82\x81Q\x81\x10a\x02\x99Wa\x02\x99a\t\xD6V[` \x02` \x01\x01Q\x90Pa\x02\xC2\x85a'\x10a\x02\xB4\x91\x90a\n\x03V[` \x83\x01Q\x90a'\x10a\x02\xD9V[` \x90\x91\x01R`\x01\x01a\x02}V[PPPPPPPV[_\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x03\x0CW_\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03kWa\x03ka\x03\x1BV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03kWa\x03ka\x03\x1BV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xDBWa\x03\xDBa\x03\x1BV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x04W_\x80\xFD[PV[_\x82`\x1F\x83\x01\x12a\x04\x16W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x040Wa\x040a\x03\x1BV[a\x04a` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x03\x94V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04uW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x04\xA1W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04\xC5Wa\x04\xC5a\x03\x1BV[\x81`@R\x82\x93P\x845\x91Pa\x04\xD9\x82a\x03\xE3V[\x90\x82R` \x84\x015\x90a\x04\xEB\x82a\x03\xE3V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x05\x15\x82a\x03\xE3V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x05.W_\x80\xFD[Pa\x05;\x85\x82\x86\x01a\x04\x07V[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x05XW_\x80\xFD[a\x05`a\x03HV[\x90P\x815a\x05m\x81a\x03\xE3V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x05\x9AW_\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\xB6Wa\x05\xB6a\x03\x1BV[a\x05\xC4\x81\x83`\x05\x1B\x01a\x03\x94V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05\xE2W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x069W\x81\x81\x8A\x03\x12\x15a\x05\xFCW_\x80\xFD[a\x06\x04a\x03HV[\x815a\x06\x0F\x81a\x03\xE3V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x06(\x81a\x03\xE3V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x05\xE6V[P\x90\x97\x96PPPPPPPV[_`\xE0\x82\x84\x03\x12\x15a\x06VW_\x80\xFD[a\x06^a\x03qV[\x90P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06wW_\x80\xFD[a\x06\x83\x85\x83\x86\x01a\x04\x91V[\x83Ra\x06\x92\x85` \x86\x01a\x05HV[` \x84\x01R`\x80\x84\x015\x91P\x80\x82\x11\x15a\x06\xAAW_\x80\xFD[a\x06\xB6\x85\x83\x86\x01a\x05\x8BV[`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a\x06\xCEW_\x80\xFD[Pa\x06\xDB\x84\x82\x85\x01a\x04\x07V[``\x83\x01RP`\xC0\x82\x015`\x80\x82\x01R\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07\x04W_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x1AW_\x80\xFD[a\x07&\x87\x82\x88\x01a\x06FV[\x94PP` \x85\x015a\x077\x81a\x03\xE3V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x07pW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x07TV[P_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x08\x0BW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\xC0V[P\x94\x95\x94PPPPPV[` \x81R_\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x08\x91a\x01\xC0\x84\x01\x82a\x07LV[\x90P` \x84\x01Qa\x08\xCF`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\t\n\x83\x83a\x07\xADV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\t(\x82\x82a\x07LV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\tSW_\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tiW_\x80\xFD[a\tu\x88\x82\x89\x01a\x06FV[\x95PP` \x86\x015a\t\x86\x81a\x03\xE3V[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x81\x015\x92`\x80\x90\x91\x015\x91PV[_\x80_``\x84\x86\x03\x12\x15a\t\xB6W_\x80\xFD[\x835a\t\xC1\x81a\x03\xE3V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\n;W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12  \xEF\x1D\xCA+;Sn\x12F\x0C\xD8y\xAE4lZ\x99\x8F\x86\x8B\xF9+~)\xB6\xFBon3\xA9\x87dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKEXCLUSIVITYLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80c\xCA\xFFN\xF6\x14a\0CW\x80c\xDF$\x81\x18\x14a\0lW\x80c\xF8\x81>\xE7\x14a\0\x7FW[_\x80\xFD[a\0Va\0Q6`\x04a\x06\xF1V[a\0\xA2V[`@Qa\0c\x91\x90a\x08\x16V[`@Q\x80\x91\x03\x90\xF3[a\0Va\0z6`\x04a\t?V[a\x01%V[a\0\x92a\0\x8D6`\x04a\t\xA4V[a\x01\xAAV[`@Q\x90\x15\x15\x81R` \x01a\0cV[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\x1C\x85\x85\x85\x85a\x01\xBEV[P\x92\x93\x92PPPV[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\xA0\x86\x86\x86\x86\x86a\x01\xD1V[P\x93\x94\x93PPPPV[_a\x01\xB6\x84\x84\x84a\x01\xE5V[\x94\x93PPPPV[a\x01\xCB\x84\x84\x84\x84Ba\x020V[PPPPV[a\x01\xDE\x85\x85\x85\x85\x85a\x020V[PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15\x80a\x02\x08WP\x82\x82\x11[\x80a\x01\xB6WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x14\x90P\x93\x92PPPV[a\x02;\x84\x84\x83a\x01\xE5V[a\x01\xDEW\x81a\x02vW`@Q\x7F\xB9\xEC\x1E\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x85\x01Q_[\x81Q\x81\x10\x15a\x02\xD0W_\x82\x82\x81Q\x81\x10a\x02\x99Wa\x02\x99a\t\xD6V[` \x02` \x01\x01Q\x90Pa\x02\xC2\x85a'\x10a\x02\xB4\x91\x90a\n\x03V[` \x83\x01Q\x90a'\x10a\x02\xD9V[` \x90\x91\x01R`\x01\x01a\x02}V[PPPPPPPV[_\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x03\x0CW_\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03kWa\x03ka\x03\x1BV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03kWa\x03ka\x03\x1BV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xDBWa\x03\xDBa\x03\x1BV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x04W_\x80\xFD[PV[_\x82`\x1F\x83\x01\x12a\x04\x16W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x040Wa\x040a\x03\x1BV[a\x04a` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x03\x94V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04uW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x04\xA1W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04\xC5Wa\x04\xC5a\x03\x1BV[\x81`@R\x82\x93P\x845\x91Pa\x04\xD9\x82a\x03\xE3V[\x90\x82R` \x84\x015\x90a\x04\xEB\x82a\x03\xE3V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x05\x15\x82a\x03\xE3V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x05.W_\x80\xFD[Pa\x05;\x85\x82\x86\x01a\x04\x07V[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x05XW_\x80\xFD[a\x05`a\x03HV[\x90P\x815a\x05m\x81a\x03\xE3V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x05\x9AW_\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\xB6Wa\x05\xB6a\x03\x1BV[a\x05\xC4\x81\x83`\x05\x1B\x01a\x03\x94V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05\xE2W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x069W\x81\x81\x8A\x03\x12\x15a\x05\xFCW_\x80\xFD[a\x06\x04a\x03HV[\x815a\x06\x0F\x81a\x03\xE3V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x06(\x81a\x03\xE3V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x05\xE6V[P\x90\x97\x96PPPPPPPV[_`\xE0\x82\x84\x03\x12\x15a\x06VW_\x80\xFD[a\x06^a\x03qV[\x90P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06wW_\x80\xFD[a\x06\x83\x85\x83\x86\x01a\x04\x91V[\x83Ra\x06\x92\x85` \x86\x01a\x05HV[` \x84\x01R`\x80\x84\x015\x91P\x80\x82\x11\x15a\x06\xAAW_\x80\xFD[a\x06\xB6\x85\x83\x86\x01a\x05\x8BV[`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a\x06\xCEW_\x80\xFD[Pa\x06\xDB\x84\x82\x85\x01a\x04\x07V[``\x83\x01RP`\xC0\x82\x015`\x80\x82\x01R\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x07\x04W_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x1AW_\x80\xFD[a\x07&\x87\x82\x88\x01a\x06FV[\x94PP` \x85\x015a\x077\x81a\x03\xE3V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x07pW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x07TV[P_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x08\x0BW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\xC0V[P\x94\x95\x94PPPPPV[` \x81R_\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x08\x91a\x01\xC0\x84\x01\x82a\x07LV[\x90P` \x84\x01Qa\x08\xCF`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\t\n\x83\x83a\x07\xADV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\t(\x82\x82a\x07LV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\tSW_\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tiW_\x80\xFD[a\tu\x88\x82\x89\x01a\x06FV[\x95PP` \x86\x015a\t\x86\x81a\x03\xE3V[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x81\x015\x92`\x80\x90\x91\x015\x91PV[_\x80_``\x84\x86\x03\x12\x15a\t\xB6W_\x80\xFD[\x835a\t\xC1\x81a\x03\xE3V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\n;W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12  \xEF\x1D\xCA+;Sn\x12F\x0C\xD8y\xAE4lZ\x99\x8F\x86\x8B\xF9+~)\xB6\xFBon3\xA9\x87dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKEXCLUSIVITYLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockExclusivityLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockExclusivityLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockExclusivityLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockExclusivityLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockExclusivityLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockExclusivityLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockExclusivityLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKEXCLUSIVITYLIB_ABI.clone(),
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
                MOCKEXCLUSIVITYLIB_ABI.clone(),
                MOCKEXCLUSIVITYLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `handleExclusiveOverrideBlock` (0xdf248118) function
        pub fn handle_exclusive_override_block(
            &self,
            order: ResolvedOrder,
            exclusive: ::ethers::core::types::Address,
            exclusivity_end: ::ethers::core::types::U256,
            exclusivity_override_bps: ::ethers::core::types::U256,
            block_numberish: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash(
                    [223, 36, 129, 24],
                    (
                        order,
                        exclusive,
                        exclusivity_end,
                        exclusivity_override_bps,
                        block_numberish,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handleExclusiveOverrideTimestamp` (0xcaff4ef6) function
        pub fn handle_exclusive_override_timestamp(
            &self,
            order: ResolvedOrder,
            exclusive: ::ethers::core::types::Address,
            exclusivity_end: ::ethers::core::types::U256,
            exclusivity_override_bps: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash(
                    [202, 255, 78, 246],
                    (order, exclusive, exclusivity_end, exclusivity_override_bps),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasFillingRights` (0xf8813ee7) function
        pub fn has_filling_rights(
            &self,
            exclusive: ::ethers::core::types::Address,
            exclusivity_end: ::ethers::core::types::U256,
            current_position: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [248, 129, 62, 231],
                    (exclusive, exclusivity_end, current_position),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockExclusivityLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NoExclusiveOverride` with signature `NoExclusiveOverride()` and selector `0xb9ec1e96`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NoExclusiveOverride", abi = "NoExclusiveOverride()")]
    pub struct NoExclusiveOverride;
    ///Container type for all input parameters for the `handleExclusiveOverrideBlock` function with signature `handleExclusiveOverrideBlock(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256,uint256)` and selector `0xdf248118`
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
    #[ethcall(
        name = "handleExclusiveOverrideBlock",
        abi = "handleExclusiveOverrideBlock(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256,uint256)"
    )]
    pub struct HandleExclusiveOverrideBlockCall {
        pub order: ResolvedOrder,
        pub exclusive: ::ethers::core::types::Address,
        pub exclusivity_end: ::ethers::core::types::U256,
        pub exclusivity_override_bps: ::ethers::core::types::U256,
        pub block_numberish: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `handleExclusiveOverrideTimestamp` function with signature `handleExclusiveOverrideTimestamp(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)` and selector `0xcaff4ef6`
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
    #[ethcall(
        name = "handleExclusiveOverrideTimestamp",
        abi = "handleExclusiveOverrideTimestamp(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)"
    )]
    pub struct HandleExclusiveOverrideTimestampCall {
        pub order: ResolvedOrder,
        pub exclusive: ::ethers::core::types::Address,
        pub exclusivity_end: ::ethers::core::types::U256,
        pub exclusivity_override_bps: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `hasFillingRights` function with signature `hasFillingRights(address,uint256,uint256)` and selector `0xf8813ee7`
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
    #[ethcall(
        name = "hasFillingRights",
        abi = "hasFillingRights(address,uint256,uint256)"
    )]
    pub struct HasFillingRightsCall {
        pub exclusive: ::ethers::core::types::Address,
        pub exclusivity_end: ::ethers::core::types::U256,
        pub current_position: ::ethers::core::types::U256,
    }
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
    pub enum MockExclusivityLibCalls {
        HandleExclusiveOverrideBlock(HandleExclusiveOverrideBlockCall),
        HandleExclusiveOverrideTimestamp(HandleExclusiveOverrideTimestampCall),
        HasFillingRights(HasFillingRightsCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockExclusivityLibCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HandleExclusiveOverrideBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HandleExclusiveOverrideBlock(decoded));
            }
            if let Ok(decoded) = <HandleExclusiveOverrideTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HandleExclusiveOverrideTimestamp(decoded));
            }
            if let Ok(decoded) = <HasFillingRightsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasFillingRights(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockExclusivityLibCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HandleExclusiveOverrideBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HandleExclusiveOverrideTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasFillingRights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockExclusivityLibCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HandleExclusiveOverrideBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HandleExclusiveOverrideTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasFillingRights(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HandleExclusiveOverrideBlockCall>
    for MockExclusivityLibCalls {
        fn from(value: HandleExclusiveOverrideBlockCall) -> Self {
            Self::HandleExclusiveOverrideBlock(value)
        }
    }
    impl ::core::convert::From<HandleExclusiveOverrideTimestampCall>
    for MockExclusivityLibCalls {
        fn from(value: HandleExclusiveOverrideTimestampCall) -> Self {
            Self::HandleExclusiveOverrideTimestamp(value)
        }
    }
    impl ::core::convert::From<HasFillingRightsCall> for MockExclusivityLibCalls {
        fn from(value: HasFillingRightsCall) -> Self {
            Self::HasFillingRights(value)
        }
    }
    ///Container type for all return fields from the `handleExclusiveOverrideBlock` function with signature `handleExclusiveOverrideBlock(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256,uint256)` and selector `0xdf248118`
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
    pub struct HandleExclusiveOverrideBlockReturn(pub ResolvedOrder);
    ///Container type for all return fields from the `handleExclusiveOverrideTimestamp` function with signature `handleExclusiveOverrideTimestamp(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)` and selector `0xcaff4ef6`
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
    pub struct HandleExclusiveOverrideTimestampReturn(pub ResolvedOrder);
    ///Container type for all return fields from the `hasFillingRights` function with signature `hasFillingRights(address,uint256,uint256)` and selector `0xf8813ee7`
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
    pub struct HasFillingRightsReturn {
        pub pass: bool,
    }
}
