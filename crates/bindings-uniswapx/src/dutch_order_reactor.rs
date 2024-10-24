pub use dutch_order_reactor::*;
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
pub mod dutch_order_reactor {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_permit2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_protocolFeeOwner"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeBatchWithCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeBatchWithCallback",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbackData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeWithCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeWithCallback",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("callbackData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeController"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IProtocolFeeController",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("permit2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProtocolFeeController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setProtocolFeeController",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newFeeController"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Fill"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Fill"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("orderHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("filler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("swapper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolFeeControllerSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProtocolFeeControllerSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldFeeController"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newFeeController"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DeadlineBeforeEndTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeadlineBeforeEndTime",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DuplicateFeeOutput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DuplicateFeeOutput"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("duplicateToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EndTimeBeforeStartTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EndTimeBeforeStartTime",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FeeTooLarge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("IncorrectAmounts"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InputAndOutputDecay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InputAndOutputDecay",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InputAndOutputFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InputAndOutputFees"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeeToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFeeToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidReactor"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DUTCHORDERREACTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\x000\x808\x03\x80b\x000\x80\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xB8V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x84\x92\x84\x92\x83\x92\x83\x92\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3PP`\x01`\x02UP`\x01`\x01`\xA0\x1B\x03\x16`\x80RPb\0\0\xF7\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xB5W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xCCW`\0\x80\xFD[\x82Qb\0\0\xD9\x81b\0\0\x9FV[` \x84\x01Q\x90\x92Pb\0\0\xEC\x81b\0\0\x9FV[\x80\x91PP\x92P\x92\x90PV[`\x80Qa/gb\0\x01\x19`\09`\0\x81\x81`\xE0\x01Ra\x17\xE9\x01Ra/g`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x9AW`\x005`\xE0\x1C\x80c-w\x13\x89\x11a\0iW\x80ci\x99\xB3w\x11a\0NW\x80ci\x99\xB3w\x14a\x01qW\x80c\x8D\xA5\xCB[\x14a\x01\x9EW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xCBW`\0\x80\xFD[\x80c-w\x13\x89\x14a\x01>W\x80c?b\x19.\x14a\x01^W`\0\x80\xFD[\x80c\r3X\x84\x14a\0\xA6W\x80c\rz\x16\xC3\x14a\0\xBBW\x80c\x12&\x1E\xE7\x14a\0\xCEW\x80c\x13\xFBr\xC7\x14a\x01+W`\0\x80\xFD[6a\0\xA1W\0[`\0\x80\xFD[a\0\xB9a\0\xB46`\x04a!\xAAV[a\x01\xEBV[\0[a\0\xB9a\0\xC96`\x04a\"XV[a\x03dV[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\x01\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB9a\x0196`\x04a\"\x9AV[a\x04\xC5V[4\x80\x15a\x01JW`\0\x80\xFD[Pa\0\xB9a\x01Y6`\x04a#(V[a\x06\x83V[a\0\xB9a\x01l6`\x04a#LV[a\x07\x8FV[4\x80\x15a\x01}W`\0\x80\xFD[P`\x01Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xAAW`\0\x80\xFD[P`\0Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xD7W`\0\x80\xFD[Pa\0\xB9a\x01\xE66`\x04a#(V[a\x08\x94V[a\x01\xF3a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x02\nW\x90PP\x90Pa\x02\xB2\x84a\t\xF6V[\x81`\0\x81Q\x81\x10a\x02\xC5Wa\x02\xC5a#\xB0V[` \x02` \x01\x01\x81\x90RPa\x02\xD9\x81a\x0BDV[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x03\x19\x90\x84\x90\x87\x90\x87\x90`\x04\x01a%\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPa\x03T\x81a\x0B\x95V[Pa\x03_`\x01`\x02UV[PPPV[a\x03la\t\x85V[\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x88Wa\x03\x88a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04CW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x03\xA6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x04\xA2Wa\x04}\x85\x85\x83\x81\x81\x10a\x04fWa\x04fa#\xB0V[\x90P` \x02\x81\x01\x90a\x04x\x91\x90a&yV[a\t\xF6V[\x82\x82\x81Q\x81\x10a\x04\x8FWa\x04\x8Fa#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04IV[Pa\x04\xAC\x81a\x0BDV[a\x04\xB5\x81a\x0B\x95V[PPa\x04\xC1`\x01`\x02UV[PPV[a\x04\xCDa\t\x85V[\x82`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xE9Wa\x04\xE9a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xA4W\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x05\x07W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x05\xECWa\x05\xC7\x87\x87\x83\x81\x81\x10a\x04fWa\x04fa#\xB0V[\x82\x82\x81Q\x81\x10a\x05\xD9Wa\x05\xD9a#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xAAV[Pa\x05\xF6\x81a\x0BDV[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x066\x90\x84\x90\x88\x90\x88\x90`\x04\x01a%\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06PW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06dW=`\0\x80>=`\0\xFD[PPPPa\x06q\x81a\x0B\x95V[PPa\x06}`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x07\x97a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x07\xAEW\x90PP\x90Pa\x08V\x82a\t\xF6V[\x81`\0\x81Q\x81\x10a\x08iWa\x08ia#\xB0V[` \x02` \x01\x01\x81\x90RPa\x08}\x81a\x0BDV[a\x08\x86\x81a\x0B\x95V[Pa\x08\x91`\x01`\x02UV[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\t\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\0V[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\no\x83\x80a&\xB7V[\x81\x01\x90a\n|\x91\x90a)\xA9V[\x90Pa\n\x87\x81a\x0C\xE8V[`@Q\x80`\xA0\x01`@R\x80\x82`\0\x01Q\x81R` \x01a\n\xBD\x83` \x01Q\x84`@\x01Q\x85``\x01Qa\r\xD3\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\n\xE3\x83` \x01Q\x84`@\x01Q\x85`\x80\x01Qa\x0E\xA5\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01\x84\x80` \x01\x90a\n\xF8\x91\x90a&\xB7V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x0B;\x83a\x0F\x8CV[\x90R\x93\x92PPPV[\x80Q`\0[\x81\x81\x10\x15a\x03_W`\0\x83\x82\x81Q\x81\x10a\x0BeWa\x0Bea#\xB0V[` \x02` \x01\x01Q\x90Pa\x0Bx\x81a\x11\xF6V[a\x0B\x82\x813a\x16\xE6V[a\x0B\x8C\x813a\x17\xE7V[P`\x01\x01a\x0BIV[\x80Q`\0[\x81\x81\x10\x15a\x0C\xD7W`\0\x83\x82\x81Q\x81\x10a\x0B\xB6Wa\x0B\xB6a#\xB0V[` \x02` \x01\x01Q\x90P`\0\x81`@\x01QQ\x90P`\0[\x81\x81\x10\x15a\x0C7W`\0\x83`@\x01Q\x82\x81Q\x81\x10a\x0B\xEDWa\x0B\xEDa#\xB0V[` \x02` \x01\x01Q\x90Pa\x0C.\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1BW\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x0B\xCDV[P\x81`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x85\x81Q\x81\x10a\x0C\x80Wa\x0C\x80a#\xB0V[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x85`\0\x01Q`@\x01Q`@Qa\x0C\xC5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x01a\x0B\x9AV[PG\x15a\x04\xC1Wa\x04\xC13Ga\x1B\x9EV[`@\x81\x01Q\x81Q``\x01Q\x10\x15a\r+W`@Q\x7Fw:a\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`@\x81\x01Q` \x90\x91\x01Q\x14a\x08\x91W`\0[\x81`\x80\x01QQ\x81\x10\x15a\x04\xC1W\x81`\x80\x01Q\x81\x81Q\x81\x10a\rfWa\rfa#\xB0V[` \x02` \x01\x01Q`@\x01Q\x82`\x80\x01Q\x82\x81Q\x81\x10a\r\x88Wa\r\x88a#\xB0V[` \x02` \x01\x01Q` \x01Q\x14a\r\xCBW`@Q\x7F\xD3\x03u\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\rCV[a\x0E\r`@Q\x80``\x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x83`@\x01Q\x84` \x01Q\x11\x15a\x0EOW`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0Ee\x85` \x01Q\x86`@\x01Q\x86\x86a\x1C8V[`@\x80Q``\x81\x01\x82R\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x92\x90\x92R\x95\x86\x01Q\x95\x81\x01\x95\x90\x95RP\x92\x93\x92PPPV[\x82Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xC3Wa\x0E\xC3a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F,W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x0E\xE1W\x90P[P\x91P`\0[\x81\x81\x10\x15a\x0F\x83Wa\x0F^\x86\x82\x81Q\x81\x10a\x0FOWa\x0FOa#\xB0V[` \x02` \x01\x01Q\x86\x86a\x1C\xB5V[\x83\x82\x81Q\x81\x10a\x0FpWa\x0Fpa#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F2V[PP\x93\x92PPPV[`@Q\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`+\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`:\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`Q\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`f\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`\x92\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\xA9\x82\x01R`\0\x90`\xBF\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.%` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a.\xA5`\x8D\x919`@Q` \x01a\x112\x93\x92\x91\x90a*\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x11U\x83`\0\x01Qa\x1D\x85V[\x83` \x01Q\x84`@\x01Q\x85``\x01Q`\0\x01Q\x86``\x01Q` \x01Q\x87``\x01Q`@\x01Qa\x11\x87\x89`\x80\x01Qa\x1E\x1FV[`@\x80Q` \x81\x01\x99\x90\x99R\x88\x01\x96\x90\x96R``\x87\x01\x94\x90\x94R`\x80\x86\x01\x92\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12\x16WPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x12m\x90\x85\x90`\x04\x01a+ V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x12\xD0\x91\x90\x81\x01\x90a+3V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x12\xE8\x82\x84a,2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\0Wa\x13\0a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13iW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x13\x1EW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x13\xBAW\x85`@\x01Q\x81\x81Q\x81\x10a\x13\x8DWa\x13\x8Da#\xB0V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x13\xA7Wa\x13\xA7a#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13oV[P`\0\x80`\0[\x84\x81\x10\x15a\x16\xD5W`\0\x87\x82\x81Q\x81\x10a\x13\xDDWa\x13\xDDa#\xB0V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x14\x9BW\x88\x81\x81Q\x81\x10a\x14\x04Wa\x14\x04a#\xB0V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x14\x93W\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[`\x01\x01a\x13\xEAV[P`\0\x80[\x88\x81\x10\x15a\x15\\W`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x14\xC0Wa\x14\xC0a#\xB0V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15SW\x85\x15a\x15=W`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x15L\x90\x84a,2V[\x92P`\x01\x96P[P`\x01\x01a\x14\xA0V[P\x81Q` \x8B\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x15\xD5W\x84\x15a\x15\xBCW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x8B\x01Q\x01Qa\x15\xCE\x90\x82a,2V[\x90P`\x01\x93P[\x80`\0\x03a\x16*W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[a\x168\x81`\x05a'\x10a\x1E\xBDV[\x82` \x01Q\x11\x15a\x16\xABW\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x07\0V[\x81\x86\x84\x8A\x01\x81Q\x81\x10a\x16\xC0Wa\x16\xC0a#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13\xC1V[PPP`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x177W`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x04\xC1W\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x17\xB3\x90\x84\x90\x86\x90`\x04\x01a,EV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xCBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\xDFW=`\0\x80>=`\0\xFD[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x18\xA7\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[a\x18\xB1\x85\x85a\x1E\xF9V[\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q` \x01a\x19\xFC\x90\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`\x1A\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`1\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`F\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`Y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`r\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\x89\x82\x01R`\x9F\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.%` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a.\xA5`\x8D\x919`@Q` \x01a\x1Ai\x93\x92\x91\x90a*\xDDV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R``\x83\x01\x90\x91R`.\x80\x83R\x90\x91\x90a.w` \x83\x019`@Q` \x01a\x1A\xBC\x92\x91\x90a,tV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x1B)\x96\x95\x94\x93\x92`\x04\x01a,\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1BCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xDFW=`\0\x80>=`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1B|Wa\x03_\x82\x82a\x1B\x9EV[a\x03_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x84\x84a\x1F@V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1B\xF8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1B\xFDV[``\x91P[PP\x90P\x80a\x03_W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x85\x03a\x1CHWP\x83a\x1C\xADV[\x82\x82\x11a\x1C\x81W`@Q\x7FC\x134S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x82\x11a\x1C\x8FWP\x82a\x1C\xADV[B\x83\x10a\x1C\x9DWP\x83a\x1C\xADV[a\x1C\xAA\x83\x83B\x88\x88a 2V[\x90P[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x83`@\x01Q\x84` \x01Q\x10\x15a\x1D\x14W`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1D*\x85` \x01Q\x86`@\x01Q\x86\x86a\x1C8V[\x90P`@Q\x80``\x01`@R\x80\x86`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81R` \x01\x86``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x91PP\x93\x92PPPV[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a.\xA5`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\x11\xD9\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E?Wa\x1E?a#\x81V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1EiW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1E\xAEW`\0a\x1E\x9C\x85\x83\x81Q\x81\x10a\x1E\x8FWa\x1E\x8Fa#\xB0V[` \x02` \x01\x01Qa MV[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a\x1EoV[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x1E\xF2W`\0\x80\xFD[P\x91\x02\x04\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x82R\x83\x81\x01Q\x81\x01Q\x90\x82\x01R[\x92\x91PPV[`\0`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a +W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[PPPPPV[`\0a A\x86\x86\x86\x86\x86a \xC4V[\x90P[\x95\x94PPPPPV[`\0`@Q\x80`\x80\x01`@R\x80`R\x81R` \x01a.%`R\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qa\x11\xD9\x96\x91\x92\x91\x01\x94\x85Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01R`@\x85\x01\x92\x90\x92R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\0\x84\x84\x10a \xD4WP\x80a DV[`\0a \xE0\x87\x86a-\x8AV[\x90P`\0a \xEE\x88\x88a-\x8AV[\x90P`\0\x85\x85\x12\x15a! Wa!\x10\x83\x83a!\t\x88\x8Aa-\x9DV[\x91\x90a\x1E\xBDV[a!\x19\x90a-\xC4V[\x90Pa!2V[a!/\x83\x83a!\t\x89\x89a-\x9DV[\x90P[a!<\x81\x87a-\xFCV[\x99\x98PPPPPPPPPV[`\0`@\x82\x84\x03\x12\x15a![W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a!sW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\x8BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!\xA3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a!\xBFW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\xD7W`\0\x80\xFD[a!\xE3\x87\x83\x88\x01a!IV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a!\xF9W`\0\x80\xFD[Pa\"\x06\x86\x82\x87\x01a!aV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\"%W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"=W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a!\xA3W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a\"kW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x82W`\0\x80\xFD[a\"\x8E\x85\x82\x86\x01a\"\x13V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\"\xB0W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xC8W`\0\x80\xFD[a\"\xD4\x88\x83\x89\x01a\"\x13V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\"\xEDW`\0\x80\xFD[Pa\"\xFA\x87\x82\x88\x01a!aV[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\x91W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#:W`\0\x80\xFD[\x815a#E\x81a#\x06V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a#^W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#uW`\0\x80\xFD[a\x1C\xAD\x84\x82\x85\x01a!IV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a#\xFAW\x81\x81\x01Q\x83\x82\x01R` \x01a#\xE2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\x1B\x81` \x86\x01` \x86\x01a#\xDFV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a$\xADW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a$bV[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra%,a\x01\xA0\x85\x01\x82a$\x03V[\x90P` \x83\x01Qa%j` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra%\x82\x82\x82a$MV[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra%\x9C\x82\x82a$\x03V[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[`\0`@\x82\x01`@\x83R\x80\x86Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x89\x01`\0[\x83\x81\x10\x15a&(W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x85Ra&\x16\x86\x83Qa$\xB8V[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a%\xDCV[PP\x85\x84\x03\x81\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x96\x95PPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a&\xADW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a&\xECW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\x07W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a!\xA3W`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xF5Wa'\xF5a#\x81V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a(\x0EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a((Wa((a#\x81V[a(Y` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a'\xAEV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a(nW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a(\x9DW`\0\x80\xFD[a(\xA5a'\x1CV[\x90P\x815a(\xB2\x81a#\x06V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xEAWa(\xEAa#\x81V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a)\x05W`\0\x80\xFD[\x815` a)\x1Aa)\x15\x83a(\xD0V[a'\xAEV[\x82\x81R`\x07\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a)9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a)\x9EW`\x80\x81\x89\x03\x12\x15a)VW`\0\x80\x81\xFD[a)^a'EV[\x815a)i\x81a#\x06V[\x81R\x81\x85\x015\x85\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015a)\x8C\x81a#\x06V[\x90\x82\x01R\x83R\x91\x83\x01\x91`\x80\x01a)=V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a)\xBBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a)\xD3W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a)\xE7W`\0\x80\xFD[a)\xEFa'hV[\x825\x82\x81\x11\x15a)\xFEW`\0\x80\xFD[\x83\x01`\xC0\x81\x88\x03\x12\x15a*\x10W`\0\x80\xFD[a*\x18a'\x8BV[\x815a*#\x81a#\x06V[\x81R` \x82\x015a*3\x81a#\x06V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a*\\\x81a#\x06V[`\x80\x82\x01R`\xA0\x82\x015\x84\x81\x11\x15a*sW`\0\x80\xFD[a*\x7F\x89\x82\x85\x01a'\xFDV[`\xA0\x83\x01RP\x80\x83RPP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra*\xAB\x86``\x85\x01a(\x8BV[``\x82\x01R`\xC0\x83\x015\x82\x81\x11\x15a*\xC2W`\0\x80\xFD[a*\xCE\x87\x82\x86\x01a(\xF4V[`\x80\x83\x01RP\x95\x94PPPPPV[`\0\x84Qa*\xEF\x81\x84` \x89\x01a#\xDFV[\x84Q\x90\x83\x01\x90a+\x03\x81\x83` \x89\x01a#\xDFV[\x84Q\x91\x01\x90a+\x16\x81\x83` \x88\x01a#\xDFV[\x01\x95\x94PPPPPV[` \x81R`\0a#E` \x83\x01\x84a$\xB8V[`\0` \x80\x83\x85\x03\x12\x15a+FW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+]W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a+nW`\0\x80\xFD[\x80Qa+|a)\x15\x82a(\xD0V[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a+\x9BW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a+\xF7W\x80\x85\x8A\x03\x12\x15a+\xB8W`\0\x80\x81\xFD[a+\xC0a'\x1CV[\x85Qa+\xCB\x81a#\x06V[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa+\xE4\x81a#\x06V[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a+\xA0V[P\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1F:Wa\x1F:a,\x03V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x1C\xAD`@\x83\x01\x84a$\xB8V[\x7FDutchOrder witness)\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa,\xAC\x81`\x13\x85\x01` \x88\x01a#\xDFV[\x83Q\x90\x83\x01\x90a,\xC3\x81`\x13\x84\x01` \x88\x01a#\xDFV[\x01`\x13\x01\x94\x93PPPPV[`\0a\x01@a,\xFF\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra-@`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra-u\x81\x84\x01\x86a$\x03V[\x90P\x82\x81\x03a\x01 \x84\x01Ra!<\x81\x85a$\x03V[\x81\x81\x03\x81\x81\x11\x15a\x1F:Wa\x1F:a,\x03V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a-\xBDWa-\xBDa,\x03V[P\x92\x91PPV[`\0\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a-\xF5Wa-\xF5a,\x03V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a.\x1CWa.\x1Ca,\x03V[PP\x92\x91PPV\xFEDutchOutput(address token,uint256 startAmount,uint256 endAmount,address recipient)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)\xA2dipfsX\"\x12 \xCD\x93T\xBF\xFD\xFC\x12V\xCCt~a0\x9F\xE6SM]\xB9\xBD>\xD0d\x9Fj\x80\xE1\x90\xCA\x13\xB2\xEBdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static DUTCHORDERREACTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x9AW`\x005`\xE0\x1C\x80c-w\x13\x89\x11a\0iW\x80ci\x99\xB3w\x11a\0NW\x80ci\x99\xB3w\x14a\x01qW\x80c\x8D\xA5\xCB[\x14a\x01\x9EW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xCBW`\0\x80\xFD[\x80c-w\x13\x89\x14a\x01>W\x80c?b\x19.\x14a\x01^W`\0\x80\xFD[\x80c\r3X\x84\x14a\0\xA6W\x80c\rz\x16\xC3\x14a\0\xBBW\x80c\x12&\x1E\xE7\x14a\0\xCEW\x80c\x13\xFBr\xC7\x14a\x01+W`\0\x80\xFD[6a\0\xA1W\0[`\0\x80\xFD[a\0\xB9a\0\xB46`\x04a!\xAAV[a\x01\xEBV[\0[a\0\xB9a\0\xC96`\x04a\"XV[a\x03dV[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\x01\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB9a\x0196`\x04a\"\x9AV[a\x04\xC5V[4\x80\x15a\x01JW`\0\x80\xFD[Pa\0\xB9a\x01Y6`\x04a#(V[a\x06\x83V[a\0\xB9a\x01l6`\x04a#LV[a\x07\x8FV[4\x80\x15a\x01}W`\0\x80\xFD[P`\x01Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xAAW`\0\x80\xFD[P`\0Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xD7W`\0\x80\xFD[Pa\0\xB9a\x01\xE66`\x04a#(V[a\x08\x94V[a\x01\xF3a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x02\nW\x90PP\x90Pa\x02\xB2\x84a\t\xF6V[\x81`\0\x81Q\x81\x10a\x02\xC5Wa\x02\xC5a#\xB0V[` \x02` \x01\x01\x81\x90RPa\x02\xD9\x81a\x0BDV[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x03\x19\x90\x84\x90\x87\x90\x87\x90`\x04\x01a%\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPa\x03T\x81a\x0B\x95V[Pa\x03_`\x01`\x02UV[PPPV[a\x03la\t\x85V[\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x88Wa\x03\x88a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04CW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x03\xA6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x04\xA2Wa\x04}\x85\x85\x83\x81\x81\x10a\x04fWa\x04fa#\xB0V[\x90P` \x02\x81\x01\x90a\x04x\x91\x90a&yV[a\t\xF6V[\x82\x82\x81Q\x81\x10a\x04\x8FWa\x04\x8Fa#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04IV[Pa\x04\xAC\x81a\x0BDV[a\x04\xB5\x81a\x0B\x95V[PPa\x04\xC1`\x01`\x02UV[PPV[a\x04\xCDa\t\x85V[\x82`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xE9Wa\x04\xE9a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xA4W\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x05\x07W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x05\xECWa\x05\xC7\x87\x87\x83\x81\x81\x10a\x04fWa\x04fa#\xB0V[\x82\x82\x81Q\x81\x10a\x05\xD9Wa\x05\xD9a#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xAAV[Pa\x05\xF6\x81a\x0BDV[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x066\x90\x84\x90\x88\x90\x88\x90`\x04\x01a%\xB3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06PW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06dW=`\0\x80>=`\0\xFD[PPPPa\x06q\x81a\x0B\x95V[PPa\x06}`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x07\x97a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x07\xAEW\x90PP\x90Pa\x08V\x82a\t\xF6V[\x81`\0\x81Q\x81\x10a\x08iWa\x08ia#\xB0V[` \x02` \x01\x01\x81\x90RPa\x08}\x81a\x0BDV[a\x08\x86\x81a\x0B\x95V[Pa\x08\x91`\x01`\x02UV[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\t\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\0V[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\no\x83\x80a&\xB7V[\x81\x01\x90a\n|\x91\x90a)\xA9V[\x90Pa\n\x87\x81a\x0C\xE8V[`@Q\x80`\xA0\x01`@R\x80\x82`\0\x01Q\x81R` \x01a\n\xBD\x83` \x01Q\x84`@\x01Q\x85``\x01Qa\r\xD3\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\n\xE3\x83` \x01Q\x84`@\x01Q\x85`\x80\x01Qa\x0E\xA5\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01\x84\x80` \x01\x90a\n\xF8\x91\x90a&\xB7V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x0B;\x83a\x0F\x8CV[\x90R\x93\x92PPPV[\x80Q`\0[\x81\x81\x10\x15a\x03_W`\0\x83\x82\x81Q\x81\x10a\x0BeWa\x0Bea#\xB0V[` \x02` \x01\x01Q\x90Pa\x0Bx\x81a\x11\xF6V[a\x0B\x82\x813a\x16\xE6V[a\x0B\x8C\x813a\x17\xE7V[P`\x01\x01a\x0BIV[\x80Q`\0[\x81\x81\x10\x15a\x0C\xD7W`\0\x83\x82\x81Q\x81\x10a\x0B\xB6Wa\x0B\xB6a#\xB0V[` \x02` \x01\x01Q\x90P`\0\x81`@\x01QQ\x90P`\0[\x81\x81\x10\x15a\x0C7W`\0\x83`@\x01Q\x82\x81Q\x81\x10a\x0B\xEDWa\x0B\xEDa#\xB0V[` \x02` \x01\x01Q\x90Pa\x0C.\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1BW\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x0B\xCDV[P\x81`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x85\x81Q\x81\x10a\x0C\x80Wa\x0C\x80a#\xB0V[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x85`\0\x01Q`@\x01Q`@Qa\x0C\xC5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x01a\x0B\x9AV[PG\x15a\x04\xC1Wa\x04\xC13Ga\x1B\x9EV[`@\x81\x01Q\x81Q``\x01Q\x10\x15a\r+W`@Q\x7Fw:a\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`@\x81\x01Q` \x90\x91\x01Q\x14a\x08\x91W`\0[\x81`\x80\x01QQ\x81\x10\x15a\x04\xC1W\x81`\x80\x01Q\x81\x81Q\x81\x10a\rfWa\rfa#\xB0V[` \x02` \x01\x01Q`@\x01Q\x82`\x80\x01Q\x82\x81Q\x81\x10a\r\x88Wa\r\x88a#\xB0V[` \x02` \x01\x01Q` \x01Q\x14a\r\xCBW`@Q\x7F\xD3\x03u\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\rCV[a\x0E\r`@Q\x80``\x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x83`@\x01Q\x84` \x01Q\x11\x15a\x0EOW`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0Ee\x85` \x01Q\x86`@\x01Q\x86\x86a\x1C8V[`@\x80Q``\x81\x01\x82R\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x92\x90\x92R\x95\x86\x01Q\x95\x81\x01\x95\x90\x95RP\x92\x93\x92PPPV[\x82Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xC3Wa\x0E\xC3a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F,W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x0E\xE1W\x90P[P\x91P`\0[\x81\x81\x10\x15a\x0F\x83Wa\x0F^\x86\x82\x81Q\x81\x10a\x0FOWa\x0FOa#\xB0V[` \x02` \x01\x01Q\x86\x86a\x1C\xB5V[\x83\x82\x81Q\x81\x10a\x0FpWa\x0Fpa#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F2V[PP\x93\x92PPPV[`@Q\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`+\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`:\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`Q\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`f\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`\x92\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\xA9\x82\x01R`\0\x90`\xBF\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.%` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a.\xA5`\x8D\x919`@Q` \x01a\x112\x93\x92\x91\x90a*\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x11U\x83`\0\x01Qa\x1D\x85V[\x83` \x01Q\x84`@\x01Q\x85``\x01Q`\0\x01Q\x86``\x01Q` \x01Q\x87``\x01Q`@\x01Qa\x11\x87\x89`\x80\x01Qa\x1E\x1FV[`@\x80Q` \x81\x01\x99\x90\x99R\x88\x01\x96\x90\x96R``\x87\x01\x94\x90\x94R`\x80\x86\x01\x92\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12\x16WPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x12m\x90\x85\x90`\x04\x01a+ V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x12\xD0\x91\x90\x81\x01\x90a+3V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x12\xE8\x82\x84a,2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\0Wa\x13\0a#\x81V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13iW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x13\x1EW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x13\xBAW\x85`@\x01Q\x81\x81Q\x81\x10a\x13\x8DWa\x13\x8Da#\xB0V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x13\xA7Wa\x13\xA7a#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13oV[P`\0\x80`\0[\x84\x81\x10\x15a\x16\xD5W`\0\x87\x82\x81Q\x81\x10a\x13\xDDWa\x13\xDDa#\xB0V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x14\x9BW\x88\x81\x81Q\x81\x10a\x14\x04Wa\x14\x04a#\xB0V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x14\x93W\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[`\x01\x01a\x13\xEAV[P`\0\x80[\x88\x81\x10\x15a\x15\\W`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x14\xC0Wa\x14\xC0a#\xB0V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15SW\x85\x15a\x15=W`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x15L\x90\x84a,2V[\x92P`\x01\x96P[P`\x01\x01a\x14\xA0V[P\x81Q` \x8B\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x15\xD5W\x84\x15a\x15\xBCW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x8B\x01Q\x01Qa\x15\xCE\x90\x82a,2V[\x90P`\x01\x93P[\x80`\0\x03a\x16*W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[a\x168\x81`\x05a'\x10a\x1E\xBDV[\x82` \x01Q\x11\x15a\x16\xABW\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x07\0V[\x81\x86\x84\x8A\x01\x81Q\x81\x10a\x16\xC0Wa\x16\xC0a#\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13\xC1V[PPP`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x177W`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x04\xC1W\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x17\xB3\x90\x84\x90\x86\x90`\x04\x01a,EV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xCBW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x17\xDFW=`\0\x80>=`\0\xFD[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x18\xA7\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[a\x18\xB1\x85\x85a\x1E\xF9V[\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q` \x01a\x19\xFC\x90\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`\x1A\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`1\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`F\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`Y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`r\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\x89\x82\x01R`\x9F\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.%` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a.\xA5`\x8D\x919`@Q` \x01a\x1Ai\x93\x92\x91\x90a*\xDDV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R``\x83\x01\x90\x91R`.\x80\x83R\x90\x91\x90a.w` \x83\x019`@Q` \x01a\x1A\xBC\x92\x91\x90a,tV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x1B)\x96\x95\x94\x93\x92`\x04\x01a,\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1BCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xDFW=`\0\x80>=`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1B|Wa\x03_\x82\x82a\x1B\x9EV[a\x03_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x84\x84a\x1F@V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1B\xF8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1B\xFDV[``\x91P[PP\x90P\x80a\x03_W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x85\x03a\x1CHWP\x83a\x1C\xADV[\x82\x82\x11a\x1C\x81W`@Q\x7FC\x134S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x82\x11a\x1C\x8FWP\x82a\x1C\xADV[B\x83\x10a\x1C\x9DWP\x83a\x1C\xADV[a\x1C\xAA\x83\x83B\x88\x88a 2V[\x90P[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x83`@\x01Q\x84` \x01Q\x10\x15a\x1D\x14W`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1D*\x85` \x01Q\x86`@\x01Q\x86\x86a\x1C8V[\x90P`@Q\x80``\x01`@R\x80\x86`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81R` \x01\x86``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x91PP\x93\x92PPPV[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a.\xA5`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\x11\xD9\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E?Wa\x1E?a#\x81V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1EiW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1E\xAEW`\0a\x1E\x9C\x85\x83\x81Q\x81\x10a\x1E\x8FWa\x1E\x8Fa#\xB0V[` \x02` \x01\x01Qa MV[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a\x1EoV[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x1E\xF2W`\0\x80\xFD[P\x91\x02\x04\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x82R\x83\x81\x01Q\x81\x01Q\x90\x82\x01R[\x92\x91PPV[`\0`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a +W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[PPPPPV[`\0a A\x86\x86\x86\x86\x86a \xC4V[\x90P[\x95\x94PPPPPV[`\0`@Q\x80`\x80\x01`@R\x80`R\x81R` \x01a.%`R\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qa\x11\xD9\x96\x91\x92\x91\x01\x94\x85Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01R`@\x85\x01\x92\x90\x92R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\0\x84\x84\x10a \xD4WP\x80a DV[`\0a \xE0\x87\x86a-\x8AV[\x90P`\0a \xEE\x88\x88a-\x8AV[\x90P`\0\x85\x85\x12\x15a! Wa!\x10\x83\x83a!\t\x88\x8Aa-\x9DV[\x91\x90a\x1E\xBDV[a!\x19\x90a-\xC4V[\x90Pa!2V[a!/\x83\x83a!\t\x89\x89a-\x9DV[\x90P[a!<\x81\x87a-\xFCV[\x99\x98PPPPPPPPPV[`\0`@\x82\x84\x03\x12\x15a![W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a!sW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\x8BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!\xA3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a!\xBFW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!\xD7W`\0\x80\xFD[a!\xE3\x87\x83\x88\x01a!IV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a!\xF9W`\0\x80\xFD[Pa\"\x06\x86\x82\x87\x01a!aV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\"%W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"=W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a!\xA3W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a\"kW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x82W`\0\x80\xFD[a\"\x8E\x85\x82\x86\x01a\"\x13V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\"\xB0W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xC8W`\0\x80\xFD[a\"\xD4\x88\x83\x89\x01a\"\x13V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\"\xEDW`\0\x80\xFD[Pa\"\xFA\x87\x82\x88\x01a!aV[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\x91W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#:W`\0\x80\xFD[\x815a#E\x81a#\x06V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a#^W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#uW`\0\x80\xFD[a\x1C\xAD\x84\x82\x85\x01a!IV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a#\xFAW\x81\x81\x01Q\x83\x82\x01R` \x01a#\xE2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\x1B\x81` \x86\x01` \x86\x01a#\xDFV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a$\xADW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a$bV[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra%,a\x01\xA0\x85\x01\x82a$\x03V[\x90P` \x83\x01Qa%j` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra%\x82\x82\x82a$MV[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra%\x9C\x82\x82a$\x03V[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[`\0`@\x82\x01`@\x83R\x80\x86Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x89\x01`\0[\x83\x81\x10\x15a&(W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x85Ra&\x16\x86\x83Qa$\xB8V[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a%\xDCV[PP\x85\x84\x03\x81\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x96\x95PPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a&\xADW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a&\xECW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\x07W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a!\xA3W`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'?Wa'?a#\x81V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\xF5Wa'\xF5a#\x81V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a(\x0EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a((Wa((a#\x81V[a(Y` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a'\xAEV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a(nW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a(\x9DW`\0\x80\xFD[a(\xA5a'\x1CV[\x90P\x815a(\xB2\x81a#\x06V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xEAWa(\xEAa#\x81V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a)\x05W`\0\x80\xFD[\x815` a)\x1Aa)\x15\x83a(\xD0V[a'\xAEV[\x82\x81R`\x07\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a)9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a)\x9EW`\x80\x81\x89\x03\x12\x15a)VW`\0\x80\x81\xFD[a)^a'EV[\x815a)i\x81a#\x06V[\x81R\x81\x85\x015\x85\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015a)\x8C\x81a#\x06V[\x90\x82\x01R\x83R\x91\x83\x01\x91`\x80\x01a)=V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a)\xBBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a)\xD3W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a)\xE7W`\0\x80\xFD[a)\xEFa'hV[\x825\x82\x81\x11\x15a)\xFEW`\0\x80\xFD[\x83\x01`\xC0\x81\x88\x03\x12\x15a*\x10W`\0\x80\xFD[a*\x18a'\x8BV[\x815a*#\x81a#\x06V[\x81R` \x82\x015a*3\x81a#\x06V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a*\\\x81a#\x06V[`\x80\x82\x01R`\xA0\x82\x015\x84\x81\x11\x15a*sW`\0\x80\xFD[a*\x7F\x89\x82\x85\x01a'\xFDV[`\xA0\x83\x01RP\x80\x83RPP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra*\xAB\x86``\x85\x01a(\x8BV[``\x82\x01R`\xC0\x83\x015\x82\x81\x11\x15a*\xC2W`\0\x80\xFD[a*\xCE\x87\x82\x86\x01a(\xF4V[`\x80\x83\x01RP\x95\x94PPPPPV[`\0\x84Qa*\xEF\x81\x84` \x89\x01a#\xDFV[\x84Q\x90\x83\x01\x90a+\x03\x81\x83` \x89\x01a#\xDFV[\x84Q\x91\x01\x90a+\x16\x81\x83` \x88\x01a#\xDFV[\x01\x95\x94PPPPPV[` \x81R`\0a#E` \x83\x01\x84a$\xB8V[`\0` \x80\x83\x85\x03\x12\x15a+FW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+]W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a+nW`\0\x80\xFD[\x80Qa+|a)\x15\x82a(\xD0V[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a+\x9BW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a+\xF7W\x80\x85\x8A\x03\x12\x15a+\xB8W`\0\x80\x81\xFD[a+\xC0a'\x1CV[\x85Qa+\xCB\x81a#\x06V[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa+\xE4\x81a#\x06V[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a+\xA0V[P\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1F:Wa\x1F:a,\x03V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x1C\xAD`@\x83\x01\x84a$\xB8V[\x7FDutchOrder witness)\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa,\xAC\x81`\x13\x85\x01` \x88\x01a#\xDFV[\x83Q\x90\x83\x01\x90a,\xC3\x81`\x13\x84\x01` \x88\x01a#\xDFV[\x01`\x13\x01\x94\x93PPPPV[`\0a\x01@a,\xFF\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra-@`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra-u\x81\x84\x01\x86a$\x03V[\x90P\x82\x81\x03a\x01 \x84\x01Ra!<\x81\x85a$\x03V[\x81\x81\x03\x81\x81\x11\x15a\x1F:Wa\x1F:a,\x03V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a-\xBDWa-\xBDa,\x03V[P\x92\x91PPV[`\0\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a-\xF5Wa-\xF5a,\x03V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a.\x1CWa.\x1Ca,\x03V[PP\x92\x91PPV\xFEDutchOutput(address token,uint256 startAmount,uint256 endAmount,address recipient)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)\xA2dipfsX\"\x12 \xCD\x93T\xBF\xFD\xFC\x12V\xCCt~a0\x9F\xE6SM]\xB9\xBD>\xD0d\x9Fj\x80\xE1\x90\xCA\x13\xB2\xEBdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static DUTCHORDERREACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DutchOrderReactor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DutchOrderReactor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DutchOrderReactor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DutchOrderReactor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DutchOrderReactor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DutchOrderReactor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DutchOrderReactor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DUTCHORDERREACTOR_ABI.clone(),
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
                DUTCHORDERREACTOR_ABI.clone(),
                DUTCHORDERREACTOR_BYTECODE.clone().into(),
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
        ///Calls the contract's `executeBatchWithCallback` (0x13fb72c7) function
        pub fn execute_batch_with_callback(
            &self,
            orders: ::std::vec::Vec<SignedOrder>,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 251, 114, 199], (orders, callback_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithCallback` (0x0d335884) function
        pub fn execute_with_callback(
            &self,
            order: SignedOrder,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 51, 88, 132], (order, callback_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeController` (0x6999b377) function
        pub fn fee_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([105, 153, 179, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit2` (0x12261ee7) function
        pub fn permit_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([18, 38, 30, 231], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFeeController` (0x2d771389) function
        pub fn set_protocol_fee_controller(
            &self,
            new_fee_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 119, 19, 137], new_fee_controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Fill` event
        pub fn fill_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FillFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProtocolFeeControllerSet` event
        pub fn protocol_fee_controller_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProtocolFeeControllerSetFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DutchOrderReactorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DutchOrderReactor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DeadlineBeforeEndTime` with signature `DeadlineBeforeEndTime()` and selector `0x773a6187`
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
    #[etherror(name = "DeadlineBeforeEndTime", abi = "DeadlineBeforeEndTime()")]
    pub struct DeadlineBeforeEndTime;
    ///Custom Error type `DuplicateFeeOutput` with signature `DuplicateFeeOutput(address)` and selector `0xfff08303`
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
    #[etherror(name = "DuplicateFeeOutput", abi = "DuplicateFeeOutput(address)")]
    pub struct DuplicateFeeOutput {
        pub duplicate_token: ::ethers::core::types::Address,
    }
    ///Custom Error type `EndTimeBeforeStartTime` with signature `EndTimeBeforeStartTime()` and selector `0x43133453`
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
    #[etherror(name = "EndTimeBeforeStartTime", abi = "EndTimeBeforeStartTime()")]
    pub struct EndTimeBeforeStartTime;
    ///Custom Error type `FeeTooLarge` with signature `FeeTooLarge(address,uint256,address)` and selector `0x82e75656`
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
    #[etherror(name = "FeeTooLarge", abi = "FeeTooLarge(address,uint256,address)")]
    pub struct FeeTooLarge {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Custom Error type `IncorrectAmounts` with signature `IncorrectAmounts()` and selector `0x7c1f8113`
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
    #[etherror(name = "IncorrectAmounts", abi = "IncorrectAmounts()")]
    pub struct IncorrectAmounts;
    ///Custom Error type `InputAndOutputDecay` with signature `InputAndOutputDecay()` and selector `0xd303758b`
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
    #[etherror(name = "InputAndOutputDecay", abi = "InputAndOutputDecay()")]
    pub struct InputAndOutputDecay;
    ///Custom Error type `InputAndOutputFees` with signature `InputAndOutputFees()` and selector `0xedc7e2e4`
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
    #[etherror(name = "InputAndOutputFees", abi = "InputAndOutputFees()")]
    pub struct InputAndOutputFees;
    ///Custom Error type `InvalidFeeToken` with signature `InvalidFeeToken(address)` and selector `0xeddf07f5`
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
    #[etherror(name = "InvalidFeeToken", abi = "InvalidFeeToken(address)")]
    pub struct InvalidFeeToken {
        pub fee_token: ::ethers::core::types::Address,
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
    ///Container type for all of the contract's custom errors
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
    pub enum DutchOrderReactorErrors {
        DeadlineBeforeEndTime(DeadlineBeforeEndTime),
        DuplicateFeeOutput(DuplicateFeeOutput),
        EndTimeBeforeStartTime(EndTimeBeforeStartTime),
        FeeTooLarge(FeeTooLarge),
        IncorrectAmounts(IncorrectAmounts),
        InputAndOutputDecay(InputAndOutputDecay),
        InputAndOutputFees(InputAndOutputFees),
        InvalidFeeToken(InvalidFeeToken),
        InvalidReactor(InvalidReactor),
        NativeTransferFailed(NativeTransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DutchOrderReactorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DeadlineBeforeEndTime as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeadlineBeforeEndTime(decoded));
            }
            if let Ok(decoded) = <DuplicateFeeOutput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DuplicateFeeOutput(decoded));
            }
            if let Ok(decoded) = <EndTimeBeforeStartTime as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EndTimeBeforeStartTime(decoded));
            }
            if let Ok(decoded) = <FeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeTooLarge(decoded));
            }
            if let Ok(decoded) = <IncorrectAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncorrectAmounts(decoded));
            }
            if let Ok(decoded) = <InputAndOutputDecay as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InputAndOutputDecay(decoded));
            }
            if let Ok(decoded) = <InputAndOutputFees as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InputAndOutputFees(decoded));
            }
            if let Ok(decoded) = <InvalidFeeToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFeeToken(decoded));
            }
            if let Ok(decoded) = <InvalidReactor as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReactor(decoded));
            }
            if let Ok(decoded) = <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NativeTransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DutchOrderReactorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeadlineBeforeEndTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DuplicateFeeOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EndTimeBeforeStartTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InputAndOutputDecay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InputAndOutputFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DutchOrderReactorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DeadlineBeforeEndTime as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DuplicateFeeOutput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EndTimeBeforeStartTime as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FeeTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <IncorrectAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InputAndOutputDecay as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InputAndOutputFees as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFeeToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidReactor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NativeTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DutchOrderReactorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeadlineBeforeEndTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DuplicateFeeOutput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EndTimeBeforeStartTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::InputAndOutputDecay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InputAndOutputFees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidFeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DutchOrderReactorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeadlineBeforeEndTime> for DutchOrderReactorErrors {
        fn from(value: DeadlineBeforeEndTime) -> Self {
            Self::DeadlineBeforeEndTime(value)
        }
    }
    impl ::core::convert::From<DuplicateFeeOutput> for DutchOrderReactorErrors {
        fn from(value: DuplicateFeeOutput) -> Self {
            Self::DuplicateFeeOutput(value)
        }
    }
    impl ::core::convert::From<EndTimeBeforeStartTime> for DutchOrderReactorErrors {
        fn from(value: EndTimeBeforeStartTime) -> Self {
            Self::EndTimeBeforeStartTime(value)
        }
    }
    impl ::core::convert::From<FeeTooLarge> for DutchOrderReactorErrors {
        fn from(value: FeeTooLarge) -> Self {
            Self::FeeTooLarge(value)
        }
    }
    impl ::core::convert::From<IncorrectAmounts> for DutchOrderReactorErrors {
        fn from(value: IncorrectAmounts) -> Self {
            Self::IncorrectAmounts(value)
        }
    }
    impl ::core::convert::From<InputAndOutputDecay> for DutchOrderReactorErrors {
        fn from(value: InputAndOutputDecay) -> Self {
            Self::InputAndOutputDecay(value)
        }
    }
    impl ::core::convert::From<InputAndOutputFees> for DutchOrderReactorErrors {
        fn from(value: InputAndOutputFees) -> Self {
            Self::InputAndOutputFees(value)
        }
    }
    impl ::core::convert::From<InvalidFeeToken> for DutchOrderReactorErrors {
        fn from(value: InvalidFeeToken) -> Self {
            Self::InvalidFeeToken(value)
        }
    }
    impl ::core::convert::From<InvalidReactor> for DutchOrderReactorErrors {
        fn from(value: InvalidReactor) -> Self {
            Self::InvalidReactor(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for DutchOrderReactorErrors {
        fn from(value: NativeTransferFailed) -> Self {
            Self::NativeTransferFailed(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Fill", abi = "Fill(bytes32,address,address,uint256)")]
    pub struct FillFilter {
        #[ethevent(indexed)]
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub filler: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub swapper: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ProtocolFeeControllerSet",
        abi = "ProtocolFeeControllerSet(address,address)"
    )]
    pub struct ProtocolFeeControllerSetFilter {
        pub old_fee_controller: ::ethers::core::types::Address,
        pub new_fee_controller: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
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
    pub enum DutchOrderReactorEvents {
        FillFilter(FillFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProtocolFeeControllerSetFilter(ProtocolFeeControllerSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for DutchOrderReactorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FillFilter::decode_log(log) {
                return Ok(DutchOrderReactorEvents::FillFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DutchOrderReactorEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProtocolFeeControllerSetFilter::decode_log(log) {
                return Ok(
                    DutchOrderReactorEvents::ProtocolFeeControllerSetFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DutchOrderReactorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FillFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProtocolFeeControllerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FillFilter> for DutchOrderReactorEvents {
        fn from(value: FillFilter) -> Self {
            Self::FillFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for DutchOrderReactorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeControllerSetFilter>
    for DutchOrderReactorEvents {
        fn from(value: ProtocolFeeControllerSetFilter) -> Self {
            Self::ProtocolFeeControllerSetFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `executeBatchWithCallback` function with signature `executeBatchWithCallback((bytes,bytes)[],bytes)` and selector `0x13fb72c7`
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
        name = "executeBatchWithCallback",
        abi = "executeBatchWithCallback((bytes,bytes)[],bytes)"
    )]
    pub struct ExecuteBatchWithCallbackCall {
        pub orders: ::std::vec::Vec<SignedOrder>,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeWithCallback` function with signature `executeWithCallback((bytes,bytes),bytes)` and selector `0x0d335884`
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
        name = "executeWithCallback",
        abi = "executeWithCallback((bytes,bytes),bytes)"
    )]
    pub struct ExecuteWithCallbackCall {
        pub order: SignedOrder,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `feeController` function with signature `feeController()` and selector `0x6999b377`
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
    #[ethcall(name = "feeController", abi = "feeController()")]
    pub struct FeeControllerCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `permit2` function with signature `permit2()` and selector `0x12261ee7`
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
    #[ethcall(name = "permit2", abi = "permit2()")]
    pub struct Permit2Call;
    ///Container type for all input parameters for the `setProtocolFeeController` function with signature `setProtocolFeeController(address)` and selector `0x2d771389`
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
        name = "setProtocolFeeController",
        abi = "setProtocolFeeController(address)"
    )]
    pub struct SetProtocolFeeControllerCall {
        pub new_fee_controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum DutchOrderReactorCalls {
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        ExecuteBatchWithCallback(ExecuteBatchWithCallbackCall),
        ExecuteWithCallback(ExecuteWithCallbackCall),
        FeeController(FeeControllerCall),
        Owner(OwnerCall),
        Permit2(Permit2Call),
        SetProtocolFeeController(SetProtocolFeeControllerCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for DutchOrderReactorCalls {
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
            if let Ok(decoded) = <ExecuteBatchWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBatchWithCallback(decoded));
            }
            if let Ok(decoded) = <ExecuteWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteWithCallback(decoded));
            }
            if let Ok(decoded) = <FeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeController(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <Permit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2(decoded));
            }
            if let Ok(decoded) = <SetProtocolFeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProtocolFeeController(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DutchOrderReactorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteBatchWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProtocolFeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DutchOrderReactorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatchWithCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteWithCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for DutchOrderReactorCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for DutchOrderReactorCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchWithCallbackCall> for DutchOrderReactorCalls {
        fn from(value: ExecuteBatchWithCallbackCall) -> Self {
            Self::ExecuteBatchWithCallback(value)
        }
    }
    impl ::core::convert::From<ExecuteWithCallbackCall> for DutchOrderReactorCalls {
        fn from(value: ExecuteWithCallbackCall) -> Self {
            Self::ExecuteWithCallback(value)
        }
    }
    impl ::core::convert::From<FeeControllerCall> for DutchOrderReactorCalls {
        fn from(value: FeeControllerCall) -> Self {
            Self::FeeController(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DutchOrderReactorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<Permit2Call> for DutchOrderReactorCalls {
        fn from(value: Permit2Call) -> Self {
            Self::Permit2(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeControllerCall> for DutchOrderReactorCalls {
        fn from(value: SetProtocolFeeControllerCall) -> Self {
            Self::SetProtocolFeeController(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DutchOrderReactorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `feeController` function with signature `feeController()` and selector `0x6999b377`
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
    pub struct FeeControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `permit2` function with signature `permit2()` and selector `0x12261ee7`
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
    pub struct Permit2Return(pub ::ethers::core::types::Address);
}
