pub use mock_fill_contract::*;
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
pub mod mock_fill_contract {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder"),
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
                    ::std::borrow::ToOwned::to_owned("executeBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orders"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder[]"),
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
                    ::std::borrow::ToOwned::to_owned("reactorCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reactorCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resolvedOrders"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NativeTransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NativeTransferFailed",
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
    pub static MOCKFILLCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x0C\x848\x03\x80a\x0C\x84\x839\x81\x01`@\x81\x90Ra\0.\x91a\0?V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0lV[_` \x82\x84\x03\x12\x15a\0OW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0eW_\x80\xFD[\x93\x92PPPV[`\x80Qa\x0B\xECa\0\x98_9_\x81\x81`\xBB\x01R\x81\x81a\x01`\x01R\x81\x81a\x02s\x01Ra\x02\xE0\x01Ra\x0B\xEC_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80c\rz\x16\xC3\x14a\0CW\x80c?b\x19.\x14a\0XW\x80cX]\xA6(\x14a\0kW[_\x80\xFD[a\0Va\0Q6`\x04a\x04(V[a\0~V[\0[a\0Va\0f6`\x04a\x04\x97V[a\x01#V[a\0Va\0y6`\x04a\x08\x0EV[a\x01\xC5V[`@Q\x7F\x13\xFBr\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13\xFBr\xC7\x90a\0\xF2\x90\x85\x90\x85\x90`\x04\x01a\n\x83V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\tW_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x1BW=_\x80>=_\xFD[PPPPPPV[`@Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\r3X\x84\x90a\x01\x95\x90\x84\x90`\x04\x01a\x0BBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xACW_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xBEW=_\x80>=_\xFD[PPPPPV[_[\x82Q\x81\x10\x15a\x03\x8DW_[\x83\x82\x81Q\x81\x10a\x01\xE4Wa\x01\xE4a\x0BjV[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x03\x84W_\x84\x83\x81Q\x81\x10a\x02\x0BWa\x02\x0Ba\x0BjV[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x02(Wa\x02(a\x0BjV[` \x02` \x01\x01Q\x90Pa\x02i\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90V[\x15a\x02\xA1Wa\x02\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01Qa\x03\x92V[a\x03{V[\x80Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x83\x01R\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03UW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03y\x91\x90a\x0B\x97V[P[P`\x01\x01a\x01\xD2V[P`\x01\x01a\x01\xC7V[PPPV[_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x03\xE8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\xEDV[``\x91P[PP\x90P\x80a\x03\x8DW`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80` \x83\x85\x03\x12\x15a\x049W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04PW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04cW_\x80\xFD[\x815\x81\x81\x11\x15a\x04qW_\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x04\x85W_\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[_` \x82\x84\x03\x12\x15a\x04\xA7W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xBDW_\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x04\xCEW_\x80\xFD[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05%Wa\x05%a\x04\xD5V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05%Wa\x05%a\x04\xD5V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x95Wa\x05\x95a\x04\xD5V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\xB6Wa\x05\xB6a\x04\xD5V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xE1W_\x80\xFD[PV[_\x82`\x1F\x83\x01\x12a\x05\xF3W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\rWa\x06\ra\x04\xD5V[a\x06>` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x05NV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06RW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x06~W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06\xA2Wa\x06\xA2a\x04\xD5V[\x81`@R\x82\x93P\x845\x91Pa\x06\xB6\x82a\x05\xC0V[\x90\x82R` \x84\x015\x90a\x06\xC8\x82a\x05\xC0V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x06\xF2\x82a\x05\xC0V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x07\x0BW_\x80\xFD[Pa\x07\x18\x85\x82\x86\x01a\x05\xE4V[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x075W_\x80\xFD[a\x07=a\x05\x02V[\x90P\x815a\x07J\x81a\x05\xC0V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x07wW_\x80\xFD[\x815` a\x07\x8Ca\x07\x87\x83a\x05\x9DV[a\x05NV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07\xAAW_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\x01W\x81\x81\x8A\x03\x12\x15a\x07\xC4W_\x80\xFD[a\x07\xCCa\x05\x02V[\x815a\x07\xD7\x81a\x05\xC0V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x07\xF0\x81a\x05\xC0V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x07\xAEV[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x08\x1FW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x086W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08IW_\x80\xFD[\x815` a\x08Ya\x07\x87\x83a\x05\x9DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x08wW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t^W\x805\x86\x81\x11\x15a\x08\x91W_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\x08\xC5W_\x80\xFD[a\x08\xCDa\x05+V[\x86\x83\x015\x89\x81\x11\x15a\x08\xDDW_\x80\xFD[a\x08\xEB\x8F\x89\x83\x87\x01\x01a\x06nV[\x82RPa\x08\xFB\x8E`@\x85\x01a\x07%V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\t\x10W_\x80\xFD[a\t\x1E\x8F\x89\x83\x87\x01\x01a\x07hV[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\t5W_\x80\xFD[a\tC\x8F\x89\x83\x87\x01\x01a\x05\xE4V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x08{V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\ttW_\x80\xFD[Pa\t\x81\x85\x82\x86\x01a\x05\xE4V[\x91PP\x92P\x92\x90PV[_\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\t\xBEW_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDDW_\x80\xFD[\x806\x03\x82\x13\x15a\t\xEBW_\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_a\nD\x82\x83a\t\x8BV[`@\x85Ra\nV`@\x86\x01\x82\x84a\t\xF2V[\x91PPa\nf` \x84\x01\x84a\t\x8BV[\x85\x83\x03` \x87\x01Ra\ny\x83\x82\x84a\t\xF2V[\x96\x95PPPPPPV[`@\x80\x82R\x81\x01\x82\x90R_```\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83[\x86\x81\x10\x15a\x0B!W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x86\x85\x03\x01\x83R\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x896\x03\x01\x81\x12a\n\xFFW_\x80\xFD[a\x0B\x0B\x85\x8A\x83\x01a\n9V[\x94PP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a\n\x9FV[PPP\x82\x81\x03` \x84\x01Ra\x0B9\x81_\x81R` \x01\x90V[\x95\x94PPPPPV[`@\x81R_a\x0BT`@\x83\x01\x84a\n9V[\x82\x81\x03` \x93\x84\x01R_\x81R\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0B\xA7W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\xCEW_\x80\xFD\xFE\xA2dipfsX\"\x12 #a\x94\"\x95\x18\x98U:j\x9B\xCB\x95\x1B `\xED\x0C\x1Am)^ke\xF9\x80\xF4\xD7*\xF1L\x06dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKFILLCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80c\rz\x16\xC3\x14a\0CW\x80c?b\x19.\x14a\0XW\x80cX]\xA6(\x14a\0kW[_\x80\xFD[a\0Va\0Q6`\x04a\x04(V[a\0~V[\0[a\0Va\0f6`\x04a\x04\x97V[a\x01#V[a\0Va\0y6`\x04a\x08\x0EV[a\x01\xC5V[`@Q\x7F\x13\xFBr\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13\xFBr\xC7\x90a\0\xF2\x90\x85\x90\x85\x90`\x04\x01a\n\x83V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\tW_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x1BW=_\x80>=_\xFD[PPPPPPV[`@Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\r3X\x84\x90a\x01\x95\x90\x84\x90`\x04\x01a\x0BBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xACW_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xBEW=_\x80>=_\xFD[PPPPPV[_[\x82Q\x81\x10\x15a\x03\x8DW_[\x83\x82\x81Q\x81\x10a\x01\xE4Wa\x01\xE4a\x0BjV[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x03\x84W_\x84\x83\x81Q\x81\x10a\x02\x0BWa\x02\x0Ba\x0BjV[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x02(Wa\x02(a\x0BjV[` \x02` \x01\x01Q\x90Pa\x02i\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90V[\x15a\x02\xA1Wa\x02\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82` \x01Qa\x03\x92V[a\x03{V[\x80Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x83\x01R\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03UW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03y\x91\x90a\x0B\x97V[P[P`\x01\x01a\x01\xD2V[P`\x01\x01a\x01\xC7V[PPPV[_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x03\xE8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\xEDV[``\x91P[PP\x90P\x80a\x03\x8DW`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80` \x83\x85\x03\x12\x15a\x049W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04PW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04cW_\x80\xFD[\x815\x81\x81\x11\x15a\x04qW_\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x04\x85W_\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[_` \x82\x84\x03\x12\x15a\x04\xA7W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xBDW_\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x04\xCEW_\x80\xFD[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05%Wa\x05%a\x04\xD5V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05%Wa\x05%a\x04\xD5V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x95Wa\x05\x95a\x04\xD5V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\xB6Wa\x05\xB6a\x04\xD5V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xE1W_\x80\xFD[PV[_\x82`\x1F\x83\x01\x12a\x05\xF3W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\rWa\x06\ra\x04\xD5V[a\x06>` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x05NV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06RW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x06~W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06\xA2Wa\x06\xA2a\x04\xD5V[\x81`@R\x82\x93P\x845\x91Pa\x06\xB6\x82a\x05\xC0V[\x90\x82R` \x84\x015\x90a\x06\xC8\x82a\x05\xC0V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x06\xF2\x82a\x05\xC0V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x07\x0BW_\x80\xFD[Pa\x07\x18\x85\x82\x86\x01a\x05\xE4V[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x075W_\x80\xFD[a\x07=a\x05\x02V[\x90P\x815a\x07J\x81a\x05\xC0V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x07wW_\x80\xFD[\x815` a\x07\x8Ca\x07\x87\x83a\x05\x9DV[a\x05NV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07\xAAW_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\x01W\x81\x81\x8A\x03\x12\x15a\x07\xC4W_\x80\xFD[a\x07\xCCa\x05\x02V[\x815a\x07\xD7\x81a\x05\xC0V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x07\xF0\x81a\x05\xC0V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x07\xAEV[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x08\x1FW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x086W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08IW_\x80\xFD[\x815` a\x08Ya\x07\x87\x83a\x05\x9DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x08wW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t^W\x805\x86\x81\x11\x15a\x08\x91W_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\x08\xC5W_\x80\xFD[a\x08\xCDa\x05+V[\x86\x83\x015\x89\x81\x11\x15a\x08\xDDW_\x80\xFD[a\x08\xEB\x8F\x89\x83\x87\x01\x01a\x06nV[\x82RPa\x08\xFB\x8E`@\x85\x01a\x07%V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\t\x10W_\x80\xFD[a\t\x1E\x8F\x89\x83\x87\x01\x01a\x07hV[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\t5W_\x80\xFD[a\tC\x8F\x89\x83\x87\x01\x01a\x05\xE4V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x08{V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\ttW_\x80\xFD[Pa\t\x81\x85\x82\x86\x01a\x05\xE4V[\x91PP\x92P\x92\x90PV[_\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\t\xBEW_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDDW_\x80\xFD[\x806\x03\x82\x13\x15a\t\xEBW_\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_a\nD\x82\x83a\t\x8BV[`@\x85Ra\nV`@\x86\x01\x82\x84a\t\xF2V[\x91PPa\nf` \x84\x01\x84a\t\x8BV[\x85\x83\x03` \x87\x01Ra\ny\x83\x82\x84a\t\xF2V[\x96\x95PPPPPPV[`@\x80\x82R\x81\x01\x82\x90R_```\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83[\x86\x81\x10\x15a\x0B!W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x86\x85\x03\x01\x83R\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x896\x03\x01\x81\x12a\n\xFFW_\x80\xFD[a\x0B\x0B\x85\x8A\x83\x01a\n9V[\x94PP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a\n\x9FV[PPP\x82\x81\x03` \x84\x01Ra\x0B9\x81_\x81R` \x01\x90V[\x95\x94PPPPPV[`@\x81R_a\x0BT`@\x83\x01\x84a\n9V[\x82\x81\x03` \x93\x84\x01R_\x81R\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0B\xA7W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\xCEW_\x80\xFD\xFE\xA2dipfsX\"\x12 #a\x94\"\x95\x18\x98U:j\x9B\xCB\x95\x1B `\xED\x0C\x1Am)^ke\xF9\x80\xF4\xD7*\xF1L\x06dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFILLCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFillContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFillContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFillContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFillContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFillContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFillContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFillContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFILLCONTRACT_ABI.clone(),
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
                MOCKFILLCONTRACT_ABI.clone(),
                MOCKFILLCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `execute` (0x3f62192e) function
        pub fn execute(
            &self,
            order: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 98, 25, 46], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0x0d7a16c3) function
        pub fn execute_batch(
            &self,
            orders: ::std::vec::Vec<SignedOrder>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 122, 22, 195], orders)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactorCallback` (0x585da628) function
        pub fn reactor_callback(
            &self,
            resolved_orders: ::std::vec::Vec<ResolvedOrder>,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 93, 166, 40], (resolved_orders, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFillContract<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NativeTransferFailed` with signature `NativeTransferFailed()` and selector `0xf4b3b1bc`
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
    #[etherror(name = "NativeTransferFailed", abi = "NativeTransferFailed()")]
    pub struct NativeTransferFailed;
    ///Container type for all input parameters for the `execute` function with signature `execute((bytes,bytes))` and selector `0x3f62192e`
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
    #[ethcall(name = "execute", abi = "execute((bytes,bytes))")]
    pub struct ExecuteCall {
        pub order: SignedOrder,
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch((bytes,bytes)[])` and selector `0x0d7a16c3`
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
    #[ethcall(name = "executeBatch", abi = "executeBatch((bytes,bytes)[])")]
    pub struct ExecuteBatchCall {
        pub orders: ::std::vec::Vec<SignedOrder>,
    }
    ///Container type for all input parameters for the `reactorCallback` function with signature `reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],bytes)` and selector `0x585da628`
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
        name = "reactorCallback",
        abi = "reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],bytes)"
    )]
    pub struct ReactorCallbackCall {
        pub resolved_orders: ::std::vec::Vec<ResolvedOrder>,
        pub p1: ::ethers::core::types::Bytes,
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
    pub enum MockFillContractCalls {
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        ReactorCallback(ReactorCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFillContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBatch(decoded));
            }
            if let Ok(decoded) = <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactorCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFillContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockFillContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for MockFillContractCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for MockFillContractCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall> for MockFillContractCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
}
