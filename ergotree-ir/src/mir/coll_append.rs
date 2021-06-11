use crate::serialization::op_code::OpCode;
use crate::serialization::sigma_byte_reader::SigmaByteRead;
use crate::serialization::sigma_byte_writer::SigmaByteWrite;
use crate::serialization::SerializationError;
use crate::serialization::SigmaSerializable;
// use crate::types::stuple::STuple;
use crate::types::stype::SType;


use super::expr::Expr;
use super::expr::InvalidArgumentError;
use crate::has_opcode::HasStaticOpCode;

/// Takes two collections as input and concatenates both into the Append Colelction
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Append {
    /// Collection - First Parameter; first half of the combined collection
    pub input: Box<Expr>,
    /// Collection - Second Parameter; later half of the combined collection
    pub output: Box<Expr>,
}

impl Append {
    /// Create new object, returns an error if any of the requirements failed
    pub fn new(input: Expr, output: Expr) -> Result<Self, InvalidArgumentError> {
        let first_elem_type : SType = input.post_eval_tpe();
        let second_elem_type : SType = output.post_eval_tpe();
        match (first_elem_type, second_elem_type) {
            (SType::SColl(x), SType::SColl(y)) => {
                if x == y {
                    Ok(Append{input: input.into(), output: output.into()})
                } else {
                    Err(InvalidArgumentError(format!(
                        "Expected Append input and output collection to have the same types; got input={0:?} output={1:?}",
                        x, y)))
                }
            }
            (SType::SColl(_), _) => {
                Err(InvalidArgumentError(format!(
                    "Expected Append output param to be a collection; got output={:?}", output.tpe())))
            }
            (_, SType::SColl(_)) => {
                Err(InvalidArgumentError(format!(
                    "Expected Append input param to be a collection; got input={:?}", input.tpe())))   
            },
            (_, _) => {
                Err(InvalidArgumentError(format!(
                    "Expected Append input and output param to be a collection; got input={:?} output={:?}", input.tpe(), output.tpe())))   
            }
        }
    }

    /// Type
    pub fn tpe(&self) -> SType {
        // Type is supposed to be the same on input and output
        // Append::new checks types but later modifications are unchecked
        // return type of first element
        self.input.tpe()
    }
}

impl HasStaticOpCode for Append {
    // OpCode Append is already defined by previous programer; static number 67 assigned
    const OP_CODE: OpCode = OpCode::APPEND;
}

impl SigmaSerializable for Append {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> Result<(), std::io::Error> {
        self.input.sigma_serialize(w)?;
        self.output.sigma_serialize(w)?;
        Ok(())
    }

    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SerializationError> {
        let input = Expr::sigma_parse(r)?.into();
        let output = Expr::sigma_parse(r)?.into();
        // InvalidArgumentError can create SerializationError; no map_err needed
        Ok(Append::new(input, output)?)
    }
}

#[cfg(test)]
#[cfg(feature = "arbitrary")]
mod tests {
    use super::*;
    use crate::mir::expr::Expr;
    use crate::mir::expr::arbitrary::ArbExprParams;
    use crate::serialization::sigma_serialize_roundtrip;
    use proptest::prelude::*;

    impl Arbitrary for Append {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                any_with::<Expr>(ArbExprParams {
                    tpe: SType::SColl(SType::SBoolean.into()),
                    depth: 1,
                }),
                any_with::<Expr>(ArbExprParams {
                    tpe: SType::SColl(SType::SBoolean.into()),
                    depth: 1,
                }),
            )                
                .prop_map(|(input, output)| Self {
                    input: input.into(),
                    output: output.into(),
                })
                .boxed()
        }
    }

    proptest! {

        #![proptest_config(ProptestConfig::with_cases(16))]

        #[test]
        fn ser_roundtrip(v in any::<Append>()) {
            let expr: Expr = v.into();
            prop_assert_eq![sigma_serialize_roundtrip(&expr), expr];
        }
    }
}
