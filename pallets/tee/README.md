# pallet-tee

A pallet for TREX that acts as a verified registry for SGX enclaves as a TEE. 
Its goal is to provide public auditability of remote attestation of SGX enclaves. Given deterministic builds of enclave code, 
this pallet closes the trust gap from source code to the MRENCLAVE of an enclave running on a genuine Intel SGX platform. 
Without the need for a license with Intel, everyone can verify what code is executed by registered service providers and that it is executed with confidentiality. 
A blockchain that integrates this pallet will, therefore, act as a public registry of remote attested services.

## IAS verify

A helper crate that verifies IAS report certificates against Intel's root CA (hard-coded).
It also parses IAs reports and extracts information for filtering and registering by pallet-tee

