pub use order_quoter::*;
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
pub mod order_quoter {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IReactor"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OrdersLengthIncorrect"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OrdersLengthIncorrect",
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
    pub static ORDERQUOTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\r\xA6\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80cA\xD8\x8Di\x14a\0CW\x80cX]\xA6(\x14a\0lW\x80cvq\xD0{\x14a\0\x81W[_\x80\xFD[a\0Va\0Q6`\x04a\x04\xA8V[a\0\xC2V[`@Qa\0c\x91\x90a\x05\xDCV[`@Q\x80\x91\x03\x90\xF3[a\0\x7Fa\0z6`\x04a\x08\xC4V[a\x02\x18V[\0[a\0\x9Da\0\x8F6`\x04a\n*V[`@\x81\x81\x01Q\x90\x91\x01\x01Q\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0cV[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`@\x80\x84\x01Q\x84\x01\x01Q`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x86\x90R\x82Q\x90\x81\x01\x83R_\x81R\x91Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92c\r3X\x84\x92a\x01\xAA\x92\x91`\x04\x01a\ndV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xC1W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xD2WP`\x01[a\x02\x12W=\x80\x80\x15a\x01\xFFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02\x04V[``\x91P[Pa\x02\x0E\x81a\x02\x97V[\x91PP[\x92\x91PPV[\x81Q`\x01\x14a\x02SW`@Q\x7F\x06\xEE\x98x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82_\x81Q\x81\x10a\x02fWa\x02fa\n\xD7V[` \x02` \x01\x01Q`@Q` \x01a\x02~\x91\x90a\x05\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q\x81` \x01\xFD[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x80\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90Ra\x01 \x84\x01\x83\x90R``a\x01@\x85\x01\x81\x90R\x91\x84R\x84Q\x80\x83\x01\x86R\x83\x81R` \x80\x82\x01\x85\x90R\x81\x87\x01\x85\x90R\x85\x01R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82Q\x90\x91\x11\x15a\x03\x12W\x81Q\x82` \x01\xFD[\x81\x80` \x01\x90Q\x81\x01\x90a\x02\x12\x91\x90a\x0C\xA5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03vWa\x03va\x03&V[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03vWa\x03va\x03&V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03vWa\x03va\x03&V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\tWa\x04\ta\x03&V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04*Wa\x04*a\x03&V[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x04eW_\x80\xFD[\x815a\x04xa\x04s\x82a\x04\x11V[a\x03\xC2V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\x8CW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x04\xB9W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xD0W_\x80\xFD[a\x04\xDC\x86\x83\x87\x01a\x04VV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x04\xF1W_\x80\xFD[Pa\x04\xFE\x85\x82\x86\x01a\x04VV[\x91PP\x92P\x92\x90PV[_[\x83\x81\x10\x15a\x05\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\nV[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x05A\x81` \x86\x01` \x86\x01a\x05\x08V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x05\xD1W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\x86V[P\x94\x95\x94PPPPPV[` \x81R_\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x06Wa\x01\xC0\x84\x01\x82a\x05*V[\x90P` \x84\x01Qa\x06\x95`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x06\xD0\x83\x83a\x05sV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x06\xEE\x82\x82a\x05*V[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x1EWa\x07\x1Ea\x03&V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07IW_\x80\xFD[PV[_`\xC0\x82\x84\x03\x12\x15a\x07\\W_\x80\xFD[a\x07da\x03SV[\x90P\x815a\x07q\x81a\x07(V[\x81R` \x82\x015a\x07\x81\x81a\x07(V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a\x07\xAA\x81a\x07(V[`\x80\x82\x01R`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xC8W_\x80\xFD[a\x07\xD4\x84\x82\x85\x01a\x04VV[`\xA0\x83\x01RP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x07\xF0W_\x80\xFD[a\x07\xF8a\x03|V[\x90P\x815a\x08\x05\x81a\x07(V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x082W_\x80\xFD[\x815` a\x08Ba\x04s\x83a\x07\x05V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08`W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\xB7W\x81\x81\x8A\x03\x12\x15a\x08zW_\x80\xFD[a\x08\x82a\x03|V[\x815a\x08\x8D\x81a\x07(V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x08\xA6\x81a\x07(V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08dV[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x08\xD5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\xECW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08\xFFW_\x80\xFD[\x815` a\t\x0Fa\x04s\x83a\x07\x05V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\t-W_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\n\x14W\x805\x86\x81\x11\x15a\tGW_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\t{W_\x80\xFD[a\t\x83a\x03\x9FV[\x86\x83\x015\x89\x81\x11\x15a\t\x93W_\x80\xFD[a\t\xA1\x8F\x89\x83\x87\x01\x01a\x07LV[\x82RPa\t\xB1\x8E`@\x85\x01a\x07\xE0V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\t\xC6W_\x80\xFD[a\t\xD4\x8F\x89\x83\x87\x01\x01a\x08#V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\t\xEBW_\x80\xFD[a\t\xF9\x8F\x89\x83\x87\x01\x01a\x04VV[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\t1V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x04\xF1W_\x80\xFD[_` \x82\x84\x03\x12\x15a\n:W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nPW_\x80\xFD[a\n\\\x84\x82\x85\x01a\x04VV[\x94\x93PPPPV[`@\x81R_\x83Q`@\x80\x84\x01Ra\n~`\x80\x84\x01\x82a\x05*V[\x90P` \x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x83\x03\x01``\x85\x01Ra\n\xB9\x82\x82a\x05*V[\x91PP\x82\x81\x03` \x84\x01Ra\n\xCE\x81\x85a\x05*V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\x0B\x13W_\x80\xFD[\x81Qa\x0B!a\x04s\x82a\x04\x11V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B5W_\x80\xFD[a\n\\\x82` \x83\x01` \x87\x01a\x05\x08V[_`\xC0\x82\x84\x03\x12\x15a\x0BVW_\x80\xFD[a\x0B^a\x03SV[\x90P\x81Qa\x0Bk\x81a\x07(V[\x81R` \x82\x01Qa\x0B{\x81a\x07(V[\x80` \x83\x01RP`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa\x0B\xA4\x81a\x07(V[`\x80\x82\x01R`\xA0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xC2W_\x80\xFD[a\x07\xD4\x84\x82\x85\x01a\x0B\x04V[_``\x82\x84\x03\x12\x15a\x0B\xDEW_\x80\xFD[a\x0B\xE6a\x03|V[\x90P\x81Qa\x0B\xF3\x81a\x07(V[\x80\x82RP` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x0C W_\x80\xFD[\x81Q` a\x0C0a\x04s\x83a\x07\x05V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x0CNW_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\xB7W\x81\x81\x8A\x03\x12\x15a\x0ChW_\x80\xFD[a\x0Cpa\x03|V[\x81Qa\x0C{\x81a\x07(V[\x81R\x81\x86\x01Q\x86\x82\x01R`@\x80\x83\x01Qa\x0C\x94\x81a\x07(V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x0CRV[_` \x82\x84\x03\x12\x15a\x0C\xB5W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xCCW_\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x0C\xDFW_\x80\xFD[a\x0C\xE7a\x03\x9FV[\x82Q\x82\x81\x11\x15a\x0C\xF5W_\x80\xFD[a\r\x01\x87\x82\x86\x01a\x0BFV[\x82RPa\r\x11\x86` \x85\x01a\x0B\xCEV[` \x82\x01R`\x80\x83\x01Q\x82\x81\x11\x15a\r'W_\x80\xFD[a\r3\x87\x82\x86\x01a\x0C\x11V[`@\x83\x01RP`\xA0\x83\x01Q\x82\x81\x11\x15a\rJW_\x80\xFD[a\rV\x87\x82\x86\x01a\x0B\x04V[``\x83\x01RP`\xC0\x92\x90\x92\x01Q`\x80\x83\x01RP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xFE\xB5\x9B\x92\xB026\x10F\xE1\x83\xA9\x95\xBD\xE7X(\x86\xEC\x94\x98\x99\x0F{vt\xB9\xC2\x17\xF8\xB1kdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static ORDERQUOTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80cA\xD8\x8Di\x14a\0CW\x80cX]\xA6(\x14a\0lW\x80cvq\xD0{\x14a\0\x81W[_\x80\xFD[a\0Va\0Q6`\x04a\x04\xA8V[a\0\xC2V[`@Qa\0c\x91\x90a\x05\xDCV[`@Q\x80\x91\x03\x90\xF3[a\0\x7Fa\0z6`\x04a\x08\xC4V[a\x02\x18V[\0[a\0\x9Da\0\x8F6`\x04a\n*V[`@\x81\x81\x01Q\x90\x91\x01\x01Q\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0cV[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`@\x80\x84\x01Q\x84\x01\x01Q`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x86\x90R\x82Q\x90\x81\x01\x83R_\x81R\x91Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92c\r3X\x84\x92a\x01\xAA\x92\x91`\x04\x01a\ndV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xC1W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xD2WP`\x01[a\x02\x12W=\x80\x80\x15a\x01\xFFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02\x04V[``\x91P[Pa\x02\x0E\x81a\x02\x97V[\x91PP[\x92\x91PPV[\x81Q`\x01\x14a\x02SW`@Q\x7F\x06\xEE\x98x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82_\x81Q\x81\x10a\x02fWa\x02fa\n\xD7V[` \x02` \x01\x01Q`@Q` \x01a\x02~\x91\x90a\x05\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q\x81` \x01\xFD[`@\x80Qa\x01`\x81\x01\x82R_`\xA0\x82\x01\x81\x81R`\xC0\x80\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90Ra\x01 \x84\x01\x83\x90R``a\x01@\x85\x01\x81\x90R\x91\x84R\x84Q\x80\x83\x01\x86R\x83\x81R` \x80\x82\x01\x85\x90R\x81\x87\x01\x85\x90R\x85\x01R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82Q\x90\x91\x11\x15a\x03\x12W\x81Q\x82` \x01\xFD[\x81\x80` \x01\x90Q\x81\x01\x90a\x02\x12\x91\x90a\x0C\xA5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03vWa\x03va\x03&V[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03vWa\x03va\x03&V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03vWa\x03va\x03&V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\tWa\x04\ta\x03&V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04*Wa\x04*a\x03&V[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x04eW_\x80\xFD[\x815a\x04xa\x04s\x82a\x04\x11V[a\x03\xC2V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\x8CW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x04\xB9W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xD0W_\x80\xFD[a\x04\xDC\x86\x83\x87\x01a\x04VV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x04\xF1W_\x80\xFD[Pa\x04\xFE\x85\x82\x86\x01a\x04VV[\x91PP\x92P\x92\x90PV[_[\x83\x81\x10\x15a\x05\"W\x81\x81\x01Q\x83\x82\x01R` \x01a\x05\nV[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x05A\x81` \x86\x01` \x86\x01a\x05\x08V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x05\xD1W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\x86V[P\x94\x95\x94PPPPPV[` \x81R_\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x06Wa\x01\xC0\x84\x01\x82a\x05*V[\x90P` \x84\x01Qa\x06\x95`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x06\xD0\x83\x83a\x05sV[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x06\xEE\x82\x82a\x05*V[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x1EWa\x07\x1Ea\x03&V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07IW_\x80\xFD[PV[_`\xC0\x82\x84\x03\x12\x15a\x07\\W_\x80\xFD[a\x07da\x03SV[\x90P\x815a\x07q\x81a\x07(V[\x81R` \x82\x015a\x07\x81\x81a\x07(V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a\x07\xAA\x81a\x07(V[`\x80\x82\x01R`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xC8W_\x80\xFD[a\x07\xD4\x84\x82\x85\x01a\x04VV[`\xA0\x83\x01RP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x07\xF0W_\x80\xFD[a\x07\xF8a\x03|V[\x90P\x815a\x08\x05\x81a\x07(V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x082W_\x80\xFD[\x815` a\x08Ba\x04s\x83a\x07\x05V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08`W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\xB7W\x81\x81\x8A\x03\x12\x15a\x08zW_\x80\xFD[a\x08\x82a\x03|V[\x815a\x08\x8D\x81a\x07(V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x08\xA6\x81a\x07(V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08dV[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x08\xD5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\xECW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08\xFFW_\x80\xFD[\x815` a\t\x0Fa\x04s\x83a\x07\x05V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\t-W_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\n\x14W\x805\x86\x81\x11\x15a\tGW_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\t{W_\x80\xFD[a\t\x83a\x03\x9FV[\x86\x83\x015\x89\x81\x11\x15a\t\x93W_\x80\xFD[a\t\xA1\x8F\x89\x83\x87\x01\x01a\x07LV[\x82RPa\t\xB1\x8E`@\x85\x01a\x07\xE0V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\t\xC6W_\x80\xFD[a\t\xD4\x8F\x89\x83\x87\x01\x01a\x08#V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\t\xEBW_\x80\xFD[a\t\xF9\x8F\x89\x83\x87\x01\x01a\x04VV[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\t1V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x04\xF1W_\x80\xFD[_` \x82\x84\x03\x12\x15a\n:W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nPW_\x80\xFD[a\n\\\x84\x82\x85\x01a\x04VV[\x94\x93PPPPV[`@\x81R_\x83Q`@\x80\x84\x01Ra\n~`\x80\x84\x01\x82a\x05*V[\x90P` \x85\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x83\x03\x01``\x85\x01Ra\n\xB9\x82\x82a\x05*V[\x91PP\x82\x81\x03` \x84\x01Ra\n\xCE\x81\x85a\x05*V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\x0B\x13W_\x80\xFD[\x81Qa\x0B!a\x04s\x82a\x04\x11V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B5W_\x80\xFD[a\n\\\x82` \x83\x01` \x87\x01a\x05\x08V[_`\xC0\x82\x84\x03\x12\x15a\x0BVW_\x80\xFD[a\x0B^a\x03SV[\x90P\x81Qa\x0Bk\x81a\x07(V[\x81R` \x82\x01Qa\x0B{\x81a\x07(V[\x80` \x83\x01RP`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa\x0B\xA4\x81a\x07(V[`\x80\x82\x01R`\xA0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xC2W_\x80\xFD[a\x07\xD4\x84\x82\x85\x01a\x0B\x04V[_``\x82\x84\x03\x12\x15a\x0B\xDEW_\x80\xFD[a\x0B\xE6a\x03|V[\x90P\x81Qa\x0B\xF3\x81a\x07(V[\x80\x82RP` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x0C W_\x80\xFD[\x81Q` a\x0C0a\x04s\x83a\x07\x05V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x0CNW_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\xB7W\x81\x81\x8A\x03\x12\x15a\x0ChW_\x80\xFD[a\x0Cpa\x03|V[\x81Qa\x0C{\x81a\x07(V[\x81R\x81\x86\x01Q\x86\x82\x01R`@\x80\x83\x01Qa\x0C\x94\x81a\x07(V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x0CRV[_` \x82\x84\x03\x12\x15a\x0C\xB5W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xCCW_\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x0C\xDFW_\x80\xFD[a\x0C\xE7a\x03\x9FV[\x82Q\x82\x81\x11\x15a\x0C\xF5W_\x80\xFD[a\r\x01\x87\x82\x86\x01a\x0BFV[\x82RPa\r\x11\x86` \x85\x01a\x0B\xCEV[` \x82\x01R`\x80\x83\x01Q\x82\x81\x11\x15a\r'W_\x80\xFD[a\r3\x87\x82\x86\x01a\x0C\x11V[`@\x83\x01RP`\xA0\x83\x01Q\x82\x81\x11\x15a\rJW_\x80\xFD[a\rV\x87\x82\x86\x01a\x0B\x04V[``\x83\x01RP`\xC0\x92\x90\x92\x01Q`\x80\x83\x01RP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xFE\xB5\x9B\x92\xB026\x10F\xE1\x83\xA9\x95\xBD\xE7X(\x86\xEC\x94\x98\x99\x0F{vt\xB9\xC2\x17\xF8\xB1kdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static ORDERQUOTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OrderQuoter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OrderQuoter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OrderQuoter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OrderQuoter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OrderQuoter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OrderQuoter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OrderQuoter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ORDERQUOTER_ABI.clone(),
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
                ORDERQUOTER_ABI.clone(),
                ORDERQUOTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getReactor` (0x7671d07b) function
        pub fn get_reactor(
            &self,
            order: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([118, 113, 208, 123], order)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0x41d88d69) function
        pub fn quote(
            &self,
            order: ::ethers::core::types::Bytes,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash([65, 216, 141, 105], (order, sig))
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
    for OrderQuoter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `OrdersLengthIncorrect` with signature `OrdersLengthIncorrect()` and selector `0x06ee9878`
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
    #[etherror(name = "OrdersLengthIncorrect", abi = "OrdersLengthIncorrect()")]
    pub struct OrdersLengthIncorrect;
    ///Container type for all input parameters for the `getReactor` function with signature `getReactor(bytes)` and selector `0x7671d07b`
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
    #[ethcall(name = "getReactor", abi = "getReactor(bytes)")]
    pub struct GetReactorCall {
        pub order: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote(bytes,bytes)` and selector `0x41d88d69`
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
    #[ethcall(name = "quote", abi = "quote(bytes,bytes)")]
    pub struct QuoteCall {
        pub order: ::ethers::core::types::Bytes,
        pub sig: ::ethers::core::types::Bytes,
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
    pub enum OrderQuoterCalls {
        GetReactor(GetReactorCall),
        Quote(QuoteCall),
        ReactorCallback(ReactorCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for OrderQuoterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetReactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReactor(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) = <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactorCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OrderQuoterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetReactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReactorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OrderQuoterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetReactorCall> for OrderQuoterCalls {
        fn from(value: GetReactorCall) -> Self {
            Self::GetReactor(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for OrderQuoterCalls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall> for OrderQuoterCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
    ///Container type for all return fields from the `getReactor` function with signature `getReactor(bytes)` and selector `0x7671d07b`
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
    pub struct GetReactorReturn {
        pub reactor: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `quote` function with signature `quote(bytes,bytes)` and selector `0x41d88d69`
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
    pub struct QuoteReturn {
        pub result: ResolvedOrder,
    }
}
