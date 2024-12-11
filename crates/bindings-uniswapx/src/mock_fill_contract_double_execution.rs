pub use mock_fill_contract_double_execution::*;
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
pub mod mock_fill_contract_double_execution {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor1"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor2"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("other"),
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
                                    name: ::std::borrow::ToOwned::to_owned("otherSignedOrder"),
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
    pub static MOCKFILLCONTRACTDOUBLEEXECUTION_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\r?8\x03\x80a\r?\x839\x81\x01`@\x81\x90Ra\0.\x91a\0`V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x91V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0[W_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\0qW_\x80\xFD[a\0z\x83a\0EV[\x91Pa\0\x88` \x84\x01a\0EV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x0C\x87a\0\xB8_9_a\x02\xD1\x01R_\x81\x81`b\x01Ra\x02\xA9\x01Ra\x0C\x87_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cP\xDF-@\x14a\08W\x80cX]\xA6(\x14a\0MW[_\x80\xFD[a\0Ka\0F6`\x04a\x03\xF0V[a\0`V[\0[a\0Ka\0[6`\x04a\x07\x91V[a\x01\x0BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x83\x83`@Q` \x01a\0\xAE\x91\x90a\t\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDA\x92\x91\x90a\n\x80V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\0\xF1W_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x03W=_\x80>=_\xFD[PPPPPPV[_[\x82Q\x81\x10\x15a\x02\x91W_[\x83\x82\x81Q\x81\x10a\x01*Wa\x01*a\n\xADV[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x02\x88W_\x84\x83\x81Q\x81\x10a\x01QWa\x01Qa\n\xADV[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x01nWa\x01na\n\xADV[` \x02` \x01\x01Q\x90Pa\x01\xAF\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90V[\x15a\x01\xC7Wa\x01\xC23\x82` \x01Qa\x03?V[a\x02\x7FV[\x80Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02YW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02}\x91\x90a\n\xDAV[P[P`\x01\x01a\x01\x18V[P`\x01\x01a\x01\rV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03;W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x82\x80` \x01\x90Q\x81\x01\x90a\x03\x1F\x91\x90a\x0BCV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDA\x91\x90a\x0B\xE4V[PPV[_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x03\x95W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x9AV[``\x91P[PP\x90P\x80a\x03\xD5W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[_`@\x82\x84\x03\x12\x15a\x03\xEAW_\x80\xFD[P\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x04\x01W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x18W_\x80\xFD[a\x04$\x86\x83\x87\x01a\x03\xDAV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x049W_\x80\xFD[Pa\x04F\x85\x82\x86\x01a\x03\xDAV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xA0Wa\x04\xA0a\x04PV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xA0Wa\x04\xA0a\x04PV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x10Wa\x05\x10a\x04PV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x051Wa\x051a\x04PV[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\\W_\x80\xFD[PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05xWa\x05xa\x04PV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x05\xB3W_\x80\xFD[\x815a\x05\xC6a\x05\xC1\x82a\x05_V[a\x04\xC9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x05\xDAW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x06\x06W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06*Wa\x06*a\x04PV[\x81`@R\x82\x93P\x845\x91Pa\x06>\x82a\x05;V[\x90\x82R` \x84\x015\x90a\x06P\x82a\x05;V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x06z\x82a\x05;V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x06\x93W_\x80\xFD[Pa\x06\xA0\x85\x82\x86\x01a\x05\xA4V[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x06\xBDW_\x80\xFD[a\x06\xC5a\x04}V[\x90P\x815a\x06\xD2\x81a\x05;V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x06\xFFW_\x80\xFD[\x815` a\x07\x0Fa\x05\xC1\x83a\x05\x18V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07-W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x07\x84W\x81\x81\x8A\x03\x12\x15a\x07GW_\x80\xFD[a\x07Oa\x04}V[\x815a\x07Z\x81a\x05;V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x07s\x81a\x05;V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x071V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x07\xA2W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB9W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07\xCCW_\x80\xFD[\x815` a\x07\xDCa\x05\xC1\x83a\x05\x18V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x07\xFAW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08\xE1W\x805\x86\x81\x11\x15a\x08\x14W_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\x08HW_\x80\xFD[a\x08Pa\x04\xA6V[\x86\x83\x015\x89\x81\x11\x15a\x08`W_\x80\xFD[a\x08n\x8F\x89\x83\x87\x01\x01a\x05\xF6V[\x82RPa\x08~\x8E`@\x85\x01a\x06\xADV[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\x08\x93W_\x80\xFD[a\x08\xA1\x8F\x89\x83\x87\x01\x01a\x06\xF0V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\x08\xB8W_\x80\xFD[a\x08\xC6\x8F\x89\x83\x87\x01\x01a\x05\xA4V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x07\xFEV[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x08\xF7W_\x80\xFD[Pa\x04F\x85\x82\x86\x01a\x05\xA4V[_\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\t7W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tVW_\x80\xFD[\x806\x03\x82\x13\x15a\tdW_\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_a\t\xBD\x82\x83a\t\x04V[`@\x85Ra\t\xCF`@\x86\x01\x82\x84a\tkV[\x91PPa\t\xDF` \x84\x01\x84a\t\x04V[\x85\x83\x03` \x87\x01Ra\t\xF2\x83\x82\x84a\tkV[\x96\x95PPPPPPV[` \x81R_a\n\x0E` \x83\x01\x84a\t\xB2V[\x93\x92PPPV[_[\x83\x81\x10\x15a\n/W\x81\x81\x01Q\x83\x82\x01R` \x01a\n\x17V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\nN\x81` \x86\x01` \x86\x01a\n\x15V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R_a\n\x92`@\x83\x01\x85a\t\xB2V[\x82\x81\x03` \x84\x01Ra\n\xA4\x81\x85a\n7V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\n\xEAW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\x0EW_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x0B\x08W_\x80\xFD[\x81Qa\x0B\x16a\x05\xC1\x82a\x05_V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B*W_\x80\xFD[a\x0B;\x82` \x83\x01` \x87\x01a\n\x15V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x0BSW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BjW_\x80\xFD[\x90\x83\x01\x90`@\x82\x86\x03\x12\x15a\x0B}W_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x0B\x98Wa\x0B\x98a\x04PV[`@R\x82Q\x82\x81\x11\x15a\x0B\xA9W_\x80\xFD[a\x0B\xB5\x87\x82\x86\x01a\n\xF9V[\x82RP` \x83\x01Q\x82\x81\x11\x15a\x0B\xC9W_\x80\xFD[a\x0B\xD5\x87\x82\x86\x01a\n\xF9V[` \x83\x01RP\x95\x94PPPPPV[`@\x81R_\x82Q`@\x80\x84\x01Ra\x0B\xFE`\x80\x84\x01\x82a\n7V[\x90P` \x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x83\x03\x01``\x85\x01Ra\x0C9\x82\x82a\n7V[\x84\x81\x03` \x95\x86\x01R_\x81R\x93\x90\x93\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xF4q\xBFk\xB1\x87\xF6b\xB2\xF5dh+\x8A9/\xBD\x99\xF3Z)s\xA99]\xCEU\xA4\xBE\t\xC4\xD3dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKFILLCONTRACTDOUBLEEXECUTION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cP\xDF-@\x14a\08W\x80cX]\xA6(\x14a\0MW[_\x80\xFD[a\0Ka\0F6`\x04a\x03\xF0V[a\0`V[\0[a\0Ka\0[6`\x04a\x07\x91V[a\x01\x0BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x83\x83`@Q` \x01a\0\xAE\x91\x90a\t\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDA\x92\x91\x90a\n\x80V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\0\xF1W_\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x03W=_\x80>=_\xFD[PPPPPPV[_[\x82Q\x81\x10\x15a\x02\x91W_[\x83\x82\x81Q\x81\x10a\x01*Wa\x01*a\n\xADV[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x02\x88W_\x84\x83\x81Q\x81\x10a\x01QWa\x01Qa\n\xADV[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x01nWa\x01na\n\xADV[` \x02` \x01\x01Q\x90Pa\x01\xAF\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90V[\x15a\x01\xC7Wa\x01\xC23\x82` \x01Qa\x03?V[a\x02\x7FV[\x80Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02YW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02}\x91\x90a\n\xDAV[P[P`\x01\x01a\x01\x18V[P`\x01\x01a\x01\rV[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03;W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x82\x80` \x01\x90Q\x81\x01\x90a\x03\x1F\x91\x90a\x0BCV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDA\x91\x90a\x0B\xE4V[PPV[_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x03\x95W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x9AV[``\x91P[PP\x90P\x80a\x03\xD5W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[_`@\x82\x84\x03\x12\x15a\x03\xEAW_\x80\xFD[P\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x04\x01W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x18W_\x80\xFD[a\x04$\x86\x83\x87\x01a\x03\xDAV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x049W_\x80\xFD[Pa\x04F\x85\x82\x86\x01a\x03\xDAV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xA0Wa\x04\xA0a\x04PV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xA0Wa\x04\xA0a\x04PV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x10Wa\x05\x10a\x04PV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x051Wa\x051a\x04PV[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\\W_\x80\xFD[PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05xWa\x05xa\x04PV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x05\xB3W_\x80\xFD[\x815a\x05\xC6a\x05\xC1\x82a\x05_V[a\x04\xC9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x05\xDAW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x06\x06W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06*Wa\x06*a\x04PV[\x81`@R\x82\x93P\x845\x91Pa\x06>\x82a\x05;V[\x90\x82R` \x84\x015\x90a\x06P\x82a\x05;V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x06z\x82a\x05;V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x06\x93W_\x80\xFD[Pa\x06\xA0\x85\x82\x86\x01a\x05\xA4V[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x06\xBDW_\x80\xFD[a\x06\xC5a\x04}V[\x90P\x815a\x06\xD2\x81a\x05;V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x06\xFFW_\x80\xFD[\x815` a\x07\x0Fa\x05\xC1\x83a\x05\x18V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07-W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x07\x84W\x81\x81\x8A\x03\x12\x15a\x07GW_\x80\xFD[a\x07Oa\x04}V[\x815a\x07Z\x81a\x05;V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x07s\x81a\x05;V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x071V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x07\xA2W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB9W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07\xCCW_\x80\xFD[\x815` a\x07\xDCa\x05\xC1\x83a\x05\x18V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x07\xFAW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08\xE1W\x805\x86\x81\x11\x15a\x08\x14W_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\x08HW_\x80\xFD[a\x08Pa\x04\xA6V[\x86\x83\x015\x89\x81\x11\x15a\x08`W_\x80\xFD[a\x08n\x8F\x89\x83\x87\x01\x01a\x05\xF6V[\x82RPa\x08~\x8E`@\x85\x01a\x06\xADV[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\x08\x93W_\x80\xFD[a\x08\xA1\x8F\x89\x83\x87\x01\x01a\x06\xF0V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\x08\xB8W_\x80\xFD[a\x08\xC6\x8F\x89\x83\x87\x01\x01a\x05\xA4V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x07\xFEV[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x08\xF7W_\x80\xFD[Pa\x04F\x85\x82\x86\x01a\x05\xA4V[_\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\t7W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tVW_\x80\xFD[\x806\x03\x82\x13\x15a\tdW_\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[_a\t\xBD\x82\x83a\t\x04V[`@\x85Ra\t\xCF`@\x86\x01\x82\x84a\tkV[\x91PPa\t\xDF` \x84\x01\x84a\t\x04V[\x85\x83\x03` \x87\x01Ra\t\xF2\x83\x82\x84a\tkV[\x96\x95PPPPPPV[` \x81R_a\n\x0E` \x83\x01\x84a\t\xB2V[\x93\x92PPPV[_[\x83\x81\x10\x15a\n/W\x81\x81\x01Q\x83\x82\x01R` \x01a\n\x17V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\nN\x81` \x86\x01` \x86\x01a\n\x15V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R_a\n\x92`@\x83\x01\x85a\t\xB2V[\x82\x81\x03` \x84\x01Ra\n\xA4\x81\x85a\n7V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\n\xEAW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\x0EW_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x0B\x08W_\x80\xFD[\x81Qa\x0B\x16a\x05\xC1\x82a\x05_V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B*W_\x80\xFD[a\x0B;\x82` \x83\x01` \x87\x01a\n\x15V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x0BSW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BjW_\x80\xFD[\x90\x83\x01\x90`@\x82\x86\x03\x12\x15a\x0B}W_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x0B\x98Wa\x0B\x98a\x04PV[`@R\x82Q\x82\x81\x11\x15a\x0B\xA9W_\x80\xFD[a\x0B\xB5\x87\x82\x86\x01a\n\xF9V[\x82RP` \x83\x01Q\x82\x81\x11\x15a\x0B\xC9W_\x80\xFD[a\x0B\xD5\x87\x82\x86\x01a\n\xF9V[` \x83\x01RP\x95\x94PPPPPV[`@\x81R_\x82Q`@\x80\x84\x01Ra\x0B\xFE`\x80\x84\x01\x82a\n7V[\x90P` \x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x83\x03\x01``\x85\x01Ra\x0C9\x82\x82a\n7V[\x84\x81\x03` \x95\x86\x01R_\x81R\x93\x90\x93\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xF4q\xBFk\xB1\x87\xF6b\xB2\xF5dh+\x8A9/\xBD\x99\xF3Z)s\xA99]\xCEU\xA4\xBE\t\xC4\xD3dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFILLCONTRACTDOUBLEEXECUTION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFillContractDoubleExecution<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFillContractDoubleExecution<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFillContractDoubleExecution<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFillContractDoubleExecution<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFillContractDoubleExecution<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFillContractDoubleExecution))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFillContractDoubleExecution<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFILLCONTRACTDOUBLEEXECUTION_ABI.clone(),
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
                MOCKFILLCONTRACTDOUBLEEXECUTION_ABI.clone(),
                MOCKFILLCONTRACTDOUBLEEXECUTION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `execute` (0x50df2d40) function
        pub fn execute(
            &self,
            order: SignedOrder,
            other: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 223, 45, 64], (order, other))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactorCallback` (0x585da628) function
        pub fn reactor_callback(
            &self,
            resolved_orders: ::std::vec::Vec<ResolvedOrder>,
            other_signed_order: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 93, 166, 40], (resolved_orders, other_signed_order))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFillContractDoubleExecution<M> {
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
    ///Container type for all input parameters for the `execute` function with signature `execute((bytes,bytes),(bytes,bytes))` and selector `0x50df2d40`
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
    #[ethcall(name = "execute", abi = "execute((bytes,bytes),(bytes,bytes))")]
    pub struct ExecuteCall {
        pub order: SignedOrder,
        pub other: SignedOrder,
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
        pub other_signed_order: ::ethers::core::types::Bytes,
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
    pub enum MockFillContractDoubleExecutionCalls {
        Execute(ExecuteCall),
        ReactorCallback(ReactorCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFillContractDoubleExecutionCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactorCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFillContractDoubleExecutionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReactorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockFillContractDoubleExecutionCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for MockFillContractDoubleExecutionCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall>
    for MockFillContractDoubleExecutionCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
}
