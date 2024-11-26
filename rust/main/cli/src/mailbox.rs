pub use mailbox::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mailbox {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Mailbox was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"_localDomain\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"hook\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"DefaultHookSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"DefaultIsmSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint32\",\n        \"name\": \"destination\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"recipient\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes\",\n        \"name\": \"message\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"Dispatch\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"DispatchId\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"version\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"Initialized\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint32\",\n        \"name\": \"origin\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"sender\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Process\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"ProcessId\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"hook\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RequiredHookSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"VERSION\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"defaultHook\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IPostDispatchHook\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"defaultIsm\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IInterchainSecurityModule\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"delivered\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"deployedBlock\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"destinationDomain\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"recipientAddress\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"messageBody\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"metadata\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"contract IPostDispatchHook\",\n        \"name\": \"hook\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"dispatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"destinationDomain\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"recipientAddress\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"messageBody\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"hookMetadata\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"dispatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"_destinationDomain\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_recipientAddress\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_messageBody\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"dispatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_defaultIsm\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_defaultHook\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_requiredHook\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"latestDispatchedId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"localDomain\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"nonce\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_metadata\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_message\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"process\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"processedAt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint48\",\n        \"name\": \"\",\n        \"type\": \"uint48\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"processor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"destinationDomain\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"recipientAddress\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"messageBody\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"metadata\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"contract IPostDispatchHook\",\n        \"name\": \"hook\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"quoteDispatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"destinationDomain\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"recipientAddress\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"messageBody\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"quoteDispatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"destinationDomain\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"recipientAddress\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"messageBody\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"defaultHookMetadata\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"quoteDispatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_recipient\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"recipientIsm\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IInterchainSecurityModule\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"requiredHook\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IPostDispatchHook\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_hook\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setDefaultHook\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_module\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setDefaultIsm\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_hook\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setRequiredHook\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MAILBOX_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Mailbox<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Mailbox<M> {
        fn clone(&self) -> Self {
            Mailbox(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Mailbox<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Mailbox<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Mailbox))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Mailbox<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MAILBOX_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `VERSION` (0xffa1ad74) function"]
        pub fn version(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultHook` (0x3d1250b7) function"]
        pub fn default_hook(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([61, 18, 80, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultIsm` (0x6e5f516e) function"]
        pub fn default_ism(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([110, 95, 81, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delivered` (0xe495f1d4) function"]
        pub fn delivered(&self, id: [u8; 32]) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 149, 241, 212], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployedBlock` (0x82ea7bfe) function"]
        pub fn deployed_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([130, 234, 123, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dispatch` (0x10b83dc0) function"]
        pub fn dispatch_2(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ethers::core::types::Bytes,
            metadata: ethers::core::types::Bytes,
            hook: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [16, 184, 61, 192],
                    (
                        destination_domain,
                        recipient_address,
                        message_body,
                        metadata,
                        hook,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dispatch` (0x48aee8d4) function"]
        pub fn dispatch_1(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ethers::core::types::Bytes,
            hook_metadata: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [72, 174, 232, 212],
                    (
                        destination_domain,
                        recipient_address,
                        message_body,
                        hook_metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dispatch` (0xfa31de01) function"]
        pub fn dispatch_0(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [250, 49, 222, 1],
                    (destination_domain, recipient_address, message_body),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xf8c8765e) function"]
        pub fn initialize(
            &self,
            owner: ethers::core::types::Address,
            default_ism: ethers::core::types::Address,
            default_hook: ethers::core::types::Address,
            required_hook: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 200, 118, 94],
                    (owner, default_ism, default_hook, required_hook),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestDispatchedId` (0x134fbb4f) function"]
        pub fn latest_dispatched_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([19, 79, 187, 79], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `localDomain` (0x8d3638f4) function"]
        pub fn local_domain(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([141, 54, 56, 244], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonce` (0xaffed0e0) function"]
        pub fn nonce(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `process` (0x7c39d130) function"]
        pub fn process(
            &self,
            metadata: ethers::core::types::Bytes,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 57, 209, 48], (metadata, message))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `processedAt` (0x07a2fda1) function"]
        pub fn processed_at(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([7, 162, 253, 161], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `processor` (0x5d1fe5a9) function"]
        pub fn processor(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([93, 31, 229, 169], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quoteDispatch` (0x81d2ea95) function"]
        pub fn quote_dispatch_with_destination_domain_and_recipient_address_and_metadata(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ethers::core::types::Bytes,
            metadata: ethers::core::types::Bytes,
            hook: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [129, 210, 234, 149],
                    (
                        destination_domain,
                        recipient_address,
                        message_body,
                        metadata,
                        hook,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quoteDispatch` (0x9c42bd18) function"]
        pub fn quote_dispatch(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [156, 66, 189, 24],
                    (destination_domain, recipient_address, message_body),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quoteDispatch` (0xf7ccd321) function"]
        pub fn quote_dispatch_with_destination_domain_and_recipient_address_and_default_hook_metadata(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ethers::core::types::Bytes,
            default_hook_metadata: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [247, 204, 211, 33],
                    (
                        destination_domain,
                        recipient_address,
                        message_body,
                        default_hook_metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `recipientIsm` (0xe70f48ac) function"]
        pub fn recipient_ism(
            &self,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([231, 15, 72, 172], recipient)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `requiredHook` (0xd6d08a09) function"]
        pub fn required_hook(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([214, 208, 138, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDefaultHook` (0x99b04809) function"]
        pub fn set_default_hook(
            &self,
            hook: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 176, 72, 9], hook)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDefaultIsm` (0xf794687a) function"]
        pub fn set_default_ism(
            &self,
            module: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 148, 104, 122], module)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRequiredHook` (0x1426b7f4) function"]
        pub fn set_required_hook(
            &self,
            hook: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 38, 183, 244], hook)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `DefaultHookSet` event"]
        pub fn default_hook_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DefaultHookSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DefaultIsmSet` event"]
        pub fn default_ism_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DefaultIsmSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Dispatch` event"]
        pub fn dispatch_filter(&self) -> ethers::contract::builders::Event<M, DispatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DispatchId` event"]
        pub fn dispatch_id_filter(&self) -> ethers::contract::builders::Event<M, DispatchIdFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Process` event"]
        pub fn process_filter(&self) -> ethers::contract::builders::Event<M, ProcessFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProcessId` event"]
        pub fn process_id_filter(&self) -> ethers::contract::builders::Event<M, ProcessIdFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RequiredHookSet` event"]
        pub fn required_hook_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RequiredHookSetFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MailboxEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Mailbox<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "DefaultHookSet", abi = "DefaultHookSet(address)")]
    pub struct DefaultHookSetFilter {
        #[ethevent(indexed)]
        pub hook: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "DefaultIsmSet", abi = "DefaultIsmSet(address)")]
    pub struct DefaultIsmSetFilter {
        #[ethevent(indexed)]
        pub module: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Dispatch", abi = "Dispatch(address,uint32,bytes32,bytes)")]
    pub struct DispatchFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: u32,
        #[ethevent(indexed)]
        pub recipient: [u8; 32],
        pub message: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "DispatchId", abi = "DispatchId(bytes32)")]
    pub struct DispatchIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Process", abi = "Process(uint32,bytes32,address)")]
    pub struct ProcessFilter {
        #[ethevent(indexed)]
        pub origin: u32,
        #[ethevent(indexed)]
        pub sender: [u8; 32],
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ProcessId", abi = "ProcessId(bytes32)")]
    pub struct ProcessIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "RequiredHookSet", abi = "RequiredHookSet(address)")]
    pub struct RequiredHookSetFilter {
        #[ethevent(indexed)]
        pub hook: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MailboxEvents {
        DefaultHookSetFilter(DefaultHookSetFilter),
        DefaultIsmSetFilter(DefaultIsmSetFilter),
        DispatchFilter(DispatchFilter),
        DispatchIdFilter(DispatchIdFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProcessFilter(ProcessFilter),
        ProcessIdFilter(ProcessIdFilter),
        RequiredHookSetFilter(RequiredHookSetFilter),
    }
    impl ethers::contract::EthLogDecode for MailboxEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DefaultHookSetFilter::decode_log(log) {
                return Ok(MailboxEvents::DefaultHookSetFilter(decoded));
            }
            if let Ok(decoded) = DefaultIsmSetFilter::decode_log(log) {
                return Ok(MailboxEvents::DefaultIsmSetFilter(decoded));
            }
            if let Ok(decoded) = DispatchFilter::decode_log(log) {
                return Ok(MailboxEvents::DispatchFilter(decoded));
            }
            if let Ok(decoded) = DispatchIdFilter::decode_log(log) {
                return Ok(MailboxEvents::DispatchIdFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MailboxEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MailboxEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProcessFilter::decode_log(log) {
                return Ok(MailboxEvents::ProcessFilter(decoded));
            }
            if let Ok(decoded) = ProcessIdFilter::decode_log(log) {
                return Ok(MailboxEvents::ProcessIdFilter(decoded));
            }
            if let Ok(decoded) = RequiredHookSetFilter::decode_log(log) {
                return Ok(MailboxEvents::RequiredHookSetFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MailboxEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MailboxEvents::DefaultHookSetFilter(element) => element.fmt(f),
                MailboxEvents::DefaultIsmSetFilter(element) => element.fmt(f),
                MailboxEvents::DispatchFilter(element) => element.fmt(f),
                MailboxEvents::DispatchIdFilter(element) => element.fmt(f),
                MailboxEvents::InitializedFilter(element) => element.fmt(f),
                MailboxEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                MailboxEvents::ProcessFilter(element) => element.fmt(f),
                MailboxEvents::ProcessIdFilter(element) => element.fmt(f),
                MailboxEvents::RequiredHookSetFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `[255, 161, 173, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    #[doc = "Container type for all input parameters for the `defaultHook` function with signature `defaultHook()` and selector `[61, 18, 80, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "defaultHook", abi = "defaultHook()")]
    pub struct DefaultHookCall;
    #[doc = "Container type for all input parameters for the `defaultIsm` function with signature `defaultIsm()` and selector `[110, 95, 81, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "defaultIsm", abi = "defaultIsm()")]
    pub struct DefaultIsmCall;
    #[doc = "Container type for all input parameters for the `delivered` function with signature `delivered(bytes32)` and selector `[228, 149, 241, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "delivered", abi = "delivered(bytes32)")]
    pub struct DeliveredCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `deployedBlock` function with signature `deployedBlock()` and selector `[130, 234, 123, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deployedBlock", abi = "deployedBlock()")]
    pub struct DeployedBlockCall;
    #[doc = "Container type for all input parameters for the `dispatch` function with signature `dispatch(uint32,bytes32,bytes,bytes,address)` and selector `[16, 184, 61, 192]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "dispatch",
        abi = "dispatch(uint32,bytes32,bytes,bytes,address)"
    )]
    pub struct Dispatch2Call {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ethers::core::types::Bytes,
        pub metadata: ethers::core::types::Bytes,
        pub hook: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `dispatch` function with signature `dispatch(uint32,bytes32,bytes,bytes)` and selector `[72, 174, 232, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "dispatch", abi = "dispatch(uint32,bytes32,bytes,bytes)")]
    pub struct Dispatch1Call {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ethers::core::types::Bytes,
        pub hook_metadata: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `[250, 49, 222, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "dispatch", abi = "dispatch(uint32,bytes32,bytes)")]
    pub struct Dispatch0Call {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address)` and selector `[248, 200, 118, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub owner: ethers::core::types::Address,
        pub default_ism: ethers::core::types::Address,
        pub default_hook: ethers::core::types::Address,
        pub required_hook: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `latestDispatchedId` function with signature `latestDispatchedId()` and selector `[19, 79, 187, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "latestDispatchedId", abi = "latestDispatchedId()")]
    pub struct LatestDispatchedIdCall;
    #[doc = "Container type for all input parameters for the `localDomain` function with signature `localDomain()` and selector `[141, 54, 56, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "localDomain", abi = "localDomain()")]
    pub struct LocalDomainCall;
    #[doc = "Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `[175, 254, 208, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `process` function with signature `process(bytes,bytes)` and selector `[124, 57, 209, 48]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "process", abi = "process(bytes,bytes)")]
    pub struct ProcessCall {
        pub metadata: ethers::core::types::Bytes,
        pub message: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `processedAt` function with signature `processedAt(bytes32)` and selector `[7, 162, 253, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "processedAt", abi = "processedAt(bytes32)")]
    pub struct ProcessedAtCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `processor` function with signature `processor(bytes32)` and selector `[93, 31, 229, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "processor", abi = "processor(bytes32)")]
    pub struct ProcessorCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `quoteDispatch` function with signature `quoteDispatch(uint32,bytes32,bytes,bytes,address)` and selector `[129, 210, 234, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "quoteDispatch",
        abi = "quoteDispatch(uint32,bytes32,bytes,bytes,address)"
    )]
    pub struct QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadataCall {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ethers::core::types::Bytes,
        pub metadata: ethers::core::types::Bytes,
        pub hook: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `quoteDispatch` function with signature `quoteDispatch(uint32,bytes32,bytes)` and selector `[156, 66, 189, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "quoteDispatch", abi = "quoteDispatch(uint32,bytes32,bytes)")]
    pub struct QuoteDispatchCall {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `quoteDispatch` function with signature `quoteDispatch(uint32,bytes32,bytes,bytes)` and selector `[247, 204, 211, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "quoteDispatch",
        abi = "quoteDispatch(uint32,bytes32,bytes,bytes)"
    )]
    pub struct QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadataCall {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ethers::core::types::Bytes,
        pub default_hook_metadata: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `recipientIsm` function with signature `recipientIsm(address)` and selector `[231, 15, 72, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "recipientIsm", abi = "recipientIsm(address)")]
    pub struct RecipientIsmCall {
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `requiredHook` function with signature `requiredHook()` and selector `[214, 208, 138, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "requiredHook", abi = "requiredHook()")]
    pub struct RequiredHookCall;
    #[doc = "Container type for all input parameters for the `setDefaultHook` function with signature `setDefaultHook(address)` and selector `[153, 176, 72, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDefaultHook", abi = "setDefaultHook(address)")]
    pub struct SetDefaultHookCall {
        pub hook: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setDefaultIsm` function with signature `setDefaultIsm(address)` and selector `[247, 148, 104, 122]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDefaultIsm", abi = "setDefaultIsm(address)")]
    pub struct SetDefaultIsmCall {
        pub module: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setRequiredHook` function with signature `setRequiredHook(address)` and selector `[20, 38, 183, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRequiredHook", abi = "setRequiredHook(address)")]
    pub struct SetRequiredHookCall {
        pub hook: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MailboxCalls {
        Version(VersionCall),
        DefaultHook(DefaultHookCall),
        DefaultIsm(DefaultIsmCall),
        Delivered(DeliveredCall),
        DeployedBlock(DeployedBlockCall),
        Dispatch2(Dispatch2Call),
        Dispatch1(Dispatch1Call),
        Dispatch0(Dispatch0Call),
        Initialize(InitializeCall),
        LatestDispatchedId(LatestDispatchedIdCall),
        LocalDomain(LocalDomainCall),
        Nonce(NonceCall),
        Owner(OwnerCall),
        Process(ProcessCall),
        ProcessedAt(ProcessedAtCall),
        Processor(ProcessorCall),
        QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadata(
            QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadataCall,
        ),
        QuoteDispatch(QuoteDispatchCall),
        QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadata(
            QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadataCall,
        ),
        RecipientIsm(RecipientIsmCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequiredHook(RequiredHookCall),
        SetDefaultHook(SetDefaultHookCall),
        SetDefaultIsm(SetDefaultIsmCall),
        SetRequiredHook(SetRequiredHookCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for MailboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Version(decoded));
            }
            if let Ok(decoded) =
                <DefaultHookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::DefaultHook(decoded));
            }
            if let Ok(decoded) =
                <DefaultIsmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::DefaultIsm(decoded));
            }
            if let Ok(decoded) =
                <DeliveredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Delivered(decoded));
            }
            if let Ok(decoded) =
                <DeployedBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::DeployedBlock(decoded));
            }
            if let Ok(decoded) =
                <Dispatch2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Dispatch2(decoded));
            }
            if let Ok(decoded) =
                <Dispatch1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Dispatch1(decoded));
            }
            if let Ok(decoded) =
                <Dispatch0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Dispatch0(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LatestDispatchedIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::LatestDispatchedId(decoded));
            }
            if let Ok(decoded) =
                <LocalDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::LocalDomain(decoded));
            }
            if let Ok(decoded) = <NonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Nonce(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <ProcessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Process(decoded));
            }
            if let Ok(decoded) =
                <ProcessedAtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::ProcessedAt(decoded));
            }
            if let Ok(decoded) =
                <ProcessorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::Processor(decoded));
            }
            if let Ok (decoded) = < QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadataCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (MailboxCalls :: QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadata (decoded)) }
            if let Ok(decoded) =
                <QuoteDispatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::QuoteDispatch(decoded));
            }
            if let Ok (decoded) = < QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadataCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (MailboxCalls :: QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadata (decoded)) }
            if let Ok(decoded) =
                <RecipientIsmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::RecipientIsm(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequiredHookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::RequiredHook(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultHookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::SetDefaultHook(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultIsmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::SetDefaultIsm(decoded));
            }
            if let Ok(decoded) =
                <SetRequiredHookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::SetRequiredHook(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MailboxCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MailboxCalls {
        fn encode(self) -> Vec<u8> {
            match self { MailboxCalls :: Version (element) => element . encode () , MailboxCalls :: DefaultHook (element) => element . encode () , MailboxCalls :: DefaultIsm (element) => element . encode () , MailboxCalls :: Delivered (element) => element . encode () , MailboxCalls :: DeployedBlock (element) => element . encode () , MailboxCalls :: Dispatch2 (element) => element . encode () , MailboxCalls :: Dispatch1 (element) => element . encode () , MailboxCalls :: Dispatch0 (element) => element . encode () , MailboxCalls :: Initialize (element) => element . encode () , MailboxCalls :: LatestDispatchedId (element) => element . encode () , MailboxCalls :: LocalDomain (element) => element . encode () , MailboxCalls :: Nonce (element) => element . encode () , MailboxCalls :: Owner (element) => element . encode () , MailboxCalls :: Process (element) => element . encode () , MailboxCalls :: ProcessedAt (element) => element . encode () , MailboxCalls :: Processor (element) => element . encode () , MailboxCalls :: QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadata (element) => element . encode () , MailboxCalls :: QuoteDispatch (element) => element . encode () , MailboxCalls :: QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadata (element) => element . encode () , MailboxCalls :: RecipientIsm (element) => element . encode () , MailboxCalls :: RenounceOwnership (element) => element . encode () , MailboxCalls :: RequiredHook (element) => element . encode () , MailboxCalls :: SetDefaultHook (element) => element . encode () , MailboxCalls :: SetDefaultIsm (element) => element . encode () , MailboxCalls :: SetRequiredHook (element) => element . encode () , MailboxCalls :: TransferOwnership (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for MailboxCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { MailboxCalls :: Version (element) => element . fmt (f) , MailboxCalls :: DefaultHook (element) => element . fmt (f) , MailboxCalls :: DefaultIsm (element) => element . fmt (f) , MailboxCalls :: Delivered (element) => element . fmt (f) , MailboxCalls :: DeployedBlock (element) => element . fmt (f) , MailboxCalls :: Dispatch2 (element) => element . fmt (f) , MailboxCalls :: Dispatch1 (element) => element . fmt (f) , MailboxCalls :: Dispatch0 (element) => element . fmt (f) , MailboxCalls :: Initialize (element) => element . fmt (f) , MailboxCalls :: LatestDispatchedId (element) => element . fmt (f) , MailboxCalls :: LocalDomain (element) => element . fmt (f) , MailboxCalls :: Nonce (element) => element . fmt (f) , MailboxCalls :: Owner (element) => element . fmt (f) , MailboxCalls :: Process (element) => element . fmt (f) , MailboxCalls :: ProcessedAt (element) => element . fmt (f) , MailboxCalls :: Processor (element) => element . fmt (f) , MailboxCalls :: QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadata (element) => element . fmt (f) , MailboxCalls :: QuoteDispatch (element) => element . fmt (f) , MailboxCalls :: QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadata (element) => element . fmt (f) , MailboxCalls :: RecipientIsm (element) => element . fmt (f) , MailboxCalls :: RenounceOwnership (element) => element . fmt (f) , MailboxCalls :: RequiredHook (element) => element . fmt (f) , MailboxCalls :: SetDefaultHook (element) => element . fmt (f) , MailboxCalls :: SetDefaultIsm (element) => element . fmt (f) , MailboxCalls :: SetRequiredHook (element) => element . fmt (f) , MailboxCalls :: TransferOwnership (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<VersionCall> for MailboxCalls {
        fn from(var: VersionCall) -> Self {
            MailboxCalls::Version(var)
        }
    }
    impl ::std::convert::From<DefaultHookCall> for MailboxCalls {
        fn from(var: DefaultHookCall) -> Self {
            MailboxCalls::DefaultHook(var)
        }
    }
    impl ::std::convert::From<DefaultIsmCall> for MailboxCalls {
        fn from(var: DefaultIsmCall) -> Self {
            MailboxCalls::DefaultIsm(var)
        }
    }
    impl ::std::convert::From<DeliveredCall> for MailboxCalls {
        fn from(var: DeliveredCall) -> Self {
            MailboxCalls::Delivered(var)
        }
    }
    impl ::std::convert::From<DeployedBlockCall> for MailboxCalls {
        fn from(var: DeployedBlockCall) -> Self {
            MailboxCalls::DeployedBlock(var)
        }
    }
    impl ::std::convert::From<Dispatch2Call> for MailboxCalls {
        fn from(var: Dispatch2Call) -> Self {
            MailboxCalls::Dispatch2(var)
        }
    }
    impl ::std::convert::From<Dispatch1Call> for MailboxCalls {
        fn from(var: Dispatch1Call) -> Self {
            MailboxCalls::Dispatch1(var)
        }
    }
    impl ::std::convert::From<Dispatch0Call> for MailboxCalls {
        fn from(var: Dispatch0Call) -> Self {
            MailboxCalls::Dispatch0(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MailboxCalls {
        fn from(var: InitializeCall) -> Self {
            MailboxCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<LatestDispatchedIdCall> for MailboxCalls {
        fn from(var: LatestDispatchedIdCall) -> Self {
            MailboxCalls::LatestDispatchedId(var)
        }
    }
    impl ::std::convert::From<LocalDomainCall> for MailboxCalls {
        fn from(var: LocalDomainCall) -> Self {
            MailboxCalls::LocalDomain(var)
        }
    }
    impl ::std::convert::From<NonceCall> for MailboxCalls {
        fn from(var: NonceCall) -> Self {
            MailboxCalls::Nonce(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for MailboxCalls {
        fn from(var: OwnerCall) -> Self {
            MailboxCalls::Owner(var)
        }
    }
    impl ::std::convert::From<ProcessCall> for MailboxCalls {
        fn from(var: ProcessCall) -> Self {
            MailboxCalls::Process(var)
        }
    }
    impl ::std::convert::From<ProcessedAtCall> for MailboxCalls {
        fn from(var: ProcessedAtCall) -> Self {
            MailboxCalls::ProcessedAt(var)
        }
    }
    impl ::std::convert::From<ProcessorCall> for MailboxCalls {
        fn from(var: ProcessorCall) -> Self {
            MailboxCalls::Processor(var)
        }
    }
    impl ::std::convert::From<QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadataCall>
        for MailboxCalls
    {
        fn from(var: QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadataCall) -> Self {
            MailboxCalls::QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadata(var)
        }
    }
    impl ::std::convert::From<QuoteDispatchCall> for MailboxCalls {
        fn from(var: QuoteDispatchCall) -> Self {
            MailboxCalls::QuoteDispatch(var)
        }
    }
    impl
        ::std::convert::From<
            QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadataCall,
        > for MailboxCalls
    {
        fn from(
            var: QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadataCall,
        ) -> Self {
            MailboxCalls :: QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadata (var)
        }
    }
    impl ::std::convert::From<RecipientIsmCall> for MailboxCalls {
        fn from(var: RecipientIsmCall) -> Self {
            MailboxCalls::RecipientIsm(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for MailboxCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            MailboxCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RequiredHookCall> for MailboxCalls {
        fn from(var: RequiredHookCall) -> Self {
            MailboxCalls::RequiredHook(var)
        }
    }
    impl ::std::convert::From<SetDefaultHookCall> for MailboxCalls {
        fn from(var: SetDefaultHookCall) -> Self {
            MailboxCalls::SetDefaultHook(var)
        }
    }
    impl ::std::convert::From<SetDefaultIsmCall> for MailboxCalls {
        fn from(var: SetDefaultIsmCall) -> Self {
            MailboxCalls::SetDefaultIsm(var)
        }
    }
    impl ::std::convert::From<SetRequiredHookCall> for MailboxCalls {
        fn from(var: SetRequiredHookCall) -> Self {
            MailboxCalls::SetRequiredHook(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for MailboxCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            MailboxCalls::TransferOwnership(var)
        }
    }
    #[doc = "Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `[255, 161, 173, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VersionReturn(pub u8);
    #[doc = "Container type for all return fields from the `defaultHook` function with signature `defaultHook()` and selector `[61, 18, 80, 183]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DefaultHookReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `defaultIsm` function with signature `defaultIsm()` and selector `[110, 95, 81, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DefaultIsmReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `delivered` function with signature `delivered(bytes32)` and selector `[228, 149, 241, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeliveredReturn(pub bool);
    #[doc = "Container type for all return fields from the `deployedBlock` function with signature `deployedBlock()` and selector `[130, 234, 123, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeployedBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `dispatch` function with signature `dispatch(uint32,bytes32,bytes,bytes,address)` and selector `[16, 184, 61, 192]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Dispatch2Return(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `dispatch` function with signature `dispatch(uint32,bytes32,bytes,bytes)` and selector `[72, 174, 232, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Dispatch1Return(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `[250, 49, 222, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Dispatch0Return(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `latestDispatchedId` function with signature `latestDispatchedId()` and selector `[19, 79, 187, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LatestDispatchedIdReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `localDomain` function with signature `localDomain()` and selector `[141, 54, 56, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LocalDomainReturn(pub u32);
    #[doc = "Container type for all return fields from the `nonce` function with signature `nonce()` and selector `[175, 254, 208, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NonceReturn(pub u32);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `processedAt` function with signature `processedAt(bytes32)` and selector `[7, 162, 253, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProcessedAtReturn(pub u64);
    #[doc = "Container type for all return fields from the `processor` function with signature `processor(bytes32)` and selector `[93, 31, 229, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProcessorReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `quoteDispatch` function with signature `quoteDispatch(uint32,bytes32,bytes,bytes,address)` and selector `[129, 210, 234, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct QuoteDispatchWithDestinationDomainAndRecipientAddressAndMetadataReturn {
        pub fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `quoteDispatch` function with signature `quoteDispatch(uint32,bytes32,bytes)` and selector `[156, 66, 189, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct QuoteDispatchReturn {
        pub fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `quoteDispatch` function with signature `quoteDispatch(uint32,bytes32,bytes,bytes)` and selector `[247, 204, 211, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct QuoteDispatchWithDestinationDomainAndRecipientAddressAndDefaultHookMetadataReturn {
        pub fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `recipientIsm` function with signature `recipientIsm(address)` and selector `[231, 15, 72, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RecipientIsmReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `requiredHook` function with signature `requiredHook()` and selector `[214, 208, 138, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RequiredHookReturn(pub ethers::core::types::Address);
}
