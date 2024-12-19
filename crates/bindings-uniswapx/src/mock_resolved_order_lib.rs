pub use mock_resolved_order_lib::*;
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
pub mod mock_resolved_order_lib {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("validate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resolvedOrder"),
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
                                    name: ::std::borrow::ToOwned::to_owned("filler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidReactor"),
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
    pub static MOCKRESOLVEDORDERLIB_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\x07\x92\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cX\xC9\xED\xBF\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x04\x88V[a\0BV[\0[a\0L\x82\x82a\0PV[PPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\0\xA1W`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\0LW\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x01\x1D\x90\x84\x90\x86\x90`\x04\x01a\x06.V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x013W_\x80\xFD[PZ\xFA\x15\x80\x15a\x01EW=_\x80>=_\xFD[PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9DWa\x01\x9Da\x01MV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9DWa\x01\x9Da\x01MV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\rWa\x02\ra\x01MV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x026W_\x80\xFD[PV[\x805a\x02D\x81a\x02\x15V[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x02XW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02rWa\x02ra\x01MV[a\x02\xA3` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x01\xC6V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x02\xB7W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x02\xE3W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x03\x07Wa\x03\x07a\x01MV[\x81`@R\x82\x93P\x845\x91Pa\x03\x1B\x82a\x02\x15V[\x90\x82R` \x84\x015\x90a\x03-\x82a\x02\x15V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x03W\x82a\x02\x15V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x03pW_\x80\xFD[Pa\x03}\x85\x82\x86\x01a\x02IV[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x03\x9AW_\x80\xFD[a\x03\xA2a\x01zV[\x90P\x815a\x03\xAF\x81a\x02\x15V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x03\xDCW_\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xF8Wa\x03\xF8a\x01MV[a\x04\x06\x81\x83`\x05\x1B\x01a\x01\xC6V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x04$W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x04{W\x81\x81\x8A\x03\x12\x15a\x04>W_\x80\xFD[a\x04Fa\x01zV[\x815a\x04Q\x81a\x02\x15V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x04j\x81a\x02\x15V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x04(V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x04\x99W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xB0W_\x80\xFD[\x90\x84\x01\x90`\xE0\x82\x87\x03\x12\x15a\x04\xC3W_\x80\xFD[a\x04\xCBa\x01\xA3V[\x825\x82\x81\x11\x15a\x04\xD9W_\x80\xFD[a\x04\xE5\x88\x82\x86\x01a\x02\xD3V[\x82RPa\x04\xF5\x87` \x85\x01a\x03\x8AV[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x05\x0BW_\x80\xFD[a\x05\x17\x88\x82\x86\x01a\x03\xCDV[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x05.W_\x80\xFD[a\x05:\x88\x82\x86\x01a\x02IV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x91Pa\x05[` \x84\x01a\x029V[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x05\x88W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05lV[P_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x06#W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\xD8V[P\x94\x95\x94PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x83R`@` \x84\x01R\x83Q`\xE0`@\x85\x01R\x81\x81Q\x16a\x01 \x85\x01R\x81` \x82\x01Q\x16a\x01@\x85\x01R`@\x81\x01Qa\x01`\x85\x01R``\x81\x01Qa\x01\x80\x85\x01R\x81`\x80\x82\x01Q\x16a\x01\xA0\x85\x01R`\xA0\x81\x01Q\x91PP`\xC0a\x01\xC0\x84\x01Ra\x06\xB1a\x01\xE0\x84\x01\x82a\x05dV[` \x85\x81\x01Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01R\x90\x81\x01Q`\x80\x86\x01R`@\x81\x01Q`\xA0\x86\x01R\x90\x91PP`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x80\x85\x84\x03\x01`\xC0\x86\x01Ra\x07%\x83\x83a\x05\xC5V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xE0\x86\x01RPa\x07C\x82\x82a\x05dV[\x91PP`\x80\x84\x01Qa\x01\0\x84\x01R\x80\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD3x\xEE\xD4\x90\x9C\xDEs\x07\xDB\xBB\x10K\x94\xFE\xDC\xFA\xBF|5c\x87\xA0\xC8\xB7\x19)T\x0C\xD7\xEF\x80dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKRESOLVEDORDERLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cX\xC9\xED\xBF\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x04\x88V[a\0BV[\0[a\0L\x82\x82a\0PV[PPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\0\xA1W`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\0LW\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x01\x1D\x90\x84\x90\x86\x90`\x04\x01a\x06.V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x013W_\x80\xFD[PZ\xFA\x15\x80\x15a\x01EW=_\x80>=_\xFD[PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9DWa\x01\x9Da\x01MV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9DWa\x01\x9Da\x01MV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\rWa\x02\ra\x01MV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x026W_\x80\xFD[PV[\x805a\x02D\x81a\x02\x15V[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x02XW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02rWa\x02ra\x01MV[a\x02\xA3` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x01\xC6V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x02\xB7W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_`\xC0\x82\x84\x03\x12\x15a\x02\xE3W_\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x03\x07Wa\x03\x07a\x01MV[\x81`@R\x82\x93P\x845\x91Pa\x03\x1B\x82a\x02\x15V[\x90\x82R` \x84\x015\x90a\x03-\x82a\x02\x15V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x03W\x82a\x02\x15V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x03pW_\x80\xFD[Pa\x03}\x85\x82\x86\x01a\x02IV[`\xA0\x83\x01RPP\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x03\x9AW_\x80\xFD[a\x03\xA2a\x01zV[\x90P\x815a\x03\xAF\x81a\x02\x15V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a\x03\xDCW_\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xF8Wa\x03\xF8a\x01MV[a\x04\x06\x81\x83`\x05\x1B\x01a\x01\xC6V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x04$W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x04{W\x81\x81\x8A\x03\x12\x15a\x04>W_\x80\xFD[a\x04Fa\x01zV[\x815a\x04Q\x81a\x02\x15V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x04j\x81a\x02\x15V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x04(V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x04\x99W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xB0W_\x80\xFD[\x90\x84\x01\x90`\xE0\x82\x87\x03\x12\x15a\x04\xC3W_\x80\xFD[a\x04\xCBa\x01\xA3V[\x825\x82\x81\x11\x15a\x04\xD9W_\x80\xFD[a\x04\xE5\x88\x82\x86\x01a\x02\xD3V[\x82RPa\x04\xF5\x87` \x85\x01a\x03\x8AV[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x05\x0BW_\x80\xFD[a\x05\x17\x88\x82\x86\x01a\x03\xCDV[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x05.W_\x80\xFD[a\x05:\x88\x82\x86\x01a\x02IV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x91Pa\x05[` \x84\x01a\x029V[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x05\x88W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05lV[P_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x06#W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\xD8V[P\x94\x95\x94PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x83R`@` \x84\x01R\x83Q`\xE0`@\x85\x01R\x81\x81Q\x16a\x01 \x85\x01R\x81` \x82\x01Q\x16a\x01@\x85\x01R`@\x81\x01Qa\x01`\x85\x01R``\x81\x01Qa\x01\x80\x85\x01R\x81`\x80\x82\x01Q\x16a\x01\xA0\x85\x01R`\xA0\x81\x01Q\x91PP`\xC0a\x01\xC0\x84\x01Ra\x06\xB1a\x01\xE0\x84\x01\x82a\x05dV[` \x85\x81\x01Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01R\x90\x81\x01Q`\x80\x86\x01R`@\x81\x01Q`\xA0\x86\x01R\x90\x91PP`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x80\x85\x84\x03\x01`\xC0\x86\x01Ra\x07%\x83\x83a\x05\xC5V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xE0\x86\x01RPa\x07C\x82\x82a\x05dV[\x91PP`\x80\x84\x01Qa\x01\0\x84\x01R\x80\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD3x\xEE\xD4\x90\x9C\xDEs\x07\xDB\xBB\x10K\x94\xFE\xDC\xFA\xBF|5c\x87\xA0\xC8\xB7\x19)T\x0C\xD7\xEF\x80dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKRESOLVEDORDERLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockResolvedOrderLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockResolvedOrderLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockResolvedOrderLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockResolvedOrderLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockResolvedOrderLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockResolvedOrderLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockResolvedOrderLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKRESOLVEDORDERLIB_ABI.clone(),
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
                MOCKRESOLVEDORDERLIB_ABI.clone(),
                MOCKRESOLVEDORDERLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `validate` (0x58c9edbf) function
        pub fn validate(
            &self,
            resolved_order: ResolvedOrder,
            filler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 201, 237, 191], (resolved_order, filler))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockResolvedOrderLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidReactor` with signature `InvalidReactor()` and selector `0x4ddf4a64`
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
    #[etherror(name = "InvalidReactor", abi = "InvalidReactor()")]
    pub struct InvalidReactor;
    ///Container type for all input parameters for the `validate` function with signature `validate(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address)` and selector `0x58c9edbf`
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
        name = "validate",
        abi = "validate(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address)"
    )]
    pub struct ValidateCall {
        pub resolved_order: ResolvedOrder,
        pub filler: ::ethers::core::types::Address,
    }
}
