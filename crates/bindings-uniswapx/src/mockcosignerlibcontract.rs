/**

Generated by the following Solidity interface...
```solidity
interface MockCosignerLibContract {
    error InvalidCosignature();

    function verify(address cosigner, bytes32 data, bytes memory cosignature) external pure;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "verify",
    "inputs": [
      {
        "name": "cosigner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "cosignature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "pure"
  },
  {
    "type": "error",
    "name": "InvalidCosignature",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod MockCosignerLibContract {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506103238061001d5f395ff3fe608060405234801561000f575f80fd5b5060043610610029575f3560e01c80631a86b5501461002d575b5f80fd5b61004061003b3660046101a2565b610042565b005b61004d838383610052565b505050565b5f8082806020019051810190610068919061029e565b915091505f83604081518110610080576100806102c0565b01602090810151604080515f80825293810180835289905260f89290921c9082018190526060820186905260808201859052925060019060a0016020604051602081039080840390855afa1580156100da573d5f803e3d5ffd5b5050506020604051035190508073ffffffffffffffffffffffffffffffffffffffff168773ffffffffffffffffffffffffffffffffffffffff16141580610135575073ffffffffffffffffffffffffffffffffffffffff8116155b1561016c576040517fd7815be100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f805f606084860312156101b4575f80fd5b833573ffffffffffffffffffffffffffffffffffffffff811681146101d7575f80fd5b925060208401359150604084013567ffffffffffffffff808211156101fa575f80fd5b818601915086601f83011261020d575f80fd5b81358181111561021f5761021f610175565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0908116603f0116810190838211818310171561026557610265610175565b8160405282815289602084870101111561027d575f80fd5b826020860160208301375f6020848301015280955050505050509250925092565b5f80604083850312156102af575f80fd5b505080516020909101519092909150565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffdfea264697066735822122011e29ec06fabe2f26998885489532bb9bdc6866588c0518d51896e3bd42c29cf64736f6c63430008180033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\x03#\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\x1A\x86\xB5P\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x01\xA2V[a\0BV[\0[a\0M\x83\x83\x83a\0RV[PPPV[_\x80\x82\x80` \x01\x90Q\x81\x01\x90a\0h\x91\x90a\x02\x9EV[\x91P\x91P_\x83`@\x81Q\x81\x10a\0\x80Wa\0\x80a\x02\xC0V[\x01` \x90\x81\x01Q`@\x80Q_\x80\x82R\x93\x81\x01\x80\x83R\x89\x90R`\xF8\x92\x90\x92\x1C\x90\x82\x01\x81\x90R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x92P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\0\xDAW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80a\x015WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a\x01lW`@Q\x7F\xD7\x81[\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_\x80_``\x84\x86\x03\x12\x15a\x01\xB4W_\x80\xFD[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xD7W_\x80\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xFAW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02\rW_\x80\xFD[\x815\x81\x81\x11\x15a\x02\x1FWa\x02\x1Fa\x01uV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02eWa\x02ea\x01uV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x02}W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a\x02\xAFW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x11\xE2\x9E\xC0o\xAB\xE2\xF2i\x98\x88T\x89S+\xB9\xBD\xC6\x86e\x88\xC0Q\x8DQ\x89n;\xD4,)\xCFdsolcC\0\x08\x18\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610029575f3560e01c80631a86b5501461002d575b5f80fd5b61004061003b3660046101a2565b610042565b005b61004d838383610052565b505050565b5f8082806020019051810190610068919061029e565b915091505f83604081518110610080576100806102c0565b01602090810151604080515f80825293810180835289905260f89290921c9082018190526060820186905260808201859052925060019060a0016020604051602081039080840390855afa1580156100da573d5f803e3d5ffd5b5050506020604051035190508073ffffffffffffffffffffffffffffffffffffffff168773ffffffffffffffffffffffffffffffffffffffff16141580610135575073ffffffffffffffffffffffffffffffffffffffff8116155b1561016c576040517fd7815be100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f805f606084860312156101b4575f80fd5b833573ffffffffffffffffffffffffffffffffffffffff811681146101d7575f80fd5b925060208401359150604084013567ffffffffffffffff808211156101fa575f80fd5b818601915086601f83011261020d575f80fd5b81358181111561021f5761021f610175565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0908116603f0116810190838211818310171561026557610265610175565b8160405282815289602084870101111561027d575f80fd5b826020860160208301375f6020848301015280955050505050509250925092565b5f80604083850312156102af575f80fd5b505080516020909101519092909150565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffdfea264697066735822122011e29ec06fabe2f26998885489532bb9bdc6866588c0518d51896e3bd42c29cf64736f6c63430008180033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\x1A\x86\xB5P\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x01\xA2V[a\0BV[\0[a\0M\x83\x83\x83a\0RV[PPPV[_\x80\x82\x80` \x01\x90Q\x81\x01\x90a\0h\x91\x90a\x02\x9EV[\x91P\x91P_\x83`@\x81Q\x81\x10a\0\x80Wa\0\x80a\x02\xC0V[\x01` \x90\x81\x01Q`@\x80Q_\x80\x82R\x93\x81\x01\x80\x83R\x89\x90R`\xF8\x92\x90\x92\x1C\x90\x82\x01\x81\x90R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x92P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\0\xDAW=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80a\x015WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a\x01lW`@Q\x7F\xD7\x81[\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_\x80_``\x84\x86\x03\x12\x15a\x01\xB4W_\x80\xFD[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xD7W_\x80\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xFAW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02\rW_\x80\xFD[\x815\x81\x81\x11\x15a\x02\x1FWa\x02\x1Fa\x01uV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02eWa\x02ea\x01uV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x02}W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a\x02\xAFW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x11\xE2\x9E\xC0o\xAB\xE2\xF2i\x98\x88T\x89S+\xB9\xBD\xC6\x86e\x88\xC0Q\x8DQ\x89n;\xD4,)\xCFdsolcC\0\x08\x18\x003",
    );
    /**Custom error with signature `InvalidCosignature()` and selector `0xd7815be1`.
```solidity
error InvalidCosignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCosignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidCosignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCosignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCosignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCosignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCosignature()";
            const SELECTOR: [u8; 4] = [215u8, 129u8, 91u8, 225u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Function with signature `verify(address,bytes32,bytes)` and selector `0x1a86b550`.
```solidity
function verify(address cosigner, bytes32 data, bytes memory cosignature) external pure;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCall {
        #[allow(missing_docs)]
        pub cosigner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub cosignature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`verify(address,bytes32,bytes)`](verifyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCall) -> Self {
                    (value.cosigner, value.data, value.cosignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        cosigner: tuple.0,
                        data: tuple.1,
                        cosignature: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verify(address,bytes32,bytes)";
            const SELECTOR: [u8; 4] = [26u8, 134u8, 181u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.cosigner,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.cosignature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`MockCosignerLibContract`](self) function calls.
    pub enum MockCosignerLibContractCalls {
        #[allow(missing_docs)]
        verify(verifyCall),
    }
    #[automatically_derived]
    impl MockCosignerLibContractCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[26u8, 134u8, 181u8, 80u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockCosignerLibContractCalls {
        const NAME: &'static str = "MockCosignerLibContractCalls";
        const MIN_DATA_LENGTH: usize = 128usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::verify(_) => <verifyCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<MockCosignerLibContractCalls>] = &[
                {
                    fn verify(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockCosignerLibContractCalls> {
                        <verifyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockCosignerLibContractCalls::verify)
                    }
                    verify
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::verify(inner) => {
                    <verifyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::verify(inner) => {
                    <verifyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`MockCosignerLibContract`](self) custom errors.
    pub enum MockCosignerLibContractErrors {
        #[allow(missing_docs)]
        InvalidCosignature(InvalidCosignature),
    }
    #[automatically_derived]
    impl MockCosignerLibContractErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[215u8, 129u8, 91u8, 225u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockCosignerLibContractErrors {
        const NAME: &'static str = "MockCosignerLibContractErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidCosignature(_) => {
                    <InvalidCosignature as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<MockCosignerLibContractErrors>] = &[
                {
                    fn InvalidCosignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockCosignerLibContractErrors> {
                        <InvalidCosignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockCosignerLibContractErrors::InvalidCosignature)
                    }
                    InvalidCosignature
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InvalidCosignature(inner) => {
                    <InvalidCosignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidCosignature(inner) => {
                    <InvalidCosignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockCosignerLibContract`](self) contract instance.

See the [wrapper's documentation](`MockCosignerLibContractInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> MockCosignerLibContractInstance<T, P, N> {
        MockCosignerLibContractInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockCosignerLibContractInstance<T, P, N>>,
    > {
        MockCosignerLibContractInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockCosignerLibContractInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`MockCosignerLibContract`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MockCosignerLibContract`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockCosignerLibContractInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for MockCosignerLibContractInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockCosignerLibContractInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MockCosignerLibContractInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`MockCosignerLibContract`](self) contract instance.

See the [wrapper's documentation](`MockCosignerLibContractInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<MockCosignerLibContractInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> MockCosignerLibContractInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockCosignerLibContractInstance<T, P, N> {
            MockCosignerLibContractInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MockCosignerLibContractInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`verify`] function.
        pub fn verify(
            &self,
            cosigner: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::FixedBytes<32>,
            cosignature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyCall, N> {
            self.call_builder(
                &verifyCall {
                    cosigner,
                    data,
                    cosignature,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MockCosignerLibContractInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
