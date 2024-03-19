// Copyright 2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

use alloy_sol_types::sol_data::{ByteCount, SupportedFixedBytes};

use super::{AbiType, ConstString};

impl<const N: usize> AbiType for alloy_primitives::FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    type SolType = alloy_sol_types::sol_data::FixedBytes<N>;

    const ABI: ConstString = ConstString::new("bytes").concat(ConstString::from_decimal_number(N));
}
