//! Box id type
use std::convert::TryInto;
use std::io;

use ergotree_ir::ir_ergo_box::IrBoxId;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use super::super::digest32::Digest32;
use ergotree_ir::serialization::{
    sigma_byte_reader::SigmaByteRead, sigma_byte_writer::SigmaByteWrite, SerializationError,
    SigmaSerializable,
};
#[cfg(test)]
use proptest_derive::Arbitrary;

/// newtype for box ids
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BoxId(pub Digest32);

impl BoxId {
    /// Size in bytes
    pub const SIZE: usize = Digest32::SIZE;

    /// All zeros
    pub fn zero() -> BoxId {
        BoxId(Digest32::zero())
    }
}

impl From<Digest32> for BoxId {
    fn from(v: Digest32) -> Self {
        BoxId(v)
    }
}

#[cfg(feature = "json")]
impl From<BoxId> for String {
    fn from(v: BoxId) -> Self {
        v.0.into()
    }
}

impl From<&IrBoxId> for BoxId {
    fn from(irb: &IrBoxId) -> Self {
        let u8bytes: Vec<u8> = irb.0.iter().map(|b| *b as u8).collect();
        let arr: [u8; Digest32::SIZE] = u8bytes.as_slice().try_into().unwrap();
        BoxId(arr.into())
    }
}

impl From<BoxId> for IrBoxId {
    fn from(id: BoxId) -> Self {
        let i8bytes: Vec<i8> = id.0 .0.iter().map(|b| *b as i8).collect();
        IrBoxId::new(i8bytes.try_into().unwrap())
    }
}

impl SigmaSerializable for BoxId {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> Result<(), io::Error> {
        self.0.sigma_serialize(w)?;
        Ok(())
    }
    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SerializationError> {
        Ok(Self(Digest32::sigma_parse(r)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ergotree_ir::serialization::sigma_serialize_roundtrip;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn ser_roundtrip(v in any::<BoxId>()) {
            prop_assert_eq![sigma_serialize_roundtrip(&v), v];
        }
    }
}
