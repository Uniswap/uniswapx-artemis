///Module containing a contract's types and functions.
/**

```solidity
library IAllowanceTransfer {
    struct PermitDetails { address token; uint160 amount; uint48 expiration; uint48 nonce; }
    struct PermitSingle { PermitDetails details; address spender; uint256 sigDeadline; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IAllowanceTransfer {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct PermitDetails { address token; uint160 amount; uint48 expiration; uint48 nonce; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PermitDetails {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub expiration: alloy::sol_types::private::primitives::aliases::U48,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U48,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<160>,
            alloy::sol_types::sol_data::Uint<48>,
            alloy::sol_types::sol_data::Uint<48>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U160,
            alloy::sol_types::private::primitives::aliases::U48,
            alloy::sol_types::private::primitives::aliases::U48,
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
        impl ::core::convert::From<PermitDetails> for UnderlyingRustTuple<'_> {
            fn from(value: PermitDetails) -> Self {
                (value.token, value.amount, value.expiration, value.nonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PermitDetails {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    amount: tuple.1,
                    expiration: tuple.2,
                    nonce: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PermitDetails {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PermitDetails {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiration),
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PermitDetails {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PermitDetails {
            const NAME: &'static str = "PermitDetails";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PermitDetails(address token,uint160 amount,uint48 expiration,uint48 nonce)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiration)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PermitDetails {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiration,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        48,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    160,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiration,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    48,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct PermitSingle { PermitDetails details; address spender; uint256 sigDeadline; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PermitSingle {
        #[allow(missing_docs)]
        pub details: <PermitDetails as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sigDeadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            PermitDetails,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <PermitDetails as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<PermitSingle> for UnderlyingRustTuple<'_> {
            fn from(value: PermitSingle) -> Self {
                (value.details, value.spender, value.sigDeadline)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PermitSingle {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    details: tuple.0,
                    spender: tuple.1,
                    sigDeadline: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PermitSingle {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PermitSingle {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <PermitDetails as alloy_sol_types::SolType>::tokenize(&self.details),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sigDeadline),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PermitSingle {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PermitSingle {
            const NAME: &'static str = "PermitSingle";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PermitSingle(PermitDetails details,address spender,uint256 sigDeadline)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <PermitDetails as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <PermitDetails as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <PermitDetails as alloy_sol_types::SolType>::eip712_data_word(
                            &self.details,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.spender,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.sigDeadline)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PermitSingle {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <PermitDetails as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.details,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.spender,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigDeadline,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <PermitDetails as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.details,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.spender,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigDeadline,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IAllowanceTransfer`](self) contract instance.

See the [wrapper's documentation](`IAllowanceTransferInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IAllowanceTransferInstance<P, N> {
        IAllowanceTransferInstance::<P, N>::new(address, __provider)
    }
    /**A [`IAllowanceTransfer`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAllowanceTransfer`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAllowanceTransferInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IAllowanceTransferInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAllowanceTransferInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IAllowanceTransferInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IAllowanceTransfer`](self) contract instance.

See the [wrapper's documentation](`IAllowanceTransferInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
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
    impl<P: ::core::clone::Clone, N> IAllowanceTransferInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAllowanceTransferInstance<P, N> {
            IAllowanceTransferInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IAllowanceTransferInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IAllowanceTransferInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IAllowanceTransfer {
    struct PermitDetails {
        address token;
        uint160 amount;
        uint48 expiration;
        uint48 nonce;
    }
    struct PermitSingle {
        PermitDetails details;
        address spender;
        uint256 sigDeadline;
    }
}

interface DCAHookHarness {
    struct DCAExecutionState {
        uint128 executedChunks;
        uint120 lastExecutionTime;
        bool cancelled;
        uint256 totalInputExecuted;
        uint256 totalOutput;
    }
    struct DCAIntent {
        address swapper;
        uint256 nonce;
        uint256 chainId;
        address hookAddress;
        bool isExactIn;
        address inputToken;
        address outputToken;
        address cosigner;
        uint256 minPeriod;
        uint256 maxPeriod;
        uint256 minChunkSize;
        uint256 maxChunkSize;
        uint256 minPrice;
        uint256 deadline;
        OutputAllocation[] outputAllocations;
        PrivateIntent privateIntent;
    }
    struct DCAOrderCosignerData {
        address swapper;
        uint96 nonce;
        uint160 execAmount;
        uint96 orderNonce;
        uint160 limitAmount;
    }
    struct FeedInfo {
        FeedTemplate feedTemplate;
        address feedAddress;
        string feedType;
    }
    struct FeedTemplate {
        string name;
        string expression;
        string[] parameters;
        string[] secrets;
        uint256 retryCount;
    }
    struct InputToken {
        address token;
        uint256 amount;
        uint256 maxAmount;
    }
    struct OrderInfo {
        address reactor;
        address swapper;
        uint256 nonce;
        uint256 deadline;
        address preExecutionHook;
        bytes preExecutionHookData;
        address postExecutionHook;
        bytes postExecutionHookData;
        address auctionResolver;
    }
    struct OutputAllocation {
        address recipient;
        uint16 basisPoints;
    }
    struct OutputToken {
        address token;
        uint256 amount;
        address recipient;
    }
    struct PermitData {
        bool hasPermit;
        IAllowanceTransfer.PermitSingle permitSingle;
        bytes signature;
    }
    struct PrivateIntent {
        uint256 totalAmount;
        uint256 exactFrequency;
        uint256 numChunks;
        bytes32 salt;
        FeedInfo[] oracleFeeds;
    }
    struct ResolvedOrder {
        OrderInfo info;
        InputToken input;
        OutputToken[] outputs;
        bytes sig;
        bytes32 hash;
        address auctionResolver;
        string witnessTypeString;
    }

    error AllocationMismatch(address recipient, uint256 actual, uint256 expected);
    error AllocationsNot100Percent(uint256 totalBasisPoints);
    error ChunkSizeAboveMax(uint256 amount, uint256 maxChunkSize);
    error ChunkSizeBelowMin(uint256 amount, uint256 minChunkSize);
    error CosignerNonceMismatch(uint96 cosignerNonce, uint256 intentNonce);
    error CosignerSwapperMismatch(address cosignerSwapper, address intentSwapper);
    error DuplicateRecipient(address recipient);
    error EmptyAllocations();
    error InputAboveLimit(uint256 inputAmount, uint256 limitAmount);
    error InputAmountMismatch(uint256 inputAmount, uint256 execAmount);
    error InsufficientOutput(uint256 totalOutput, uint256 limitAmount);
    error IntentAlreadyCancelled(bytes32 intentId);
    error IntentExpired(uint256 currentTime, uint256 deadline);
    error IntentIsCancelled(bytes32 intentId);
    error InvalidCosignerSignature(address recoveredCosigner, address expectedCosigner);
    error InvalidSwapperSignature(address recoveredSigner, address expectedSwapper);
    error PriceBelowMin(uint256 executionPrice, uint256 minPrice);
    error SwapperMismatch(address orderSwapper, address intentSwapper);
    error TooLate(uint256 elapsed, uint256 maxPeriod);
    error TooSoon(uint256 elapsed, uint256 minPeriod);
    error WrongChain(uint256 providedChainId, uint256 currentChainId);
    error WrongChunkNonce(uint96 providedNonce, uint96 expectedNonce);
    error WrongHook(address providedHook, address expectedHook);
    error WrongInputToken(address orderInputToken, address intentInputToken);
    error WrongOutputToken(address outputToken, address expectedToken);
    error WrongTotalOutput(uint256 totalOutput, uint256 execAmount);
    error ZeroAllocation();
    error ZeroInput();

    event ChunkExecuted(bytes32 indexed intentId, uint256 execAmount, uint256 limitAmount, uint256 totalInputExecuted, uint256 totalOutput);
    event IntentCancelled(bytes32 indexed intentId, address indexed swapper);

    constructor(address p, address r);

    function DOMAIN_SEPARATOR() external view returns (bytes32);
    function __setExecutedMeta(bytes32 intentId, uint120 lastExecutionTime) external;
    function __setPacked(bytes32 intentId, uint128 executedChunks, bool cancelled) external;
    function __setTotals(bytes32 intentId, uint256 totalInputExecuted, uint256 totalOutput) external;
    function cancelIntent(uint256 nonce) external;
    function cancelIntents(uint256[] memory nonces) external;
    function computeIntentId(address swapper, uint256 nonce) external pure returns (bytes32);
    function createTestCosignerData(address swapper, uint96 nonce, uint160 execAmount, uint160 limitAmount, uint96 orderNonce) external pure returns (DCAOrderCosignerData memory);
    function createTestIntent(address swapper, uint96 nonce, bool isExactIn, uint256 minChunk, uint256 maxChunk) external view returns (DCAIntent memory);
    function getExecutionState(bytes32 intentId) external view returns (DCAExecutionState memory);
    function getIntentStatistics(bytes32 intentId) external view returns (uint256 totalChunks, uint256 totalInput, uint256 totalOutput, uint256 lastExecutionTime);
    function getNextNonce(bytes32 intentId) external view returns (uint96);
    function isIntentActive(bytes32 intentId, uint256 maxPeriod, uint256 deadline) external view returns (bool);
    function permit2() external view returns (address);
    function preExecutionHook(address filler, ResolvedOrder memory resolvedOrder) external;
    function reactor() external view returns (address);
    function transferInputTokens(ResolvedOrder memory order, address to, PermitData memory permitData) external;
    function validateAllocationStructure(OutputAllocation[] memory outputAllocations) external pure;
    function validateChunkSize(DCAIntent memory intent, DCAOrderCosignerData memory cosignerData, uint256 inputAmount) external pure;
    function validatePriceFloor(bool isExactIn, uint160 execAmount, uint160 limitAmount, uint256 minPrice) external pure;
    function validateStaticFields(DCAIntent memory intent, ResolvedOrder memory resolvedOrder) external view;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "p",
        "type": "address",
        "internalType": "contract IPermit2"
      },
      {
        "name": "r",
        "type": "address",
        "internalType": "contract IReactor"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "DOMAIN_SEPARATOR",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "__setExecutedMeta",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "lastExecutionTime",
        "type": "uint120",
        "internalType": "uint120"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__setPacked",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "executedChunks",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "cancelled",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__setTotals",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "totalInputExecuted",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalOutput",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cancelIntent",
    "inputs": [
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cancelIntents",
    "inputs": [
      {
        "name": "nonces",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "computeIntentId",
    "inputs": [
      {
        "name": "swapper",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "createTestCosignerData",
    "inputs": [
      {
        "name": "swapper",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "execAmount",
        "type": "uint160",
        "internalType": "uint160"
      },
      {
        "name": "limitAmount",
        "type": "uint160",
        "internalType": "uint160"
      },
      {
        "name": "orderNonce",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct DCAOrderCosignerData",
        "components": [
          {
            "name": "swapper",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint96",
            "internalType": "uint96"
          },
          {
            "name": "execAmount",
            "type": "uint160",
            "internalType": "uint160"
          },
          {
            "name": "orderNonce",
            "type": "uint96",
            "internalType": "uint96"
          },
          {
            "name": "limitAmount",
            "type": "uint160",
            "internalType": "uint160"
          }
        ]
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "createTestIntent",
    "inputs": [
      {
        "name": "swapper",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "isExactIn",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "minChunk",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxChunk",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct DCAIntent",
        "components": [
          {
            "name": "swapper",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hookAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "isExactIn",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "inputToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "outputToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "cosigner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "minPeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxPeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minChunkSize",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxChunkSize",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minPrice",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "outputAllocations",
            "type": "tuple[]",
            "internalType": "struct OutputAllocation[]",
            "components": [
              {
                "name": "recipient",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "basisPoints",
                "type": "uint16",
                "internalType": "uint16"
              }
            ]
          },
          {
            "name": "privateIntent",
            "type": "tuple",
            "internalType": "struct PrivateIntent",
            "components": [
              {
                "name": "totalAmount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "exactFrequency",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "numChunks",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "salt",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "oracleFeeds",
                "type": "tuple[]",
                "internalType": "struct FeedInfo[]",
                "components": [
                  {
                    "name": "feedTemplate",
                    "type": "tuple",
                    "internalType": "struct FeedTemplate",
                    "components": [
                      {
                        "name": "name",
                        "type": "string",
                        "internalType": "string"
                      },
                      {
                        "name": "expression",
                        "type": "string",
                        "internalType": "string"
                      },
                      {
                        "name": "parameters",
                        "type": "string[]",
                        "internalType": "string[]"
                      },
                      {
                        "name": "secrets",
                        "type": "string[]",
                        "internalType": "string[]"
                      },
                      {
                        "name": "retryCount",
                        "type": "uint256",
                        "internalType": "uint256"
                      }
                    ]
                  },
                  {
                    "name": "feedAddress",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "feedType",
                    "type": "string",
                    "internalType": "string"
                  }
                ]
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getExecutionState",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct DCAExecutionState",
        "components": [
          {
            "name": "executedChunks",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "lastExecutionTime",
            "type": "uint120",
            "internalType": "uint120"
          },
          {
            "name": "cancelled",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "totalInputExecuted",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalOutput",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getIntentStatistics",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "totalChunks",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalInput",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalOutput",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "lastExecutionTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getNextNonce",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isIntentActive",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "maxPeriod",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permit2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPermit2"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "preExecutionHook",
    "inputs": [
      {
        "name": "filler",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "resolvedOrder",
        "type": "tuple",
        "internalType": "struct ResolvedOrder",
        "components": [
          {
            "name": "info",
            "type": "tuple",
            "internalType": "struct OrderInfo",
            "components": [
              {
                "name": "reactor",
                "type": "address",
                "internalType": "contract IReactor"
              },
              {
                "name": "swapper",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "nonce",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "deadline",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "preExecutionHook",
                "type": "address",
                "internalType": "contract IPreExecutionHook"
              },
              {
                "name": "preExecutionHookData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "postExecutionHook",
                "type": "address",
                "internalType": "contract IPostExecutionHook"
              },
              {
                "name": "postExecutionHookData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "auctionResolver",
                "type": "address",
                "internalType": "contract IAuctionResolver"
              }
            ]
          },
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct InputToken",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "contract ERC20"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "maxAmount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "outputs",
            "type": "tuple[]",
            "internalType": "struct OutputToken[]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "recipient",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "sig",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "auctionResolver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "witnessTypeString",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "reactor",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IReactor"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferInputTokens",
    "inputs": [
      {
        "name": "order",
        "type": "tuple",
        "internalType": "struct ResolvedOrder",
        "components": [
          {
            "name": "info",
            "type": "tuple",
            "internalType": "struct OrderInfo",
            "components": [
              {
                "name": "reactor",
                "type": "address",
                "internalType": "contract IReactor"
              },
              {
                "name": "swapper",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "nonce",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "deadline",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "preExecutionHook",
                "type": "address",
                "internalType": "contract IPreExecutionHook"
              },
              {
                "name": "preExecutionHookData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "postExecutionHook",
                "type": "address",
                "internalType": "contract IPostExecutionHook"
              },
              {
                "name": "postExecutionHookData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "auctionResolver",
                "type": "address",
                "internalType": "contract IAuctionResolver"
              }
            ]
          },
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct InputToken",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "contract ERC20"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "maxAmount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "outputs",
            "type": "tuple[]",
            "internalType": "struct OutputToken[]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "recipient",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "sig",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "auctionResolver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "witnessTypeString",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "permitData",
        "type": "tuple",
        "internalType": "struct PermitData",
        "components": [
          {
            "name": "hasPermit",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "permitSingle",
            "type": "tuple",
            "internalType": "struct IAllowanceTransfer.PermitSingle",
            "components": [
              {
                "name": "details",
                "type": "tuple",
                "internalType": "struct IAllowanceTransfer.PermitDetails",
                "components": [
                  {
                    "name": "token",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "amount",
                    "type": "uint160",
                    "internalType": "uint160"
                  },
                  {
                    "name": "expiration",
                    "type": "uint48",
                    "internalType": "uint48"
                  },
                  {
                    "name": "nonce",
                    "type": "uint48",
                    "internalType": "uint48"
                  }
                ]
              },
              {
                "name": "spender",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "sigDeadline",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "validateAllocationStructure",
    "inputs": [
      {
        "name": "outputAllocations",
        "type": "tuple[]",
        "internalType": "struct OutputAllocation[]",
        "components": [
          {
            "name": "recipient",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "basisPoints",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "validateChunkSize",
    "inputs": [
      {
        "name": "intent",
        "type": "tuple",
        "internalType": "struct DCAIntent",
        "components": [
          {
            "name": "swapper",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hookAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "isExactIn",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "inputToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "outputToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "cosigner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "minPeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxPeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minChunkSize",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxChunkSize",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minPrice",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "outputAllocations",
            "type": "tuple[]",
            "internalType": "struct OutputAllocation[]",
            "components": [
              {
                "name": "recipient",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "basisPoints",
                "type": "uint16",
                "internalType": "uint16"
              }
            ]
          },
          {
            "name": "privateIntent",
            "type": "tuple",
            "internalType": "struct PrivateIntent",
            "components": [
              {
                "name": "totalAmount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "exactFrequency",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "numChunks",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "salt",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "oracleFeeds",
                "type": "tuple[]",
                "internalType": "struct FeedInfo[]",
                "components": [
                  {
                    "name": "feedTemplate",
                    "type": "tuple",
                    "internalType": "struct FeedTemplate",
                    "components": [
                      {
                        "name": "name",
                        "type": "string",
                        "internalType": "string"
                      },
                      {
                        "name": "expression",
                        "type": "string",
                        "internalType": "string"
                      },
                      {
                        "name": "parameters",
                        "type": "string[]",
                        "internalType": "string[]"
                      },
                      {
                        "name": "secrets",
                        "type": "string[]",
                        "internalType": "string[]"
                      },
                      {
                        "name": "retryCount",
                        "type": "uint256",
                        "internalType": "uint256"
                      }
                    ]
                  },
                  {
                    "name": "feedAddress",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "feedType",
                    "type": "string",
                    "internalType": "string"
                  }
                ]
              }
            ]
          }
        ]
      },
      {
        "name": "cosignerData",
        "type": "tuple",
        "internalType": "struct DCAOrderCosignerData",
        "components": [
          {
            "name": "swapper",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint96",
            "internalType": "uint96"
          },
          {
            "name": "execAmount",
            "type": "uint160",
            "internalType": "uint160"
          },
          {
            "name": "orderNonce",
            "type": "uint96",
            "internalType": "uint96"
          },
          {
            "name": "limitAmount",
            "type": "uint160",
            "internalType": "uint160"
          }
        ]
      },
      {
        "name": "inputAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "validatePriceFloor",
    "inputs": [
      {
        "name": "isExactIn",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "execAmount",
        "type": "uint160",
        "internalType": "uint160"
      },
      {
        "name": "limitAmount",
        "type": "uint160",
        "internalType": "uint160"
      },
      {
        "name": "minPrice",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "validateStaticFields",
    "inputs": [
      {
        "name": "intent",
        "type": "tuple",
        "internalType": "struct DCAIntent",
        "components": [
          {
            "name": "swapper",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "hookAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "isExactIn",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "inputToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "outputToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "cosigner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "minPeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxPeriod",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minChunkSize",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "maxChunkSize",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "minPrice",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "deadline",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "outputAllocations",
            "type": "tuple[]",
            "internalType": "struct OutputAllocation[]",
            "components": [
              {
                "name": "recipient",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "basisPoints",
                "type": "uint16",
                "internalType": "uint16"
              }
            ]
          },
          {
            "name": "privateIntent",
            "type": "tuple",
            "internalType": "struct PrivateIntent",
            "components": [
              {
                "name": "totalAmount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "exactFrequency",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "numChunks",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "salt",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "oracleFeeds",
                "type": "tuple[]",
                "internalType": "struct FeedInfo[]",
                "components": [
                  {
                    "name": "feedTemplate",
                    "type": "tuple",
                    "internalType": "struct FeedTemplate",
                    "components": [
                      {
                        "name": "name",
                        "type": "string",
                        "internalType": "string"
                      },
                      {
                        "name": "expression",
                        "type": "string",
                        "internalType": "string"
                      },
                      {
                        "name": "parameters",
                        "type": "string[]",
                        "internalType": "string[]"
                      },
                      {
                        "name": "secrets",
                        "type": "string[]",
                        "internalType": "string[]"
                      },
                      {
                        "name": "retryCount",
                        "type": "uint256",
                        "internalType": "uint256"
                      }
                    ]
                  },
                  {
                    "name": "feedAddress",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "feedType",
                    "type": "string",
                    "internalType": "string"
                  }
                ]
              }
            ]
          }
        ]
      },
      {
        "name": "resolvedOrder",
        "type": "tuple",
        "internalType": "struct ResolvedOrder",
        "components": [
          {
            "name": "info",
            "type": "tuple",
            "internalType": "struct OrderInfo",
            "components": [
              {
                "name": "reactor",
                "type": "address",
                "internalType": "contract IReactor"
              },
              {
                "name": "swapper",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "nonce",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "deadline",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "preExecutionHook",
                "type": "address",
                "internalType": "contract IPreExecutionHook"
              },
              {
                "name": "preExecutionHookData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "postExecutionHook",
                "type": "address",
                "internalType": "contract IPostExecutionHook"
              },
              {
                "name": "postExecutionHookData",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "auctionResolver",
                "type": "address",
                "internalType": "contract IAuctionResolver"
              }
            ]
          },
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct InputToken",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "contract ERC20"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "maxAmount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "outputs",
            "type": "tuple[]",
            "internalType": "struct OutputToken[]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "recipient",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "sig",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "hash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "auctionResolver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "witnessTypeString",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "ChunkExecuted",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "execAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "limitAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "totalInputExecuted",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "totalOutput",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "IntentCancelled",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "swapper",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AllocationMismatch",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "actual",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "expected",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "AllocationsNot100Percent",
    "inputs": [
      {
        "name": "totalBasisPoints",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ChunkSizeAboveMax",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxChunkSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ChunkSizeBelowMin",
    "inputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minChunkSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CosignerNonceMismatch",
    "inputs": [
      {
        "name": "cosignerNonce",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "intentNonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CosignerSwapperMismatch",
    "inputs": [
      {
        "name": "cosignerSwapper",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "intentSwapper",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "DuplicateRecipient",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "EmptyAllocations",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAboveLimit",
    "inputs": [
      {
        "name": "inputAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "limitAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InputAmountMismatch",
    "inputs": [
      {
        "name": "inputAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "execAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InsufficientOutput",
    "inputs": [
      {
        "name": "totalOutput",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "limitAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "IntentAlreadyCancelled",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "IntentExpired",
    "inputs": [
      {
        "name": "currentTime",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "deadline",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "IntentIsCancelled",
    "inputs": [
      {
        "name": "intentId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidCosignerSignature",
    "inputs": [
      {
        "name": "recoveredCosigner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expectedCosigner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidSwapperSignature",
    "inputs": [
      {
        "name": "recoveredSigner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expectedSwapper",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "PriceBelowMin",
    "inputs": [
      {
        "name": "executionPrice",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minPrice",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "SwapperMismatch",
    "inputs": [
      {
        "name": "orderSwapper",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "intentSwapper",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "TooLate",
    "inputs": [
      {
        "name": "elapsed",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "maxPeriod",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "TooSoon",
    "inputs": [
      {
        "name": "elapsed",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minPeriod",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongChain",
    "inputs": [
      {
        "name": "providedChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "currentChainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongChunkNonce",
    "inputs": [
      {
        "name": "providedNonce",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "expectedNonce",
        "type": "uint96",
        "internalType": "uint96"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongHook",
    "inputs": [
      {
        "name": "providedHook",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expectedHook",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongInputToken",
    "inputs": [
      {
        "name": "orderInputToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "intentInputToken",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongOutputToken",
    "inputs": [
      {
        "name": "outputToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expectedToken",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongTotalOutput",
    "inputs": [
      {
        "name": "totalOutput",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "execAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroAllocation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroInput",
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
pub mod DCAHookHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f5ffd5b50604051614b47380380614b4783398101604081905261002f91610149565b6001600160a01b03808316608052811660a0524660e0528181610125306040805180820182526007815266444341486f6f6b60c81b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa20374b09dfce3316b287b36aa908c96cdb9329755a7e08683c5b42908026635818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201526001600160a01b039390931660a0808501919091528251808503909101815260c0909301909152815191012090565b60c0525061018192505050565b6001600160a01b0381168114610146575f5ffd5b50565b5f5f6040838503121561015a575f5ffd5b825161016581610132565b602084015190925061017681610132565b809150509250929050565b60805160a05160c05160e05161497e6101c95f395f6108a101525f6108d701525f81816106ab0152610bf901525f81816101e90152818161184d01526118e4015261497e5ff3fe608060405234801561000f575f5ffd5b506004361061016e575f3560e01c80635e29fa37116100d2578063a0a31aac11610088578063b202a7f311610063578063b202a7f3146106e0578063d8b61ded14610766578063fe7823ac14610779575f5ffd5b8063a0a31aac14610693578063ab572650146106a6578063b1c13908146106cd575f5ffd5b80638184e353116100b85780638184e3531461064d5780638345eb561461066057806383bc6ab614610673575f5ffd5b80635e29fa371461062757806380cb1b501461063a575f5ffd5b80632fd0109b11610127578063308ea6c91161010d578063308ea6c9146103a65780633644e515146104d057806349d8033e146104e6575f5ffd5b80632fd0109b146103705780633079008114610383575f5ffd5b806316b853a21161015757806316b853a2146102355780631ce24d021461030f57806329c8ad4e1461035d575f5ffd5b80630209e7101461017257806312261ee7146101e4575b5f5ffd5b6101e2610180366004612d92565b5f9182526020829052604090912080546effffffffffffffffffffffffffffff909216700100000000000000000000000000000000027fff000000000000000000000000000000ffffffffffffffffffffffffffffffff909216919091179055565b005b61020b7f000000000000000000000000000000000000000000000000000000000000000081565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b6102ef610243366004612dd3565b5f9081526020818152604091829020825160a08101845281546fffffffffffffffffffffffffffffffff811680835270010000000000000000000000000000000082046effffffffffffffffffffffffffffff169483018590527f010000000000000000000000000000000000000000000000000000000000000090910460ff161515948201949094526001820154606082018190526002909201546080909101819052929390929190565b60408051948552602085019390935291830152606082015260800161022c565b61034061031d366004612dd3565b5f908152602081905260409020546fffffffffffffffffffffffffffffffff1690565b6040516bffffffffffffffffffffffff909116815260200161022c565b6101e261036b36600461357e565b6107a3565b6101e261037e366004613896565b6107b3565b6103966103913660046138fb565b6107c1565b604051901515815260200161022c565b6104316103b4366004613924565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506040805160a08101825273ffffffffffffffffffffffffffffffffffffffff96871681526bffffffffffffffffffffffff95861660208201529386169084015290921660608201529116608082015290565b60405161022c91905f60a08201905073ffffffffffffffffffffffffffffffffffffffff83511682526bffffffffffffffffffffffff602084015116602083015273ffffffffffffffffffffffffffffffffffffffff60408401511660408301526bffffffffffffffffffffffff606084015116606083015273ffffffffffffffffffffffffffffffffffffffff608084015116608083015292915050565b6104d861089e565b60405190815260200161022c565b6105c16104f4366004612dd3565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152505f9081526020818152604091829020825160a08101845281546fffffffffffffffffffffffffffffffff8116825270010000000000000000000000000000000081046effffffffffffffffffffffffffffff16938201939093527f010000000000000000000000000000000000000000000000000000000000000090920460ff1615159282019290925260018201546060820152600290910154608082015290565b60405161022c91905f60a0820190506fffffffffffffffffffffffffffffffff83511682526effffffffffffffffffffffffffffff6020840151166020830152604083015115156040830152606083015160608301526080830151608083015292915050565b6101e261063536600461398b565b6108f9565b6101e26106483660046139fc565b610934565b6101e261065b366004613a48565b6109a5565b6101e261066e366004613bb7565b6109b1565b610686610681366004613c2d565b6109bc565b60405161022c9190613f6a565b6101e26106a1366004612dd3565b610b7c565b61020b7f000000000000000000000000000000000000000000000000000000000000000081565b6104d86106db3660046140ee565b610b86565b6101e26106ee366004614118565b5f9283526020839052604090922080549215157f0100000000000000000000000000000000000000000000000000000000000000027effffffffffffffffffffffffffffff000000000000000000000000000000009093166fffffffffffffffffffffffffffffffff90921691909117919091179055565b6101e2610774366004614167565b610be1565b6101e26107873660046138fb565b5f92835260208390526040909220600181019190915560020155565b6107ae838383610dbd565b505050565b6107bd8282611000565b5050565b5f83815260208190526040812080547f0100000000000000000000000000000000000000000000000000000000000000900460ff1615610804575f915050610897565b821580159061081257508242115b15610820575f915050610897565b80546fffffffffffffffffffffffffffffffff165f03610844576001915050610897565b83158015906108835750805484906108819070010000000000000000000000000000000090046effffffffffffffffffffffffffffff16426141d7565b115b15610891575f915050610897565b60019150505b9392505050565b5f7f000000000000000000000000000000000000000000000000000000000000000046146108d4576108cf306112c7565b905090565b507f000000000000000000000000000000000000000000000000000000000000000090565b805f5b8181101561092e576109263385858481811061091a5761091a6141ea565b905060200201356113e8565b6001016108fc565b50505050565b61093c612c39565b6040805160a0810182525f808252602082018190526060820152861515608084810191909152610180840185905273ffffffffffffffffffffffffffffffffffffffff878116938301939093529185169181019190915261099d8282611560565b505050505050565b6109ae81611653565b50565b6107ae83838361182f565b6109c4612c39565b6040805160018082528183019092525f91816020015b604080518082019091525f80825260208201528152602001906001900390816109da57505060408051808201909152619abc8152612710602082015281519192509082905f90610a2c57610a2c6141ea565b60200260200101819052505f6040518060a00160405280683635c9adc5dea000008152602001610e108152602001600a81526020015f5f1b81526020015f67ffffffffffffffff811115610a8257610a82612dea565b604051908082528060200260200182016040528015610abb57816020015b610aa8612d43565b815260200190600190039081610aa05790505b509052604080516102008101825273ffffffffffffffffffffffffffffffffffffffff8b1681526bffffffffffffffffffffffff8a1660208201524691810191909152306060820152871515608082015261aaaa60a082015261bbbb60c082015261567860e082015261012c610100820152611c20610120820152610140810187905261016081018690525f6101808201529091506101a08101610b624262015180614217565b815260208101939093526040909201529695505050505050565b6109ae33826113e8565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606084901b166020820152603481018290525f906054015b6040516020818303038152906040528051906020012090505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610c22575f5ffd5b5f8080808080610c32878061422a565b610c409060a0810190614266565b810190610c4d91906142ce565b9550955095509550955095505f865f01518760200151604051602001610ca492919060609290921b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000168252601482015260340190565b604051602081830303815290604052805190602001209050610ccb8782878988888e61190a565b5f80610d388360408c0135610ce360808e018e6143a3565b808060200260200160405190810160405280939291908181526020015f905b82821015610d2e57610d1f60608302860136819003810190614406565b81526020019060010190610d02565b50505050506119d4565b91509150610d478a8c8661182f565b604080870151608080890151835173ffffffffffffffffffffffffffffffffffffffff9384168152921660208301529181018490526060810183905284917f8595ab03f1b5496cccc75833b235b1b95dee73096282d3ac851b628df24bd0f2910160405180910390a25050505050505050505050565b826101400151826040015173ffffffffffffffffffffffffffffffffffffffff161015610e475760408083015161014085015191517fc9e3198300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff909116600482015260248101919091526044015b60405180910390fd5b826101600151826040015173ffffffffffffffffffffffffffffffffffffffff161115610ecc5760408083015161016085015191517fa24e3c0700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024810191909152604401610e3e565b826080015115610f4e57816040015173ffffffffffffffffffffffffffffffffffffffff1681146107ae5760408083015190517f9994be180000000000000000000000000000000000000000000000000000000081526004810183905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b805f03610f87576040517faf458c0700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b816080015173ffffffffffffffffffffffffffffffffffffffff168111156107ae5760808201516040517ff00f44f50000000000000000000000000000000000000000000000000000000081526004810183905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b606082015173ffffffffffffffffffffffffffffffffffffffff1630146110775760608201516040517ff94e883c00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604401610e3e565b468260400151146110c35760408083015190517f24497bc30000000000000000000000000000000000000000000000000000000081526004810191909152466024820152604401610e3e565b815181516020015173ffffffffffffffffffffffffffffffffffffffff9081169116146111445780516020015182516040517fbaa8d2ad00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b8160a0015173ffffffffffffffffffffffffffffffffffffffff1681602001515f015173ffffffffffffffffffffffffffffffffffffffff16146111df5760208101515160a08301516040517f2c63212600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b6040810151515f5b8181101561092e578360c0015173ffffffffffffffffffffffffffffffffffffffff1683604001518281518110611220576112206141ea565b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff16146112bf578260400151818151811061125c5761125c6141ea565b60209081029190910101515160c08501516040517fd064a3f700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b6001016111e7565b604080518082018252600781527f444341486f6f6b0000000000000000000000000000000000000000000000000060209182015281518083018352600181527f31000000000000000000000000000000000000000000000000000000000000009082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f918101919091527fa20374b09dfce3316b287b36aa908c96cdb9329755a7e08683c5b42908026635918101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6606082015246608082015273ffffffffffffffffffffffffffffffffffffffff821660a08201525f9060c0015b604051602081830303815290604052805190602001209050919050565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606084901b166020820152603481018290525f90605401604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815291815281516020928301205f818152928390529120549091507f0100000000000000000000000000000000000000000000000000000000000000900460ff16156114c7576040517f0b1f884600000000000000000000000000000000000000000000000000000000815260048101829052602401610e3e565b5f8181526020819052604080822080547effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01000000000000000000000000000000000000000000000000000000000000001790555173ffffffffffffffffffffffffffffffffffffffff85169183917fac5a4b90e421002a2fdb9f132b9b32c24fa4ae16ec480516c85932de208d2a339190a3505050565b5f8260800151156115b9576115b2826080015173ffffffffffffffffffffffffffffffffffffffff16670de0b6b3a7640000846040015173ffffffffffffffffffffffffffffffffffffffff16611c0b565b9050611603565b611600826040015173ffffffffffffffffffffffffffffffffffffffff16670de0b6b3a7640000846080015173ffffffffffffffffffffffffffffffffffffffff16611c0b565b90505b8261018001518110156107ae576101808301516040517fba04e36d000000000000000000000000000000000000000000000000000000008152610e3e918391600401918252602082015260400190565b80515f81900361168f576040517f949efc9600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805b828110156117f0575f8482815181106116ad576116ad6141ea565b60200260200101516020015190508061ffff165f036116f8576040517fba0d87b500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f85838151811061170b5761170b6141ea565b60200260200101515f015190505f8360016117269190614217565b90505b858110156117d2578173ffffffffffffffffffffffffffffffffffffffff1687828151811061175a5761175a6141ea565b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff16036117ca576040517f3e91857900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83166004820152602401610e3e565b600101611729565b506117e161ffff831685614217565b93508260010192505050611692565b5061271081146107ae576040517fbc9dfe8c00000000000000000000000000000000000000000000000000000000815260048101829052602401610e3e565b8051156118df5773ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016632b67b57061187c858061422a565b61188d906040810190602001614420565b836020015184604001516040518463ffffffff1660e01b81526004016118b59392919061443b565b5f604051808303815f87803b1580156118cc575f5ffd5b505af19250505080156118dd575060015b505b6107ae7f00000000000000000000000000000000000000000000000000000000000000008484611d31565b611915878686611e0a565b61192787611922836144f9565b611000565b611935876101c00151611653565b611940878484611e97565b61194b868885612002565b61195a87846040840135610dbd565b6119648784611560565b6119cb878461197660808501856143a3565b808060200260200160405190810160405280939291908181526020015f905b828210156119c1576119b260608302860136819003810190614406565b81526020019060010190611995565b50505050506122a5565b50505050505050565b5f83815260208181526040808320815160a08101835281546fffffffffffffffffffffffffffffffff8116825270010000000000000000000000000000000081046effffffffffffffffffffffffffffff16948201949094527f010000000000000000000000000000000000000000000000000000000000000090930460ff161515918301919091526001810154606083015260020154608082015282518291908290815b81811015611ab457868181518110611a9357611a936141ea565b60200260200101516020015183611aaa9190614217565b9250600101611a79565b50825183611ac182614504565b6fffffffffffffffffffffffffffffffff169052506effffffffffffffffffffffffffffff42166020840152606083018051889190611b01908390614217565b905250608083018051839190611b18908390614217565b90525050505f958652602086815260409687902082518154928401519884015115157f0100000000000000000000000000000000000000000000000000000000000000027effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6effffffffffffffffffffffffffffff909a16700100000000000000000000000000000000027fff000000000000000000000000000000000000000000000000000000000000009094166fffffffffffffffffffffffffffffffff9092169190911792909217979097161786556060810151600187018190556080909101516002909601869055959350505050565b5f80807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff858709858702925082811083820303915050805f03611c6157838281611c5757611c57614540565b0492505050610897565b808411611cca576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601560248201527f4d6174683a206d756c446976206f766572666c6f7700000000000000000000006044820152606401610e3e565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b73ffffffffffffffffffffffffffffffffffffffff83166336c78516611d57848061422a565b611d68906040810190602001614420565b8360408601803590611d7d9060208901614420565b60405160e086901b7fffffffff0000000000000000000000000000000000000000000000000000000016815273ffffffffffffffffffffffffffffffffffffffff94851660048201529284166024840152908316604483015290911660648201526084015f604051808303815f87803b158015611df8575f5ffd5b505af11580156119cb573d5f5f3e3d5ffd5b5f611e1584846125dd565b90505f611e29611e2361089e565b836126dc565b9050611e39855f0151828561271d565b611e905784516040517f17aa11790000000000000000000000000000000000000000000000000000000081525f600482015273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b5050505050565b5f611ea183612731565b90505f611eaf611e2361089e565b9050611ec08560e00151828561271d565b611f1a5760e08501516040517fca5612f60000000000000000000000000000000000000000000000000000000081525f600482015273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b8451845173ffffffffffffffffffffffffffffffffffffffff908116911614611f9357835185516040517f438429d500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b846020015184602001516bffffffffffffffffffffffff1614611e9057602080850151908601516040517fbbccf88f0000000000000000000000000000000000000000000000000000000081526bffffffffffffffffffffffff90921660048301526024820152604401610e3e565b5f8381526020818152604091829020825160a08101845281546fffffffffffffffffffffffffffffffff8116825270010000000000000000000000000000000081046effffffffffffffffffffffffffffff16938201939093527f010000000000000000000000000000000000000000000000000000000000000090920460ff161580159383019390935260018101546060830152600201546080820152906120da576040517f2660161b00000000000000000000000000000000000000000000000000000000815260048101859052602401610e3e565b6101a0830151158015906120f25750826101a0015142115b15612139576101a08301516040517f6f08ee6e0000000000000000000000000000000000000000000000000000000081524260048201526024810191909152604401610e3e565b805f01516fffffffffffffffffffffffffffffffff1682606001516bffffffffffffffffffffffff16146121b857606082015181516040517f1ca8c0620000000000000000000000000000000000000000000000000000000081526bffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b80516fffffffffffffffffffffffffffffffff161561092e575f81602001516effffffffffffffffffffffffffffff16426121f391906141d7565b9050836101000151811015612245576101008401516040517fb1b9b6ee000000000000000000000000000000000000000000000000000000008152610e3e918391600401918252602082015260400190565b6101208401511580159061225d575083610120015181115b15611e90576101208401516040517f388b0173000000000000000000000000000000000000000000000000000000008152610e3e918391600401918252602082015260400190565b80515f90815b818110156122e6578381815181106122c5576122c56141ea565b602002602001015160200151836122dc9190614217565b92506001016122ab565b506101c0850151515f5b818110156124dc575f876101c001518281518110612310576123106141ea565b60200260200101515f015190505f612351868a6101c001518581518110612339576123396141ea565b60200260200101516020015161ffff16612710611c0b565b90505f805b868110156123e2578373ffffffffffffffffffffffffffffffffffffffff16898281518110612387576123876141ea565b60200260200101516040015173ffffffffffffffffffffffffffffffffffffffff16036123da578881815181106123c0576123c06141ea565b602002602001015160200151826123d79190614217565b91505b600101612356565b5089608001511561247257816123f9826001614217565b10158015612411575061240d826001614217565b8111155b61246d576040517f66c6a6af00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024810182905260448101839052606401610e3e565b6124d1565b8181146124d1576040517f66c6a6af00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024810182905260448101839052606401610e3e565b5050506001016122f0565b5085608001511561256557846080015173ffffffffffffffffffffffffffffffffffffffff168310156125605760808501516040517f2c19b8b80000000000000000000000000000000000000000000000000000000081526004810185905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b61099d565b846040015173ffffffffffffffffffffffffffffffffffffffff16831461099d5760408086015190517f7ebbdeab0000000000000000000000000000000000000000000000000000000081526004810185905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b5f5f6125ed846101c001516127c5565b90505f604051806102c00160405280610293815260200161464b61029391398051906020012090505f60405182815286516020820152602087015160408201526040870151606082015260608701516080820152608087015160a082015260a087015160c082015260c087015160e082015260e08701516101008201526101008701516101208201526101208701516101408201526101408701516101608201526101608701516101808201526101808701516101a08201526101a08701516101c0820152836101e0820152856102008201526102208120915061022081016040525080935050505092915050565b6040517f1901000000000000000000000000000000000000000000000000000000000000602082015260228101839052604281018290525f90606201610bc2565b5f61272984848461291d565b949350505050565b5f6040518060a00160405280606b81526020016148de606b913980516020918201208351848301516040808701516060880151608089015192516113cb97929391920195865273ffffffffffffffffffffffffffffffffffffffff94851660208701526bffffffffffffffffffffffff9384166040870152918416606086015290911660808401521660a082015260c00190565b80515f90818167ffffffffffffffff8111156127e3576127e3612dea565b60405190808252806020026020018201604052801561280c578160200160208202803683370190505b5090505f5b828110156128ec57604051806060016040528060368152602001614615603691398051906020012085828151811061284b5761284b6141ea565b60200260200101515f0151868381518110612868576128686141ea565b6020026020010151602001516040516020016128b19392919092835273ffffffffffffffffffffffffffffffffffffffff91909116602083015261ffff16604082015260600190565b604051602081830303815290604052805190602001208282815181106128d9576128d96141ea565b6020908102919091010152600101612811565b50806040516020016128fe919061456d565b6040516020818303038152906040528051906020012092505050919050565b5f8373ffffffffffffffffffffffffffffffffffffffff163b5f036129a2575f5f61294885856129b4565b5090925090505f816003811115612961576129616145a2565b14801561299957508573ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16145b92505050610897565b6129ad8484846129fd565b9050610897565b5f5f5f83516041036129eb576020840151604085015160608601515f1a6129dd88828585612b46565b9550955095505050506129f6565b505081515f91506002905b9250925092565b5f5f5f8573ffffffffffffffffffffffffffffffffffffffff168585604051602401612a2a9291906145cf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f1626ba7e0000000000000000000000000000000000000000000000000000000017905251612aab91906145e7565b5f60405180830381855afa9150503d805f8114612ae3576040519150601f19603f3d011682016040523d82523d5f602084013e612ae8565b606091505b5091509150818015612afc57506020815110155b8015612b3c575080517f1626ba7e0000000000000000000000000000000000000000000000000000000090612b3a90830160209081019084016145fd565b145b9695505050505050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115612b7f57505f91506003905082612c2f565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612bd0573d5f5f3e3d5ffd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff8116612c2657505f925060019150829050612c2f565b92505f91508190505b9450945094915050565b6040518061020001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f151581526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f815260200160608152602001612d3e6040518060a001604052805f81526020015f81526020015f81526020015f8152602001606081525090565b905290565b6040518060600160405280612d7f6040518060a00160405280606081526020016060815260200160608152602001606081526020015f81525090565b81525f6020820152606060409091015290565b5f5f60408385031215612da3575f5ffd5b8235915060208301356effffffffffffffffffffffffffffff81168114612dc8575f5ffd5b809150509250929050565b5f60208284031215612de3575f5ffd5b5035919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b60405290565b60405160a0810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b6040516060810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b604051610200810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b604051610120810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b60405160e0810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b6040516080810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715612f5b57612f5b612dea565b604052919050565b73ffffffffffffffffffffffffffffffffffffffff811681146109ae575f5ffd5b8035612f8f81612f63565b919050565b80358015158114612f8f575f5ffd5b5f67ffffffffffffffff821115612fbc57612fbc612dea565b5060051b60200190565b5f82601f830112612fd5575f5ffd5b8135612fe8612fe382612fa3565b612f14565b8082825260208201915060208360061b860101925085831115613009575f5ffd5b602085015b838110156130675760408188031215613025575f5ffd5b61302d612e17565b813561303881612f63565b8152602082013561ffff8116811461304e575f5ffd5b602082810191909152908452929092019160400161300e565b5095945050505050565b5f82601f830112613080575f5ffd5b8135602083015f5f67ffffffffffffffff8411156130a0576130a0612dea565b50601f83017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0166020016130d381612f14565b9150508281528583830111156130e7575f5ffd5b828260208301375f92810160200192909252509392505050565b5f82601f830112613110575f5ffd5b813561311e612fe382612fa3565b8082825260208201915060208360051b86010192508583111561313f575f5ffd5b602085015b8381101561306757803567ffffffffffffffff811115613162575f5ffd5b613171886020838a0101613071565b84525060209283019201613144565b5f60a08284031215613190575f5ffd5b613198612e40565b823581526020808401359082015260408084013590820152606080840135908201529050608082013567ffffffffffffffff8111156131d5575f5ffd5b8201601f810184136131e5575f5ffd5b80356131f3612fe382612fa3565b8082825260208201915060208360051b850101925086831115613214575f5ffd5b602084015b838110156133ad57803567ffffffffffffffff811115613237575f5ffd5b85016060818a037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001121561326a575f5ffd5b613272612e63565b602082013567ffffffffffffffff81111561328b575f5ffd5b602081840101905060a0818c0312156132a2575f5ffd5b6132aa612e40565b813567ffffffffffffffff8111156132c0575f5ffd5b6132cc8d828501613071565b825250602082013567ffffffffffffffff8111156132e8575f5ffd5b6132f48d828501613071565b602083015250604082013567ffffffffffffffff811115613313575f5ffd5b61331f8d828501613101565b604083015250606082013567ffffffffffffffff81111561333e575f5ffd5b61334a8d828501613101565b60608301525060809182013591810191909152815261336b60408301612f84565b6020820152606082013567ffffffffffffffff811115613389575f5ffd5b6133988b602083860101613071565b60408301525084525060209283019201613219565b5060808501525091949350505050565b5f61020082840312156133ce575f5ffd5b6133d6612e86565b90506133e182612f84565b8152602082810135908201526040808301359082015261340360608301612f84565b606082015261341460808301612f94565b608082015261342560a08301612f84565b60a082015261343660c08301612f84565b60c082015261344760e08301612f84565b60e0820152610100828101359082015261012080830135908201526101408083013590820152610160808301359082015261018080830135908201526101a080830135908201526101c082013567ffffffffffffffff8111156134a8575f5ffd5b6134b484828501612fc6565b6101c0830152506101e082013567ffffffffffffffff8111156134d5575f5ffd5b6134e184828501613180565b6101e08301525092915050565b80356bffffffffffffffffffffffff81168114612f8f575f5ffd5b5f60a08284031215613519575f5ffd5b613521612e40565b9050813561352e81612f63565b815261353c602083016134ee565b6020820152604082013561354f81612f63565b6040820152613560606083016134ee565b6060820152608082013561357381612f63565b608082015292915050565b5f5f5f60e08486031215613590575f5ffd5b833567ffffffffffffffff8111156135a6575f5ffd5b6135b2868287016133bd565b9350506135c28560208601613509565b9295929450505060c0919091013590565b5f61012082840312156135e4575f5ffd5b6135ec612eaa565b90506135f782612f84565b815261360560208301612f84565b6020820152604082810135908201526060808301359082015261362a60808301612f84565b608082015260a082013567ffffffffffffffff811115613648575f5ffd5b61365484828501613071565b60a08301525061366660c08301612f84565b60c082015260e082013567ffffffffffffffff811115613684575f5ffd5b61369084828501613071565b60e0830152506136a36101008301612f84565b61010082015292915050565b5f606082840312156136bf575f5ffd5b6136c7612e63565b905081356136d481612f63565b81526020828101359082015260409182013591810191909152919050565b5f60608284031215613702575f5ffd5b61370a612e63565b9050813561371781612f63565b815260208281013590820152604082013561373181612f63565b604082015292915050565b5f82601f83011261374b575f5ffd5b8135613759612fe382612fa3565b8082825260208201915060206060840286010192508583111561377a575f5ffd5b602085015b838110156130675761379187826136f2565b835260209092019160600161377f565b5f61012082840312156137b2575f5ffd5b6137ba612ece565b9050813567ffffffffffffffff8111156137d2575f5ffd5b6137de848285016135d3565b8252506137ee83602084016136af565b6020820152608082013567ffffffffffffffff81111561380c575f5ffd5b6138188482850161373c565b60408301525060a082013567ffffffffffffffff811115613837575f5ffd5b61384384828501613071565b60608301525060c0820135608082015261385f60e08301612f84565b60a082015261010082013567ffffffffffffffff81111561387e575f5ffd5b61388a84828501613071565b60c08301525092915050565b5f5f604083850312156138a7575f5ffd5b823567ffffffffffffffff8111156138bd575f5ffd5b6138c9858286016133bd565b925050602083013567ffffffffffffffff8111156138e5575f5ffd5b6138f1858286016137a1565b9150509250929050565b5f5f5f6060848603121561390d575f5ffd5b505081359360208301359350604090920135919050565b5f5f5f5f5f60a08688031215613938575f5ffd5b853561394381612f63565b9450613951602087016134ee565b9350604086013561396181612f63565b9250606086013561397181612f63565b915061397f608087016134ee565b90509295509295909350565b5f5f6020838503121561399c575f5ffd5b823567ffffffffffffffff8111156139b2575f5ffd5b8301601f810185136139c2575f5ffd5b803567ffffffffffffffff8111156139d8575f5ffd5b8560208260051b84010111156139ec575f5ffd5b6020919091019590945092505050565b5f5f5f5f60808587031215613a0f575f5ffd5b613a1885612f94565b93506020850135613a2881612f63565b92506040850135613a3881612f63565b9396929550929360600135925050565b5f60208284031215613a58575f5ffd5b813567ffffffffffffffff811115613a6e575f5ffd5b61272984828501612fc6565b5f6101208284031215613a8b575f5ffd5b50919050565b803565ffffffffffff81168114612f8f575f5ffd5b5f818303610100811215613ab8575f5ffd5b613ac0612e63565b9150613acb83612f94565b82527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00160c0811215613afc575f5ffd5b613b04612e63565b6080821215613b11575f5ffd5b613b19612ef1565b91506020840135613b2981612f63565b82526040840135613b3981612f63565b6020830152613b4a60608501613a91565b6040830152613b5b60808501613a91565b6060830152818152613b6f60a08501612f84565b60208281019190915260c085013560408301528301525060e082013567ffffffffffffffff811115613b9f575f5ffd5b613bab84828501613071565b60408301525092915050565b5f5f5f60608486031215613bc9575f5ffd5b833567ffffffffffffffff811115613bdf575f5ffd5b613beb86828701613a7a565b9350506020840135613bfc81612f63565b9150604084013567ffffffffffffffff811115613c17575f5ffd5b613c2386828701613aa6565b9150509250925092565b5f5f5f5f5f60a08688031215613c41575f5ffd5b8535613c4c81612f63565b9450613c5a602087016134ee565b9350613c6860408701612f94565b94979396509394606081013594506080013592915050565b5f8151808452602084019350602083015f5b82811015613cd6578151805173ffffffffffffffffffffffffffffffffffffffff16875260209081015161ffff168188015260409096019590910190600101613c92565b5093949350505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b83811015613d98577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0858403018852613d82838351613ce0565b6020988901989093509190910190600101613d48565b50909695505050505050565b5f60a0830182518452602083015160208501526040830151604085015260608301516060850152608083015160a0608086015281815180845260c08701915060c08160051b88010193506020830192505f5b81811015613f5e577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408886030183528351805160608752805160a06060890152613e44610100890182613ce0565b905060208201517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08983030160808a0152613e7f8282613ce0565b91505060408201517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08983030160a08a0152613ebb8282613d2c565b91505060608201517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08983030160c08a0152613ef78282613d2c565b915050608082015160e089015260208301519150613f2d602089018373ffffffffffffffffffffffffffffffffffffffff169052565b604083015192508781036040890152613f468184613ce0565b97505050602094850194939093019250600101613df6565b50929695505050505050565b60208152613f9160208201835173ffffffffffffffffffffffffffffffffffffffff169052565b60208201516040820152604082015160608201525f6060830151613fcd608084018273ffffffffffffffffffffffffffffffffffffffff169052565b50608083015180151560a08401525060a083015173ffffffffffffffffffffffffffffffffffffffff811660c08401525060c083015173ffffffffffffffffffffffffffffffffffffffff811660e08401525060e083015173ffffffffffffffffffffffffffffffffffffffff8116610100840152506101008301516101208301526101208301516101408301526101408301516101608301526101608301516101808301526101808301516101a08301526101a08301516101c08301526101c08301516102006101e08401526140a8610220840182613c80565b90506101e08401517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0848303016102008501526140e58282613da4565b95945050505050565b5f5f604083850312156140ff575f5ffd5b823561410a81612f63565b946020939093013593505050565b5f5f5f6060848603121561412a575f5ffd5b8335925060208401356fffffffffffffffffffffffffffffffff81168114614150575f5ffd5b915061415e60408501612f94565b90509250925092565b5f5f60408385031215614178575f5ffd5b823561418381612f63565b9150602083013567ffffffffffffffff81111561419e575f5ffd5b6138f185828601613a7a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115610bdb57610bdb6141aa565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115610bdb57610bdb6141aa565b5f82357ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffee183360301811261425c575f5ffd5b9190910192915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112614299575f5ffd5b83018035915067ffffffffffffffff8211156142b3575f5ffd5b6020019150368190038213156142c7575f5ffd5b9250929050565b5f5f5f5f5f5f61014087890312156142e4575f5ffd5b863567ffffffffffffffff8111156142fa575f5ffd5b61430689828a016133bd565b965050602087013567ffffffffffffffff811115614322575f5ffd5b61432e89828a01613071565b955050604087013593506143458860608901613509565b925061010087013567ffffffffffffffff811115614361575f5ffd5b61436d89828a01613071565b92505061012087013567ffffffffffffffff81111561438a575f5ffd5b61439689828a01613aa6565b9150509295509295509295565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126143d6575f5ffd5b83018035915067ffffffffffffffff8211156143f0575f5ffd5b60200191506060810236038213156142c7575f5ffd5b5f60608284031215614416575f5ffd5b61089783836136f2565b5f60208284031215614430575f5ffd5b813561089781612f63565b73ffffffffffffffffffffffffffffffffffffffff841681525f835173ffffffffffffffffffffffffffffffffffffffff815116602084015273ffffffffffffffffffffffffffffffffffffffff602082015116604084015265ffffffffffff604082015116606084015265ffffffffffff60608201511660808401525073ffffffffffffffffffffffffffffffffffffffff60208501511660a0830152604084015160c083015261010060e08301526140e5610100830184613ce0565b5f610bdb36836137a1565b5f6fffffffffffffffffffffffffffffffff82166fffffffffffffffffffffffffffffffff8103614537576145376141aa565b60010192915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b81515f90829060208501835b82811015614597578151845260209384019390910190600101614579565b509195945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b828152604060208201525f6127296040830184613ce0565b5f82518060208501845e5f920191825250919050565b5f6020828403121561460d575f5ffd5b505191905056fe4f7574707574416c6c6f636174696f6e286164647265737320726563697069656e742c75696e743136206261736973506f696e747329444341496e74656e74286164647265737320737761707065722c75696e74323536206e6f6e63652c75696e7432353620636861696e49642c6164647265737320686f6f6b416464726573732c626f6f6c2069734578616374496e2c6164647265737320696e707574546f6b656e2c61646472657373206f7574707574546f6b656e2c6164647265737320636f7369676e65722c75696e74323536206d696e506572696f642c75696e74323536206d6178506572696f642c75696e74323536206d696e4368756e6b53697a652c75696e74323536206d61784368756e6b53697a652c75696e74323536206d696e50726963652c75696e7432353620646561646c696e652c4f7574707574416c6c6f636174696f6e5b5d206f7574707574416c6c6f636174696f6e732c50726976617465496e74656e742070726976617465496e74656e742946656564496e666f284665656454656d706c617465206665656454656d706c6174652c616464726573732066656564416464726573732c737472696e67206665656454797065294665656454656d706c61746528737472696e67206e616d652c737472696e672065787072657373696f6e2c737472696e675b5d20706172616d65746572732c737472696e675b5d20736563726574732c75696e74323536207265747279436f756e74294f7574707574416c6c6f636174696f6e286164647265737320726563697069656e742c75696e743136206261736973506f696e74732950726976617465496e74656e742875696e7432353620746f74616c416d6f756e742c75696e743235362065786163744672657175656e63792c75696e74323536206e756d4368756e6b732c627974657333322073616c742c46656564496e666f5b5d206f7261636c654665656473294443414f72646572436f7369676e657244617461286164647265737320737761707065722c75696e743936206e6f6e63652c75696e743136302065786563416d6f756e742c75696e743936206f726465724e6f6e63652c75696e74313630206c696d6974416d6f756e7429a26469706673582212205c61877b5f80d1b707a9c4ef9c78458d1e6e27cd18a81ef2576a787e3a75a67964736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@QaKG8\x03\x80aKG\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01IV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x80R\x81\x16`\xA0RF`\xE0R\x81\x81a\x01%0`@\x80Q\x80\x82\x01\x82R`\x07\x81RfDCAHook`\xC8\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\xA2\x03t\xB0\x9D\xFC\xE31k({6\xAA\x90\x8C\x96\xCD\xB92\x97U\xA7\xE0\x86\x83\xC5\xB4)\x08\x02f5\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`\xA0\x80\x85\x01\x91\x90\x91R\x82Q\x80\x85\x03\x90\x91\x01\x81R`\xC0\x90\x93\x01\x90\x91R\x81Q\x91\x01 \x90V[`\xC0RPa\x01\x81\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01FW__\xFD[PV[__`@\x83\x85\x03\x12\x15a\x01ZW__\xFD[\x82Qa\x01e\x81a\x012V[` \x84\x01Q\x90\x92Pa\x01v\x81a\x012V[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0QaI~a\x01\xC9_9_a\x08\xA1\x01R_a\x08\xD7\x01R_\x81\x81a\x06\xAB\x01Ra\x0B\xF9\x01R_\x81\x81a\x01\xE9\x01R\x81\x81a\x18M\x01Ra\x18\xE4\x01RaI~_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01nW_5`\xE0\x1C\x80c^)\xFA7\x11a\0\xD2W\x80c\xA0\xA3\x1A\xAC\x11a\0\x88W\x80c\xB2\x02\xA7\xF3\x11a\0cW\x80c\xB2\x02\xA7\xF3\x14a\x06\xE0W\x80c\xD8\xB6\x1D\xED\x14a\x07fW\x80c\xFEx#\xAC\x14a\x07yW__\xFD[\x80c\xA0\xA3\x1A\xAC\x14a\x06\x93W\x80c\xABW&P\x14a\x06\xA6W\x80c\xB1\xC19\x08\x14a\x06\xCDW__\xFD[\x80c\x81\x84\xE3S\x11a\0\xB8W\x80c\x81\x84\xE3S\x14a\x06MW\x80c\x83E\xEBV\x14a\x06`W\x80c\x83\xBCj\xB6\x14a\x06sW__\xFD[\x80c^)\xFA7\x14a\x06'W\x80c\x80\xCB\x1BP\x14a\x06:W__\xFD[\x80c/\xD0\x10\x9B\x11a\x01'W\x80c0\x8E\xA6\xC9\x11a\x01\rW\x80c0\x8E\xA6\xC9\x14a\x03\xA6W\x80c6D\xE5\x15\x14a\x04\xD0W\x80cI\xD8\x03>\x14a\x04\xE6W__\xFD[\x80c/\xD0\x10\x9B\x14a\x03pW\x80c0y\0\x81\x14a\x03\x83W__\xFD[\x80c\x16\xB8S\xA2\x11a\x01WW\x80c\x16\xB8S\xA2\x14a\x025W\x80c\x1C\xE2M\x02\x14a\x03\x0FW\x80c)\xC8\xADN\x14a\x03]W__\xFD[\x80c\x02\t\xE7\x10\x14a\x01rW\x80c\x12&\x1E\xE7\x14a\x01\xE4W[__\xFD[a\x01\xE2a\x01\x806`\x04a-\x92V[_\x91\x82R` \x82\x90R`@\x90\x91 \x80Tn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[a\x02\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xEFa\x02C6`\x04a-\xD3V[_\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x83Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x85\x90R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15\x94\x82\x01\x94\x90\x94R`\x01\x82\x01T``\x82\x01\x81\x90R`\x02\x90\x92\x01T`\x80\x90\x91\x01\x81\x90R\x92\x93\x90\x92\x91\x90V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02,V[a\x03@a\x03\x1D6`\x04a-\xD3V[_\x90\x81R` \x81\x90R`@\x90 To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02,V[a\x01\xE2a\x03k6`\x04a5~V[a\x07\xA3V[a\x01\xE2a\x03~6`\x04a8\x96V[a\x07\xB3V[a\x03\x96a\x03\x916`\x04a8\xFBV[a\x07\xC1V[`@Q\x90\x15\x15\x81R` \x01a\x02,V[a\x041a\x03\xB46`\x04a9$V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x82\x01R\x93\x86\x16\x90\x84\x01R\x90\x92\x16``\x82\x01R\x91\x16`\x80\x82\x01R\x90V[`@Qa\x02,\x91\x90_`\xA0\x82\x01\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x82Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x01Q\x16`@\x83\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x84\x01Q\x16``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[a\x04\xD8a\x08\x9EV[`@Q\x90\x81R` \x01a\x02,V[a\x05\xC1a\x04\xF46`\x04a-\xD3V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP_\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x82\x01\x93\x90\x93R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04`\xFF\x16\x15\x15\x92\x82\x01\x92\x90\x92R`\x01\x82\x01T``\x82\x01R`\x02\x90\x91\x01T`\x80\x82\x01R\x90V[`@Qa\x02,\x91\x90_`\xA0\x82\x01\x90Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x82Rn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q\x15\x15`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R\x92\x91PPV[a\x01\xE2a\x0656`\x04a9\x8BV[a\x08\xF9V[a\x01\xE2a\x06H6`\x04a9\xFCV[a\t4V[a\x01\xE2a\x06[6`\x04a:HV[a\t\xA5V[a\x01\xE2a\x06n6`\x04a;\xB7V[a\t\xB1V[a\x06\x86a\x06\x816`\x04a<-V[a\t\xBCV[`@Qa\x02,\x91\x90a?jV[a\x01\xE2a\x06\xA16`\x04a-\xD3V[a\x0B|V[a\x02\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xD8a\x06\xDB6`\x04a@\xEEV[a\x0B\x86V[a\x01\xE2a\x06\xEE6`\x04aA\x18V[_\x92\x83R` \x83\x90R`@\x90\x92 \x80T\x92\x15\x15\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UV[a\x01\xE2a\x07t6`\x04aAgV[a\x0B\xE1V[a\x01\xE2a\x07\x876`\x04a8\xFBV[_\x92\x83R` \x83\x90R`@\x90\x92 `\x01\x81\x01\x91\x90\x91U`\x02\x01UV[a\x07\xAE\x83\x83\x83a\r\xBDV[PPPV[a\x07\xBD\x82\x82a\x10\0V[PPV[_\x83\x81R` \x81\x90R`@\x81 \x80T\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x08\x04W_\x91PPa\x08\x97V[\x82\x15\x80\x15\x90a\x08\x12WP\x82B\x11[\x15a\x08 W_\x91PPa\x08\x97V[\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x08DW`\x01\x91PPa\x08\x97V[\x83\x15\x80\x15\x90a\x08\x83WP\x80T\x84\x90a\x08\x81\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaA\xD7V[\x11[\x15a\x08\x91W_\x91PPa\x08\x97V[`\x01\x91PP[\x93\x92PPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x08\xD4Wa\x08\xCF0a\x12\xC7V[\x90P\x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[\x80_[\x81\x81\x10\x15a\t.Wa\t&3\x85\x85\x84\x81\x81\x10a\t\x1AWa\t\x1AaA\xEAV[\x90P` \x02\x015a\x13\xE8V[`\x01\x01a\x08\xFCV[PPPPV[a\t<a,9V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R``\x82\x01R\x86\x15\x15`\x80\x84\x81\x01\x91\x90\x91Ra\x01\x80\x84\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16\x93\x83\x01\x93\x90\x93R\x91\x85\x16\x91\x81\x01\x91\x90\x91Ra\t\x9D\x82\x82a\x15`V[PPPPPPV[a\t\xAE\x81a\x16SV[PV[a\x07\xAE\x83\x83\x83a\x18/V[a\t\xC4a,9V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xDAWPP`@\x80Q\x80\x82\x01\x90\x91Ra\x9A\xBC\x81Ra'\x10` \x82\x01R\x81Q\x91\x92P\x90\x82\x90_\x90a\n,Wa\n,aA\xEAV[` \x02` \x01\x01\x81\x90RP_`@Q\x80`\xA0\x01`@R\x80h65\xC9\xAD\xC5\xDE\xA0\0\0\x81R` \x01a\x0E\x10\x81R` \x01`\n\x81R` \x01__\x1B\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x82Wa\n\x82a-\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xBBW\x81` \x01[a\n\xA8a-CV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xA0W\x90P[P\x90R`@\x80Qa\x02\0\x81\x01\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16` \x82\x01RF\x91\x81\x01\x91\x90\x91R0``\x82\x01R\x87\x15\x15`\x80\x82\x01Ra\xAA\xAA`\xA0\x82\x01Ra\xBB\xBB`\xC0\x82\x01RaVx`\xE0\x82\x01Ra\x01,a\x01\0\x82\x01Ra\x1C a\x01 \x82\x01Ra\x01@\x81\x01\x87\x90Ra\x01`\x81\x01\x86\x90R_a\x01\x80\x82\x01R\x90\x91Pa\x01\xA0\x81\x01a\x0BbBb\x01Q\x80aB\x17V[\x81R` \x81\x01\x93\x90\x93R`@\x90\x92\x01R\x96\x95PPPPPPV[a\t\xAE3\x82a\x13\xE8V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16` \x82\x01R`4\x81\x01\x82\x90R_\x90`T\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\"W__\xFD[_\x80\x80\x80\x80\x80a\x0C2\x87\x80aB*V[a\x0C@\x90`\xA0\x81\x01\x90aBfV[\x81\x01\x90a\x0CM\x91\x90aB\xCEV[\x95P\x95P\x95P\x95P\x95P\x95P_\x86_\x01Q\x87` \x01Q`@Q` \x01a\x0C\xA4\x92\x91\x90``\x92\x90\x92\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`\x14\x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0C\xCB\x87\x82\x87\x89\x88\x88\x8Ea\x19\nV[_\x80a\r8\x83`@\x8C\x015a\x0C\xE3`\x80\x8E\x01\x8EaC\xA3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\r.Wa\r\x1F``\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aD\x06V[\x81R` \x01\x90`\x01\x01\x90a\r\x02V[PPPPPa\x19\xD4V[\x91P\x91Pa\rG\x8A\x8C\x86a\x18/V[`@\x80\x87\x01Q`\x80\x80\x89\x01Q\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x16` \x83\x01R\x91\x81\x01\x84\x90R``\x81\x01\x83\x90R\x84\x91\x7F\x85\x95\xAB\x03\xF1\xB5Il\xCC\xC7X3\xB25\xB1\xB9]\xEEs\tb\x82\xD3\xAC\x85\x1Bb\x8D\xF2K\xD0\xF2\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPV[\x82a\x01@\x01Q\x82`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x0EGW`@\x80\x83\x01Qa\x01@\x85\x01Q\x91Q\x7F\xC9\xE3\x19\x83\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01[`@Q\x80\x91\x03\x90\xFD[\x82a\x01`\x01Q\x82`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0E\xCCW`@\x80\x83\x01Qa\x01`\x85\x01Q\x91Q\x7F\xA2N<\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x0E>V[\x82`\x80\x01Q\x15a\x0FNW\x81`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x14a\x07\xAEW`@\x80\x83\x01Q\x90Q\x7F\x99\x94\xBE\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x80_\x03a\x0F\x87W`@Q\x7F\xAFE\x8C\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x07\xAEW`\x80\x82\x01Q`@Q\x7F\xF0\x0FD\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[``\x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x10wW``\x82\x01Q`@Q\x7F\xF9N\x88<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x01a\x0E>V[F\x82`@\x01Q\x14a\x10\xC3W`@\x80\x83\x01Q\x90Q\x7F$I{\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91RF`$\x82\x01R`D\x01a\x0E>V[\x81Q\x81Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x11DW\x80Q` \x01Q\x82Q`@Q\x7F\xBA\xA8\xD2\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x81`\xA0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11\xDFW` \x81\x01QQ`\xA0\x83\x01Q`@Q\x7F,c!&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[`@\x81\x01QQ_[\x81\x81\x10\x15a\t.W\x83`\xC0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@\x01Q\x82\x81Q\x81\x10a\x12 Wa\x12 aA\xEAV[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x12\xBFW\x82`@\x01Q\x81\x81Q\x81\x10a\x12\\Wa\x12\\aA\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`\xC0\x85\x01Q`@Q\x7F\xD0d\xA3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[`\x01\x01a\x11\xE7V[`@\x80Q\x80\x82\x01\x82R`\x07\x81R\x7FDCAHook\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x91\x81\x01\x91\x90\x91R\x7F\xA2\x03t\xB0\x9D\xFC\xE31k({6\xAA\x90\x8C\x96\xCD\xB92\x97U\xA7\xE0\x86\x83\xC5\xB4)\x08\x02f5\x91\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\xA0\x82\x01R_\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16` \x82\x01R`4\x81\x01\x82\x90R_\x90`T\x01`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R\x92\x83\x90R\x91 T\x90\x91P\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x14\xC7W`@Q\x7F\x0B\x1F\x88F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0E>V[_\x81\x81R` \x81\x90R`@\x80\x82 \x80T~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91\x83\x91\x7F\xACZK\x90\xE4!\0*/\xDB\x9F\x13+\x9B2\xC2O\xA4\xAE\x16\xECH\x05\x16\xC8Y2\xDE \x8D*3\x91\x90\xA3PPPV[_\x82`\x80\x01Q\x15a\x15\xB9Wa\x15\xB2\x82`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\r\xE0\xB6\xB3\xA7d\0\0\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C\x0BV[\x90Pa\x16\x03V[a\x16\0\x82`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\r\xE0\xB6\xB3\xA7d\0\0\x84`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C\x0BV[\x90P[\x82a\x01\x80\x01Q\x81\x10\x15a\x07\xAEWa\x01\x80\x83\x01Q`@Q\x7F\xBA\x04\xE3m\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0E>\x91\x83\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80Q_\x81\x90\x03a\x16\x8FW`@Q\x7F\x94\x9E\xFC\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82\x81\x10\x15a\x17\xF0W_\x84\x82\x81Q\x81\x10a\x16\xADWa\x16\xADaA\xEAV[` \x02` \x01\x01Q` \x01Q\x90P\x80a\xFF\xFF\x16_\x03a\x16\xF8W`@Q\x7F\xBA\r\x87\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x85\x83\x81Q\x81\x10a\x17\x0BWa\x17\x0BaA\xEAV[` \x02` \x01\x01Q_\x01Q\x90P_\x83`\x01a\x17&\x91\x90aB\x17V[\x90P[\x85\x81\x10\x15a\x17\xD2W\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x82\x81Q\x81\x10a\x17ZWa\x17ZaA\xEAV[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xCAW`@Q\x7F>\x91\x85y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x0E>V[`\x01\x01a\x17)V[Pa\x17\xE1a\xFF\xFF\x83\x16\x85aB\x17V[\x93P\x82`\x01\x01\x92PPPa\x16\x92V[Pa'\x10\x81\x14a\x07\xAEW`@Q\x7F\xBC\x9D\xFE\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0E>V[\x80Q\x15a\x18\xDFWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c+g\xB5pa\x18|\x85\x80aB*V[a\x18\x8D\x90`@\x81\x01\x90` \x01aD V[\x83` \x01Q\x84`@\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xB5\x93\x92\x91\x90aD;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xCCW__\xFD[PZ\xF1\x92PPP\x80\x15a\x18\xDDWP`\x01[P[a\x07\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\x1D1V[a\x19\x15\x87\x86\x86a\x1E\nV[a\x19'\x87a\x19\"\x83aD\xF9V[a\x10\0V[a\x195\x87a\x01\xC0\x01Qa\x16SV[a\x19@\x87\x84\x84a\x1E\x97V[a\x19K\x86\x88\x85a \x02V[a\x19Z\x87\x84`@\x84\x015a\r\xBDV[a\x19d\x87\x84a\x15`V[a\x19\xCB\x87\x84a\x19v`\x80\x85\x01\x85aC\xA3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x19\xC1Wa\x19\xB2``\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aD\x06V[\x81R` \x01\x90`\x01\x01\x90a\x19\x95V[PPPPPa\"\xA5V[PPPPPPPV[_\x83\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94\x82\x01\x94\x90\x94R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x04`\xFF\x16\x15\x15\x91\x83\x01\x91\x90\x91R`\x01\x81\x01T``\x83\x01R`\x02\x01T`\x80\x82\x01R\x82Q\x82\x91\x90\x82\x90\x81[\x81\x81\x10\x15a\x1A\xB4W\x86\x81\x81Q\x81\x10a\x1A\x93Wa\x1A\x93aA\xEAV[` \x02` \x01\x01Q` \x01Q\x83a\x1A\xAA\x91\x90aB\x17V[\x92P`\x01\x01a\x1AyV[P\x82Q\x83a\x1A\xC1\x82aE\x04V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16` \x84\x01R``\x83\x01\x80Q\x88\x91\x90a\x1B\x01\x90\x83\x90aB\x17V[\x90RP`\x80\x83\x01\x80Q\x83\x91\x90a\x1B\x18\x90\x83\x90aB\x17V[\x90RPPP_\x95\x86R` \x86\x81R`@\x96\x87\x90 \x82Q\x81T\x92\x84\x01Q\x98\x84\x01Q\x15\x15\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x9A\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x97\x90\x97\x16\x17\x86U``\x81\x01Q`\x01\x87\x01\x81\x90U`\x80\x90\x91\x01Q`\x02\x90\x96\x01\x86\x90U\x95\x93PPPPV[_\x80\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a\x1CaW\x83\x82\x81a\x1CWWa\x1CWaE@V[\x04\x92PPPa\x08\x97V[\x80\x84\x11a\x1C\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FMath: mulDiv overflow\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E>V[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16c6\xC7\x85\x16a\x1DW\x84\x80aB*V[a\x1Dh\x90`@\x81\x01\x90` \x01aD V[\x83`@\x86\x01\x805\x90a\x1D}\x90` \x89\x01aD V[`@Q`\xE0\x86\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16`\x04\x82\x01R\x92\x84\x16`$\x84\x01R\x90\x83\x16`D\x83\x01R\x90\x91\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\xF8W__\xFD[PZ\xF1\x15\x80\x15a\x19\xCBW=__>=_\xFD[_a\x1E\x15\x84\x84a%\xDDV[\x90P_a\x1E)a\x1E#a\x08\x9EV[\x83a&\xDCV[\x90Pa\x1E9\x85_\x01Q\x82\x85a'\x1DV[a\x1E\x90W\x84Q`@Q\x7F\x17\xAA\x11y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[PPPPPV[_a\x1E\xA1\x83a'1V[\x90P_a\x1E\xAFa\x1E#a\x08\x9EV[\x90Pa\x1E\xC0\x85`\xE0\x01Q\x82\x85a'\x1DV[a\x1F\x1AW`\xE0\x85\x01Q`@Q\x7F\xCAV\x12\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x84Q\x84Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1F\x93W\x83Q\x85Q`@Q\x7FC\x84)\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x84` \x01Q\x84` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1E\x90W` \x80\x85\x01Q\x90\x86\x01Q`@Q\x7F\xBB\xCC\xF8\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x0E>V[_\x83\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x82\x01\x93\x90\x93R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04`\xFF\x16\x15\x80\x15\x93\x83\x01\x93\x90\x93R`\x01\x81\x01T``\x83\x01R`\x02\x01T`\x80\x82\x01R\x90a \xDAW`@Q\x7F&`\x16\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90R`$\x01a\x0E>V[a\x01\xA0\x83\x01Q\x15\x80\x15\x90a \xF2WP\x82a\x01\xA0\x01QB\x11[\x15a!9Wa\x01\xA0\x83\x01Q`@Q\x7Fo\x08\xEEn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RB`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x0E>V[\x80_\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a!\xB8W``\x82\x01Q\x81Q`@Q\x7F\x1C\xA8\xC0b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\t.W_\x81` \x01Qn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba!\xF3\x91\x90aA\xD7V[\x90P\x83a\x01\0\x01Q\x81\x10\x15a\"EWa\x01\0\x84\x01Q`@Q\x7F\xB1\xB9\xB6\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0E>\x91\x83\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[a\x01 \x84\x01Q\x15\x80\x15\x90a\"]WP\x83a\x01 \x01Q\x81\x11[\x15a\x1E\x90Wa\x01 \x84\x01Q`@Q\x7F8\x8B\x01s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0E>\x91\x83\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80Q_\x90\x81[\x81\x81\x10\x15a\"\xE6W\x83\x81\x81Q\x81\x10a\"\xC5Wa\"\xC5aA\xEAV[` \x02` \x01\x01Q` \x01Q\x83a\"\xDC\x91\x90aB\x17V[\x92P`\x01\x01a\"\xABV[Pa\x01\xC0\x85\x01QQ_[\x81\x81\x10\x15a$\xDCW_\x87a\x01\xC0\x01Q\x82\x81Q\x81\x10a#\x10Wa#\x10aA\xEAV[` \x02` \x01\x01Q_\x01Q\x90P_a#Q\x86\x8Aa\x01\xC0\x01Q\x85\x81Q\x81\x10a#9Wa#9aA\xEAV[` \x02` \x01\x01Q` \x01Qa\xFF\xFF\x16a'\x10a\x1C\x0BV[\x90P_\x80[\x86\x81\x10\x15a#\xE2W\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x82\x81Q\x81\x10a#\x87Wa#\x87aA\xEAV[` \x02` \x01\x01Q`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a#\xDAW\x88\x81\x81Q\x81\x10a#\xC0Wa#\xC0aA\xEAV[` \x02` \x01\x01Q` \x01Q\x82a#\xD7\x91\x90aB\x17V[\x91P[`\x01\x01a#VV[P\x89`\x80\x01Q\x15a$rW\x81a#\xF9\x82`\x01aB\x17V[\x10\x15\x80\x15a$\x11WPa$\r\x82`\x01aB\x17V[\x81\x11\x15[a$mW`@Q\x7Ff\xC6\xA6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x0E>V[a$\xD1V[\x81\x81\x14a$\xD1W`@Q\x7Ff\xC6\xA6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x0E>V[PPP`\x01\x01a\"\xF0V[P\x85`\x80\x01Q\x15a%eW\x84`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x10\x15a%`W`\x80\x85\x01Q`@Q\x7F,\x19\xB8\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[a\t\x9DV[\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x14a\t\x9DW`@\x80\x86\x01Q\x90Q\x7F~\xBB\xDE\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[__a%\xED\x84a\x01\xC0\x01Qa'\xC5V[\x90P_`@Q\x80a\x02\xC0\x01`@R\x80a\x02\x93\x81R` \x01aFKa\x02\x93\x919\x80Q\x90` \x01 \x90P_`@Q\x82\x81R\x86Q` \x82\x01R` \x87\x01Q`@\x82\x01R`@\x87\x01Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`\x80\x87\x01Q`\xA0\x82\x01R`\xA0\x87\x01Q`\xC0\x82\x01R`\xC0\x87\x01Q`\xE0\x82\x01R`\xE0\x87\x01Qa\x01\0\x82\x01Ra\x01\0\x87\x01Qa\x01 \x82\x01Ra\x01 \x87\x01Qa\x01@\x82\x01Ra\x01@\x87\x01Qa\x01`\x82\x01Ra\x01`\x87\x01Qa\x01\x80\x82\x01Ra\x01\x80\x87\x01Qa\x01\xA0\x82\x01Ra\x01\xA0\x87\x01Qa\x01\xC0\x82\x01R\x83a\x01\xE0\x82\x01R\x85a\x02\0\x82\x01Ra\x02 \x81 \x91Pa\x02 \x81\x01`@RP\x80\x93PPPP\x92\x91PPV[`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R_\x90`b\x01a\x0B\xC2V[_a')\x84\x84\x84a)\x1DV[\x94\x93PPPPV[_`@Q\x80`\xA0\x01`@R\x80`k\x81R` \x01aH\xDE`k\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q\x92Qa\x13\xCB\x97\x92\x93\x91\x92\x01\x95\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`@\x87\x01R\x91\x84\x16``\x86\x01R\x90\x91\x16`\x80\x84\x01R\x16`\xA0\x82\x01R`\xC0\x01\x90V[\x80Q_\x90\x81\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xE3Wa'\xE3a-\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\x0CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a(\xECW`@Q\x80``\x01`@R\x80`6\x81R` \x01aF\x15`6\x919\x80Q\x90` \x01 \x85\x82\x81Q\x81\x10a(KWa(KaA\xEAV[` \x02` \x01\x01Q_\x01Q\x86\x83\x81Q\x81\x10a(hWa(haA\xEAV[` \x02` \x01\x01Q` \x01Q`@Q` \x01a(\xB1\x93\x92\x91\x90\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01Ra\xFF\xFF\x16`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a(\xD9Wa(\xD9aA\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a(\x11V[P\x80`@Q` \x01a(\xFE\x91\x90aEmV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92PPP\x91\x90PV[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;_\x03a)\xA2W__a)H\x85\x85a)\xB4V[P\x90\x92P\x90P_\x81`\x03\x81\x11\x15a)aWa)aaE\xA2V[\x14\x80\x15a)\x99WP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x92PPPa\x08\x97V[a)\xAD\x84\x84\x84a)\xFDV[\x90Pa\x08\x97V[___\x83Q`A\x03a)\xEBW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa)\xDD\x88\x82\x85\x85a+FV[\x95P\x95P\x95PPPPa)\xF6V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[___\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85`@Q`$\x01a**\x92\x91\x90aE\xCFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa*\xAB\x91\x90aE\xE7V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a*\xE3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a*\xE8V[``\x91P[P\x91P\x91P\x81\x80\x15a*\xFCWP` \x81Q\x10\x15[\x80\x15a+<WP\x80Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a+:\x90\x83\x01` \x90\x81\x01\x90\x84\x01aE\xFDV[\x14[\x96\x95PPPPPPV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a+\x7FWP_\x91P`\x03\x90P\x82a,/V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a+\xD0W=__>=_\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a,&WP_\x92P`\x01\x91P\x82\x90Pa,/V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`@Q\x80a\x02\0\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01``\x81R` \x01a->`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80a-\x7F`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81RP\x90V[\x81R_` \x82\x01R```@\x90\x91\x01R\x90V[__`@\x83\x85\x03\x12\x15a-\xA3W__\xFD[\x825\x91P` \x83\x015n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-\xC8W__\xFD[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a-\xE3W__\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Qa\x02\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/[Wa/[a-\xEAV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xAEW__\xFD[\x805a/\x8F\x81a/cV[\x91\x90PV[\x805\x80\x15\x15\x81\x14a/\x8FW__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/\xBCWa/\xBCa-\xEAV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a/\xD5W__\xFD[\x815a/\xE8a/\xE3\x82a/\xA3V[a/\x14V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a0\tW__\xFD[` \x85\x01[\x83\x81\x10\x15a0gW`@\x81\x88\x03\x12\x15a0%W__\xFD[a0-a.\x17V[\x815a08\x81a/cV[\x81R` \x82\x015a\xFF\xFF\x81\x16\x81\x14a0NW__\xFD[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01a0\x0EV[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12a0\x80W__\xFD[\x815` \x83\x01__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a0\xA0Wa0\xA0a-\xEAV[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a0\xD3\x81a/\x14V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15a0\xE7W__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a1\x10W__\xFD[\x815a1\x1Ea/\xE3\x82a/\xA3V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a1?W__\xFD[` \x85\x01[\x83\x81\x10\x15a0gW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1bW__\xFD[a1q\x88` \x83\x8A\x01\x01a0qV[\x84RP` \x92\x83\x01\x92\x01a1DV[_`\xA0\x82\x84\x03\x12\x15a1\x90W__\xFD[a1\x98a.@V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R``\x80\x84\x015\x90\x82\x01R\x90P`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xD5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1\xE5W__\xFD[\x805a1\xF3a/\xE3\x82a/\xA3V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a2\x14W__\xFD[` \x84\x01[\x83\x81\x10\x15a3\xADW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a27W__\xFD[\x85\x01``\x81\x8A\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x12\x15a2jW__\xFD[a2ra.cV[` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x8BW__\xFD[` \x81\x84\x01\x01\x90P`\xA0\x81\x8C\x03\x12\x15a2\xA2W__\xFD[a2\xAAa.@V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xC0W__\xFD[a2\xCC\x8D\x82\x85\x01a0qV[\x82RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xE8W__\xFD[a2\xF4\x8D\x82\x85\x01a0qV[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x13W__\xFD[a3\x1F\x8D\x82\x85\x01a1\x01V[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3>W__\xFD[a3J\x8D\x82\x85\x01a1\x01V[``\x83\x01RP`\x80\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x81Ra3k`@\x83\x01a/\x84V[` \x82\x01R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x89W__\xFD[a3\x98\x8B` \x83\x86\x01\x01a0qV[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01a2\x19V[P`\x80\x85\x01RP\x91\x94\x93PPPPV[_a\x02\0\x82\x84\x03\x12\x15a3\xCEW__\xFD[a3\xD6a.\x86V[\x90Pa3\xE1\x82a/\x84V[\x81R` \x82\x81\x015\x90\x82\x01R`@\x80\x83\x015\x90\x82\x01Ra4\x03``\x83\x01a/\x84V[``\x82\x01Ra4\x14`\x80\x83\x01a/\x94V[`\x80\x82\x01Ra4%`\xA0\x83\x01a/\x84V[`\xA0\x82\x01Ra46`\xC0\x83\x01a/\x84V[`\xC0\x82\x01Ra4G`\xE0\x83\x01a/\x84V[`\xE0\x82\x01Ra\x01\0\x82\x81\x015\x90\x82\x01Ra\x01 \x80\x83\x015\x90\x82\x01Ra\x01@\x80\x83\x015\x90\x82\x01Ra\x01`\x80\x83\x015\x90\x82\x01Ra\x01\x80\x80\x83\x015\x90\x82\x01Ra\x01\xA0\x80\x83\x015\x90\x82\x01Ra\x01\xC0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xA8W__\xFD[a4\xB4\x84\x82\x85\x01a/\xC6V[a\x01\xC0\x83\x01RPa\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xD5W__\xFD[a4\xE1\x84\x82\x85\x01a1\x80V[a\x01\xE0\x83\x01RP\x92\x91PPV[\x805k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\x8FW__\xFD[_`\xA0\x82\x84\x03\x12\x15a5\x19W__\xFD[a5!a.@V[\x90P\x815a5.\x81a/cV[\x81Ra5<` \x83\x01a4\xEEV[` \x82\x01R`@\x82\x015a5O\x81a/cV[`@\x82\x01Ra5```\x83\x01a4\xEEV[``\x82\x01R`\x80\x82\x015a5s\x81a/cV[`\x80\x82\x01R\x92\x91PPV[___`\xE0\x84\x86\x03\x12\x15a5\x90W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xA6W__\xFD[a5\xB2\x86\x82\x87\x01a3\xBDV[\x93PPa5\xC2\x85` \x86\x01a5\tV[\x92\x95\x92\x94PPP`\xC0\x91\x90\x91\x015\x90V[_a\x01 \x82\x84\x03\x12\x15a5\xE4W__\xFD[a5\xECa.\xAAV[\x90Pa5\xF7\x82a/\x84V[\x81Ra6\x05` \x83\x01a/\x84V[` \x82\x01R`@\x82\x81\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01Ra6*`\x80\x83\x01a/\x84V[`\x80\x82\x01R`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6HW__\xFD[a6T\x84\x82\x85\x01a0qV[`\xA0\x83\x01RPa6f`\xC0\x83\x01a/\x84V[`\xC0\x82\x01R`\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x84W__\xFD[a6\x90\x84\x82\x85\x01a0qV[`\xE0\x83\x01RPa6\xA3a\x01\0\x83\x01a/\x84V[a\x01\0\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a6\xBFW__\xFD[a6\xC7a.cV[\x90P\x815a6\xD4\x81a/cV[\x81R` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[_``\x82\x84\x03\x12\x15a7\x02W__\xFD[a7\na.cV[\x90P\x815a7\x17\x81a/cV[\x81R` \x82\x81\x015\x90\x82\x01R`@\x82\x015a71\x81a/cV[`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a7KW__\xFD[\x815a7Ya/\xE3\x82a/\xA3V[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x86\x01\x01\x92P\x85\x83\x11\x15a7zW__\xFD[` \x85\x01[\x83\x81\x10\x15a0gWa7\x91\x87\x82a6\xF2V[\x83R` \x90\x92\x01\x91``\x01a7\x7FV[_a\x01 \x82\x84\x03\x12\x15a7\xB2W__\xFD[a7\xBAa.\xCEV[\x90P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xD2W__\xFD[a7\xDE\x84\x82\x85\x01a5\xD3V[\x82RPa7\xEE\x83` \x84\x01a6\xAFV[` \x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\x0CW__\xFD[a8\x18\x84\x82\x85\x01a7<V[`@\x83\x01RP`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a87W__\xFD[a8C\x84\x82\x85\x01a0qV[``\x83\x01RP`\xC0\x82\x015`\x80\x82\x01Ra8_`\xE0\x83\x01a/\x84V[`\xA0\x82\x01Ra\x01\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8~W__\xFD[a8\x8A\x84\x82\x85\x01a0qV[`\xC0\x83\x01RP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a8\xA7W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xBDW__\xFD[a8\xC9\x85\x82\x86\x01a3\xBDV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xE5W__\xFD[a8\xF1\x85\x82\x86\x01a7\xA1V[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a9\rW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[_____`\xA0\x86\x88\x03\x12\x15a98W__\xFD[\x855a9C\x81a/cV[\x94Pa9Q` \x87\x01a4\xEEV[\x93P`@\x86\x015a9a\x81a/cV[\x92P``\x86\x015a9q\x81a/cV[\x91Pa9\x7F`\x80\x87\x01a4\xEEV[\x90P\x92\x95P\x92\x95\x90\x93PV[__` \x83\x85\x03\x12\x15a9\x9CW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xB2W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a9\xC2W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xD8W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a9\xECW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[____`\x80\x85\x87\x03\x12\x15a:\x0FW__\xFD[a:\x18\x85a/\x94V[\x93P` \x85\x015a:(\x81a/cV[\x92P`@\x85\x015a:8\x81a/cV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_` \x82\x84\x03\x12\x15a:XW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:nW__\xFD[a')\x84\x82\x85\x01a/\xC6V[_a\x01 \x82\x84\x03\x12\x15a:\x8BW__\xFD[P\x91\x90PV[\x805e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\x8FW__\xFD[_\x81\x83\x03a\x01\0\x81\x12\x15a:\xB8W__\xFD[a:\xC0a.cV[\x91Pa:\xCB\x83a/\x94V[\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01`\xC0\x81\x12\x15a:\xFCW__\xFD[a;\x04a.cV[`\x80\x82\x12\x15a;\x11W__\xFD[a;\x19a.\xF1V[\x91P` \x84\x015a;)\x81a/cV[\x82R`@\x84\x015a;9\x81a/cV[` \x83\x01Ra;J``\x85\x01a:\x91V[`@\x83\x01Ra;[`\x80\x85\x01a:\x91V[``\x83\x01R\x81\x81Ra;o`\xA0\x85\x01a/\x84V[` \x82\x81\x01\x91\x90\x91R`\xC0\x85\x015`@\x83\x01R\x83\x01RP`\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x9FW__\xFD[a;\xAB\x84\x82\x85\x01a0qV[`@\x83\x01RP\x92\x91PPV[___``\x84\x86\x03\x12\x15a;\xC9W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xDFW__\xFD[a;\xEB\x86\x82\x87\x01a:zV[\x93PP` \x84\x015a;\xFC\x81a/cV[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\x17W__\xFD[a<#\x86\x82\x87\x01a:\xA6V[\x91PP\x92P\x92P\x92V[_____`\xA0\x86\x88\x03\x12\x15a<AW__\xFD[\x855a<L\x81a/cV[\x94Pa<Z` \x87\x01a4\xEEV[\x93Pa<h`@\x87\x01a/\x94V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a<\xD6W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R` \x90\x81\x01Qa\xFF\xFF\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01a<\x92V[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a=\x98W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x84\x03\x01\x88Ra=\x82\x83\x83Qa<\xE0V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a=HV[P\x90\x96\x95PPPPPPV[_`\xA0\x83\x01\x82Q\x84R` \x83\x01Q` \x85\x01R`@\x83\x01Q`@\x85\x01R``\x83\x01Q``\x85\x01R`\x80\x83\x01Q`\xA0`\x80\x86\x01R\x81\x81Q\x80\x84R`\xC0\x87\x01\x91P`\xC0\x81`\x05\x1B\x88\x01\x01\x93P` \x83\x01\x92P_[\x81\x81\x10\x15a?^W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x88\x86\x03\x01\x83R\x83Q\x80Q``\x87R\x80Q`\xA0``\x89\x01Ra>Da\x01\0\x89\x01\x82a<\xE0V[\x90P` \x82\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x83\x03\x01`\x80\x8A\x01Ra>\x7F\x82\x82a<\xE0V[\x91PP`@\x82\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x83\x03\x01`\xA0\x8A\x01Ra>\xBB\x82\x82a=,V[\x91PP``\x82\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x83\x03\x01`\xC0\x8A\x01Ra>\xF7\x82\x82a=,V[\x91PP`\x80\x82\x01Q`\xE0\x89\x01R` \x83\x01Q\x91Pa?-` \x89\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[`@\x83\x01Q\x92P\x87\x81\x03`@\x89\x01Ra?F\x81\x84a<\xE0V[\x97PPP` \x94\x85\x01\x94\x93\x90\x93\x01\x92P`\x01\x01a=\xF6V[P\x92\x96\x95PPPPPPV[` \x81Ra?\x91` \x82\x01\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[` \x82\x01Q`@\x82\x01R`@\x82\x01Q``\x82\x01R_``\x83\x01Qa?\xCD`\x80\x84\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[P`\x80\x83\x01Q\x80\x15\x15`\xA0\x84\x01RP`\xA0\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RP`\xC0\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\xE0\x84\x01RP`\xE0\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x01\0\x84\x01RPa\x01\0\x83\x01Qa\x01 \x83\x01Ra\x01 \x83\x01Qa\x01@\x83\x01Ra\x01@\x83\x01Qa\x01`\x83\x01Ra\x01`\x83\x01Qa\x01\x80\x83\x01Ra\x01\x80\x83\x01Qa\x01\xA0\x83\x01Ra\x01\xA0\x83\x01Qa\x01\xC0\x83\x01Ra\x01\xC0\x83\x01Qa\x02\0a\x01\xE0\x84\x01Ra@\xA8a\x02 \x84\x01\x82a<\x80V[\x90Pa\x01\xE0\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x84\x83\x03\x01a\x02\0\x85\x01Ra@\xE5\x82\x82a=\xA4V[\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a@\xFFW__\xFD[\x825aA\n\x81a/cV[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15aA*W__\xFD[\x835\x92P` \x84\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aAPW__\xFD[\x91PaA^`@\x85\x01a/\x94V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aAxW__\xFD[\x825aA\x83\x81a/cV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\x9EW__\xFD[a8\xF1\x85\x82\x86\x01a:zV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0B\xDBWa\x0B\xDBaA\xAAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xDBWa\x0B\xDBaA\xAAV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xE1\x836\x03\x01\x81\x12aB\\W__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12aB\x99W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\xB3W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aB\xC7W__\xFD[\x92P\x92\x90PV[______a\x01@\x87\x89\x03\x12\x15aB\xE4W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xFAW__\xFD[aC\x06\x89\x82\x8A\x01a3\xBDV[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\"W__\xFD[aC.\x89\x82\x8A\x01a0qV[\x95PP`@\x87\x015\x93PaCE\x88``\x89\x01a5\tV[\x92Pa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aCaW__\xFD[aCm\x89\x82\x8A\x01a0qV[\x92PPa\x01 \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\x8AW__\xFD[aC\x96\x89\x82\x8A\x01a:\xA6V[\x91PP\x92\x95P\x92\x95P\x92\x95V[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12aC\xD6W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aC\xF0W__\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aB\xC7W__\xFD[_``\x82\x84\x03\x12\x15aD\x16W__\xFD[a\x08\x97\x83\x83a6\xF2V[_` \x82\x84\x03\x12\x15aD0W__\xFD[\x815a\x08\x97\x81a/cV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R_\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`@\x84\x01Re\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01Q\x16``\x84\x01Re\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16`\x80\x84\x01RPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x01Q\x16`\xA0\x83\x01R`@\x84\x01Q`\xC0\x83\x01Ra\x01\0`\xE0\x83\x01Ra@\xE5a\x01\0\x83\x01\x84a<\xE0V[_a\x0B\xDB6\x83a7\xA1V[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x03aE7WaE7aA\xAAV[`\x01\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81Q_\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15aE\x97W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aEyV[P\x91\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a')`@\x83\x01\x84a<\xE0V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15aF\rW__\xFD[PQ\x91\x90PV\xFEOutputAllocation(address recipient,uint16 basisPoints)DCAIntent(address swapper,uint256 nonce,uint256 chainId,address hookAddress,bool isExactIn,address inputToken,address outputToken,address cosigner,uint256 minPeriod,uint256 maxPeriod,uint256 minChunkSize,uint256 maxChunkSize,uint256 minPrice,uint256 deadline,OutputAllocation[] outputAllocations,PrivateIntent privateIntent)FeedInfo(FeedTemplate feedTemplate,address feedAddress,string feedType)FeedTemplate(string name,string expression,string[] parameters,string[] secrets,uint256 retryCount)OutputAllocation(address recipient,uint16 basisPoints)PrivateIntent(uint256 totalAmount,uint256 exactFrequency,uint256 numChunks,bytes32 salt,FeedInfo[] oracleFeeds)DCAOrderCosignerData(address swapper,uint96 nonce,uint160 execAmount,uint96 orderNonce,uint160 limitAmount)\xA2dipfsX\"\x12 \\a\x87{_\x80\xD1\xB7\x07\xA9\xC4\xEF\x9CxE\x8D\x1En'\xCD\x18\xA8\x1E\xF2Wjx~:u\xA6ydsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061016e575f3560e01c80635e29fa37116100d2578063a0a31aac11610088578063b202a7f311610063578063b202a7f3146106e0578063d8b61ded14610766578063fe7823ac14610779575f5ffd5b8063a0a31aac14610693578063ab572650146106a6578063b1c13908146106cd575f5ffd5b80638184e353116100b85780638184e3531461064d5780638345eb561461066057806383bc6ab614610673575f5ffd5b80635e29fa371461062757806380cb1b501461063a575f5ffd5b80632fd0109b11610127578063308ea6c91161010d578063308ea6c9146103a65780633644e515146104d057806349d8033e146104e6575f5ffd5b80632fd0109b146103705780633079008114610383575f5ffd5b806316b853a21161015757806316b853a2146102355780631ce24d021461030f57806329c8ad4e1461035d575f5ffd5b80630209e7101461017257806312261ee7146101e4575b5f5ffd5b6101e2610180366004612d92565b5f9182526020829052604090912080546effffffffffffffffffffffffffffff909216700100000000000000000000000000000000027fff000000000000000000000000000000ffffffffffffffffffffffffffffffff909216919091179055565b005b61020b7f000000000000000000000000000000000000000000000000000000000000000081565b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b6102ef610243366004612dd3565b5f9081526020818152604091829020825160a08101845281546fffffffffffffffffffffffffffffffff811680835270010000000000000000000000000000000082046effffffffffffffffffffffffffffff169483018590527f010000000000000000000000000000000000000000000000000000000000000090910460ff161515948201949094526001820154606082018190526002909201546080909101819052929390929190565b60408051948552602085019390935291830152606082015260800161022c565b61034061031d366004612dd3565b5f908152602081905260409020546fffffffffffffffffffffffffffffffff1690565b6040516bffffffffffffffffffffffff909116815260200161022c565b6101e261036b36600461357e565b6107a3565b6101e261037e366004613896565b6107b3565b6103966103913660046138fb565b6107c1565b604051901515815260200161022c565b6104316103b4366004613924565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506040805160a08101825273ffffffffffffffffffffffffffffffffffffffff96871681526bffffffffffffffffffffffff95861660208201529386169084015290921660608201529116608082015290565b60405161022c91905f60a08201905073ffffffffffffffffffffffffffffffffffffffff83511682526bffffffffffffffffffffffff602084015116602083015273ffffffffffffffffffffffffffffffffffffffff60408401511660408301526bffffffffffffffffffffffff606084015116606083015273ffffffffffffffffffffffffffffffffffffffff608084015116608083015292915050565b6104d861089e565b60405190815260200161022c565b6105c16104f4366004612dd3565b6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152505f9081526020818152604091829020825160a08101845281546fffffffffffffffffffffffffffffffff8116825270010000000000000000000000000000000081046effffffffffffffffffffffffffffff16938201939093527f010000000000000000000000000000000000000000000000000000000000000090920460ff1615159282019290925260018201546060820152600290910154608082015290565b60405161022c91905f60a0820190506fffffffffffffffffffffffffffffffff83511682526effffffffffffffffffffffffffffff6020840151166020830152604083015115156040830152606083015160608301526080830151608083015292915050565b6101e261063536600461398b565b6108f9565b6101e26106483660046139fc565b610934565b6101e261065b366004613a48565b6109a5565b6101e261066e366004613bb7565b6109b1565b610686610681366004613c2d565b6109bc565b60405161022c9190613f6a565b6101e26106a1366004612dd3565b610b7c565b61020b7f000000000000000000000000000000000000000000000000000000000000000081565b6104d86106db3660046140ee565b610b86565b6101e26106ee366004614118565b5f9283526020839052604090922080549215157f0100000000000000000000000000000000000000000000000000000000000000027effffffffffffffffffffffffffffff000000000000000000000000000000009093166fffffffffffffffffffffffffffffffff90921691909117919091179055565b6101e2610774366004614167565b610be1565b6101e26107873660046138fb565b5f92835260208390526040909220600181019190915560020155565b6107ae838383610dbd565b505050565b6107bd8282611000565b5050565b5f83815260208190526040812080547f0100000000000000000000000000000000000000000000000000000000000000900460ff1615610804575f915050610897565b821580159061081257508242115b15610820575f915050610897565b80546fffffffffffffffffffffffffffffffff165f03610844576001915050610897565b83158015906108835750805484906108819070010000000000000000000000000000000090046effffffffffffffffffffffffffffff16426141d7565b115b15610891575f915050610897565b60019150505b9392505050565b5f7f000000000000000000000000000000000000000000000000000000000000000046146108d4576108cf306112c7565b905090565b507f000000000000000000000000000000000000000000000000000000000000000090565b805f5b8181101561092e576109263385858481811061091a5761091a6141ea565b905060200201356113e8565b6001016108fc565b50505050565b61093c612c39565b6040805160a0810182525f808252602082018190526060820152861515608084810191909152610180840185905273ffffffffffffffffffffffffffffffffffffffff878116938301939093529185169181019190915261099d8282611560565b505050505050565b6109ae81611653565b50565b6107ae83838361182f565b6109c4612c39565b6040805160018082528183019092525f91816020015b604080518082019091525f80825260208201528152602001906001900390816109da57505060408051808201909152619abc8152612710602082015281519192509082905f90610a2c57610a2c6141ea565b60200260200101819052505f6040518060a00160405280683635c9adc5dea000008152602001610e108152602001600a81526020015f5f1b81526020015f67ffffffffffffffff811115610a8257610a82612dea565b604051908082528060200260200182016040528015610abb57816020015b610aa8612d43565b815260200190600190039081610aa05790505b509052604080516102008101825273ffffffffffffffffffffffffffffffffffffffff8b1681526bffffffffffffffffffffffff8a1660208201524691810191909152306060820152871515608082015261aaaa60a082015261bbbb60c082015261567860e082015261012c610100820152611c20610120820152610140810187905261016081018690525f6101808201529091506101a08101610b624262015180614217565b815260208101939093526040909201529695505050505050565b6109ae33826113e8565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606084901b166020820152603481018290525f906054015b6040516020818303038152906040528051906020012090505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610c22575f5ffd5b5f8080808080610c32878061422a565b610c409060a0810190614266565b810190610c4d91906142ce565b9550955095509550955095505f865f01518760200151604051602001610ca492919060609290921b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000168252601482015260340190565b604051602081830303815290604052805190602001209050610ccb8782878988888e61190a565b5f80610d388360408c0135610ce360808e018e6143a3565b808060200260200160405190810160405280939291908181526020015f905b82821015610d2e57610d1f60608302860136819003810190614406565b81526020019060010190610d02565b50505050506119d4565b91509150610d478a8c8661182f565b604080870151608080890151835173ffffffffffffffffffffffffffffffffffffffff9384168152921660208301529181018490526060810183905284917f8595ab03f1b5496cccc75833b235b1b95dee73096282d3ac851b628df24bd0f2910160405180910390a25050505050505050505050565b826101400151826040015173ffffffffffffffffffffffffffffffffffffffff161015610e475760408083015161014085015191517fc9e3198300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff909116600482015260248101919091526044015b60405180910390fd5b826101600151826040015173ffffffffffffffffffffffffffffffffffffffff161115610ecc5760408083015161016085015191517fa24e3c0700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024810191909152604401610e3e565b826080015115610f4e57816040015173ffffffffffffffffffffffffffffffffffffffff1681146107ae5760408083015190517f9994be180000000000000000000000000000000000000000000000000000000081526004810183905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b805f03610f87576040517faf458c0700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b816080015173ffffffffffffffffffffffffffffffffffffffff168111156107ae5760808201516040517ff00f44f50000000000000000000000000000000000000000000000000000000081526004810183905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b606082015173ffffffffffffffffffffffffffffffffffffffff1630146110775760608201516040517ff94e883c00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604401610e3e565b468260400151146110c35760408083015190517f24497bc30000000000000000000000000000000000000000000000000000000081526004810191909152466024820152604401610e3e565b815181516020015173ffffffffffffffffffffffffffffffffffffffff9081169116146111445780516020015182516040517fbaa8d2ad00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b8160a0015173ffffffffffffffffffffffffffffffffffffffff1681602001515f015173ffffffffffffffffffffffffffffffffffffffff16146111df5760208101515160a08301516040517f2c63212600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b6040810151515f5b8181101561092e578360c0015173ffffffffffffffffffffffffffffffffffffffff1683604001518281518110611220576112206141ea565b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff16146112bf578260400151818151811061125c5761125c6141ea565b60209081029190910101515160c08501516040517fd064a3f700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b6001016111e7565b604080518082018252600781527f444341486f6f6b0000000000000000000000000000000000000000000000000060209182015281518083018352600181527f31000000000000000000000000000000000000000000000000000000000000009082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f918101919091527fa20374b09dfce3316b287b36aa908c96cdb9329755a7e08683c5b42908026635918101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6606082015246608082015273ffffffffffffffffffffffffffffffffffffffff821660a08201525f9060c0015b604051602081830303815290604052805190602001209050919050565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606084901b166020820152603481018290525f90605401604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815291815281516020928301205f818152928390529120549091507f0100000000000000000000000000000000000000000000000000000000000000900460ff16156114c7576040517f0b1f884600000000000000000000000000000000000000000000000000000000815260048101829052602401610e3e565b5f8181526020819052604080822080547effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f01000000000000000000000000000000000000000000000000000000000000001790555173ffffffffffffffffffffffffffffffffffffffff85169183917fac5a4b90e421002a2fdb9f132b9b32c24fa4ae16ec480516c85932de208d2a339190a3505050565b5f8260800151156115b9576115b2826080015173ffffffffffffffffffffffffffffffffffffffff16670de0b6b3a7640000846040015173ffffffffffffffffffffffffffffffffffffffff16611c0b565b9050611603565b611600826040015173ffffffffffffffffffffffffffffffffffffffff16670de0b6b3a7640000846080015173ffffffffffffffffffffffffffffffffffffffff16611c0b565b90505b8261018001518110156107ae576101808301516040517fba04e36d000000000000000000000000000000000000000000000000000000008152610e3e918391600401918252602082015260400190565b80515f81900361168f576040517f949efc9600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805b828110156117f0575f8482815181106116ad576116ad6141ea565b60200260200101516020015190508061ffff165f036116f8576040517fba0d87b500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f85838151811061170b5761170b6141ea565b60200260200101515f015190505f8360016117269190614217565b90505b858110156117d2578173ffffffffffffffffffffffffffffffffffffffff1687828151811061175a5761175a6141ea565b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff16036117ca576040517f3e91857900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83166004820152602401610e3e565b600101611729565b506117e161ffff831685614217565b93508260010192505050611692565b5061271081146107ae576040517fbc9dfe8c00000000000000000000000000000000000000000000000000000000815260048101829052602401610e3e565b8051156118df5773ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016632b67b57061187c858061422a565b61188d906040810190602001614420565b836020015184604001516040518463ffffffff1660e01b81526004016118b59392919061443b565b5f604051808303815f87803b1580156118cc575f5ffd5b505af19250505080156118dd575060015b505b6107ae7f00000000000000000000000000000000000000000000000000000000000000008484611d31565b611915878686611e0a565b61192787611922836144f9565b611000565b611935876101c00151611653565b611940878484611e97565b61194b868885612002565b61195a87846040840135610dbd565b6119648784611560565b6119cb878461197660808501856143a3565b808060200260200160405190810160405280939291908181526020015f905b828210156119c1576119b260608302860136819003810190614406565b81526020019060010190611995565b50505050506122a5565b50505050505050565b5f83815260208181526040808320815160a08101835281546fffffffffffffffffffffffffffffffff8116825270010000000000000000000000000000000081046effffffffffffffffffffffffffffff16948201949094527f010000000000000000000000000000000000000000000000000000000000000090930460ff161515918301919091526001810154606083015260020154608082015282518291908290815b81811015611ab457868181518110611a9357611a936141ea565b60200260200101516020015183611aaa9190614217565b9250600101611a79565b50825183611ac182614504565b6fffffffffffffffffffffffffffffffff169052506effffffffffffffffffffffffffffff42166020840152606083018051889190611b01908390614217565b905250608083018051839190611b18908390614217565b90525050505f958652602086815260409687902082518154928401519884015115157f0100000000000000000000000000000000000000000000000000000000000000027effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6effffffffffffffffffffffffffffff909a16700100000000000000000000000000000000027fff000000000000000000000000000000000000000000000000000000000000009094166fffffffffffffffffffffffffffffffff9092169190911792909217979097161786556060810151600187018190556080909101516002909601869055959350505050565b5f80807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff858709858702925082811083820303915050805f03611c6157838281611c5757611c57614540565b0492505050610897565b808411611cca576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601560248201527f4d6174683a206d756c446976206f766572666c6f7700000000000000000000006044820152606401610e3e565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b73ffffffffffffffffffffffffffffffffffffffff83166336c78516611d57848061422a565b611d68906040810190602001614420565b8360408601803590611d7d9060208901614420565b60405160e086901b7fffffffff0000000000000000000000000000000000000000000000000000000016815273ffffffffffffffffffffffffffffffffffffffff94851660048201529284166024840152908316604483015290911660648201526084015f604051808303815f87803b158015611df8575f5ffd5b505af11580156119cb573d5f5f3e3d5ffd5b5f611e1584846125dd565b90505f611e29611e2361089e565b836126dc565b9050611e39855f0151828561271d565b611e905784516040517f17aa11790000000000000000000000000000000000000000000000000000000081525f600482015273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b5050505050565b5f611ea183612731565b90505f611eaf611e2361089e565b9050611ec08560e00151828561271d565b611f1a5760e08501516040517fca5612f60000000000000000000000000000000000000000000000000000000081525f600482015273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b8451845173ffffffffffffffffffffffffffffffffffffffff908116911614611f9357835185516040517f438429d500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b846020015184602001516bffffffffffffffffffffffff1614611e9057602080850151908601516040517fbbccf88f0000000000000000000000000000000000000000000000000000000081526bffffffffffffffffffffffff90921660048301526024820152604401610e3e565b5f8381526020818152604091829020825160a08101845281546fffffffffffffffffffffffffffffffff8116825270010000000000000000000000000000000081046effffffffffffffffffffffffffffff16938201939093527f010000000000000000000000000000000000000000000000000000000000000090920460ff161580159383019390935260018101546060830152600201546080820152906120da576040517f2660161b00000000000000000000000000000000000000000000000000000000815260048101859052602401610e3e565b6101a0830151158015906120f25750826101a0015142115b15612139576101a08301516040517f6f08ee6e0000000000000000000000000000000000000000000000000000000081524260048201526024810191909152604401610e3e565b805f01516fffffffffffffffffffffffffffffffff1682606001516bffffffffffffffffffffffff16146121b857606082015181516040517f1ca8c0620000000000000000000000000000000000000000000000000000000081526bffffffffffffffffffffffff928316600482015291166024820152604401610e3e565b80516fffffffffffffffffffffffffffffffff161561092e575f81602001516effffffffffffffffffffffffffffff16426121f391906141d7565b9050836101000151811015612245576101008401516040517fb1b9b6ee000000000000000000000000000000000000000000000000000000008152610e3e918391600401918252602082015260400190565b6101208401511580159061225d575083610120015181115b15611e90576101208401516040517f388b0173000000000000000000000000000000000000000000000000000000008152610e3e918391600401918252602082015260400190565b80515f90815b818110156122e6578381815181106122c5576122c56141ea565b602002602001015160200151836122dc9190614217565b92506001016122ab565b506101c0850151515f5b818110156124dc575f876101c001518281518110612310576123106141ea565b60200260200101515f015190505f612351868a6101c001518581518110612339576123396141ea565b60200260200101516020015161ffff16612710611c0b565b90505f805b868110156123e2578373ffffffffffffffffffffffffffffffffffffffff16898281518110612387576123876141ea565b60200260200101516040015173ffffffffffffffffffffffffffffffffffffffff16036123da578881815181106123c0576123c06141ea565b602002602001015160200151826123d79190614217565b91505b600101612356565b5089608001511561247257816123f9826001614217565b10158015612411575061240d826001614217565b8111155b61246d576040517f66c6a6af00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024810182905260448101839052606401610e3e565b6124d1565b8181146124d1576040517f66c6a6af00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024810182905260448101839052606401610e3e565b5050506001016122f0565b5085608001511561256557846080015173ffffffffffffffffffffffffffffffffffffffff168310156125605760808501516040517f2c19b8b80000000000000000000000000000000000000000000000000000000081526004810185905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b61099d565b846040015173ffffffffffffffffffffffffffffffffffffffff16831461099d5760408086015190517f7ebbdeab0000000000000000000000000000000000000000000000000000000081526004810185905273ffffffffffffffffffffffffffffffffffffffff9091166024820152604401610e3e565b5f5f6125ed846101c001516127c5565b90505f604051806102c00160405280610293815260200161464b61029391398051906020012090505f60405182815286516020820152602087015160408201526040870151606082015260608701516080820152608087015160a082015260a087015160c082015260c087015160e082015260e08701516101008201526101008701516101208201526101208701516101408201526101408701516101608201526101608701516101808201526101808701516101a08201526101a08701516101c0820152836101e0820152856102008201526102208120915061022081016040525080935050505092915050565b6040517f1901000000000000000000000000000000000000000000000000000000000000602082015260228101839052604281018290525f90606201610bc2565b5f61272984848461291d565b949350505050565b5f6040518060a00160405280606b81526020016148de606b913980516020918201208351848301516040808701516060880151608089015192516113cb97929391920195865273ffffffffffffffffffffffffffffffffffffffff94851660208701526bffffffffffffffffffffffff9384166040870152918416606086015290911660808401521660a082015260c00190565b80515f90818167ffffffffffffffff8111156127e3576127e3612dea565b60405190808252806020026020018201604052801561280c578160200160208202803683370190505b5090505f5b828110156128ec57604051806060016040528060368152602001614615603691398051906020012085828151811061284b5761284b6141ea565b60200260200101515f0151868381518110612868576128686141ea565b6020026020010151602001516040516020016128b19392919092835273ffffffffffffffffffffffffffffffffffffffff91909116602083015261ffff16604082015260600190565b604051602081830303815290604052805190602001208282815181106128d9576128d96141ea565b6020908102919091010152600101612811565b50806040516020016128fe919061456d565b6040516020818303038152906040528051906020012092505050919050565b5f8373ffffffffffffffffffffffffffffffffffffffff163b5f036129a2575f5f61294885856129b4565b5090925090505f816003811115612961576129616145a2565b14801561299957508573ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16145b92505050610897565b6129ad8484846129fd565b9050610897565b5f5f5f83516041036129eb576020840151604085015160608601515f1a6129dd88828585612b46565b9550955095505050506129f6565b505081515f91506002905b9250925092565b5f5f5f8573ffffffffffffffffffffffffffffffffffffffff168585604051602401612a2a9291906145cf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529181526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f1626ba7e0000000000000000000000000000000000000000000000000000000017905251612aab91906145e7565b5f60405180830381855afa9150503d805f8114612ae3576040519150601f19603f3d011682016040523d82523d5f602084013e612ae8565b606091505b5091509150818015612afc57506020815110155b8015612b3c575080517f1626ba7e0000000000000000000000000000000000000000000000000000000090612b3a90830160209081019084016145fd565b145b9695505050505050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115612b7f57505f91506003905082612c2f565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612bd0573d5f5f3e3d5ffd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff8116612c2657505f925060019150829050612c2f565b92505f91508190505b9450945094915050565b6040518061020001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f81526020015f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f151581526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f815260200160608152602001612d3e6040518060a001604052805f81526020015f81526020015f81526020015f8152602001606081525090565b905290565b6040518060600160405280612d7f6040518060a00160405280606081526020016060815260200160608152602001606081526020015f81525090565b81525f6020820152606060409091015290565b5f5f60408385031215612da3575f5ffd5b8235915060208301356effffffffffffffffffffffffffffff81168114612dc8575f5ffd5b809150509250929050565b5f60208284031215612de3575f5ffd5b5035919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b60405290565b60405160a0810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b6040516060810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b604051610200810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b604051610120810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b60405160e0810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b6040516080810167ffffffffffffffff81118282101715612e3a57612e3a612dea565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715612f5b57612f5b612dea565b604052919050565b73ffffffffffffffffffffffffffffffffffffffff811681146109ae575f5ffd5b8035612f8f81612f63565b919050565b80358015158114612f8f575f5ffd5b5f67ffffffffffffffff821115612fbc57612fbc612dea565b5060051b60200190565b5f82601f830112612fd5575f5ffd5b8135612fe8612fe382612fa3565b612f14565b8082825260208201915060208360061b860101925085831115613009575f5ffd5b602085015b838110156130675760408188031215613025575f5ffd5b61302d612e17565b813561303881612f63565b8152602082013561ffff8116811461304e575f5ffd5b602082810191909152908452929092019160400161300e565b5095945050505050565b5f82601f830112613080575f5ffd5b8135602083015f5f67ffffffffffffffff8411156130a0576130a0612dea565b50601f83017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0166020016130d381612f14565b9150508281528583830111156130e7575f5ffd5b828260208301375f92810160200192909252509392505050565b5f82601f830112613110575f5ffd5b813561311e612fe382612fa3565b8082825260208201915060208360051b86010192508583111561313f575f5ffd5b602085015b8381101561306757803567ffffffffffffffff811115613162575f5ffd5b613171886020838a0101613071565b84525060209283019201613144565b5f60a08284031215613190575f5ffd5b613198612e40565b823581526020808401359082015260408084013590820152606080840135908201529050608082013567ffffffffffffffff8111156131d5575f5ffd5b8201601f810184136131e5575f5ffd5b80356131f3612fe382612fa3565b8082825260208201915060208360051b850101925086831115613214575f5ffd5b602084015b838110156133ad57803567ffffffffffffffff811115613237575f5ffd5b85016060818a037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001121561326a575f5ffd5b613272612e63565b602082013567ffffffffffffffff81111561328b575f5ffd5b602081840101905060a0818c0312156132a2575f5ffd5b6132aa612e40565b813567ffffffffffffffff8111156132c0575f5ffd5b6132cc8d828501613071565b825250602082013567ffffffffffffffff8111156132e8575f5ffd5b6132f48d828501613071565b602083015250604082013567ffffffffffffffff811115613313575f5ffd5b61331f8d828501613101565b604083015250606082013567ffffffffffffffff81111561333e575f5ffd5b61334a8d828501613101565b60608301525060809182013591810191909152815261336b60408301612f84565b6020820152606082013567ffffffffffffffff811115613389575f5ffd5b6133988b602083860101613071565b60408301525084525060209283019201613219565b5060808501525091949350505050565b5f61020082840312156133ce575f5ffd5b6133d6612e86565b90506133e182612f84565b8152602082810135908201526040808301359082015261340360608301612f84565b606082015261341460808301612f94565b608082015261342560a08301612f84565b60a082015261343660c08301612f84565b60c082015261344760e08301612f84565b60e0820152610100828101359082015261012080830135908201526101408083013590820152610160808301359082015261018080830135908201526101a080830135908201526101c082013567ffffffffffffffff8111156134a8575f5ffd5b6134b484828501612fc6565b6101c0830152506101e082013567ffffffffffffffff8111156134d5575f5ffd5b6134e184828501613180565b6101e08301525092915050565b80356bffffffffffffffffffffffff81168114612f8f575f5ffd5b5f60a08284031215613519575f5ffd5b613521612e40565b9050813561352e81612f63565b815261353c602083016134ee565b6020820152604082013561354f81612f63565b6040820152613560606083016134ee565b6060820152608082013561357381612f63565b608082015292915050565b5f5f5f60e08486031215613590575f5ffd5b833567ffffffffffffffff8111156135a6575f5ffd5b6135b2868287016133bd565b9350506135c28560208601613509565b9295929450505060c0919091013590565b5f61012082840312156135e4575f5ffd5b6135ec612eaa565b90506135f782612f84565b815261360560208301612f84565b6020820152604082810135908201526060808301359082015261362a60808301612f84565b608082015260a082013567ffffffffffffffff811115613648575f5ffd5b61365484828501613071565b60a08301525061366660c08301612f84565b60c082015260e082013567ffffffffffffffff811115613684575f5ffd5b61369084828501613071565b60e0830152506136a36101008301612f84565b61010082015292915050565b5f606082840312156136bf575f5ffd5b6136c7612e63565b905081356136d481612f63565b81526020828101359082015260409182013591810191909152919050565b5f60608284031215613702575f5ffd5b61370a612e63565b9050813561371781612f63565b815260208281013590820152604082013561373181612f63565b604082015292915050565b5f82601f83011261374b575f5ffd5b8135613759612fe382612fa3565b8082825260208201915060206060840286010192508583111561377a575f5ffd5b602085015b838110156130675761379187826136f2565b835260209092019160600161377f565b5f61012082840312156137b2575f5ffd5b6137ba612ece565b9050813567ffffffffffffffff8111156137d2575f5ffd5b6137de848285016135d3565b8252506137ee83602084016136af565b6020820152608082013567ffffffffffffffff81111561380c575f5ffd5b6138188482850161373c565b60408301525060a082013567ffffffffffffffff811115613837575f5ffd5b61384384828501613071565b60608301525060c0820135608082015261385f60e08301612f84565b60a082015261010082013567ffffffffffffffff81111561387e575f5ffd5b61388a84828501613071565b60c08301525092915050565b5f5f604083850312156138a7575f5ffd5b823567ffffffffffffffff8111156138bd575f5ffd5b6138c9858286016133bd565b925050602083013567ffffffffffffffff8111156138e5575f5ffd5b6138f1858286016137a1565b9150509250929050565b5f5f5f6060848603121561390d575f5ffd5b505081359360208301359350604090920135919050565b5f5f5f5f5f60a08688031215613938575f5ffd5b853561394381612f63565b9450613951602087016134ee565b9350604086013561396181612f63565b9250606086013561397181612f63565b915061397f608087016134ee565b90509295509295909350565b5f5f6020838503121561399c575f5ffd5b823567ffffffffffffffff8111156139b2575f5ffd5b8301601f810185136139c2575f5ffd5b803567ffffffffffffffff8111156139d8575f5ffd5b8560208260051b84010111156139ec575f5ffd5b6020919091019590945092505050565b5f5f5f5f60808587031215613a0f575f5ffd5b613a1885612f94565b93506020850135613a2881612f63565b92506040850135613a3881612f63565b9396929550929360600135925050565b5f60208284031215613a58575f5ffd5b813567ffffffffffffffff811115613a6e575f5ffd5b61272984828501612fc6565b5f6101208284031215613a8b575f5ffd5b50919050565b803565ffffffffffff81168114612f8f575f5ffd5b5f818303610100811215613ab8575f5ffd5b613ac0612e63565b9150613acb83612f94565b82527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe00160c0811215613afc575f5ffd5b613b04612e63565b6080821215613b11575f5ffd5b613b19612ef1565b91506020840135613b2981612f63565b82526040840135613b3981612f63565b6020830152613b4a60608501613a91565b6040830152613b5b60808501613a91565b6060830152818152613b6f60a08501612f84565b60208281019190915260c085013560408301528301525060e082013567ffffffffffffffff811115613b9f575f5ffd5b613bab84828501613071565b60408301525092915050565b5f5f5f60608486031215613bc9575f5ffd5b833567ffffffffffffffff811115613bdf575f5ffd5b613beb86828701613a7a565b9350506020840135613bfc81612f63565b9150604084013567ffffffffffffffff811115613c17575f5ffd5b613c2386828701613aa6565b9150509250925092565b5f5f5f5f5f60a08688031215613c41575f5ffd5b8535613c4c81612f63565b9450613c5a602087016134ee565b9350613c6860408701612f94565b94979396509394606081013594506080013592915050565b5f8151808452602084019350602083015f5b82811015613cd6578151805173ffffffffffffffffffffffffffffffffffffffff16875260209081015161ffff168188015260409096019590910190600101613c92565b5093949350505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b83811015613d98577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0858403018852613d82838351613ce0565b6020988901989093509190910190600101613d48565b50909695505050505050565b5f60a0830182518452602083015160208501526040830151604085015260608301516060850152608083015160a0608086015281815180845260c08701915060c08160051b88010193506020830192505f5b81811015613f5e577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff408886030183528351805160608752805160a06060890152613e44610100890182613ce0565b905060208201517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08983030160808a0152613e7f8282613ce0565b91505060408201517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08983030160a08a0152613ebb8282613d2c565b91505060608201517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08983030160c08a0152613ef78282613d2c565b915050608082015160e089015260208301519150613f2d602089018373ffffffffffffffffffffffffffffffffffffffff169052565b604083015192508781036040890152613f468184613ce0565b97505050602094850194939093019250600101613df6565b50929695505050505050565b60208152613f9160208201835173ffffffffffffffffffffffffffffffffffffffff169052565b60208201516040820152604082015160608201525f6060830151613fcd608084018273ffffffffffffffffffffffffffffffffffffffff169052565b50608083015180151560a08401525060a083015173ffffffffffffffffffffffffffffffffffffffff811660c08401525060c083015173ffffffffffffffffffffffffffffffffffffffff811660e08401525060e083015173ffffffffffffffffffffffffffffffffffffffff8116610100840152506101008301516101208301526101208301516101408301526101408301516101608301526101608301516101808301526101808301516101a08301526101a08301516101c08301526101c08301516102006101e08401526140a8610220840182613c80565b90506101e08401517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0848303016102008501526140e58282613da4565b95945050505050565b5f5f604083850312156140ff575f5ffd5b823561410a81612f63565b946020939093013593505050565b5f5f5f6060848603121561412a575f5ffd5b8335925060208401356fffffffffffffffffffffffffffffffff81168114614150575f5ffd5b915061415e60408501612f94565b90509250925092565b5f5f60408385031215614178575f5ffd5b823561418381612f63565b9150602083013567ffffffffffffffff81111561419e575f5ffd5b6138f185828601613a7a565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115610bdb57610bdb6141aa565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115610bdb57610bdb6141aa565b5f82357ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffee183360301811261425c575f5ffd5b9190910192915050565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112614299575f5ffd5b83018035915067ffffffffffffffff8211156142b3575f5ffd5b6020019150368190038213156142c7575f5ffd5b9250929050565b5f5f5f5f5f5f61014087890312156142e4575f5ffd5b863567ffffffffffffffff8111156142fa575f5ffd5b61430689828a016133bd565b965050602087013567ffffffffffffffff811115614322575f5ffd5b61432e89828a01613071565b955050604087013593506143458860608901613509565b925061010087013567ffffffffffffffff811115614361575f5ffd5b61436d89828a01613071565b92505061012087013567ffffffffffffffff81111561438a575f5ffd5b61439689828a01613aa6565b9150509295509295509295565b5f5f83357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe18436030181126143d6575f5ffd5b83018035915067ffffffffffffffff8211156143f0575f5ffd5b60200191506060810236038213156142c7575f5ffd5b5f60608284031215614416575f5ffd5b61089783836136f2565b5f60208284031215614430575f5ffd5b813561089781612f63565b73ffffffffffffffffffffffffffffffffffffffff841681525f835173ffffffffffffffffffffffffffffffffffffffff815116602084015273ffffffffffffffffffffffffffffffffffffffff602082015116604084015265ffffffffffff604082015116606084015265ffffffffffff60608201511660808401525073ffffffffffffffffffffffffffffffffffffffff60208501511660a0830152604084015160c083015261010060e08301526140e5610100830184613ce0565b5f610bdb36836137a1565b5f6fffffffffffffffffffffffffffffffff82166fffffffffffffffffffffffffffffffff8103614537576145376141aa565b60010192915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b81515f90829060208501835b82811015614597578151845260209384019390910190600101614579565b509195945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b828152604060208201525f6127296040830184613ce0565b5f82518060208501845e5f920191825250919050565b5f6020828403121561460d575f5ffd5b505191905056fe4f7574707574416c6c6f636174696f6e286164647265737320726563697069656e742c75696e743136206261736973506f696e747329444341496e74656e74286164647265737320737761707065722c75696e74323536206e6f6e63652c75696e7432353620636861696e49642c6164647265737320686f6f6b416464726573732c626f6f6c2069734578616374496e2c6164647265737320696e707574546f6b656e2c61646472657373206f7574707574546f6b656e2c6164647265737320636f7369676e65722c75696e74323536206d696e506572696f642c75696e74323536206d6178506572696f642c75696e74323536206d696e4368756e6b53697a652c75696e74323536206d61784368756e6b53697a652c75696e74323536206d696e50726963652c75696e7432353620646561646c696e652c4f7574707574416c6c6f636174696f6e5b5d206f7574707574416c6c6f636174696f6e732c50726976617465496e74656e742070726976617465496e74656e742946656564496e666f284665656454656d706c617465206665656454656d706c6174652c616464726573732066656564416464726573732c737472696e67206665656454797065294665656454656d706c61746528737472696e67206e616d652c737472696e672065787072657373696f6e2c737472696e675b5d20706172616d65746572732c737472696e675b5d20736563726574732c75696e74323536207265747279436f756e74294f7574707574416c6c6f636174696f6e286164647265737320726563697069656e742c75696e743136206261736973506f696e74732950726976617465496e74656e742875696e7432353620746f74616c416d6f756e742c75696e743235362065786163744672657175656e63792c75696e74323536206e756d4368756e6b732c627974657333322073616c742c46656564496e666f5b5d206f7261636c654665656473294443414f72646572436f7369676e657244617461286164647265737320737761707065722c75696e743936206e6f6e63652c75696e743136302065786563416d6f756e742c75696e743936206f726465724e6f6e63652c75696e74313630206c696d6974416d6f756e7429a26469706673582212205c61877b5f80d1b707a9c4ef9c78458d1e6e27cd18a81ef2576a787e3a75a67964736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01nW_5`\xE0\x1C\x80c^)\xFA7\x11a\0\xD2W\x80c\xA0\xA3\x1A\xAC\x11a\0\x88W\x80c\xB2\x02\xA7\xF3\x11a\0cW\x80c\xB2\x02\xA7\xF3\x14a\x06\xE0W\x80c\xD8\xB6\x1D\xED\x14a\x07fW\x80c\xFEx#\xAC\x14a\x07yW__\xFD[\x80c\xA0\xA3\x1A\xAC\x14a\x06\x93W\x80c\xABW&P\x14a\x06\xA6W\x80c\xB1\xC19\x08\x14a\x06\xCDW__\xFD[\x80c\x81\x84\xE3S\x11a\0\xB8W\x80c\x81\x84\xE3S\x14a\x06MW\x80c\x83E\xEBV\x14a\x06`W\x80c\x83\xBCj\xB6\x14a\x06sW__\xFD[\x80c^)\xFA7\x14a\x06'W\x80c\x80\xCB\x1BP\x14a\x06:W__\xFD[\x80c/\xD0\x10\x9B\x11a\x01'W\x80c0\x8E\xA6\xC9\x11a\x01\rW\x80c0\x8E\xA6\xC9\x14a\x03\xA6W\x80c6D\xE5\x15\x14a\x04\xD0W\x80cI\xD8\x03>\x14a\x04\xE6W__\xFD[\x80c/\xD0\x10\x9B\x14a\x03pW\x80c0y\0\x81\x14a\x03\x83W__\xFD[\x80c\x16\xB8S\xA2\x11a\x01WW\x80c\x16\xB8S\xA2\x14a\x025W\x80c\x1C\xE2M\x02\x14a\x03\x0FW\x80c)\xC8\xADN\x14a\x03]W__\xFD[\x80c\x02\t\xE7\x10\x14a\x01rW\x80c\x12&\x1E\xE7\x14a\x01\xE4W[__\xFD[a\x01\xE2a\x01\x806`\x04a-\x92V[_\x91\x82R` \x82\x90R`@\x90\x91 \x80Tn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[a\x02\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xEFa\x02C6`\x04a-\xD3V[_\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x83Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94\x83\x01\x85\x90R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x04`\xFF\x16\x15\x15\x94\x82\x01\x94\x90\x94R`\x01\x82\x01T``\x82\x01\x81\x90R`\x02\x90\x92\x01T`\x80\x90\x91\x01\x81\x90R\x92\x93\x90\x92\x91\x90V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02,V[a\x03@a\x03\x1D6`\x04a-\xD3V[_\x90\x81R` \x81\x90R`@\x90 To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02,V[a\x01\xE2a\x03k6`\x04a5~V[a\x07\xA3V[a\x01\xE2a\x03~6`\x04a8\x96V[a\x07\xB3V[a\x03\x96a\x03\x916`\x04a8\xFBV[a\x07\xC1V[`@Q\x90\x15\x15\x81R` \x01a\x02,V[a\x041a\x03\xB46`\x04a9$V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x82\x01R\x93\x86\x16\x90\x84\x01R\x90\x92\x16``\x82\x01R\x91\x16`\x80\x82\x01R\x90V[`@Qa\x02,\x91\x90_`\xA0\x82\x01\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x82Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x01Q\x16`@\x83\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x84\x01Q\x16``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[a\x04\xD8a\x08\x9EV[`@Q\x90\x81R` \x01a\x02,V[a\x05\xC1a\x04\xF46`\x04a-\xD3V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP_\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x82\x01\x93\x90\x93R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04`\xFF\x16\x15\x15\x92\x82\x01\x92\x90\x92R`\x01\x82\x01T``\x82\x01R`\x02\x90\x91\x01T`\x80\x82\x01R\x90V[`@Qa\x02,\x91\x90_`\xA0\x82\x01\x90Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x82Rn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q\x15\x15`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R\x92\x91PPV[a\x01\xE2a\x0656`\x04a9\x8BV[a\x08\xF9V[a\x01\xE2a\x06H6`\x04a9\xFCV[a\t4V[a\x01\xE2a\x06[6`\x04a:HV[a\t\xA5V[a\x01\xE2a\x06n6`\x04a;\xB7V[a\t\xB1V[a\x06\x86a\x06\x816`\x04a<-V[a\t\xBCV[`@Qa\x02,\x91\x90a?jV[a\x01\xE2a\x06\xA16`\x04a-\xD3V[a\x0B|V[a\x02\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xD8a\x06\xDB6`\x04a@\xEEV[a\x0B\x86V[a\x01\xE2a\x06\xEE6`\x04aA\x18V[_\x92\x83R` \x83\x90R`@\x90\x92 \x80T\x92\x15\x15\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90UV[a\x01\xE2a\x07t6`\x04aAgV[a\x0B\xE1V[a\x01\xE2a\x07\x876`\x04a8\xFBV[_\x92\x83R` \x83\x90R`@\x90\x92 `\x01\x81\x01\x91\x90\x91U`\x02\x01UV[a\x07\xAE\x83\x83\x83a\r\xBDV[PPPV[a\x07\xBD\x82\x82a\x10\0V[PPV[_\x83\x81R` \x81\x90R`@\x81 \x80T\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x08\x04W_\x91PPa\x08\x97V[\x82\x15\x80\x15\x90a\x08\x12WP\x82B\x11[\x15a\x08 W_\x91PPa\x08\x97V[\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x08DW`\x01\x91PPa\x08\x97V[\x83\x15\x80\x15\x90a\x08\x83WP\x80T\x84\x90a\x08\x81\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16BaA\xD7V[\x11[\x15a\x08\x91W_\x91PPa\x08\x97V[`\x01\x91PP[\x93\x92PPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x08\xD4Wa\x08\xCF0a\x12\xC7V[\x90P\x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[\x80_[\x81\x81\x10\x15a\t.Wa\t&3\x85\x85\x84\x81\x81\x10a\t\x1AWa\t\x1AaA\xEAV[\x90P` \x02\x015a\x13\xE8V[`\x01\x01a\x08\xFCV[PPPPV[a\t<a,9V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R``\x82\x01R\x86\x15\x15`\x80\x84\x81\x01\x91\x90\x91Ra\x01\x80\x84\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16\x93\x83\x01\x93\x90\x93R\x91\x85\x16\x91\x81\x01\x91\x90\x91Ra\t\x9D\x82\x82a\x15`V[PPPPPPV[a\t\xAE\x81a\x16SV[PV[a\x07\xAE\x83\x83\x83a\x18/V[a\t\xC4a,9V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xDAWPP`@\x80Q\x80\x82\x01\x90\x91Ra\x9A\xBC\x81Ra'\x10` \x82\x01R\x81Q\x91\x92P\x90\x82\x90_\x90a\n,Wa\n,aA\xEAV[` \x02` \x01\x01\x81\x90RP_`@Q\x80`\xA0\x01`@R\x80h65\xC9\xAD\xC5\xDE\xA0\0\0\x81R` \x01a\x0E\x10\x81R` \x01`\n\x81R` \x01__\x1B\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x82Wa\n\x82a-\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xBBW\x81` \x01[a\n\xA8a-CV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xA0W\x90P[P\x90R`@\x80Qa\x02\0\x81\x01\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16` \x82\x01RF\x91\x81\x01\x91\x90\x91R0``\x82\x01R\x87\x15\x15`\x80\x82\x01Ra\xAA\xAA`\xA0\x82\x01Ra\xBB\xBB`\xC0\x82\x01RaVx`\xE0\x82\x01Ra\x01,a\x01\0\x82\x01Ra\x1C a\x01 \x82\x01Ra\x01@\x81\x01\x87\x90Ra\x01`\x81\x01\x86\x90R_a\x01\x80\x82\x01R\x90\x91Pa\x01\xA0\x81\x01a\x0BbBb\x01Q\x80aB\x17V[\x81R` \x81\x01\x93\x90\x93R`@\x90\x92\x01R\x96\x95PPPPPPV[a\t\xAE3\x82a\x13\xE8V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16` \x82\x01R`4\x81\x01\x82\x90R_\x90`T\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\"W__\xFD[_\x80\x80\x80\x80\x80a\x0C2\x87\x80aB*V[a\x0C@\x90`\xA0\x81\x01\x90aBfV[\x81\x01\x90a\x0CM\x91\x90aB\xCEV[\x95P\x95P\x95P\x95P\x95P\x95P_\x86_\x01Q\x87` \x01Q`@Q` \x01a\x0C\xA4\x92\x91\x90``\x92\x90\x92\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82R`\x14\x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0C\xCB\x87\x82\x87\x89\x88\x88\x8Ea\x19\nV[_\x80a\r8\x83`@\x8C\x015a\x0C\xE3`\x80\x8E\x01\x8EaC\xA3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\r.Wa\r\x1F``\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aD\x06V[\x81R` \x01\x90`\x01\x01\x90a\r\x02V[PPPPPa\x19\xD4V[\x91P\x91Pa\rG\x8A\x8C\x86a\x18/V[`@\x80\x87\x01Q`\x80\x80\x89\x01Q\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x16` \x83\x01R\x91\x81\x01\x84\x90R``\x81\x01\x83\x90R\x84\x91\x7F\x85\x95\xAB\x03\xF1\xB5Il\xCC\xC7X3\xB25\xB1\xB9]\xEEs\tb\x82\xD3\xAC\x85\x1Bb\x8D\xF2K\xD0\xF2\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPV[\x82a\x01@\x01Q\x82`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15a\x0EGW`@\x80\x83\x01Qa\x01@\x85\x01Q\x91Q\x7F\xC9\xE3\x19\x83\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01[`@Q\x80\x91\x03\x90\xFD[\x82a\x01`\x01Q\x82`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0E\xCCW`@\x80\x83\x01Qa\x01`\x85\x01Q\x91Q\x7F\xA2N<\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x0E>V[\x82`\x80\x01Q\x15a\x0FNW\x81`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x14a\x07\xAEW`@\x80\x83\x01Q\x90Q\x7F\x99\x94\xBE\x18\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x80_\x03a\x0F\x87W`@Q\x7F\xAFE\x8C\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x11\x15a\x07\xAEW`\x80\x82\x01Q`@Q\x7F\xF0\x0FD\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[``\x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x10wW``\x82\x01Q`@Q\x7F\xF9N\x88<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x01a\x0E>V[F\x82`@\x01Q\x14a\x10\xC3W`@\x80\x83\x01Q\x90Q\x7F$I{\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x91\x90\x91RF`$\x82\x01R`D\x01a\x0E>V[\x81Q\x81Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x11DW\x80Q` \x01Q\x82Q`@Q\x7F\xBA\xA8\xD2\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x81`\xA0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11\xDFW` \x81\x01QQ`\xA0\x83\x01Q`@Q\x7F,c!&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[`@\x81\x01QQ_[\x81\x81\x10\x15a\t.W\x83`\xC0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@\x01Q\x82\x81Q\x81\x10a\x12 Wa\x12 aA\xEAV[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x12\xBFW\x82`@\x01Q\x81\x81Q\x81\x10a\x12\\Wa\x12\\aA\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`\xC0\x85\x01Q`@Q\x7F\xD0d\xA3\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[`\x01\x01a\x11\xE7V[`@\x80Q\x80\x82\x01\x82R`\x07\x81R\x7FDCAHook\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x91\x81\x01\x91\x90\x91R\x7F\xA2\x03t\xB0\x9D\xFC\xE31k({6\xAA\x90\x8C\x96\xCD\xB92\x97U\xA7\xE0\x86\x83\xC5\xB4)\x08\x02f5\x91\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\xA0\x82\x01R_\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16` \x82\x01R`4\x81\x01\x82\x90R_\x90`T\x01`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R\x92\x83\x90R\x91 T\x90\x91P\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\x14\xC7W`@Q\x7F\x0B\x1F\x88F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0E>V[_\x81\x81R` \x81\x90R`@\x80\x82 \x80T~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90UQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91\x83\x91\x7F\xACZK\x90\xE4!\0*/\xDB\x9F\x13+\x9B2\xC2O\xA4\xAE\x16\xECH\x05\x16\xC8Y2\xDE \x8D*3\x91\x90\xA3PPPV[_\x82`\x80\x01Q\x15a\x15\xB9Wa\x15\xB2\x82`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\r\xE0\xB6\xB3\xA7d\0\0\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C\x0BV[\x90Pa\x16\x03V[a\x16\0\x82`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\r\xE0\xB6\xB3\xA7d\0\0\x84`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C\x0BV[\x90P[\x82a\x01\x80\x01Q\x81\x10\x15a\x07\xAEWa\x01\x80\x83\x01Q`@Q\x7F\xBA\x04\xE3m\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0E>\x91\x83\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80Q_\x81\x90\x03a\x16\x8FW`@Q\x7F\x94\x9E\xFC\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[\x82\x81\x10\x15a\x17\xF0W_\x84\x82\x81Q\x81\x10a\x16\xADWa\x16\xADaA\xEAV[` \x02` \x01\x01Q` \x01Q\x90P\x80a\xFF\xFF\x16_\x03a\x16\xF8W`@Q\x7F\xBA\r\x87\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x85\x83\x81Q\x81\x10a\x17\x0BWa\x17\x0BaA\xEAV[` \x02` \x01\x01Q_\x01Q\x90P_\x83`\x01a\x17&\x91\x90aB\x17V[\x90P[\x85\x81\x10\x15a\x17\xD2W\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x82\x81Q\x81\x10a\x17ZWa\x17ZaA\xEAV[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17\xCAW`@Q\x7F>\x91\x85y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x01a\x0E>V[`\x01\x01a\x17)V[Pa\x17\xE1a\xFF\xFF\x83\x16\x85aB\x17V[\x93P\x82`\x01\x01\x92PPPa\x16\x92V[Pa'\x10\x81\x14a\x07\xAEW`@Q\x7F\xBC\x9D\xFE\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R`$\x01a\x0E>V[\x80Q\x15a\x18\xDFWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c+g\xB5pa\x18|\x85\x80aB*V[a\x18\x8D\x90`@\x81\x01\x90` \x01aD V[\x83` \x01Q\x84`@\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xB5\x93\x92\x91\x90aD;V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xCCW__\xFD[PZ\xF1\x92PPP\x80\x15a\x18\xDDWP`\x01[P[a\x07\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\x1D1V[a\x19\x15\x87\x86\x86a\x1E\nV[a\x19'\x87a\x19\"\x83aD\xF9V[a\x10\0V[a\x195\x87a\x01\xC0\x01Qa\x16SV[a\x19@\x87\x84\x84a\x1E\x97V[a\x19K\x86\x88\x85a \x02V[a\x19Z\x87\x84`@\x84\x015a\r\xBDV[a\x19d\x87\x84a\x15`V[a\x19\xCB\x87\x84a\x19v`\x80\x85\x01\x85aC\xA3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x19\xC1Wa\x19\xB2``\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aD\x06V[\x81R` \x01\x90`\x01\x01\x90a\x19\x95V[PPPPPa\"\xA5V[PPPPPPPV[_\x83\x81R` \x81\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94\x82\x01\x94\x90\x94R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x04`\xFF\x16\x15\x15\x91\x83\x01\x91\x90\x91R`\x01\x81\x01T``\x83\x01R`\x02\x01T`\x80\x82\x01R\x82Q\x82\x91\x90\x82\x90\x81[\x81\x81\x10\x15a\x1A\xB4W\x86\x81\x81Q\x81\x10a\x1A\x93Wa\x1A\x93aA\xEAV[` \x02` \x01\x01Q` \x01Q\x83a\x1A\xAA\x91\x90aB\x17V[\x92P`\x01\x01a\x1AyV[P\x82Q\x83a\x1A\xC1\x82aE\x04V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RPn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFB\x16` \x84\x01R``\x83\x01\x80Q\x88\x91\x90a\x1B\x01\x90\x83\x90aB\x17V[\x90RP`\x80\x83\x01\x80Q\x83\x91\x90a\x1B\x18\x90\x83\x90aB\x17V[\x90RPPP_\x95\x86R` \x86\x81R`@\x96\x87\x90 \x82Q\x81T\x92\x84\x01Q\x98\x84\x01Q\x15\x15\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x9A\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x94\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x97\x90\x97\x16\x17\x86U``\x81\x01Q`\x01\x87\x01\x81\x90U`\x80\x90\x91\x01Q`\x02\x90\x96\x01\x86\x90U\x95\x93PPPPV[_\x80\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a\x1CaW\x83\x82\x81a\x1CWWa\x1CWaE@V[\x04\x92PPPa\x08\x97V[\x80\x84\x11a\x1C\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FMath: mulDiv overflow\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E>V[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16c6\xC7\x85\x16a\x1DW\x84\x80aB*V[a\x1Dh\x90`@\x81\x01\x90` \x01aD V[\x83`@\x86\x01\x805\x90a\x1D}\x90` \x89\x01aD V[`@Q`\xE0\x86\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16`\x04\x82\x01R\x92\x84\x16`$\x84\x01R\x90\x83\x16`D\x83\x01R\x90\x91\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\xF8W__\xFD[PZ\xF1\x15\x80\x15a\x19\xCBW=__>=_\xFD[_a\x1E\x15\x84\x84a%\xDDV[\x90P_a\x1E)a\x1E#a\x08\x9EV[\x83a&\xDCV[\x90Pa\x1E9\x85_\x01Q\x82\x85a'\x1DV[a\x1E\x90W\x84Q`@Q\x7F\x17\xAA\x11y\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[PPPPPV[_a\x1E\xA1\x83a'1V[\x90P_a\x1E\xAFa\x1E#a\x08\x9EV[\x90Pa\x1E\xC0\x85`\xE0\x01Q\x82\x85a'\x1DV[a\x1F\x1AW`\xE0\x85\x01Q`@Q\x7F\xCAV\x12\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x84Q\x84Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1F\x93W\x83Q\x85Q`@Q\x7FC\x84)\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x84` \x01Q\x84` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1E\x90W` \x80\x85\x01Q\x90\x86\x01Q`@Q\x7F\xBB\xCC\xF8\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x0E>V[_\x83\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x81To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82Rp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x04n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93\x82\x01\x93\x90\x93R\x7F\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x04`\xFF\x16\x15\x80\x15\x93\x83\x01\x93\x90\x93R`\x01\x81\x01T``\x83\x01R`\x02\x01T`\x80\x82\x01R\x90a \xDAW`@Q\x7F&`\x16\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90R`$\x01a\x0E>V[a\x01\xA0\x83\x01Q\x15\x80\x15\x90a \xF2WP\x82a\x01\xA0\x01QB\x11[\x15a!9Wa\x01\xA0\x83\x01Q`@Q\x7Fo\x08\xEEn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RB`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x0E>V[\x80_\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82``\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a!\xB8W``\x82\x01Q\x81Q`@Q\x7F\x1C\xA8\xC0b\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0E>V[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\t.W_\x81` \x01Qn\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Ba!\xF3\x91\x90aA\xD7V[\x90P\x83a\x01\0\x01Q\x81\x10\x15a\"EWa\x01\0\x84\x01Q`@Q\x7F\xB1\xB9\xB6\xEE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0E>\x91\x83\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[a\x01 \x84\x01Q\x15\x80\x15\x90a\"]WP\x83a\x01 \x01Q\x81\x11[\x15a\x1E\x90Wa\x01 \x84\x01Q`@Q\x7F8\x8B\x01s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x0E>\x91\x83\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80Q_\x90\x81[\x81\x81\x10\x15a\"\xE6W\x83\x81\x81Q\x81\x10a\"\xC5Wa\"\xC5aA\xEAV[` \x02` \x01\x01Q` \x01Q\x83a\"\xDC\x91\x90aB\x17V[\x92P`\x01\x01a\"\xABV[Pa\x01\xC0\x85\x01QQ_[\x81\x81\x10\x15a$\xDCW_\x87a\x01\xC0\x01Q\x82\x81Q\x81\x10a#\x10Wa#\x10aA\xEAV[` \x02` \x01\x01Q_\x01Q\x90P_a#Q\x86\x8Aa\x01\xC0\x01Q\x85\x81Q\x81\x10a#9Wa#9aA\xEAV[` \x02` \x01\x01Q` \x01Qa\xFF\xFF\x16a'\x10a\x1C\x0BV[\x90P_\x80[\x86\x81\x10\x15a#\xE2W\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x82\x81Q\x81\x10a#\x87Wa#\x87aA\xEAV[` \x02` \x01\x01Q`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a#\xDAW\x88\x81\x81Q\x81\x10a#\xC0Wa#\xC0aA\xEAV[` \x02` \x01\x01Q` \x01Q\x82a#\xD7\x91\x90aB\x17V[\x91P[`\x01\x01a#VV[P\x89`\x80\x01Q\x15a$rW\x81a#\xF9\x82`\x01aB\x17V[\x10\x15\x80\x15a$\x11WPa$\r\x82`\x01aB\x17V[\x81\x11\x15[a$mW`@Q\x7Ff\xC6\xA6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x0E>V[a$\xD1V[\x81\x81\x14a$\xD1W`@Q\x7Ff\xC6\xA6\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x0E>V[PPP`\x01\x01a\"\xF0V[P\x85`\x80\x01Q\x15a%eW\x84`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x10\x15a%`W`\x80\x85\x01Q`@Q\x7F,\x19\xB8\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[a\t\x9DV[\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x14a\t\x9DW`@\x80\x86\x01Q\x90Q\x7F~\xBB\xDE\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x85\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`$\x82\x01R`D\x01a\x0E>V[__a%\xED\x84a\x01\xC0\x01Qa'\xC5V[\x90P_`@Q\x80a\x02\xC0\x01`@R\x80a\x02\x93\x81R` \x01aFKa\x02\x93\x919\x80Q\x90` \x01 \x90P_`@Q\x82\x81R\x86Q` \x82\x01R` \x87\x01Q`@\x82\x01R`@\x87\x01Q``\x82\x01R``\x87\x01Q`\x80\x82\x01R`\x80\x87\x01Q`\xA0\x82\x01R`\xA0\x87\x01Q`\xC0\x82\x01R`\xC0\x87\x01Q`\xE0\x82\x01R`\xE0\x87\x01Qa\x01\0\x82\x01Ra\x01\0\x87\x01Qa\x01 \x82\x01Ra\x01 \x87\x01Qa\x01@\x82\x01Ra\x01@\x87\x01Qa\x01`\x82\x01Ra\x01`\x87\x01Qa\x01\x80\x82\x01Ra\x01\x80\x87\x01Qa\x01\xA0\x82\x01Ra\x01\xA0\x87\x01Qa\x01\xC0\x82\x01R\x83a\x01\xE0\x82\x01R\x85a\x02\0\x82\x01Ra\x02 \x81 \x91Pa\x02 \x81\x01`@RP\x80\x93PPPP\x92\x91PPV[`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R_\x90`b\x01a\x0B\xC2V[_a')\x84\x84\x84a)\x1DV[\x94\x93PPPPV[_`@Q\x80`\xA0\x01`@R\x80`k\x81R` \x01aH\xDE`k\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q\x92Qa\x13\xCB\x97\x92\x93\x91\x92\x01\x95\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`@\x87\x01R\x91\x84\x16``\x86\x01R\x90\x91\x16`\x80\x84\x01R\x16`\xA0\x82\x01R`\xC0\x01\x90V[\x80Q_\x90\x81\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xE3Wa'\xE3a-\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\x0CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a(\xECW`@Q\x80``\x01`@R\x80`6\x81R` \x01aF\x15`6\x919\x80Q\x90` \x01 \x85\x82\x81Q\x81\x10a(KWa(KaA\xEAV[` \x02` \x01\x01Q_\x01Q\x86\x83\x81Q\x81\x10a(hWa(haA\xEAV[` \x02` \x01\x01Q` \x01Q`@Q` \x01a(\xB1\x93\x92\x91\x90\x92\x83Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01Ra\xFF\xFF\x16`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a(\xD9Wa(\xD9aA\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a(\x11V[P\x80`@Q` \x01a(\xFE\x91\x90aEmV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92PPP\x91\x90PV[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;_\x03a)\xA2W__a)H\x85\x85a)\xB4V[P\x90\x92P\x90P_\x81`\x03\x81\x11\x15a)aWa)aaE\xA2V[\x14\x80\x15a)\x99WP\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x92PPPa\x08\x97V[a)\xAD\x84\x84\x84a)\xFDV[\x90Pa\x08\x97V[___\x83Q`A\x03a)\xEBW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa)\xDD\x88\x82\x85\x85a+FV[\x95P\x95P\x95PPPPa)\xF6V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[___\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85`@Q`$\x01a**\x92\x91\x90aE\xCFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90RQa*\xAB\x91\x90aE\xE7V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a*\xE3W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a*\xE8V[``\x91P[P\x91P\x91P\x81\x80\x15a*\xFCWP` \x81Q\x10\x15[\x80\x15a+<WP\x80Q\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a+:\x90\x83\x01` \x90\x81\x01\x90\x84\x01aE\xFDV[\x14[\x96\x95PPPPPPV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a+\x7FWP_\x91P`\x03\x90P\x82a,/V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a+\xD0W=__>=_\xFD[PP`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01Q\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a,&WP_\x92P`\x01\x91P\x82\x90Pa,/V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[`@Q\x80a\x02\0\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01``\x81R` \x01a->`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80a-\x7F`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81RP\x90V[\x81R_` \x82\x01R```@\x90\x91\x01R\x90V[__`@\x83\x85\x03\x12\x15a-\xA3W__\xFD[\x825\x91P` \x83\x015n\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a-\xC8W__\xFD[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a-\xE3W__\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Qa\x02\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.:Wa.:a-\xEAV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a/[Wa/[a-\xEAV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xAEW__\xFD[\x805a/\x8F\x81a/cV[\x91\x90PV[\x805\x80\x15\x15\x81\x14a/\x8FW__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/\xBCWa/\xBCa-\xEAV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a/\xD5W__\xFD[\x815a/\xE8a/\xE3\x82a/\xA3V[a/\x14V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a0\tW__\xFD[` \x85\x01[\x83\x81\x10\x15a0gW`@\x81\x88\x03\x12\x15a0%W__\xFD[a0-a.\x17V[\x815a08\x81a/cV[\x81R` \x82\x015a\xFF\xFF\x81\x16\x81\x14a0NW__\xFD[` \x82\x81\x01\x91\x90\x91R\x90\x84R\x92\x90\x92\x01\x91`@\x01a0\x0EV[P\x95\x94PPPPPV[_\x82`\x1F\x83\x01\x12a0\x80W__\xFD[\x815` \x83\x01__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a0\xA0Wa0\xA0a-\xEAV[P`\x1F\x83\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01a0\xD3\x81a/\x14V[\x91PP\x82\x81R\x85\x83\x83\x01\x11\x15a0\xE7W__\xFD[\x82\x82` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a1\x10W__\xFD[\x815a1\x1Ea/\xE3\x82a/\xA3V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a1?W__\xFD[` \x85\x01[\x83\x81\x10\x15a0gW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1bW__\xFD[a1q\x88` \x83\x8A\x01\x01a0qV[\x84RP` \x92\x83\x01\x92\x01a1DV[_`\xA0\x82\x84\x03\x12\x15a1\x90W__\xFD[a1\x98a.@V[\x825\x81R` \x80\x84\x015\x90\x82\x01R`@\x80\x84\x015\x90\x82\x01R``\x80\x84\x015\x90\x82\x01R\x90P`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xD5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a1\xE5W__\xFD[\x805a1\xF3a/\xE3\x82a/\xA3V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a2\x14W__\xFD[` \x84\x01[\x83\x81\x10\x15a3\xADW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a27W__\xFD[\x85\x01``\x81\x8A\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x12\x15a2jW__\xFD[a2ra.cV[` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x8BW__\xFD[` \x81\x84\x01\x01\x90P`\xA0\x81\x8C\x03\x12\x15a2\xA2W__\xFD[a2\xAAa.@V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xC0W__\xFD[a2\xCC\x8D\x82\x85\x01a0qV[\x82RP` \x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\xE8W__\xFD[a2\xF4\x8D\x82\x85\x01a0qV[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x13W__\xFD[a3\x1F\x8D\x82\x85\x01a1\x01V[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3>W__\xFD[a3J\x8D\x82\x85\x01a1\x01V[``\x83\x01RP`\x80\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x81Ra3k`@\x83\x01a/\x84V[` \x82\x01R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3\x89W__\xFD[a3\x98\x8B` \x83\x86\x01\x01a0qV[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01a2\x19V[P`\x80\x85\x01RP\x91\x94\x93PPPPV[_a\x02\0\x82\x84\x03\x12\x15a3\xCEW__\xFD[a3\xD6a.\x86V[\x90Pa3\xE1\x82a/\x84V[\x81R` \x82\x81\x015\x90\x82\x01R`@\x80\x83\x015\x90\x82\x01Ra4\x03``\x83\x01a/\x84V[``\x82\x01Ra4\x14`\x80\x83\x01a/\x94V[`\x80\x82\x01Ra4%`\xA0\x83\x01a/\x84V[`\xA0\x82\x01Ra46`\xC0\x83\x01a/\x84V[`\xC0\x82\x01Ra4G`\xE0\x83\x01a/\x84V[`\xE0\x82\x01Ra\x01\0\x82\x81\x015\x90\x82\x01Ra\x01 \x80\x83\x015\x90\x82\x01Ra\x01@\x80\x83\x015\x90\x82\x01Ra\x01`\x80\x83\x015\x90\x82\x01Ra\x01\x80\x80\x83\x015\x90\x82\x01Ra\x01\xA0\x80\x83\x015\x90\x82\x01Ra\x01\xC0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xA8W__\xFD[a4\xB4\x84\x82\x85\x01a/\xC6V[a\x01\xC0\x83\x01RPa\x01\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xD5W__\xFD[a4\xE1\x84\x82\x85\x01a1\x80V[a\x01\xE0\x83\x01RP\x92\x91PPV[\x805k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\x8FW__\xFD[_`\xA0\x82\x84\x03\x12\x15a5\x19W__\xFD[a5!a.@V[\x90P\x815a5.\x81a/cV[\x81Ra5<` \x83\x01a4\xEEV[` \x82\x01R`@\x82\x015a5O\x81a/cV[`@\x82\x01Ra5```\x83\x01a4\xEEV[``\x82\x01R`\x80\x82\x015a5s\x81a/cV[`\x80\x82\x01R\x92\x91PPV[___`\xE0\x84\x86\x03\x12\x15a5\x90W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xA6W__\xFD[a5\xB2\x86\x82\x87\x01a3\xBDV[\x93PPa5\xC2\x85` \x86\x01a5\tV[\x92\x95\x92\x94PPP`\xC0\x91\x90\x91\x015\x90V[_a\x01 \x82\x84\x03\x12\x15a5\xE4W__\xFD[a5\xECa.\xAAV[\x90Pa5\xF7\x82a/\x84V[\x81Ra6\x05` \x83\x01a/\x84V[` \x82\x01R`@\x82\x81\x015\x90\x82\x01R``\x80\x83\x015\x90\x82\x01Ra6*`\x80\x83\x01a/\x84V[`\x80\x82\x01R`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6HW__\xFD[a6T\x84\x82\x85\x01a0qV[`\xA0\x83\x01RPa6f`\xC0\x83\x01a/\x84V[`\xC0\x82\x01R`\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x84W__\xFD[a6\x90\x84\x82\x85\x01a0qV[`\xE0\x83\x01RPa6\xA3a\x01\0\x83\x01a/\x84V[a\x01\0\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a6\xBFW__\xFD[a6\xC7a.cV[\x90P\x815a6\xD4\x81a/cV[\x81R` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x90PV[_``\x82\x84\x03\x12\x15a7\x02W__\xFD[a7\na.cV[\x90P\x815a7\x17\x81a/cV[\x81R` \x82\x81\x015\x90\x82\x01R`@\x82\x015a71\x81a/cV[`@\x82\x01R\x92\x91PPV[_\x82`\x1F\x83\x01\x12a7KW__\xFD[\x815a7Ya/\xE3\x82a/\xA3V[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x86\x01\x01\x92P\x85\x83\x11\x15a7zW__\xFD[` \x85\x01[\x83\x81\x10\x15a0gWa7\x91\x87\x82a6\xF2V[\x83R` \x90\x92\x01\x91``\x01a7\x7FV[_a\x01 \x82\x84\x03\x12\x15a7\xB2W__\xFD[a7\xBAa.\xCEV[\x90P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xD2W__\xFD[a7\xDE\x84\x82\x85\x01a5\xD3V[\x82RPa7\xEE\x83` \x84\x01a6\xAFV[` \x82\x01R`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\x0CW__\xFD[a8\x18\x84\x82\x85\x01a7<V[`@\x83\x01RP`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a87W__\xFD[a8C\x84\x82\x85\x01a0qV[``\x83\x01RP`\xC0\x82\x015`\x80\x82\x01Ra8_`\xE0\x83\x01a/\x84V[`\xA0\x82\x01Ra\x01\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8~W__\xFD[a8\x8A\x84\x82\x85\x01a0qV[`\xC0\x83\x01RP\x92\x91PPV[__`@\x83\x85\x03\x12\x15a8\xA7W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xBDW__\xFD[a8\xC9\x85\x82\x86\x01a3\xBDV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a8\xE5W__\xFD[a8\xF1\x85\x82\x86\x01a7\xA1V[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a9\rW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[_____`\xA0\x86\x88\x03\x12\x15a98W__\xFD[\x855a9C\x81a/cV[\x94Pa9Q` \x87\x01a4\xEEV[\x93P`@\x86\x015a9a\x81a/cV[\x92P``\x86\x015a9q\x81a/cV[\x91Pa9\x7F`\x80\x87\x01a4\xEEV[\x90P\x92\x95P\x92\x95\x90\x93PV[__` \x83\x85\x03\x12\x15a9\x9CW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xB2W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a9\xC2W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xD8W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a9\xECW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[____`\x80\x85\x87\x03\x12\x15a:\x0FW__\xFD[a:\x18\x85a/\x94V[\x93P` \x85\x015a:(\x81a/cV[\x92P`@\x85\x015a:8\x81a/cV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_` \x82\x84\x03\x12\x15a:XW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:nW__\xFD[a')\x84\x82\x85\x01a/\xC6V[_a\x01 \x82\x84\x03\x12\x15a:\x8BW__\xFD[P\x91\x90PV[\x805e\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\x8FW__\xFD[_\x81\x83\x03a\x01\0\x81\x12\x15a:\xB8W__\xFD[a:\xC0a.cV[\x91Pa:\xCB\x83a/\x94V[\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01`\xC0\x81\x12\x15a:\xFCW__\xFD[a;\x04a.cV[`\x80\x82\x12\x15a;\x11W__\xFD[a;\x19a.\xF1V[\x91P` \x84\x015a;)\x81a/cV[\x82R`@\x84\x015a;9\x81a/cV[` \x83\x01Ra;J``\x85\x01a:\x91V[`@\x83\x01Ra;[`\x80\x85\x01a:\x91V[``\x83\x01R\x81\x81Ra;o`\xA0\x85\x01a/\x84V[` \x82\x81\x01\x91\x90\x91R`\xC0\x85\x015`@\x83\x01R\x83\x01RP`\xE0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\x9FW__\xFD[a;\xAB\x84\x82\x85\x01a0qV[`@\x83\x01RP\x92\x91PPV[___``\x84\x86\x03\x12\x15a;\xC9W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xDFW__\xFD[a;\xEB\x86\x82\x87\x01a:zV[\x93PP` \x84\x015a;\xFC\x81a/cV[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\x17W__\xFD[a<#\x86\x82\x87\x01a:\xA6V[\x91PP\x92P\x92P\x92V[_____`\xA0\x86\x88\x03\x12\x15a<AW__\xFD[\x855a<L\x81a/cV[\x94Pa<Z` \x87\x01a4\xEEV[\x93Pa<h`@\x87\x01a/\x94V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a<\xD6W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R` \x90\x81\x01Qa\xFF\xFF\x16\x81\x88\x01R`@\x90\x96\x01\x95\x90\x91\x01\x90`\x01\x01a<\x92V[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a=\x98W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x85\x84\x03\x01\x88Ra=\x82\x83\x83Qa<\xE0V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a=HV[P\x90\x96\x95PPPPPPV[_`\xA0\x83\x01\x82Q\x84R` \x83\x01Q` \x85\x01R`@\x83\x01Q`@\x85\x01R``\x83\x01Q``\x85\x01R`\x80\x83\x01Q`\xA0`\x80\x86\x01R\x81\x81Q\x80\x84R`\xC0\x87\x01\x91P`\xC0\x81`\x05\x1B\x88\x01\x01\x93P` \x83\x01\x92P_[\x81\x81\x10\x15a?^W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x88\x86\x03\x01\x83R\x83Q\x80Q``\x87R\x80Q`\xA0``\x89\x01Ra>Da\x01\0\x89\x01\x82a<\xE0V[\x90P` \x82\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x83\x03\x01`\x80\x8A\x01Ra>\x7F\x82\x82a<\xE0V[\x91PP`@\x82\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x83\x03\x01`\xA0\x8A\x01Ra>\xBB\x82\x82a=,V[\x91PP``\x82\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x89\x83\x03\x01`\xC0\x8A\x01Ra>\xF7\x82\x82a=,V[\x91PP`\x80\x82\x01Q`\xE0\x89\x01R` \x83\x01Q\x91Pa?-` \x89\x01\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[`@\x83\x01Q\x92P\x87\x81\x03`@\x89\x01Ra?F\x81\x84a<\xE0V[\x97PPP` \x94\x85\x01\x94\x93\x90\x93\x01\x92P`\x01\x01a=\xF6V[P\x92\x96\x95PPPPPPV[` \x81Ra?\x91` \x82\x01\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[` \x82\x01Q`@\x82\x01R`@\x82\x01Q``\x82\x01R_``\x83\x01Qa?\xCD`\x80\x84\x01\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90RV[P`\x80\x83\x01Q\x80\x15\x15`\xA0\x84\x01RP`\xA0\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RP`\xC0\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\xE0\x84\x01RP`\xE0\x83\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x01\0\x84\x01RPa\x01\0\x83\x01Qa\x01 \x83\x01Ra\x01 \x83\x01Qa\x01@\x83\x01Ra\x01@\x83\x01Qa\x01`\x83\x01Ra\x01`\x83\x01Qa\x01\x80\x83\x01Ra\x01\x80\x83\x01Qa\x01\xA0\x83\x01Ra\x01\xA0\x83\x01Qa\x01\xC0\x83\x01Ra\x01\xC0\x83\x01Qa\x02\0a\x01\xE0\x84\x01Ra@\xA8a\x02 \x84\x01\x82a<\x80V[\x90Pa\x01\xE0\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x84\x83\x03\x01a\x02\0\x85\x01Ra@\xE5\x82\x82a=\xA4V[\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a@\xFFW__\xFD[\x825aA\n\x81a/cV[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15aA*W__\xFD[\x835\x92P` \x84\x015o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aAPW__\xFD[\x91PaA^`@\x85\x01a/\x94V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aAxW__\xFD[\x825aA\x83\x81a/cV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA\x9EW__\xFD[a8\xF1\x85\x82\x86\x01a:zV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0B\xDBWa\x0B\xDBaA\xAAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xDBWa\x0B\xDBaA\xAAV[_\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xE1\x836\x03\x01\x81\x12aB\\W__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12aB\x99W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\xB3W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aB\xC7W__\xFD[\x92P\x92\x90PV[______a\x01@\x87\x89\x03\x12\x15aB\xE4W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xFAW__\xFD[aC\x06\x89\x82\x8A\x01a3\xBDV[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\"W__\xFD[aC.\x89\x82\x8A\x01a0qV[\x95PP`@\x87\x015\x93PaCE\x88``\x89\x01a5\tV[\x92Pa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aCaW__\xFD[aCm\x89\x82\x8A\x01a0qV[\x92PPa\x01 \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\x8AW__\xFD[aC\x96\x89\x82\x8A\x01a:\xA6V[\x91PP\x92\x95P\x92\x95P\x92\x95V[__\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12aC\xD6W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aC\xF0W__\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aB\xC7W__\xFD[_``\x82\x84\x03\x12\x15aD\x16W__\xFD[a\x08\x97\x83\x83a6\xF2V[_` \x82\x84\x03\x12\x15aD0W__\xFD[\x815a\x08\x97\x81a/cV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R_\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`@\x84\x01Re\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01Q\x16``\x84\x01Re\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16`\x80\x84\x01RPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x85\x01Q\x16`\xA0\x83\x01R`@\x84\x01Q`\xC0\x83\x01Ra\x01\0`\xE0\x83\x01Ra@\xE5a\x01\0\x83\x01\x84a<\xE0V[_a\x0B\xDB6\x83a7\xA1V[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x03aE7WaE7aA\xAAV[`\x01\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[\x81Q_\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15aE\x97W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aEyV[P\x91\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a')`@\x83\x01\x84a<\xE0V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15aF\rW__\xFD[PQ\x91\x90PV\xFEOutputAllocation(address recipient,uint16 basisPoints)DCAIntent(address swapper,uint256 nonce,uint256 chainId,address hookAddress,bool isExactIn,address inputToken,address outputToken,address cosigner,uint256 minPeriod,uint256 maxPeriod,uint256 minChunkSize,uint256 maxChunkSize,uint256 minPrice,uint256 deadline,OutputAllocation[] outputAllocations,PrivateIntent privateIntent)FeedInfo(FeedTemplate feedTemplate,address feedAddress,string feedType)FeedTemplate(string name,string expression,string[] parameters,string[] secrets,uint256 retryCount)OutputAllocation(address recipient,uint16 basisPoints)PrivateIntent(uint256 totalAmount,uint256 exactFrequency,uint256 numChunks,bytes32 salt,FeedInfo[] oracleFeeds)DCAOrderCosignerData(address swapper,uint96 nonce,uint160 execAmount,uint96 orderNonce,uint160 limitAmount)\xA2dipfsX\"\x12 \\a\x87{_\x80\xD1\xB7\x07\xA9\xC4\xEF\x9CxE\x8D\x1En'\xCD\x18\xA8\x1E\xF2Wjx~:u\xA6ydsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct DCAExecutionState { uint128 executedChunks; uint120 lastExecutionTime; bool cancelled; uint256 totalInputExecuted; uint256 totalOutput; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DCAExecutionState {
        #[allow(missing_docs)]
        pub executedChunks: u128,
        #[allow(missing_docs)]
        pub lastExecutionTime: alloy::sol_types::private::primitives::aliases::U120,
        #[allow(missing_docs)]
        pub cancelled: bool,
        #[allow(missing_docs)]
        pub totalInputExecuted: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalOutput: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<120>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u128,
            alloy::sol_types::private::primitives::aliases::U120,
            bool,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<DCAExecutionState> for UnderlyingRustTuple<'_> {
            fn from(value: DCAExecutionState) -> Self {
                (
                    value.executedChunks,
                    value.lastExecutionTime,
                    value.cancelled,
                    value.totalInputExecuted,
                    value.totalOutput,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DCAExecutionState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    executedChunks: tuple.0,
                    lastExecutionTime: tuple.1,
                    cancelled: tuple.2,
                    totalInputExecuted: tuple.3,
                    totalOutput: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DCAExecutionState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DCAExecutionState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.executedChunks),
                    <alloy::sol_types::sol_data::Uint<
                        120,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastExecutionTime),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.cancelled,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalInputExecuted),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalOutput),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DCAExecutionState {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for DCAExecutionState {
            const NAME: &'static str = "DCAExecutionState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DCAExecutionState(uint128 executedChunks,uint120 lastExecutionTime,bool cancelled,uint256 totalInputExecuted,uint256 totalOutput)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.executedChunks,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        120,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.lastExecutionTime,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cancelled,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalInputExecuted,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalOutput)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DCAExecutionState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.executedChunks,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        120,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lastExecutionTime,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cancelled,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalInputExecuted,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalOutput,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.executedChunks,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    120,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lastExecutionTime,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cancelled,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalInputExecuted,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalOutput,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct DCAIntent { address swapper; uint256 nonce; uint256 chainId; address hookAddress; bool isExactIn; address inputToken; address outputToken; address cosigner; uint256 minPeriod; uint256 maxPeriod; uint256 minChunkSize; uint256 maxChunkSize; uint256 minPrice; uint256 deadline; OutputAllocation[] outputAllocations; PrivateIntent privateIntent; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DCAIntent {
        #[allow(missing_docs)]
        pub swapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hookAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub isExactIn: bool,
        #[allow(missing_docs)]
        pub inputToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub outputToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub cosigner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub minPeriod: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxPeriod: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minChunkSize: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxChunkSize: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minPrice: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub outputAllocations: alloy::sol_types::private::Vec<
            <OutputAllocation as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub privateIntent: <PrivateIntent as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<OutputAllocation>,
            PrivateIntent,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            bool,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Vec<
                <OutputAllocation as alloy::sol_types::SolType>::RustType,
            >,
            <PrivateIntent as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<DCAIntent> for UnderlyingRustTuple<'_> {
            fn from(value: DCAIntent) -> Self {
                (
                    value.swapper,
                    value.nonce,
                    value.chainId,
                    value.hookAddress,
                    value.isExactIn,
                    value.inputToken,
                    value.outputToken,
                    value.cosigner,
                    value.minPeriod,
                    value.maxPeriod,
                    value.minChunkSize,
                    value.maxChunkSize,
                    value.minPrice,
                    value.deadline,
                    value.outputAllocations,
                    value.privateIntent,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DCAIntent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    swapper: tuple.0,
                    nonce: tuple.1,
                    chainId: tuple.2,
                    hookAddress: tuple.3,
                    isExactIn: tuple.4,
                    inputToken: tuple.5,
                    outputToken: tuple.6,
                    cosigner: tuple.7,
                    minPeriod: tuple.8,
                    maxPeriod: tuple.9,
                    minChunkSize: tuple.10,
                    maxChunkSize: tuple.11,
                    minPrice: tuple.12,
                    deadline: tuple.13,
                    outputAllocations: tuple.14,
                    privateIntent: tuple.15,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DCAIntent {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DCAIntent {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.swapper,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.hookAddress,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isExactIn,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.inputToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.outputToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.cosigner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minPeriod),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxPeriod),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minChunkSize),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxChunkSize),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minPrice),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Array<
                        OutputAllocation,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputAllocations),
                    <PrivateIntent as alloy_sol_types::SolType>::tokenize(
                        &self.privateIntent,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DCAIntent {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for DCAIntent {
            const NAME: &'static str = "DCAIntent";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DCAIntent(address swapper,uint256 nonce,uint256 chainId,address hookAddress,bool isExactIn,address inputToken,address outputToken,address cosigner,uint256 minPeriod,uint256 maxPeriod,uint256 minChunkSize,uint256 maxChunkSize,uint256 minPrice,uint256 deadline,OutputAllocation[] outputAllocations,PrivateIntent privateIntent)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <OutputAllocation as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OutputAllocation as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <PrivateIntent as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <PrivateIntent as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.swapper,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.chainId)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hookAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isExactIn,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.inputToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.outputToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cosigner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minPeriod)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxPeriod)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minChunkSize)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxChunkSize)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.minPrice)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.deadline)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OutputAllocation,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.outputAllocations,
                        )
                        .0,
                    <PrivateIntent as alloy_sol_types::SolType>::eip712_data_word(
                            &self.privateIntent,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DCAIntent {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.swapper,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hookAddress,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isExactIn,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inputToken,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputToken,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cosigner,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minPeriod,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxPeriod,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minChunkSize,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxChunkSize,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.minPrice,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deadline,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OutputAllocation,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputAllocations,
                    )
                    + <PrivateIntent as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.privateIntent,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.swapper,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chainId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hookAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isExactIn,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.inputToken,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outputToken,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cosigner,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minPeriod,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxPeriod,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minChunkSize,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxChunkSize,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.minPrice,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deadline,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OutputAllocation,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outputAllocations,
                    out,
                );
                <PrivateIntent as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.privateIntent,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct DCAOrderCosignerData { address swapper; uint96 nonce; uint160 execAmount; uint96 orderNonce; uint160 limitAmount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DCAOrderCosignerData {
        #[allow(missing_docs)]
        pub swapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub execAmount: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub orderNonce: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub limitAmount: alloy::sol_types::private::primitives::aliases::U160,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<96>,
            alloy::sol_types::sol_data::Uint<160>,
            alloy::sol_types::sol_data::Uint<96>,
            alloy::sol_types::sol_data::Uint<160>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
            alloy::sol_types::private::primitives::aliases::U160,
            alloy::sol_types::private::primitives::aliases::U96,
            alloy::sol_types::private::primitives::aliases::U160,
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
        impl ::core::convert::From<DCAOrderCosignerData> for UnderlyingRustTuple<'_> {
            fn from(value: DCAOrderCosignerData) -> Self {
                (
                    value.swapper,
                    value.nonce,
                    value.execAmount,
                    value.orderNonce,
                    value.limitAmount,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DCAOrderCosignerData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    swapper: tuple.0,
                    nonce: tuple.1,
                    execAmount: tuple.2,
                    orderNonce: tuple.3,
                    limitAmount: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DCAOrderCosignerData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DCAOrderCosignerData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.swapper,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.execAmount),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderNonce),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.limitAmount),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DCAOrderCosignerData {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for DCAOrderCosignerData {
            const NAME: &'static str = "DCAOrderCosignerData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DCAOrderCosignerData(address swapper,uint96 nonce,uint160 execAmount,uint96 orderNonce,uint160 limitAmount)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.swapper,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.execAmount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.orderNonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.limitAmount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DCAOrderCosignerData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.swapper,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.execAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.orderNonce,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.limitAmount,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.swapper,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    160,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.execAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.orderNonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    160,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.limitAmount,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FeedInfo { FeedTemplate feedTemplate; address feedAddress; string feedType; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeedInfo {
        #[allow(missing_docs)]
        pub feedTemplate: <FeedTemplate as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub feedAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub feedType: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            FeedTemplate,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <FeedTemplate as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<FeedInfo> for UnderlyingRustTuple<'_> {
            fn from(value: FeedInfo) -> Self {
                (value.feedTemplate, value.feedAddress, value.feedType)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FeedInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    feedTemplate: tuple.0,
                    feedAddress: tuple.1,
                    feedType: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FeedInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FeedInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <FeedTemplate as alloy_sol_types::SolType>::tokenize(
                        &self.feedTemplate,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.feedAddress,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.feedType,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FeedInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FeedInfo {
            const NAME: &'static str = "FeedInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FeedInfo(FeedTemplate feedTemplate,address feedAddress,string feedType)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <FeedTemplate as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <FeedTemplate as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <FeedTemplate as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feedTemplate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feedAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feedType,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FeedInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <FeedTemplate as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feedTemplate,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feedAddress,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feedType,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <FeedTemplate as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feedTemplate,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feedAddress,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feedType,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FeedTemplate { string name; string expression; string[] parameters; string[] secrets; uint256 retryCount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeedTemplate {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub expression: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub parameters: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub secrets: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        #[allow(missing_docs)]
        pub retryCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::String,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<FeedTemplate> for UnderlyingRustTuple<'_> {
            fn from(value: FeedTemplate) -> Self {
                (
                    value.name,
                    value.expression,
                    value.parameters,
                    value.secrets,
                    value.retryCount,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FeedTemplate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    name: tuple.0,
                    expression: tuple.1,
                    parameters: tuple.2,
                    secrets: tuple.3,
                    retryCount: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FeedTemplate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FeedTemplate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.expression,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.parameters),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.secrets),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.retryCount),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FeedTemplate {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FeedTemplate {
            const NAME: &'static str = "FeedTemplate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FeedTemplate(string name,string expression,string[] parameters,string[] secrets,uint256 retryCount)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.name,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.expression,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.parameters)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.secrets)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.retryCount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FeedTemplate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.name,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expression,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.parameters,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.secrets,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.retryCount,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.name,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expression,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.parameters,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.secrets,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.retryCount,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct InputToken { address token; uint256 amount; uint256 maxAmount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputToken {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InputToken> for UnderlyingRustTuple<'_> {
            fn from(value: InputToken) -> Self {
                (value.token, value.amount, value.maxAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    amount: tuple.1,
                    maxAmount: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for InputToken {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for InputToken {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxAmount),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for InputToken {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for InputToken {
            const NAME: &'static str = "InputToken";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "InputToken(address token,uint256 amount,uint256 maxAmount)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.maxAmount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for InputToken {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxAmount,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxAmount,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct OrderInfo { address reactor; address swapper; uint256 nonce; uint256 deadline; address preExecutionHook; bytes preExecutionHookData; address postExecutionHook; bytes postExecutionHookData; address auctionResolver; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OrderInfo {
        #[allow(missing_docs)]
        pub reactor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub swapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub preExecutionHook: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub preExecutionHookData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub postExecutionHook: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub postExecutionHookData: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub auctionResolver: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<OrderInfo> for UnderlyingRustTuple<'_> {
            fn from(value: OrderInfo) -> Self {
                (
                    value.reactor,
                    value.swapper,
                    value.nonce,
                    value.deadline,
                    value.preExecutionHook,
                    value.preExecutionHookData,
                    value.postExecutionHook,
                    value.postExecutionHookData,
                    value.auctionResolver,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OrderInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    reactor: tuple.0,
                    swapper: tuple.1,
                    nonce: tuple.2,
                    deadline: tuple.3,
                    preExecutionHook: tuple.4,
                    preExecutionHookData: tuple.5,
                    postExecutionHook: tuple.6,
                    postExecutionHookData: tuple.7,
                    auctionResolver: tuple.8,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OrderInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OrderInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.reactor,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.swapper,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.preExecutionHook,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.preExecutionHookData,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.postExecutionHook,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.postExecutionHookData,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.auctionResolver,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OrderInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OrderInfo {
            const NAME: &'static str = "OrderInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address preExecutionHook,bytes preExecutionHookData,address postExecutionHook,bytes postExecutionHookData,address auctionResolver)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.reactor,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.swapper,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.deadline)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.preExecutionHook,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.preExecutionHookData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.postExecutionHook,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.postExecutionHookData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.auctionResolver,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OrderInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.reactor,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.swapper,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deadline,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.preExecutionHook,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.preExecutionHookData,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.postExecutionHook,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.postExecutionHookData,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.auctionResolver,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.reactor,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.swapper,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deadline,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.preExecutionHook,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.preExecutionHookData,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.postExecutionHook,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.postExecutionHookData,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.auctionResolver,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct OutputAllocation { address recipient; uint16 basisPoints; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutputAllocation {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub basisPoints: u16,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u16);
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
        impl ::core::convert::From<OutputAllocation> for UnderlyingRustTuple<'_> {
            fn from(value: OutputAllocation) -> Self {
                (value.recipient, value.basisPoints)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutputAllocation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    recipient: tuple.0,
                    basisPoints: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OutputAllocation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OutputAllocation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.basisPoints),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OutputAllocation {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OutputAllocation {
            const NAME: &'static str = "OutputAllocation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OutputAllocation(address recipient,uint16 basisPoints)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.recipient,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.basisPoints)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OutputAllocation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.recipient,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.basisPoints,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.recipient,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.basisPoints,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct OutputToken { address token; uint256 amount; address recipient; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutputToken {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<OutputToken> for UnderlyingRustTuple<'_> {
            fn from(value: OutputToken) -> Self {
                (value.token, value.amount, value.recipient)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutputToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    amount: tuple.1,
                    recipient: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OutputToken {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OutputToken {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OutputToken {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OutputToken {
            const NAME: &'static str = "OutputToken";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OutputToken(address token,uint256 amount,address recipient)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.recipient,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OutputToken {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.recipient,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.recipient,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**```solidity
struct PermitData { bool hasPermit; IAllowanceTransfer.PermitSingle permitSingle; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PermitData {
        #[allow(missing_docs)]
        pub hasPermit: bool,
        #[allow(missing_docs)]
        pub permitSingle: <IAllowanceTransfer::PermitSingle as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bool,
            IAllowanceTransfer::PermitSingle,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            bool,
            <IAllowanceTransfer::PermitSingle as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<PermitData> for UnderlyingRustTuple<'_> {
            fn from(value: PermitData) -> Self {
                (value.hasPermit, value.permitSingle, value.signature)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PermitData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    hasPermit: tuple.0,
                    permitSingle: tuple.1,
                    signature: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PermitData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PermitData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.hasPermit,
                    ),
                    <IAllowanceTransfer::PermitSingle as alloy_sol_types::SolType>::tokenize(
                        &self.permitSingle,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PermitData {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PermitData {
            const NAME: &'static str = "PermitData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PermitData(bool hasPermit,PermitSingle permitSingle,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <IAllowanceTransfer::PermitSingle as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <IAllowanceTransfer::PermitSingle as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hasPermit,
                        )
                        .0,
                    <IAllowanceTransfer::PermitSingle as alloy_sol_types::SolType>::eip712_data_word(
                            &self.permitSingle,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PermitData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hasPermit,
                    )
                    + <IAllowanceTransfer::PermitSingle as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.permitSingle,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hasPermit,
                    out,
                );
                <IAllowanceTransfer::PermitSingle as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.permitSingle,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct PrivateIntent { uint256 totalAmount; uint256 exactFrequency; uint256 numChunks; bytes32 salt; FeedInfo[] oracleFeeds; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PrivateIntent {
        #[allow(missing_docs)]
        pub totalAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub exactFrequency: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub numChunks: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub oracleFeeds: alloy::sol_types::private::Vec<
            <FeedInfo as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<FeedInfo>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Vec<
                <FeedInfo as alloy::sol_types::SolType>::RustType,
            >,
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
        impl ::core::convert::From<PrivateIntent> for UnderlyingRustTuple<'_> {
            fn from(value: PrivateIntent) -> Self {
                (
                    value.totalAmount,
                    value.exactFrequency,
                    value.numChunks,
                    value.salt,
                    value.oracleFeeds,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PrivateIntent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    totalAmount: tuple.0,
                    exactFrequency: tuple.1,
                    numChunks: tuple.2,
                    salt: tuple.3,
                    oracleFeeds: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PrivateIntent {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PrivateIntent {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.exactFrequency),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.numChunks),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Array<
                        FeedInfo,
                    > as alloy_sol_types::SolType>::tokenize(&self.oracleFeeds),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PrivateIntent {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PrivateIntent {
            const NAME: &'static str = "PrivateIntent";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PrivateIntent(uint256 totalAmount,uint256 exactFrequency,uint256 numChunks,bytes32 salt,FeedInfo[] oracleFeeds)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<FeedInfo as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <FeedInfo as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalAmount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.exactFrequency,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.numChunks)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        FeedInfo,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.oracleFeeds)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PrivateIntent {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.exactFrequency,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numChunks,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Array<
                        FeedInfo,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.oracleFeeds,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.exactFrequency,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numChunks,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    FeedInfo,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.oracleFeeds,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ResolvedOrder { OrderInfo info; InputToken input; OutputToken[] outputs; bytes sig; bytes32 hash; address auctionResolver; string witnessTypeString; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ResolvedOrder {
        #[allow(missing_docs)]
        pub info: <OrderInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub input: <InputToken as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub outputs: alloy::sol_types::private::Vec<
            <OutputToken as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub sig: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub hash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub auctionResolver: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub witnessTypeString: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            OrderInfo,
            InputToken,
            alloy::sol_types::sol_data::Array<OutputToken>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <OrderInfo as alloy::sol_types::SolType>::RustType,
            <InputToken as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<
                <OutputToken as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<ResolvedOrder> for UnderlyingRustTuple<'_> {
            fn from(value: ResolvedOrder) -> Self {
                (
                    value.info,
                    value.input,
                    value.outputs,
                    value.sig,
                    value.hash,
                    value.auctionResolver,
                    value.witnessTypeString,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ResolvedOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    info: tuple.0,
                    input: tuple.1,
                    outputs: tuple.2,
                    sig: tuple.3,
                    hash: tuple.4,
                    auctionResolver: tuple.5,
                    witnessTypeString: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ResolvedOrder {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ResolvedOrder {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <OrderInfo as alloy_sol_types::SolType>::tokenize(&self.info),
                    <InputToken as alloy_sol_types::SolType>::tokenize(&self.input),
                    <alloy::sol_types::sol_data::Array<
                        OutputToken,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputs),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.sig,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.auctionResolver,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.witnessTypeString,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ResolvedOrder {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ResolvedOrder {
            const NAME: &'static str = "ResolvedOrder";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ResolvedOrder(OrderInfo info,InputToken input,OutputToken[] outputs,bytes sig,bytes32 hash,address auctionResolver,string witnessTypeString)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
                components
                    .push(<OrderInfo as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <OrderInfo as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <InputToken as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <InputToken as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <OutputToken as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OutputToken as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <OrderInfo as alloy_sol_types::SolType>::eip712_data_word(&self.info)
                        .0,
                    <InputToken as alloy_sol_types::SolType>::eip712_data_word(
                            &self.input,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OutputToken,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.outputs)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sig,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.hash)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.auctionResolver,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.witnessTypeString,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ResolvedOrder {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <OrderInfo as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.info,
                    )
                    + <InputToken as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.input,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OutputToken,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.outputs,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sig,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.hash)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.auctionResolver,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.witnessTypeString,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <OrderInfo as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.info,
                    out,
                );
                <InputToken as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.input,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OutputToken,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.outputs,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sig,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hash,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.auctionResolver,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.witnessTypeString,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AllocationMismatch(address,uint256,uint256)` and selector `0x66c6a6af`.
```solidity
error AllocationMismatch(address recipient, uint256 actual, uint256 expected);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AllocationMismatch {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub actual: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub expected: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AllocationMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: AllocationMismatch) -> Self {
                (value.recipient, value.actual, value.expected)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AllocationMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    recipient: tuple.0,
                    actual: tuple.1,
                    expected: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AllocationMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AllocationMismatch(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [102u8, 198u8, 166u8, 175u8];
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
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actual),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expected),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AllocationsNot100Percent(uint256)` and selector `0xbc9dfe8c`.
```solidity
error AllocationsNot100Percent(uint256 totalBasisPoints);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AllocationsNot100Percent {
        #[allow(missing_docs)]
        pub totalBasisPoints: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AllocationsNot100Percent>
        for UnderlyingRustTuple<'_> {
            fn from(value: AllocationsNot100Percent) -> Self {
                (value.totalBasisPoints,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AllocationsNot100Percent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { totalBasisPoints: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AllocationsNot100Percent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AllocationsNot100Percent(uint256)";
            const SELECTOR: [u8; 4] = [188u8, 157u8, 254u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalBasisPoints),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ChunkSizeAboveMax(uint256,uint256)` and selector `0xa24e3c07`.
```solidity
error ChunkSizeAboveMax(uint256 amount, uint256 maxChunkSize);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChunkSizeAboveMax {
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxChunkSize: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ChunkSizeAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: ChunkSizeAboveMax) -> Self {
                (value.amount, value.maxChunkSize)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChunkSizeAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount: tuple.0,
                    maxChunkSize: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChunkSizeAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChunkSizeAboveMax(uint256,uint256)";
            const SELECTOR: [u8; 4] = [162u8, 78u8, 60u8, 7u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxChunkSize),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ChunkSizeBelowMin(uint256,uint256)` and selector `0xc9e31983`.
```solidity
error ChunkSizeBelowMin(uint256 amount, uint256 minChunkSize);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChunkSizeBelowMin {
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minChunkSize: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ChunkSizeBelowMin> for UnderlyingRustTuple<'_> {
            fn from(value: ChunkSizeBelowMin) -> Self {
                (value.amount, value.minChunkSize)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChunkSizeBelowMin {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount: tuple.0,
                    minChunkSize: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChunkSizeBelowMin {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChunkSizeBelowMin(uint256,uint256)";
            const SELECTOR: [u8; 4] = [201u8, 227u8, 25u8, 131u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minChunkSize),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CosignerNonceMismatch(uint96,uint256)` and selector `0xbbccf88f`.
```solidity
error CosignerNonceMismatch(uint96 cosignerNonce, uint256 intentNonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CosignerNonceMismatch {
        #[allow(missing_docs)]
        pub cosignerNonce: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub intentNonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<96>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U96,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CosignerNonceMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: CosignerNonceMismatch) -> Self {
                (value.cosignerNonce, value.intentNonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CosignerNonceMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    cosignerNonce: tuple.0,
                    intentNonce: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CosignerNonceMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CosignerNonceMismatch(uint96,uint256)";
            const SELECTOR: [u8; 4] = [187u8, 204u8, 248u8, 143u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.cosignerNonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentNonce),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CosignerSwapperMismatch(address,address)` and selector `0x438429d5`.
```solidity
error CosignerSwapperMismatch(address cosignerSwapper, address intentSwapper);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CosignerSwapperMismatch {
        #[allow(missing_docs)]
        pub cosignerSwapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub intentSwapper: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<CosignerSwapperMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: CosignerSwapperMismatch) -> Self {
                (value.cosignerSwapper, value.intentSwapper)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CosignerSwapperMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    cosignerSwapper: tuple.0,
                    intentSwapper: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CosignerSwapperMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CosignerSwapperMismatch(address,address)";
            const SELECTOR: [u8; 4] = [67u8, 132u8, 41u8, 213u8];
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
                        &self.cosignerSwapper,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.intentSwapper,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DuplicateRecipient(address)` and selector `0x3e918579`.
```solidity
error DuplicateRecipient(address recipient);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DuplicateRecipient {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
        impl ::core::convert::From<DuplicateRecipient> for UnderlyingRustTuple<'_> {
            fn from(value: DuplicateRecipient) -> Self {
                (value.recipient,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DuplicateRecipient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { recipient: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DuplicateRecipient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DuplicateRecipient(address)";
            const SELECTOR: [u8; 4] = [62u8, 145u8, 133u8, 121u8];
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
                        &self.recipient,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyAllocations()` and selector `0x949efc96`.
```solidity
error EmptyAllocations();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyAllocations;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<EmptyAllocations> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyAllocations) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyAllocations {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyAllocations {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyAllocations()";
            const SELECTOR: [u8; 4] = [148u8, 158u8, 252u8, 150u8];
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputAboveLimit(uint256,uint256)` and selector `0xf00f44f5`.
```solidity
error InputAboveLimit(uint256 inputAmount, uint256 limitAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAboveLimit {
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub limitAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InputAboveLimit> for UnderlyingRustTuple<'_> {
            fn from(value: InputAboveLimit) -> Self {
                (value.inputAmount, value.limitAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputAboveLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    inputAmount: tuple.0,
                    limitAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputAboveLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputAboveLimit(uint256,uint256)";
            const SELECTOR: [u8; 4] = [240u8, 15u8, 68u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.limitAmount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputAmountMismatch(uint256,uint256)` and selector `0x9994be18`.
```solidity
error InputAmountMismatch(uint256 inputAmount, uint256 execAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAmountMismatch {
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub execAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InputAmountMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputAmountMismatch) -> Self {
                (value.inputAmount, value.execAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputAmountMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    inputAmount: tuple.0,
                    execAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputAmountMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputAmountMismatch(uint256,uint256)";
            const SELECTOR: [u8; 4] = [153u8, 148u8, 190u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.execAmount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InsufficientOutput(uint256,uint256)` and selector `0x2c19b8b8`.
```solidity
error InsufficientOutput(uint256 totalOutput, uint256 limitAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientOutput {
        #[allow(missing_docs)]
        pub totalOutput: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub limitAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InsufficientOutput> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientOutput) -> Self {
                (value.totalOutput, value.limitAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientOutput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    totalOutput: tuple.0,
                    limitAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientOutput {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientOutput(uint256,uint256)";
            const SELECTOR: [u8; 4] = [44u8, 25u8, 184u8, 184u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalOutput),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.limitAmount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `IntentAlreadyCancelled(bytes32)` and selector `0x0b1f8846`.
```solidity
error IntentAlreadyCancelled(bytes32 intentId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IntentAlreadyCancelled {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
        impl ::core::convert::From<IntentAlreadyCancelled> for UnderlyingRustTuple<'_> {
            fn from(value: IntentAlreadyCancelled) -> Self {
                (value.intentId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IntentAlreadyCancelled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { intentId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IntentAlreadyCancelled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IntentAlreadyCancelled(bytes32)";
            const SELECTOR: [u8; 4] = [11u8, 31u8, 136u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `IntentExpired(uint256,uint256)` and selector `0x6f08ee6e`.
```solidity
error IntentExpired(uint256 currentTime, uint256 deadline);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IntentExpired {
        #[allow(missing_docs)]
        pub currentTime: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<IntentExpired> for UnderlyingRustTuple<'_> {
            fn from(value: IntentExpired) -> Self {
                (value.currentTime, value.deadline)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IntentExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentTime: tuple.0,
                    deadline: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IntentExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IntentExpired(uint256,uint256)";
            const SELECTOR: [u8; 4] = [111u8, 8u8, 238u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentTime),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `IntentIsCancelled(bytes32)` and selector `0x2660161b`.
```solidity
error IntentIsCancelled(bytes32 intentId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IntentIsCancelled {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
        impl ::core::convert::From<IntentIsCancelled> for UnderlyingRustTuple<'_> {
            fn from(value: IntentIsCancelled) -> Self {
                (value.intentId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IntentIsCancelled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { intentId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IntentIsCancelled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IntentIsCancelled(bytes32)";
            const SELECTOR: [u8; 4] = [38u8, 96u8, 22u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidCosignerSignature(address,address)` and selector `0xca5612f6`.
```solidity
error InvalidCosignerSignature(address recoveredCosigner, address expectedCosigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCosignerSignature {
        #[allow(missing_docs)]
        pub recoveredCosigner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub expectedCosigner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<InvalidCosignerSignature>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCosignerSignature) -> Self {
                (value.recoveredCosigner, value.expectedCosigner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidCosignerSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    recoveredCosigner: tuple.0,
                    expectedCosigner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCosignerSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCosignerSignature(address,address)";
            const SELECTOR: [u8; 4] = [202u8, 86u8, 18u8, 246u8];
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
                        &self.recoveredCosigner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.expectedCosigner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidSwapperSignature(address,address)` and selector `0x17aa1179`.
```solidity
error InvalidSwapperSignature(address recoveredSigner, address expectedSwapper);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSwapperSignature {
        #[allow(missing_docs)]
        pub recoveredSigner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub expectedSwapper: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<InvalidSwapperSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSwapperSignature) -> Self {
                (value.recoveredSigner, value.expectedSwapper)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSwapperSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    recoveredSigner: tuple.0,
                    expectedSwapper: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSwapperSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSwapperSignature(address,address)";
            const SELECTOR: [u8; 4] = [23u8, 170u8, 17u8, 121u8];
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
                        &self.recoveredSigner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.expectedSwapper,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `PriceBelowMin(uint256,uint256)` and selector `0xba04e36d`.
```solidity
error PriceBelowMin(uint256 executionPrice, uint256 minPrice);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PriceBelowMin {
        #[allow(missing_docs)]
        pub executionPrice: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minPrice: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<PriceBelowMin> for UnderlyingRustTuple<'_> {
            fn from(value: PriceBelowMin) -> Self {
                (value.executionPrice, value.minPrice)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PriceBelowMin {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    executionPrice: tuple.0,
                    minPrice: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PriceBelowMin {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PriceBelowMin(uint256,uint256)";
            const SELECTOR: [u8; 4] = [186u8, 4u8, 227u8, 109u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.executionPrice),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minPrice),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `SwapperMismatch(address,address)` and selector `0xbaa8d2ad`.
```solidity
error SwapperMismatch(address orderSwapper, address intentSwapper);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SwapperMismatch {
        #[allow(missing_docs)]
        pub orderSwapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub intentSwapper: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<SwapperMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: SwapperMismatch) -> Self {
                (value.orderSwapper, value.intentSwapper)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapperMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    orderSwapper: tuple.0,
                    intentSwapper: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SwapperMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SwapperMismatch(address,address)";
            const SELECTOR: [u8; 4] = [186u8, 168u8, 210u8, 173u8];
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
                        &self.orderSwapper,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.intentSwapper,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TooLate(uint256,uint256)` and selector `0x388b0173`.
```solidity
error TooLate(uint256 elapsed, uint256 maxPeriod);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TooLate {
        #[allow(missing_docs)]
        pub elapsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxPeriod: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<TooLate> for UnderlyingRustTuple<'_> {
            fn from(value: TooLate) -> Self {
                (value.elapsed, value.maxPeriod)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TooLate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    elapsed: tuple.0,
                    maxPeriod: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TooLate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TooLate(uint256,uint256)";
            const SELECTOR: [u8; 4] = [56u8, 139u8, 1u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.elapsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxPeriod),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TooSoon(uint256,uint256)` and selector `0xb1b9b6ee`.
```solidity
error TooSoon(uint256 elapsed, uint256 minPeriod);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TooSoon {
        #[allow(missing_docs)]
        pub elapsed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minPeriod: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<TooSoon> for UnderlyingRustTuple<'_> {
            fn from(value: TooSoon) -> Self {
                (value.elapsed, value.minPeriod)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TooSoon {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    elapsed: tuple.0,
                    minPeriod: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TooSoon {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TooSoon(uint256,uint256)";
            const SELECTOR: [u8; 4] = [177u8, 185u8, 182u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.elapsed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minPeriod),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongChain(uint256,uint256)` and selector `0x24497bc3`.
```solidity
error WrongChain(uint256 providedChainId, uint256 currentChainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongChain {
        #[allow(missing_docs)]
        pub providedChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub currentChainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<WrongChain> for UnderlyingRustTuple<'_> {
            fn from(value: WrongChain) -> Self {
                (value.providedChainId, value.currentChainId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    providedChainId: tuple.0,
                    currentChainId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongChain(uint256,uint256)";
            const SELECTOR: [u8; 4] = [36u8, 73u8, 123u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.providedChainId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentChainId),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongChunkNonce(uint96,uint96)` and selector `0x1ca8c062`.
```solidity
error WrongChunkNonce(uint96 providedNonce, uint96 expectedNonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongChunkNonce {
        #[allow(missing_docs)]
        pub providedNonce: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub expectedNonce: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<96>,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U96,
            alloy::sol_types::private::primitives::aliases::U96,
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
        impl ::core::convert::From<WrongChunkNonce> for UnderlyingRustTuple<'_> {
            fn from(value: WrongChunkNonce) -> Self {
                (value.providedNonce, value.expectedNonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongChunkNonce {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    providedNonce: tuple.0,
                    expectedNonce: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongChunkNonce {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongChunkNonce(uint96,uint96)";
            const SELECTOR: [u8; 4] = [28u8, 168u8, 192u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.providedNonce),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedNonce),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongHook(address,address)` and selector `0xf94e883c`.
```solidity
error WrongHook(address providedHook, address expectedHook);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongHook {
        #[allow(missing_docs)]
        pub providedHook: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub expectedHook: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<WrongHook> for UnderlyingRustTuple<'_> {
            fn from(value: WrongHook) -> Self {
                (value.providedHook, value.expectedHook)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongHook {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    providedHook: tuple.0,
                    expectedHook: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongHook {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongHook(address,address)";
            const SELECTOR: [u8; 4] = [249u8, 78u8, 136u8, 60u8];
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
                        &self.providedHook,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.expectedHook,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongInputToken(address,address)` and selector `0x2c632126`.
```solidity
error WrongInputToken(address orderInputToken, address intentInputToken);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongInputToken {
        #[allow(missing_docs)]
        pub orderInputToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub intentInputToken: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<WrongInputToken> for UnderlyingRustTuple<'_> {
            fn from(value: WrongInputToken) -> Self {
                (value.orderInputToken, value.intentInputToken)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongInputToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    orderInputToken: tuple.0,
                    intentInputToken: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongInputToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongInputToken(address,address)";
            const SELECTOR: [u8; 4] = [44u8, 99u8, 33u8, 38u8];
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
                        &self.orderInputToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.intentInputToken,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongOutputToken(address,address)` and selector `0xd064a3f7`.
```solidity
error WrongOutputToken(address outputToken, address expectedToken);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongOutputToken {
        #[allow(missing_docs)]
        pub outputToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub expectedToken: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<WrongOutputToken> for UnderlyingRustTuple<'_> {
            fn from(value: WrongOutputToken) -> Self {
                (value.outputToken, value.expectedToken)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongOutputToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    outputToken: tuple.0,
                    expectedToken: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongOutputToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongOutputToken(address,address)";
            const SELECTOR: [u8; 4] = [208u8, 100u8, 163u8, 247u8];
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
                        &self.outputToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.expectedToken,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WrongTotalOutput(uint256,uint256)` and selector `0x7ebbdeab`.
```solidity
error WrongTotalOutput(uint256 totalOutput, uint256 execAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongTotalOutput {
        #[allow(missing_docs)]
        pub totalOutput: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub execAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<WrongTotalOutput> for UnderlyingRustTuple<'_> {
            fn from(value: WrongTotalOutput) -> Self {
                (value.totalOutput, value.execAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongTotalOutput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    totalOutput: tuple.0,
                    execAmount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongTotalOutput {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongTotalOutput(uint256,uint256)";
            const SELECTOR: [u8; 4] = [126u8, 187u8, 222u8, 171u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalOutput),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.execAmount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroAllocation()` and selector `0xba0d87b5`.
```solidity
error ZeroAllocation();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAllocation;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ZeroAllocation> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAllocation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAllocation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAllocation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAllocation()";
            const SELECTOR: [u8; 4] = [186u8, 13u8, 135u8, 181u8];
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroInput()` and selector `0xaf458c07`.
```solidity
error ZeroInput();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroInput;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
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
        impl ::core::convert::From<ZeroInput> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroInput) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroInput {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroInput()";
            const SELECTOR: [u8; 4] = [175u8, 69u8, 140u8, 7u8];
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
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChunkExecuted(bytes32,uint256,uint256,uint256,uint256)` and selector `0x8595ab03f1b5496cccc75833b235b1b95dee73096282d3ac851b628df24bd0f2`.
```solidity
event ChunkExecuted(bytes32 indexed intentId, uint256 execAmount, uint256 limitAmount, uint256 totalInputExecuted, uint256 totalOutput);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChunkExecuted {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub execAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub limitAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalInputExecuted: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalOutput: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ChunkExecuted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ChunkExecuted(bytes32,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                133u8, 149u8, 171u8, 3u8, 241u8, 181u8, 73u8, 108u8, 204u8, 199u8, 88u8,
                51u8, 178u8, 53u8, 177u8, 185u8, 93u8, 238u8, 115u8, 9u8, 98u8, 130u8,
                211u8, 172u8, 133u8, 27u8, 98u8, 141u8, 242u8, 75u8, 208u8, 242u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    intentId: topics.1,
                    execAmount: data.0,
                    limitAmount: data.1,
                    totalInputExecuted: data.2,
                    totalOutput: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.execAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.limitAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalInputExecuted),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalOutput),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.intentId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.intentId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChunkExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChunkExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChunkExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `IntentCancelled(bytes32,address)` and selector `0xac5a4b90e421002a2fdb9f132b9b32c24fa4ae16ec480516c85932de208d2a33`.
```solidity
event IntentCancelled(bytes32 indexed intentId, address indexed swapper);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct IntentCancelled {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub swapper: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for IntentCancelled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "IntentCancelled(bytes32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                172u8, 90u8, 75u8, 144u8, 228u8, 33u8, 0u8, 42u8, 47u8, 219u8, 159u8,
                19u8, 43u8, 155u8, 50u8, 194u8, 79u8, 164u8, 174u8, 22u8, 236u8, 72u8,
                5u8, 22u8, 200u8, 89u8, 50u8, 222u8, 32u8, 141u8, 42u8, 51u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    intentId: topics.1,
                    swapper: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.intentId.clone(),
                    self.swapper.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.intentId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.swapper,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for IntentCancelled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&IntentCancelled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &IntentCancelled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address p, address r);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub p: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub r: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.p, value.r)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { p: tuple.0, r: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.p,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.r,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`.
```solidity
function DOMAIN_SEPARATOR() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`DOMAIN_SEPARATOR()`](DOMAIN_SEPARATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_SEPARATORReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<DOMAIN_SEPARATORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<DOMAIN_SEPARATORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_SEPARATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DOMAIN_SEPARATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DOMAIN_SEPARATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DOMAIN_SEPARATOR()";
            const SELECTOR: [u8; 4] = [54u8, 68u8, 229u8, 21u8];
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
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: DOMAIN_SEPARATORReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: DOMAIN_SEPARATORReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__setExecutedMeta(bytes32,uint120)` and selector `0x0209e710`.
```solidity
function __setExecutedMeta(bytes32 intentId, uint120 lastExecutionTime) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __setExecutedMetaCall {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub lastExecutionTime: alloy::sol_types::private::primitives::aliases::U120,
    }
    ///Container type for the return parameters of the [`__setExecutedMeta(bytes32,uint120)`](__setExecutedMetaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __setExecutedMetaReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<120>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U120,
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
            impl ::core::convert::From<__setExecutedMetaCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: __setExecutedMetaCall) -> Self {
                    (value.intentId, value.lastExecutionTime)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __setExecutedMetaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        intentId: tuple.0,
                        lastExecutionTime: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<__setExecutedMetaReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: __setExecutedMetaReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __setExecutedMetaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl __setExecutedMetaReturn {
            fn _tokenize(
                &self,
            ) -> <__setExecutedMetaCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __setExecutedMetaCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<120>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __setExecutedMetaReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__setExecutedMeta(bytes32,uint120)";
            const SELECTOR: [u8; 4] = [2u8, 9u8, 231u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                    <alloy::sol_types::sol_data::Uint<
                        120,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastExecutionTime),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                __setExecutedMetaReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__setPacked(bytes32,uint128,bool)` and selector `0xb202a7f3`.
```solidity
function __setPacked(bytes32 intentId, uint128 executedChunks, bool cancelled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __setPackedCall {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub executedChunks: u128,
        #[allow(missing_docs)]
        pub cancelled: bool,
    }
    ///Container type for the return parameters of the [`__setPacked(bytes32,uint128,bool)`](__setPackedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __setPackedReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u128,
                bool,
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
            impl ::core::convert::From<__setPackedCall> for UnderlyingRustTuple<'_> {
                fn from(value: __setPackedCall) -> Self {
                    (value.intentId, value.executedChunks, value.cancelled)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __setPackedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        intentId: tuple.0,
                        executedChunks: tuple.1,
                        cancelled: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<__setPackedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __setPackedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __setPackedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl __setPackedReturn {
            fn _tokenize(
                &self,
            ) -> <__setPackedCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __setPackedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __setPackedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__setPacked(bytes32,uint128,bool)";
            const SELECTOR: [u8; 4] = [178u8, 2u8, 167u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.executedChunks),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.cancelled,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                __setPackedReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `__setTotals(bytes32,uint256,uint256)` and selector `0xfe7823ac`.
```solidity
function __setTotals(bytes32 intentId, uint256 totalInputExecuted, uint256 totalOutput) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __setTotalsCall {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub totalInputExecuted: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalOutput: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`__setTotals(bytes32,uint256,uint256)`](__setTotalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __setTotalsReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<__setTotalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: __setTotalsCall) -> Self {
                    (value.intentId, value.totalInputExecuted, value.totalOutput)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __setTotalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        intentId: tuple.0,
                        totalInputExecuted: tuple.1,
                        totalOutput: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<__setTotalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __setTotalsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __setTotalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl __setTotalsReturn {
            fn _tokenize(
                &self,
            ) -> <__setTotalsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __setTotalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __setTotalsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__setTotals(bytes32,uint256,uint256)";
            const SELECTOR: [u8; 4] = [254u8, 120u8, 35u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalInputExecuted),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalOutput),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                __setTotalsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelIntent(uint256)` and selector `0xa0a31aac`.
```solidity
function cancelIntent(uint256 nonce) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelIntentCall {
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`cancelIntent(uint256)`](cancelIntentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelIntentReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<cancelIntentCall> for UnderlyingRustTuple<'_> {
                fn from(value: cancelIntentCall) -> Self {
                    (value.nonce,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelIntentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nonce: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<cancelIntentReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cancelIntentReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelIntentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelIntentReturn {
            fn _tokenize(
                &self,
            ) -> <cancelIntentCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelIntentCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelIntentReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelIntent(uint256)";
            const SELECTOR: [u8; 4] = [160u8, 163u8, 26u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelIntentReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `cancelIntents(uint256[])` and selector `0x5e29fa37`.
```solidity
function cancelIntents(uint256[] memory nonces) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelIntentsCall {
        #[allow(missing_docs)]
        pub nonces: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`cancelIntents(uint256[])`](cancelIntentsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelIntentsReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<cancelIntentsCall> for UnderlyingRustTuple<'_> {
                fn from(value: cancelIntentsCall) -> Self {
                    (value.nonces,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelIntentsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nonces: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<cancelIntentsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cancelIntentsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelIntentsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cancelIntentsReturn {
            fn _tokenize(
                &self,
            ) -> <cancelIntentsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelIntentsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelIntentsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelIntents(uint256[])";
            const SELECTOR: [u8; 4] = [94u8, 41u8, 250u8, 55u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonces),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                cancelIntentsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `computeIntentId(address,uint256)` and selector `0xb1c13908`.
```solidity
function computeIntentId(address swapper, uint256 nonce) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeIntentIdCall {
        #[allow(missing_docs)]
        pub swapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`computeIntentId(address,uint256)`](computeIntentIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct computeIntentIdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<computeIntentIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: computeIntentIdCall) -> Self {
                    (value.swapper, value.nonce)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for computeIntentIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        swapper: tuple.0,
                        nonce: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<computeIntentIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: computeIntentIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for computeIntentIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for computeIntentIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "computeIntentId(address,uint256)";
            const SELECTOR: [u8; 4] = [177u8, 193u8, 57u8, 8u8];
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
                        &self.swapper,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: computeIntentIdReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: computeIntentIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `createTestCosignerData(address,uint96,uint160,uint160,uint96)` and selector `0x308ea6c9`.
```solidity
function createTestCosignerData(address swapper, uint96 nonce, uint160 execAmount, uint160 limitAmount, uint96 orderNonce) external pure returns (DCAOrderCosignerData memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTestCosignerDataCall {
        #[allow(missing_docs)]
        pub swapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub execAmount: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub limitAmount: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub orderNonce: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createTestCosignerData(address,uint96,uint160,uint160,uint96)`](createTestCosignerDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTestCosignerDataReturn {
        #[allow(missing_docs)]
        pub _0: <DCAOrderCosignerData as alloy::sol_types::SolType>::RustType,
    }
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::primitives::aliases::U160,
                alloy::sol_types::private::primitives::aliases::U160,
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<createTestCosignerDataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTestCosignerDataCall) -> Self {
                    (
                        value.swapper,
                        value.nonce,
                        value.execAmount,
                        value.limitAmount,
                        value.orderNonce,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTestCosignerDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        swapper: tuple.0,
                        nonce: tuple.1,
                        execAmount: tuple.2,
                        limitAmount: tuple.3,
                        orderNonce: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (DCAOrderCosignerData,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DCAOrderCosignerData as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createTestCosignerDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTestCosignerDataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTestCosignerDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createTestCosignerDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <DCAOrderCosignerData as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (DCAOrderCosignerData,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createTestCosignerData(address,uint96,uint160,uint160,uint96)";
            const SELECTOR: [u8; 4] = [48u8, 142u8, 166u8, 201u8];
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
                        &self.swapper,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.execAmount),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.limitAmount),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.orderNonce),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<DCAOrderCosignerData as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: createTestCosignerDataReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: createTestCosignerDataReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `createTestIntent(address,uint96,bool,uint256,uint256)` and selector `0x83bc6ab6`.
```solidity
function createTestIntent(address swapper, uint96 nonce, bool isExactIn, uint256 minChunk, uint256 maxChunk) external view returns (DCAIntent memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTestIntentCall {
        #[allow(missing_docs)]
        pub swapper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U96,
        #[allow(missing_docs)]
        pub isExactIn: bool,
        #[allow(missing_docs)]
        pub minChunk: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxChunk: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`createTestIntent(address,uint96,bool,uint256,uint256)`](createTestIntentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createTestIntentReturn {
        #[allow(missing_docs)]
        pub _0: <DCAIntent as alloy::sol_types::SolType>::RustType,
    }
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U96,
                bool,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<createTestIntentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTestIntentCall) -> Self {
                    (
                        value.swapper,
                        value.nonce,
                        value.isExactIn,
                        value.minChunk,
                        value.maxChunk,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTestIntentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        swapper: tuple.0,
                        nonce: tuple.1,
                        isExactIn: tuple.2,
                        minChunk: tuple.3,
                        maxChunk: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (DCAIntent,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DCAIntent as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createTestIntentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createTestIntentReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createTestIntentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createTestIntentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <DCAIntent as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (DCAIntent,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createTestIntent(address,uint96,bool,uint256,uint256)";
            const SELECTOR: [u8; 4] = [131u8, 188u8, 106u8, 182u8];
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
                        &self.swapper,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isExactIn,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minChunk),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxChunk),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<DCAIntent as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: createTestIntentReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: createTestIntentReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getExecutionState(bytes32)` and selector `0x49d8033e`.
```solidity
function getExecutionState(bytes32 intentId) external view returns (DCAExecutionState memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionStateCall {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getExecutionState(bytes32)`](getExecutionStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionStateReturn {
        #[allow(missing_docs)]
        pub _0: <DCAExecutionState as alloy::sol_types::SolType>::RustType,
    }
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getExecutionStateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionStateCall) -> Self {
                    (value.intentId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { intentId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (DCAExecutionState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DCAExecutionState as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getExecutionStateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionStateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getExecutionStateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <DCAExecutionState as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (DCAExecutionState,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getExecutionState(bytes32)";
            const SELECTOR: [u8; 4] = [73u8, 216u8, 3u8, 62u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<DCAExecutionState as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getExecutionStateReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getExecutionStateReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getIntentStatistics(bytes32)` and selector `0x16b853a2`.
```solidity
function getIntentStatistics(bytes32 intentId) external view returns (uint256 totalChunks, uint256 totalInput, uint256 totalOutput, uint256 lastExecutionTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getIntentStatisticsCall {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getIntentStatistics(bytes32)`](getIntentStatisticsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getIntentStatisticsReturn {
        #[allow(missing_docs)]
        pub totalChunks: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalInput: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalOutput: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub lastExecutionTime: alloy::sol_types::private::primitives::aliases::U256,
    }
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getIntentStatisticsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getIntentStatisticsCall) -> Self {
                    (value.intentId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getIntentStatisticsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { intentId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getIntentStatisticsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getIntentStatisticsReturn) -> Self {
                    (
                        value.totalChunks,
                        value.totalInput,
                        value.totalOutput,
                        value.lastExecutionTime,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getIntentStatisticsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        totalChunks: tuple.0,
                        totalInput: tuple.1,
                        totalOutput: tuple.2,
                        lastExecutionTime: tuple.3,
                    }
                }
            }
        }
        impl getIntentStatisticsReturn {
            fn _tokenize(
                &self,
            ) -> <getIntentStatisticsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalChunks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalInput),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalOutput),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastExecutionTime),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getIntentStatisticsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getIntentStatisticsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getIntentStatistics(bytes32)";
            const SELECTOR: [u8; 4] = [22u8, 184u8, 83u8, 162u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getIntentStatisticsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getNextNonce(bytes32)` and selector `0x1ce24d02`.
```solidity
function getNextNonce(bytes32 intentId) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNextNonceCall {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getNextNonce(bytes32)`](getNextNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNextNonceReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
    }
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getNextNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: getNextNonceCall) -> Self {
                    (value.intentId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNextNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { intentId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getNextNonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getNextNonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNextNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNextNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U96;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getNextNonce(bytes32)";
            const SELECTOR: [u8; 4] = [28u8, 226u8, 77u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getNextNonceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: getNextNonceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isIntentActive(bytes32,uint256,uint256)` and selector `0x30790081`.
```solidity
function isIntentActive(bytes32 intentId, uint256 maxPeriod, uint256 deadline) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isIntentActiveCall {
        #[allow(missing_docs)]
        pub intentId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub maxPeriod: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deadline: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isIntentActive(bytes32,uint256,uint256)`](isIntentActiveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isIntentActiveReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<isIntentActiveCall> for UnderlyingRustTuple<'_> {
                fn from(value: isIntentActiveCall) -> Self {
                    (value.intentId, value.maxPeriod, value.deadline)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isIntentActiveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        intentId: tuple.0,
                        maxPeriod: tuple.1,
                        deadline: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
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
            impl ::core::convert::From<isIntentActiveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isIntentActiveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isIntentActiveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isIntentActiveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isIntentActive(bytes32,uint256,uint256)";
            const SELECTOR: [u8; 4] = [48u8, 121u8, 0u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.intentId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxPeriod),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.deadline),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isIntentActiveReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: isIntentActiveReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `permit2()` and selector `0x12261ee7`.
```solidity
function permit2() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permit2Call;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`permit2()`](permit2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permit2Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<permit2Call> for UnderlyingRustTuple<'_> {
                fn from(value: permit2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permit2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<permit2Return> for UnderlyingRustTuple<'_> {
                fn from(value: permit2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permit2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permit2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permit2()";
            const SELECTOR: [u8; 4] = [18u8, 38u8, 30u8, 231u8];
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
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: permit2Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: permit2Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `preExecutionHook(address,((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string))` and selector `0xd8b61ded`.
```solidity
function preExecutionHook(address filler, ResolvedOrder memory resolvedOrder) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preExecutionHookCall {
        #[allow(missing_docs)]
        pub filler: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub resolvedOrder: <ResolvedOrder as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`preExecutionHook(address,((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string))`](preExecutionHookCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preExecutionHookReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                ResolvedOrder,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ResolvedOrder as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<preExecutionHookCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: preExecutionHookCall) -> Self {
                    (value.filler, value.resolvedOrder)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for preExecutionHookCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        filler: tuple.0,
                        resolvedOrder: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<preExecutionHookReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: preExecutionHookReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for preExecutionHookReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl preExecutionHookReturn {
            fn _tokenize(
                &self,
            ) -> <preExecutionHookCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for preExecutionHookCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, ResolvedOrder);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = preExecutionHookReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "preExecutionHook(address,((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string))";
            const SELECTOR: [u8; 4] = [216u8, 182u8, 29u8, 237u8];
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
                        &self.filler,
                    ),
                    <ResolvedOrder as alloy_sol_types::SolType>::tokenize(
                        &self.resolvedOrder,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                preExecutionHookReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `reactor()` and selector `0xab572650`.
```solidity
function reactor() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reactorCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`reactor()`](reactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reactorReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
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
            #[allow(dead_code)]
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
            impl ::core::convert::From<reactorCall> for UnderlyingRustTuple<'_> {
                fn from(value: reactorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reactorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<reactorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: reactorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reactorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reactor()";
            const SELECTOR: [u8; 4] = [171u8, 87u8, 38u8, 80u8];
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
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: reactorReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: reactorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Function with signature `transferInputTokens(((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string),address,(bool,((address,uint160,uint48,uint48),address,uint256),bytes))` and selector `0x8345eb56`.
```solidity
function transferInputTokens(ResolvedOrder memory order, address to, PermitData memory permitData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferInputTokensCall {
        #[allow(missing_docs)]
        pub order: <ResolvedOrder as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub permitData: <PermitData as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`transferInputTokens(((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string),address,(bool,((address,uint160,uint48,uint48),address,uint256),bytes))`](transferInputTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferInputTokensReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                ResolvedOrder,
                alloy::sol_types::sol_data::Address,
                PermitData,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ResolvedOrder as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                <PermitData as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<transferInputTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferInputTokensCall) -> Self {
                    (value.order, value.to, value.permitData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferInputTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        order: tuple.0,
                        to: tuple.1,
                        permitData: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<transferInputTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferInputTokensReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferInputTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferInputTokensReturn {
            fn _tokenize(
                &self,
            ) -> <transferInputTokensCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferInputTokensCall {
            type Parameters<'a> = (
                ResolvedOrder,
                alloy::sol_types::sol_data::Address,
                PermitData,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferInputTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferInputTokens(((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string),address,(bool,((address,uint160,uint48,uint48),address,uint256),bytes))";
            const SELECTOR: [u8; 4] = [131u8, 69u8, 235u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ResolvedOrder as alloy_sol_types::SolType>::tokenize(&self.order),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <PermitData as alloy_sol_types::SolType>::tokenize(&self.permitData),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferInputTokensReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validateAllocationStructure((address,uint16)[])` and selector `0x8184e353`.
```solidity
function validateAllocationStructure(OutputAllocation[] memory outputAllocations) external pure;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateAllocationStructureCall {
        #[allow(missing_docs)]
        pub outputAllocations: alloy::sol_types::private::Vec<
            <OutputAllocation as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`validateAllocationStructure((address,uint16)[])`](validateAllocationStructureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateAllocationStructureReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OutputAllocation>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OutputAllocation as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<validateAllocationStructureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateAllocationStructureCall) -> Self {
                    (value.outputAllocations,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateAllocationStructureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { outputAllocations: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<validateAllocationStructureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateAllocationStructureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateAllocationStructureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl validateAllocationStructureReturn {
            fn _tokenize(
                &self,
            ) -> <validateAllocationStructureCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateAllocationStructureCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Array<OutputAllocation>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validateAllocationStructureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateAllocationStructure((address,uint16)[])";
            const SELECTOR: [u8; 4] = [129u8, 132u8, 227u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        OutputAllocation,
                    > as alloy_sol_types::SolType>::tokenize(&self.outputAllocations),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                validateAllocationStructureReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validateChunkSize((address,uint256,uint256,address,bool,address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,(address,uint16)[],(uint256,uint256,uint256,bytes32,((string,string,string[],string[],uint256),address,string)[])),(address,uint96,uint160,uint96,uint160),uint256)` and selector `0x29c8ad4e`.
```solidity
function validateChunkSize(DCAIntent memory intent, DCAOrderCosignerData memory cosignerData, uint256 inputAmount) external pure;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateChunkSizeCall {
        #[allow(missing_docs)]
        pub intent: <DCAIntent as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cosignerData: <DCAOrderCosignerData as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub inputAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`validateChunkSize((address,uint256,uint256,address,bool,address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,(address,uint16)[],(uint256,uint256,uint256,bytes32,((string,string,string[],string[],uint256),address,string)[])),(address,uint96,uint160,uint96,uint160),uint256)`](validateChunkSizeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateChunkSizeReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                DCAIntent,
                DCAOrderCosignerData,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DCAIntent as alloy::sol_types::SolType>::RustType,
                <DCAOrderCosignerData as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<validateChunkSizeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateChunkSizeCall) -> Self {
                    (value.intent, value.cosignerData, value.inputAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateChunkSizeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        intent: tuple.0,
                        cosignerData: tuple.1,
                        inputAmount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<validateChunkSizeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateChunkSizeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateChunkSizeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl validateChunkSizeReturn {
            fn _tokenize(
                &self,
            ) -> <validateChunkSizeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateChunkSizeCall {
            type Parameters<'a> = (
                DCAIntent,
                DCAOrderCosignerData,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validateChunkSizeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateChunkSize((address,uint256,uint256,address,bool,address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,(address,uint16)[],(uint256,uint256,uint256,bytes32,((string,string,string[],string[],uint256),address,string)[])),(address,uint96,uint160,uint96,uint160),uint256)";
            const SELECTOR: [u8; 4] = [41u8, 200u8, 173u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <DCAIntent as alloy_sol_types::SolType>::tokenize(&self.intent),
                    <DCAOrderCosignerData as alloy_sol_types::SolType>::tokenize(
                        &self.cosignerData,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                validateChunkSizeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validatePriceFloor(bool,uint160,uint160,uint256)` and selector `0x80cb1b50`.
```solidity
function validatePriceFloor(bool isExactIn, uint160 execAmount, uint160 limitAmount, uint256 minPrice) external pure;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePriceFloorCall {
        #[allow(missing_docs)]
        pub isExactIn: bool,
        #[allow(missing_docs)]
        pub execAmount: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub limitAmount: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub minPrice: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`validatePriceFloor(bool,uint160,uint160,uint256)`](validatePriceFloorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePriceFloorReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                alloy::sol_types::private::primitives::aliases::U160,
                alloy::sol_types::private::primitives::aliases::U160,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<validatePriceFloorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatePriceFloorCall) -> Self {
                    (
                        value.isExactIn,
                        value.execAmount,
                        value.limitAmount,
                        value.minPrice,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatePriceFloorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        isExactIn: tuple.0,
                        execAmount: tuple.1,
                        limitAmount: tuple.2,
                        minPrice: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<validatePriceFloorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatePriceFloorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatePriceFloorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl validatePriceFloorReturn {
            fn _tokenize(
                &self,
            ) -> <validatePriceFloorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatePriceFloorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatePriceFloorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatePriceFloor(bool,uint160,uint160,uint256)";
            const SELECTOR: [u8; 4] = [128u8, 203u8, 27u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isExactIn,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.execAmount),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.limitAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minPrice),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                validatePriceFloorReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validateStaticFields((address,uint256,uint256,address,bool,address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,(address,uint16)[],(uint256,uint256,uint256,bytes32,((string,string,string[],string[],uint256),address,string)[])),((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string))` and selector `0x2fd0109b`.
```solidity
function validateStaticFields(DCAIntent memory intent, ResolvedOrder memory resolvedOrder) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateStaticFieldsCall {
        #[allow(missing_docs)]
        pub intent: <DCAIntent as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub resolvedOrder: <ResolvedOrder as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`validateStaticFields((address,uint256,uint256,address,bool,address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,(address,uint16)[],(uint256,uint256,uint256,bytes32,((string,string,string[],string[],uint256),address,string)[])),((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string))`](validateStaticFieldsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validateStaticFieldsReturn {}
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
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (DCAIntent, ResolvedOrder);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DCAIntent as alloy::sol_types::SolType>::RustType,
                <ResolvedOrder as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validateStaticFieldsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateStaticFieldsCall) -> Self {
                    (value.intent, value.resolvedOrder)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateStaticFieldsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        intent: tuple.0,
                        resolvedOrder: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<validateStaticFieldsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validateStaticFieldsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validateStaticFieldsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl validateStaticFieldsReturn {
            fn _tokenize(
                &self,
            ) -> <validateStaticFieldsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validateStaticFieldsCall {
            type Parameters<'a> = (DCAIntent, ResolvedOrder);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validateStaticFieldsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validateStaticFields((address,uint256,uint256,address,bool,address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,(address,uint16)[],(uint256,uint256,uint256,bytes32,((string,string,string[],string[],uint256),address,string)[])),((address,address,uint256,uint256,address,bytes,address,bytes,address),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32,address,string))";
            const SELECTOR: [u8; 4] = [47u8, 208u8, 16u8, 155u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <DCAIntent as alloy_sol_types::SolType>::tokenize(&self.intent),
                    <ResolvedOrder as alloy_sol_types::SolType>::tokenize(
                        &self.resolvedOrder,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                validateStaticFieldsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`DCAHookHarness`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum DCAHookHarnessCalls {
        #[allow(missing_docs)]
        DOMAIN_SEPARATOR(DOMAIN_SEPARATORCall),
        #[allow(missing_docs)]
        __setExecutedMeta(__setExecutedMetaCall),
        #[allow(missing_docs)]
        __setPacked(__setPackedCall),
        #[allow(missing_docs)]
        __setTotals(__setTotalsCall),
        #[allow(missing_docs)]
        cancelIntent(cancelIntentCall),
        #[allow(missing_docs)]
        cancelIntents(cancelIntentsCall),
        #[allow(missing_docs)]
        computeIntentId(computeIntentIdCall),
        #[allow(missing_docs)]
        createTestCosignerData(createTestCosignerDataCall),
        #[allow(missing_docs)]
        createTestIntent(createTestIntentCall),
        #[allow(missing_docs)]
        getExecutionState(getExecutionStateCall),
        #[allow(missing_docs)]
        getIntentStatistics(getIntentStatisticsCall),
        #[allow(missing_docs)]
        getNextNonce(getNextNonceCall),
        #[allow(missing_docs)]
        isIntentActive(isIntentActiveCall),
        #[allow(missing_docs)]
        permit2(permit2Call),
        #[allow(missing_docs)]
        preExecutionHook(preExecutionHookCall),
        #[allow(missing_docs)]
        reactor(reactorCall),
        #[allow(missing_docs)]
        transferInputTokens(transferInputTokensCall),
        #[allow(missing_docs)]
        validateAllocationStructure(validateAllocationStructureCall),
        #[allow(missing_docs)]
        validateChunkSize(validateChunkSizeCall),
        #[allow(missing_docs)]
        validatePriceFloor(validatePriceFloorCall),
        #[allow(missing_docs)]
        validateStaticFields(validateStaticFieldsCall),
    }
    impl DCAHookHarnessCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [2u8, 9u8, 231u8, 16u8],
            [18u8, 38u8, 30u8, 231u8],
            [22u8, 184u8, 83u8, 162u8],
            [28u8, 226u8, 77u8, 2u8],
            [41u8, 200u8, 173u8, 78u8],
            [47u8, 208u8, 16u8, 155u8],
            [48u8, 121u8, 0u8, 129u8],
            [48u8, 142u8, 166u8, 201u8],
            [54u8, 68u8, 229u8, 21u8],
            [73u8, 216u8, 3u8, 62u8],
            [94u8, 41u8, 250u8, 55u8],
            [128u8, 203u8, 27u8, 80u8],
            [129u8, 132u8, 227u8, 83u8],
            [131u8, 69u8, 235u8, 86u8],
            [131u8, 188u8, 106u8, 182u8],
            [160u8, 163u8, 26u8, 172u8],
            [171u8, 87u8, 38u8, 80u8],
            [177u8, 193u8, 57u8, 8u8],
            [178u8, 2u8, 167u8, 243u8],
            [216u8, 182u8, 29u8, 237u8],
            [254u8, 120u8, 35u8, 172u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(__setExecutedMeta),
            ::core::stringify!(permit2),
            ::core::stringify!(getIntentStatistics),
            ::core::stringify!(getNextNonce),
            ::core::stringify!(validateChunkSize),
            ::core::stringify!(validateStaticFields),
            ::core::stringify!(isIntentActive),
            ::core::stringify!(createTestCosignerData),
            ::core::stringify!(DOMAIN_SEPARATOR),
            ::core::stringify!(getExecutionState),
            ::core::stringify!(cancelIntents),
            ::core::stringify!(validatePriceFloor),
            ::core::stringify!(validateAllocationStructure),
            ::core::stringify!(transferInputTokens),
            ::core::stringify!(createTestIntent),
            ::core::stringify!(cancelIntent),
            ::core::stringify!(reactor),
            ::core::stringify!(computeIntentId),
            ::core::stringify!(__setPacked),
            ::core::stringify!(preExecutionHook),
            ::core::stringify!(__setTotals),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <__setExecutedMetaCall as alloy_sol_types::SolCall>::SIGNATURE,
            <permit2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <getIntentStatisticsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getNextNonceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validateChunkSizeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validateStaticFieldsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isIntentActiveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createTestCosignerDataCall as alloy_sol_types::SolCall>::SIGNATURE,
            <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getExecutionStateCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelIntentsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validatePriceFloorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <validateAllocationStructureCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferInputTokensCall as alloy_sol_types::SolCall>::SIGNATURE,
            <createTestIntentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cancelIntentCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reactorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <computeIntentIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <__setPackedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <preExecutionHookCall as alloy_sol_types::SolCall>::SIGNATURE,
            <__setTotalsCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DCAHookHarnessCalls {
        const NAME: &'static str = "DCAHookHarnessCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 21usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DOMAIN_SEPARATOR(_) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__setExecutedMeta(_) => {
                    <__setExecutedMetaCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__setPacked(_) => {
                    <__setPackedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__setTotals(_) => {
                    <__setTotalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelIntent(_) => {
                    <cancelIntentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelIntents(_) => {
                    <cancelIntentsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::computeIntentId(_) => {
                    <computeIntentIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createTestCosignerData(_) => {
                    <createTestCosignerDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createTestIntent(_) => {
                    <createTestIntentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getExecutionState(_) => {
                    <getExecutionStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getIntentStatistics(_) => {
                    <getIntentStatisticsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getNextNonce(_) => {
                    <getNextNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isIntentActive(_) => {
                    <isIntentActiveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::permit2(_) => <permit2Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::preExecutionHook(_) => {
                    <preExecutionHookCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reactor(_) => <reactorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferInputTokens(_) => {
                    <transferInputTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateAllocationStructure(_) => {
                    <validateAllocationStructureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateChunkSize(_) => {
                    <validateChunkSizeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatePriceFloor(_) => {
                    <validatePriceFloorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validateStaticFields(_) => {
                    <validateStaticFieldsCall as alloy_sol_types::SolCall>::SELECTOR
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DCAHookHarnessCalls>] = &[
                {
                    fn __setExecutedMeta(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <__setExecutedMetaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::__setExecutedMeta)
                    }
                    __setExecutedMeta
                },
                {
                    fn permit2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <permit2Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DCAHookHarnessCalls::permit2)
                    }
                    permit2
                },
                {
                    fn getIntentStatistics(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <getIntentStatisticsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::getIntentStatistics)
                    }
                    getIntentStatistics
                },
                {
                    fn getNextNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <getNextNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::getNextNonce)
                    }
                    getNextNonce
                },
                {
                    fn validateChunkSize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validateChunkSizeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validateChunkSize)
                    }
                    validateChunkSize
                },
                {
                    fn validateStaticFields(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validateStaticFieldsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validateStaticFields)
                    }
                    validateStaticFields
                },
                {
                    fn isIntentActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <isIntentActiveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::isIntentActive)
                    }
                    isIntentActive
                },
                {
                    fn createTestCosignerData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <createTestCosignerDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::createTestCosignerData)
                    }
                    createTestCosignerData
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn getExecutionState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <getExecutionStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::getExecutionState)
                    }
                    getExecutionState
                },
                {
                    fn cancelIntents(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <cancelIntentsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::cancelIntents)
                    }
                    cancelIntents
                },
                {
                    fn validatePriceFloor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validatePriceFloorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validatePriceFloor)
                    }
                    validatePriceFloor
                },
                {
                    fn validateAllocationStructure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validateAllocationStructureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validateAllocationStructure)
                    }
                    validateAllocationStructure
                },
                {
                    fn transferInputTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <transferInputTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::transferInputTokens)
                    }
                    transferInputTokens
                },
                {
                    fn createTestIntent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <createTestIntentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::createTestIntent)
                    }
                    createTestIntent
                },
                {
                    fn cancelIntent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <cancelIntentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::cancelIntent)
                    }
                    cancelIntent
                },
                {
                    fn reactor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <reactorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DCAHookHarnessCalls::reactor)
                    }
                    reactor
                },
                {
                    fn computeIntentId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <computeIntentIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::computeIntentId)
                    }
                    computeIntentId
                },
                {
                    fn __setPacked(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <__setPackedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::__setPacked)
                    }
                    __setPacked
                },
                {
                    fn preExecutionHook(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <preExecutionHookCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::preExecutionHook)
                    }
                    preExecutionHook
                },
                {
                    fn __setTotals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <__setTotalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessCalls::__setTotals)
                    }
                    __setTotals
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DCAHookHarnessCalls>] = &[
                {
                    fn __setExecutedMeta(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <__setExecutedMetaCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::__setExecutedMeta)
                    }
                    __setExecutedMeta
                },
                {
                    fn permit2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <permit2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::permit2)
                    }
                    permit2
                },
                {
                    fn getIntentStatistics(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <getIntentStatisticsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::getIntentStatistics)
                    }
                    getIntentStatistics
                },
                {
                    fn getNextNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <getNextNonceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::getNextNonce)
                    }
                    getNextNonce
                },
                {
                    fn validateChunkSize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validateChunkSizeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validateChunkSize)
                    }
                    validateChunkSize
                },
                {
                    fn validateStaticFields(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validateStaticFieldsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validateStaticFields)
                    }
                    validateStaticFields
                },
                {
                    fn isIntentActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <isIntentActiveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::isIntentActive)
                    }
                    isIntentActive
                },
                {
                    fn createTestCosignerData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <createTestCosignerDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::createTestCosignerData)
                    }
                    createTestCosignerData
                },
                {
                    fn DOMAIN_SEPARATOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::DOMAIN_SEPARATOR)
                    }
                    DOMAIN_SEPARATOR
                },
                {
                    fn getExecutionState(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <getExecutionStateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::getExecutionState)
                    }
                    getExecutionState
                },
                {
                    fn cancelIntents(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <cancelIntentsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::cancelIntents)
                    }
                    cancelIntents
                },
                {
                    fn validatePriceFloor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validatePriceFloorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validatePriceFloor)
                    }
                    validatePriceFloor
                },
                {
                    fn validateAllocationStructure(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <validateAllocationStructureCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::validateAllocationStructure)
                    }
                    validateAllocationStructure
                },
                {
                    fn transferInputTokens(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <transferInputTokensCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::transferInputTokens)
                    }
                    transferInputTokens
                },
                {
                    fn createTestIntent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <createTestIntentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::createTestIntent)
                    }
                    createTestIntent
                },
                {
                    fn cancelIntent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <cancelIntentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::cancelIntent)
                    }
                    cancelIntent
                },
                {
                    fn reactor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <reactorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::reactor)
                    }
                    reactor
                },
                {
                    fn computeIntentId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <computeIntentIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::computeIntentId)
                    }
                    computeIntentId
                },
                {
                    fn __setPacked(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <__setPackedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::__setPacked)
                    }
                    __setPacked
                },
                {
                    fn preExecutionHook(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <preExecutionHookCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::preExecutionHook)
                    }
                    preExecutionHook
                },
                {
                    fn __setTotals(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessCalls> {
                        <__setTotalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessCalls::__setTotals)
                    }
                    __setTotals
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
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::__setExecutedMeta(inner) => {
                    <__setExecutedMetaCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::__setPacked(inner) => {
                    <__setPackedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::__setTotals(inner) => {
                    <__setTotalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelIntent(inner) => {
                    <cancelIntentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelIntents(inner) => {
                    <cancelIntentsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::computeIntentId(inner) => {
                    <computeIntentIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createTestCosignerData(inner) => {
                    <createTestCosignerDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createTestIntent(inner) => {
                    <createTestIntentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getExecutionState(inner) => {
                    <getExecutionStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getIntentStatistics(inner) => {
                    <getIntentStatisticsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getNextNonce(inner) => {
                    <getNextNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isIntentActive(inner) => {
                    <isIntentActiveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::permit2(inner) => {
                    <permit2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::preExecutionHook(inner) => {
                    <preExecutionHookCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reactor(inner) => {
                    <reactorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferInputTokens(inner) => {
                    <transferInputTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validateAllocationStructure(inner) => {
                    <validateAllocationStructureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validateChunkSize(inner) => {
                    <validateChunkSizeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatePriceFloor(inner) => {
                    <validatePriceFloorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validateStaticFields(inner) => {
                    <validateStaticFieldsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DOMAIN_SEPARATOR(inner) => {
                    <DOMAIN_SEPARATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__setExecutedMeta(inner) => {
                    <__setExecutedMetaCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__setPacked(inner) => {
                    <__setPackedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__setTotals(inner) => {
                    <__setTotalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelIntent(inner) => {
                    <cancelIntentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelIntents(inner) => {
                    <cancelIntentsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::computeIntentId(inner) => {
                    <computeIntentIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createTestCosignerData(inner) => {
                    <createTestCosignerDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createTestIntent(inner) => {
                    <createTestIntentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getExecutionState(inner) => {
                    <getExecutionStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getIntentStatistics(inner) => {
                    <getIntentStatisticsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getNextNonce(inner) => {
                    <getNextNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isIntentActive(inner) => {
                    <isIntentActiveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::permit2(inner) => {
                    <permit2Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::preExecutionHook(inner) => {
                    <preExecutionHookCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reactor(inner) => {
                    <reactorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::transferInputTokens(inner) => {
                    <transferInputTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateAllocationStructure(inner) => {
                    <validateAllocationStructureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateChunkSize(inner) => {
                    <validateChunkSizeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatePriceFloor(inner) => {
                    <validatePriceFloorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validateStaticFields(inner) => {
                    <validateStaticFieldsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`DCAHookHarness`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DCAHookHarnessErrors {
        #[allow(missing_docs)]
        AllocationMismatch(AllocationMismatch),
        #[allow(missing_docs)]
        AllocationsNot100Percent(AllocationsNot100Percent),
        #[allow(missing_docs)]
        ChunkSizeAboveMax(ChunkSizeAboveMax),
        #[allow(missing_docs)]
        ChunkSizeBelowMin(ChunkSizeBelowMin),
        #[allow(missing_docs)]
        CosignerNonceMismatch(CosignerNonceMismatch),
        #[allow(missing_docs)]
        CosignerSwapperMismatch(CosignerSwapperMismatch),
        #[allow(missing_docs)]
        DuplicateRecipient(DuplicateRecipient),
        #[allow(missing_docs)]
        EmptyAllocations(EmptyAllocations),
        #[allow(missing_docs)]
        InputAboveLimit(InputAboveLimit),
        #[allow(missing_docs)]
        InputAmountMismatch(InputAmountMismatch),
        #[allow(missing_docs)]
        InsufficientOutput(InsufficientOutput),
        #[allow(missing_docs)]
        IntentAlreadyCancelled(IntentAlreadyCancelled),
        #[allow(missing_docs)]
        IntentExpired(IntentExpired),
        #[allow(missing_docs)]
        IntentIsCancelled(IntentIsCancelled),
        #[allow(missing_docs)]
        InvalidCosignerSignature(InvalidCosignerSignature),
        #[allow(missing_docs)]
        InvalidSwapperSignature(InvalidSwapperSignature),
        #[allow(missing_docs)]
        PriceBelowMin(PriceBelowMin),
        #[allow(missing_docs)]
        SwapperMismatch(SwapperMismatch),
        #[allow(missing_docs)]
        TooLate(TooLate),
        #[allow(missing_docs)]
        TooSoon(TooSoon),
        #[allow(missing_docs)]
        WrongChain(WrongChain),
        #[allow(missing_docs)]
        WrongChunkNonce(WrongChunkNonce),
        #[allow(missing_docs)]
        WrongHook(WrongHook),
        #[allow(missing_docs)]
        WrongInputToken(WrongInputToken),
        #[allow(missing_docs)]
        WrongOutputToken(WrongOutputToken),
        #[allow(missing_docs)]
        WrongTotalOutput(WrongTotalOutput),
        #[allow(missing_docs)]
        ZeroAllocation(ZeroAllocation),
        #[allow(missing_docs)]
        ZeroInput(ZeroInput),
    }
    impl DCAHookHarnessErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 31u8, 136u8, 70u8],
            [23u8, 170u8, 17u8, 121u8],
            [28u8, 168u8, 192u8, 98u8],
            [36u8, 73u8, 123u8, 195u8],
            [38u8, 96u8, 22u8, 27u8],
            [44u8, 25u8, 184u8, 184u8],
            [44u8, 99u8, 33u8, 38u8],
            [56u8, 139u8, 1u8, 115u8],
            [62u8, 145u8, 133u8, 121u8],
            [67u8, 132u8, 41u8, 213u8],
            [102u8, 198u8, 166u8, 175u8],
            [111u8, 8u8, 238u8, 110u8],
            [126u8, 187u8, 222u8, 171u8],
            [148u8, 158u8, 252u8, 150u8],
            [153u8, 148u8, 190u8, 24u8],
            [162u8, 78u8, 60u8, 7u8],
            [175u8, 69u8, 140u8, 7u8],
            [177u8, 185u8, 182u8, 238u8],
            [186u8, 4u8, 227u8, 109u8],
            [186u8, 13u8, 135u8, 181u8],
            [186u8, 168u8, 210u8, 173u8],
            [187u8, 204u8, 248u8, 143u8],
            [188u8, 157u8, 254u8, 140u8],
            [201u8, 227u8, 25u8, 131u8],
            [202u8, 86u8, 18u8, 246u8],
            [208u8, 100u8, 163u8, 247u8],
            [240u8, 15u8, 68u8, 245u8],
            [249u8, 78u8, 136u8, 60u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(IntentAlreadyCancelled),
            ::core::stringify!(InvalidSwapperSignature),
            ::core::stringify!(WrongChunkNonce),
            ::core::stringify!(WrongChain),
            ::core::stringify!(IntentIsCancelled),
            ::core::stringify!(InsufficientOutput),
            ::core::stringify!(WrongInputToken),
            ::core::stringify!(TooLate),
            ::core::stringify!(DuplicateRecipient),
            ::core::stringify!(CosignerSwapperMismatch),
            ::core::stringify!(AllocationMismatch),
            ::core::stringify!(IntentExpired),
            ::core::stringify!(WrongTotalOutput),
            ::core::stringify!(EmptyAllocations),
            ::core::stringify!(InputAmountMismatch),
            ::core::stringify!(ChunkSizeAboveMax),
            ::core::stringify!(ZeroInput),
            ::core::stringify!(TooSoon),
            ::core::stringify!(PriceBelowMin),
            ::core::stringify!(ZeroAllocation),
            ::core::stringify!(SwapperMismatch),
            ::core::stringify!(CosignerNonceMismatch),
            ::core::stringify!(AllocationsNot100Percent),
            ::core::stringify!(ChunkSizeBelowMin),
            ::core::stringify!(InvalidCosignerSignature),
            ::core::stringify!(WrongOutputToken),
            ::core::stringify!(InputAboveLimit),
            ::core::stringify!(WrongHook),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <IntentAlreadyCancelled as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSwapperSignature as alloy_sol_types::SolError>::SIGNATURE,
            <WrongChunkNonce as alloy_sol_types::SolError>::SIGNATURE,
            <WrongChain as alloy_sol_types::SolError>::SIGNATURE,
            <IntentIsCancelled as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientOutput as alloy_sol_types::SolError>::SIGNATURE,
            <WrongInputToken as alloy_sol_types::SolError>::SIGNATURE,
            <TooLate as alloy_sol_types::SolError>::SIGNATURE,
            <DuplicateRecipient as alloy_sol_types::SolError>::SIGNATURE,
            <CosignerSwapperMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <AllocationMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <IntentExpired as alloy_sol_types::SolError>::SIGNATURE,
            <WrongTotalOutput as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyAllocations as alloy_sol_types::SolError>::SIGNATURE,
            <InputAmountMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <ChunkSizeAboveMax as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroInput as alloy_sol_types::SolError>::SIGNATURE,
            <TooSoon as alloy_sol_types::SolError>::SIGNATURE,
            <PriceBelowMin as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroAllocation as alloy_sol_types::SolError>::SIGNATURE,
            <SwapperMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <CosignerNonceMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <AllocationsNot100Percent as alloy_sol_types::SolError>::SIGNATURE,
            <ChunkSizeBelowMin as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidCosignerSignature as alloy_sol_types::SolError>::SIGNATURE,
            <WrongOutputToken as alloy_sol_types::SolError>::SIGNATURE,
            <InputAboveLimit as alloy_sol_types::SolError>::SIGNATURE,
            <WrongHook as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DCAHookHarnessErrors {
        const NAME: &'static str = "DCAHookHarnessErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AllocationMismatch(_) => {
                    <AllocationMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AllocationsNot100Percent(_) => {
                    <AllocationsNot100Percent as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChunkSizeAboveMax(_) => {
                    <ChunkSizeAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChunkSizeBelowMin(_) => {
                    <ChunkSizeBelowMin as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CosignerNonceMismatch(_) => {
                    <CosignerNonceMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CosignerSwapperMismatch(_) => {
                    <CosignerSwapperMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DuplicateRecipient(_) => {
                    <DuplicateRecipient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyAllocations(_) => {
                    <EmptyAllocations as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAboveLimit(_) => {
                    <InputAboveLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAmountMismatch(_) => {
                    <InputAmountMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientOutput(_) => {
                    <InsufficientOutput as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IntentAlreadyCancelled(_) => {
                    <IntentAlreadyCancelled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IntentExpired(_) => {
                    <IntentExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IntentIsCancelled(_) => {
                    <IntentIsCancelled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCosignerSignature(_) => {
                    <InvalidCosignerSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSwapperSignature(_) => {
                    <InvalidSwapperSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PriceBelowMin(_) => {
                    <PriceBelowMin as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SwapperMismatch(_) => {
                    <SwapperMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TooLate(_) => <TooLate as alloy_sol_types::SolError>::SELECTOR,
                Self::TooSoon(_) => <TooSoon as alloy_sol_types::SolError>::SELECTOR,
                Self::WrongChain(_) => {
                    <WrongChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongChunkNonce(_) => {
                    <WrongChunkNonce as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongHook(_) => <WrongHook as alloy_sol_types::SolError>::SELECTOR,
                Self::WrongInputToken(_) => {
                    <WrongInputToken as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongOutputToken(_) => {
                    <WrongOutputToken as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongTotalOutput(_) => {
                    <WrongTotalOutput as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAllocation(_) => {
                    <ZeroAllocation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroInput(_) => <ZeroInput as alloy_sol_types::SolError>::SELECTOR,
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DCAHookHarnessErrors>] = &[
                {
                    fn IntentAlreadyCancelled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <IntentAlreadyCancelled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::IntentAlreadyCancelled)
                    }
                    IntentAlreadyCancelled
                },
                {
                    fn InvalidSwapperSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InvalidSwapperSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InvalidSwapperSignature)
                    }
                    InvalidSwapperSignature
                },
                {
                    fn WrongChunkNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongChunkNonce as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongChunkNonce)
                    }
                    WrongChunkNonce
                },
                {
                    fn WrongChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongChain as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DCAHookHarnessErrors::WrongChain)
                    }
                    WrongChain
                },
                {
                    fn IntentIsCancelled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <IntentIsCancelled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::IntentIsCancelled)
                    }
                    IntentIsCancelled
                },
                {
                    fn InsufficientOutput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InsufficientOutput as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InsufficientOutput)
                    }
                    InsufficientOutput
                },
                {
                    fn WrongInputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongInputToken as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongInputToken)
                    }
                    WrongInputToken
                },
                {
                    fn TooLate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <TooLate as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DCAHookHarnessErrors::TooLate)
                    }
                    TooLate
                },
                {
                    fn DuplicateRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <DuplicateRecipient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::DuplicateRecipient)
                    }
                    DuplicateRecipient
                },
                {
                    fn CosignerSwapperMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <CosignerSwapperMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::CosignerSwapperMismatch)
                    }
                    CosignerSwapperMismatch
                },
                {
                    fn AllocationMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <AllocationMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::AllocationMismatch)
                    }
                    AllocationMismatch
                },
                {
                    fn IntentExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <IntentExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::IntentExpired)
                    }
                    IntentExpired
                },
                {
                    fn WrongTotalOutput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongTotalOutput as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongTotalOutput)
                    }
                    WrongTotalOutput
                },
                {
                    fn EmptyAllocations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <EmptyAllocations as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::EmptyAllocations)
                    }
                    EmptyAllocations
                },
                {
                    fn InputAmountMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InputAmountMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InputAmountMismatch)
                    }
                    InputAmountMismatch
                },
                {
                    fn ChunkSizeAboveMax(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ChunkSizeAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::ChunkSizeAboveMax)
                    }
                    ChunkSizeAboveMax
                },
                {
                    fn ZeroInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ZeroInput as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DCAHookHarnessErrors::ZeroInput)
                    }
                    ZeroInput
                },
                {
                    fn TooSoon(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <TooSoon as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DCAHookHarnessErrors::TooSoon)
                    }
                    TooSoon
                },
                {
                    fn PriceBelowMin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <PriceBelowMin as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::PriceBelowMin)
                    }
                    PriceBelowMin
                },
                {
                    fn ZeroAllocation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ZeroAllocation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::ZeroAllocation)
                    }
                    ZeroAllocation
                },
                {
                    fn SwapperMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <SwapperMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::SwapperMismatch)
                    }
                    SwapperMismatch
                },
                {
                    fn CosignerNonceMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <CosignerNonceMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::CosignerNonceMismatch)
                    }
                    CosignerNonceMismatch
                },
                {
                    fn AllocationsNot100Percent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <AllocationsNot100Percent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::AllocationsNot100Percent)
                    }
                    AllocationsNot100Percent
                },
                {
                    fn ChunkSizeBelowMin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ChunkSizeBelowMin as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::ChunkSizeBelowMin)
                    }
                    ChunkSizeBelowMin
                },
                {
                    fn InvalidCosignerSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InvalidCosignerSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InvalidCosignerSignature)
                    }
                    InvalidCosignerSignature
                },
                {
                    fn WrongOutputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongOutputToken as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongOutputToken)
                    }
                    WrongOutputToken
                },
                {
                    fn InputAboveLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InputAboveLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InputAboveLimit)
                    }
                    InputAboveLimit
                },
                {
                    fn WrongHook(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongHook as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DCAHookHarnessErrors::WrongHook)
                    }
                    WrongHook
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
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DCAHookHarnessErrors>] = &[
                {
                    fn IntentAlreadyCancelled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <IntentAlreadyCancelled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::IntentAlreadyCancelled)
                    }
                    IntentAlreadyCancelled
                },
                {
                    fn InvalidSwapperSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InvalidSwapperSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InvalidSwapperSignature)
                    }
                    InvalidSwapperSignature
                },
                {
                    fn WrongChunkNonce(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongChunkNonce as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongChunkNonce)
                    }
                    WrongChunkNonce
                },
                {
                    fn WrongChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongChain)
                    }
                    WrongChain
                },
                {
                    fn IntentIsCancelled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <IntentIsCancelled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::IntentIsCancelled)
                    }
                    IntentIsCancelled
                },
                {
                    fn InsufficientOutput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InsufficientOutput as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InsufficientOutput)
                    }
                    InsufficientOutput
                },
                {
                    fn WrongInputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongInputToken as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongInputToken)
                    }
                    WrongInputToken
                },
                {
                    fn TooLate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <TooLate as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::TooLate)
                    }
                    TooLate
                },
                {
                    fn DuplicateRecipient(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <DuplicateRecipient as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::DuplicateRecipient)
                    }
                    DuplicateRecipient
                },
                {
                    fn CosignerSwapperMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <CosignerSwapperMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::CosignerSwapperMismatch)
                    }
                    CosignerSwapperMismatch
                },
                {
                    fn AllocationMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <AllocationMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::AllocationMismatch)
                    }
                    AllocationMismatch
                },
                {
                    fn IntentExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <IntentExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::IntentExpired)
                    }
                    IntentExpired
                },
                {
                    fn WrongTotalOutput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongTotalOutput as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongTotalOutput)
                    }
                    WrongTotalOutput
                },
                {
                    fn EmptyAllocations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <EmptyAllocations as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::EmptyAllocations)
                    }
                    EmptyAllocations
                },
                {
                    fn InputAmountMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InputAmountMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InputAmountMismatch)
                    }
                    InputAmountMismatch
                },
                {
                    fn ChunkSizeAboveMax(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ChunkSizeAboveMax as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::ChunkSizeAboveMax)
                    }
                    ChunkSizeAboveMax
                },
                {
                    fn ZeroInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ZeroInput as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::ZeroInput)
                    }
                    ZeroInput
                },
                {
                    fn TooSoon(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <TooSoon as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::TooSoon)
                    }
                    TooSoon
                },
                {
                    fn PriceBelowMin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <PriceBelowMin as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::PriceBelowMin)
                    }
                    PriceBelowMin
                },
                {
                    fn ZeroAllocation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ZeroAllocation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::ZeroAllocation)
                    }
                    ZeroAllocation
                },
                {
                    fn SwapperMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <SwapperMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::SwapperMismatch)
                    }
                    SwapperMismatch
                },
                {
                    fn CosignerNonceMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <CosignerNonceMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::CosignerNonceMismatch)
                    }
                    CosignerNonceMismatch
                },
                {
                    fn AllocationsNot100Percent(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <AllocationsNot100Percent as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::AllocationsNot100Percent)
                    }
                    AllocationsNot100Percent
                },
                {
                    fn ChunkSizeBelowMin(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <ChunkSizeBelowMin as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::ChunkSizeBelowMin)
                    }
                    ChunkSizeBelowMin
                },
                {
                    fn InvalidCosignerSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InvalidCosignerSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InvalidCosignerSignature)
                    }
                    InvalidCosignerSignature
                },
                {
                    fn WrongOutputToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongOutputToken as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongOutputToken)
                    }
                    WrongOutputToken
                },
                {
                    fn InputAboveLimit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <InputAboveLimit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::InputAboveLimit)
                    }
                    InputAboveLimit
                },
                {
                    fn WrongHook(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DCAHookHarnessErrors> {
                        <WrongHook as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DCAHookHarnessErrors::WrongHook)
                    }
                    WrongHook
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
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AllocationMismatch(inner) => {
                    <AllocationMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AllocationsNot100Percent(inner) => {
                    <AllocationsNot100Percent as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChunkSizeAboveMax(inner) => {
                    <ChunkSizeAboveMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChunkSizeBelowMin(inner) => {
                    <ChunkSizeBelowMin as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CosignerNonceMismatch(inner) => {
                    <CosignerNonceMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CosignerSwapperMismatch(inner) => {
                    <CosignerSwapperMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DuplicateRecipient(inner) => {
                    <DuplicateRecipient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyAllocations(inner) => {
                    <EmptyAllocations as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputAboveLimit(inner) => {
                    <InputAboveLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputAmountMismatch(inner) => {
                    <InputAmountMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientOutput(inner) => {
                    <InsufficientOutput as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IntentAlreadyCancelled(inner) => {
                    <IntentAlreadyCancelled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IntentExpired(inner) => {
                    <IntentExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::IntentIsCancelled(inner) => {
                    <IntentIsCancelled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCosignerSignature(inner) => {
                    <InvalidCosignerSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSwapperSignature(inner) => {
                    <InvalidSwapperSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PriceBelowMin(inner) => {
                    <PriceBelowMin as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SwapperMismatch(inner) => {
                    <SwapperMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TooLate(inner) => {
                    <TooLate as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TooSoon(inner) => {
                    <TooSoon as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::WrongChain(inner) => {
                    <WrongChain as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::WrongChunkNonce(inner) => {
                    <WrongChunkNonce as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongHook(inner) => {
                    <WrongHook as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::WrongInputToken(inner) => {
                    <WrongInputToken as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongOutputToken(inner) => {
                    <WrongOutputToken as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongTotalOutput(inner) => {
                    <WrongTotalOutput as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroAllocation(inner) => {
                    <ZeroAllocation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroInput(inner) => {
                    <ZeroInput as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AllocationMismatch(inner) => {
                    <AllocationMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AllocationsNot100Percent(inner) => {
                    <AllocationsNot100Percent as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChunkSizeAboveMax(inner) => {
                    <ChunkSizeAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChunkSizeBelowMin(inner) => {
                    <ChunkSizeBelowMin as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CosignerNonceMismatch(inner) => {
                    <CosignerNonceMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CosignerSwapperMismatch(inner) => {
                    <CosignerSwapperMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DuplicateRecipient(inner) => {
                    <DuplicateRecipient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyAllocations(inner) => {
                    <EmptyAllocations as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputAboveLimit(inner) => {
                    <InputAboveLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputAmountMismatch(inner) => {
                    <InputAmountMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientOutput(inner) => {
                    <InsufficientOutput as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IntentAlreadyCancelled(inner) => {
                    <IntentAlreadyCancelled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IntentExpired(inner) => {
                    <IntentExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IntentIsCancelled(inner) => {
                    <IntentIsCancelled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCosignerSignature(inner) => {
                    <InvalidCosignerSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSwapperSignature(inner) => {
                    <InvalidSwapperSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PriceBelowMin(inner) => {
                    <PriceBelowMin as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SwapperMismatch(inner) => {
                    <SwapperMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TooLate(inner) => {
                    <TooLate as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::TooSoon(inner) => {
                    <TooSoon as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::WrongChain(inner) => {
                    <WrongChain as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::WrongChunkNonce(inner) => {
                    <WrongChunkNonce as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongHook(inner) => {
                    <WrongHook as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::WrongInputToken(inner) => {
                    <WrongInputToken as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongOutputToken(inner) => {
                    <WrongOutputToken as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongTotalOutput(inner) => {
                    <WrongTotalOutput as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroAllocation(inner) => {
                    <ZeroAllocation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroInput(inner) => {
                    <ZeroInput as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`DCAHookHarness`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DCAHookHarnessEvents {
        #[allow(missing_docs)]
        ChunkExecuted(ChunkExecuted),
        #[allow(missing_docs)]
        IntentCancelled(IntentCancelled),
    }
    impl DCAHookHarnessEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                133u8, 149u8, 171u8, 3u8, 241u8, 181u8, 73u8, 108u8, 204u8, 199u8, 88u8,
                51u8, 178u8, 53u8, 177u8, 185u8, 93u8, 238u8, 115u8, 9u8, 98u8, 130u8,
                211u8, 172u8, 133u8, 27u8, 98u8, 141u8, 242u8, 75u8, 208u8, 242u8,
            ],
            [
                172u8, 90u8, 75u8, 144u8, 228u8, 33u8, 0u8, 42u8, 47u8, 219u8, 159u8,
                19u8, 43u8, 155u8, 50u8, 194u8, 79u8, 164u8, 174u8, 22u8, 236u8, 72u8,
                5u8, 22u8, 200u8, 89u8, 50u8, 222u8, 32u8, 141u8, 42u8, 51u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(ChunkExecuted),
            ::core::stringify!(IntentCancelled),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <ChunkExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <IntentCancelled as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DCAHookHarnessEvents {
        const NAME: &'static str = "DCAHookHarnessEvents";
        const COUNT: usize = 2usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ChunkExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChunkExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChunkExecuted)
                }
                Some(<IntentCancelled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <IntentCancelled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::IntentCancelled)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for DCAHookHarnessEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChunkExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::IntentCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChunkExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::IntentCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DCAHookHarness`](self) contract instance.

See the [wrapper's documentation](`DCAHookHarnessInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> DCAHookHarnessInstance<P, N> {
        DCAHookHarnessInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        p: alloy::sol_types::private::Address,
        r: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DCAHookHarnessInstance<P, N>>,
    > {
        DCAHookHarnessInstance::<P, N>::deploy(__provider, p, r)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        p: alloy::sol_types::private::Address,
        r: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        DCAHookHarnessInstance::<P, N>::deploy_builder(__provider, p, r)
    }
    /**A [`DCAHookHarness`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DCAHookHarness`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DCAHookHarnessInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for DCAHookHarnessInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DCAHookHarnessInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DCAHookHarnessInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`DCAHookHarness`](self) contract instance.

See the [wrapper's documentation](`DCAHookHarnessInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            p: alloy::sol_types::private::Address,
            r: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<DCAHookHarnessInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, p, r);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            p: alloy::sol_types::private::Address,
            r: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { p, r },
                    )[..],
                ]
                    .concat()
                    .into(),
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
    impl<P: ::core::clone::Clone, N> DCAHookHarnessInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DCAHookHarnessInstance<P, N> {
            DCAHookHarnessInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DCAHookHarnessInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`DOMAIN_SEPARATOR`] function.
        pub fn DOMAIN_SEPARATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, DOMAIN_SEPARATORCall, N> {
            self.call_builder(&DOMAIN_SEPARATORCall)
        }
        ///Creates a new call builder for the [`__setExecutedMeta`] function.
        pub fn __setExecutedMeta(
            &self,
            intentId: alloy::sol_types::private::FixedBytes<32>,
            lastExecutionTime: alloy::sol_types::private::primitives::aliases::U120,
        ) -> alloy_contract::SolCallBuilder<&P, __setExecutedMetaCall, N> {
            self.call_builder(
                &__setExecutedMetaCall {
                    intentId,
                    lastExecutionTime,
                },
            )
        }
        ///Creates a new call builder for the [`__setPacked`] function.
        pub fn __setPacked(
            &self,
            intentId: alloy::sol_types::private::FixedBytes<32>,
            executedChunks: u128,
            cancelled: bool,
        ) -> alloy_contract::SolCallBuilder<&P, __setPackedCall, N> {
            self.call_builder(
                &__setPackedCall {
                    intentId,
                    executedChunks,
                    cancelled,
                },
            )
        }
        ///Creates a new call builder for the [`__setTotals`] function.
        pub fn __setTotals(
            &self,
            intentId: alloy::sol_types::private::FixedBytes<32>,
            totalInputExecuted: alloy::sol_types::private::primitives::aliases::U256,
            totalOutput: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, __setTotalsCall, N> {
            self.call_builder(
                &__setTotalsCall {
                    intentId,
                    totalInputExecuted,
                    totalOutput,
                },
            )
        }
        ///Creates a new call builder for the [`cancelIntent`] function.
        pub fn cancelIntent(
            &self,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, cancelIntentCall, N> {
            self.call_builder(&cancelIntentCall { nonce })
        }
        ///Creates a new call builder for the [`cancelIntents`] function.
        pub fn cancelIntents(
            &self,
            nonces: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, cancelIntentsCall, N> {
            self.call_builder(&cancelIntentsCall { nonces })
        }
        ///Creates a new call builder for the [`computeIntentId`] function.
        pub fn computeIntentId(
            &self,
            swapper: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, computeIntentIdCall, N> {
            self.call_builder(
                &computeIntentIdCall {
                    swapper,
                    nonce,
                },
            )
        }
        ///Creates a new call builder for the [`createTestCosignerData`] function.
        pub fn createTestCosignerData(
            &self,
            swapper: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U96,
            execAmount: alloy::sol_types::private::primitives::aliases::U160,
            limitAmount: alloy::sol_types::private::primitives::aliases::U160,
            orderNonce: alloy::sol_types::private::primitives::aliases::U96,
        ) -> alloy_contract::SolCallBuilder<&P, createTestCosignerDataCall, N> {
            self.call_builder(
                &createTestCosignerDataCall {
                    swapper,
                    nonce,
                    execAmount,
                    limitAmount,
                    orderNonce,
                },
            )
        }
        ///Creates a new call builder for the [`createTestIntent`] function.
        pub fn createTestIntent(
            &self,
            swapper: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U96,
            isExactIn: bool,
            minChunk: alloy::sol_types::private::primitives::aliases::U256,
            maxChunk: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, createTestIntentCall, N> {
            self.call_builder(
                &createTestIntentCall {
                    swapper,
                    nonce,
                    isExactIn,
                    minChunk,
                    maxChunk,
                },
            )
        }
        ///Creates a new call builder for the [`getExecutionState`] function.
        pub fn getExecutionState(
            &self,
            intentId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getExecutionStateCall, N> {
            self.call_builder(&getExecutionStateCall { intentId })
        }
        ///Creates a new call builder for the [`getIntentStatistics`] function.
        pub fn getIntentStatistics(
            &self,
            intentId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getIntentStatisticsCall, N> {
            self.call_builder(
                &getIntentStatisticsCall {
                    intentId,
                },
            )
        }
        ///Creates a new call builder for the [`getNextNonce`] function.
        pub fn getNextNonce(
            &self,
            intentId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getNextNonceCall, N> {
            self.call_builder(&getNextNonceCall { intentId })
        }
        ///Creates a new call builder for the [`isIntentActive`] function.
        pub fn isIntentActive(
            &self,
            intentId: alloy::sol_types::private::FixedBytes<32>,
            maxPeriod: alloy::sol_types::private::primitives::aliases::U256,
            deadline: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isIntentActiveCall, N> {
            self.call_builder(
                &isIntentActiveCall {
                    intentId,
                    maxPeriod,
                    deadline,
                },
            )
        }
        ///Creates a new call builder for the [`permit2`] function.
        pub fn permit2(&self) -> alloy_contract::SolCallBuilder<&P, permit2Call, N> {
            self.call_builder(&permit2Call)
        }
        ///Creates a new call builder for the [`preExecutionHook`] function.
        pub fn preExecutionHook(
            &self,
            filler: alloy::sol_types::private::Address,
            resolvedOrder: <ResolvedOrder as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, preExecutionHookCall, N> {
            self.call_builder(
                &preExecutionHookCall {
                    filler,
                    resolvedOrder,
                },
            )
        }
        ///Creates a new call builder for the [`reactor`] function.
        pub fn reactor(&self) -> alloy_contract::SolCallBuilder<&P, reactorCall, N> {
            self.call_builder(&reactorCall)
        }
        ///Creates a new call builder for the [`transferInputTokens`] function.
        pub fn transferInputTokens(
            &self,
            order: <ResolvedOrder as alloy::sol_types::SolType>::RustType,
            to: alloy::sol_types::private::Address,
            permitData: <PermitData as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, transferInputTokensCall, N> {
            self.call_builder(
                &transferInputTokensCall {
                    order,
                    to,
                    permitData,
                },
            )
        }
        ///Creates a new call builder for the [`validateAllocationStructure`] function.
        pub fn validateAllocationStructure(
            &self,
            outputAllocations: alloy::sol_types::private::Vec<
                <OutputAllocation as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, validateAllocationStructureCall, N> {
            self.call_builder(
                &validateAllocationStructureCall {
                    outputAllocations,
                },
            )
        }
        ///Creates a new call builder for the [`validateChunkSize`] function.
        pub fn validateChunkSize(
            &self,
            intent: <DCAIntent as alloy::sol_types::SolType>::RustType,
            cosignerData: <DCAOrderCosignerData as alloy::sol_types::SolType>::RustType,
            inputAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, validateChunkSizeCall, N> {
            self.call_builder(
                &validateChunkSizeCall {
                    intent,
                    cosignerData,
                    inputAmount,
                },
            )
        }
        ///Creates a new call builder for the [`validatePriceFloor`] function.
        pub fn validatePriceFloor(
            &self,
            isExactIn: bool,
            execAmount: alloy::sol_types::private::primitives::aliases::U160,
            limitAmount: alloy::sol_types::private::primitives::aliases::U160,
            minPrice: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, validatePriceFloorCall, N> {
            self.call_builder(
                &validatePriceFloorCall {
                    isExactIn,
                    execAmount,
                    limitAmount,
                    minPrice,
                },
            )
        }
        ///Creates a new call builder for the [`validateStaticFields`] function.
        pub fn validateStaticFields(
            &self,
            intent: <DCAIntent as alloy::sol_types::SolType>::RustType,
            resolvedOrder: <ResolvedOrder as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, validateStaticFieldsCall, N> {
            self.call_builder(
                &validateStaticFieldsCall {
                    intent,
                    resolvedOrder,
                },
            )
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DCAHookHarnessInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ChunkExecuted`] event.
        pub fn ChunkExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChunkExecuted, N> {
            self.event_filter::<ChunkExecuted>()
        }
        ///Creates a new event filter for the [`IntentCancelled`] event.
        pub fn IntentCancelled_filter(
            &self,
        ) -> alloy_contract::Event<&P, IntentCancelled, N> {
            self.event_filter::<IntentCancelled>()
        }
    }
}
