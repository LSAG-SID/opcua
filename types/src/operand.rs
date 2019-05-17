use std::convert::TryFrom;

use crate::{
    ExtensionObject, DecodingLimits, Variant,
    service_types::{
        ElementOperand, LiteralOperand, AttributeOperand, SimpleAttributeOperand,
        ContentFilterElement, FilterOperator,
    },
    status_code::StatusCode,
    node_ids::ObjectId,
};

#[derive(PartialEq)]
pub enum OperandType {
    ElementOperand,
    LiteralOperand,
    AttributeOperand,
    SimpleAttributeOperand,
}

pub enum Operand {
    ElementOperand(ElementOperand),
    LiteralOperand(LiteralOperand),
    AttributeOperand(AttributeOperand),
    SimpleAttributeOperand(SimpleAttributeOperand),
}

impl From<i8> for Operand {
    fn from(v: i8) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<u8> for Operand {
    fn from(v: u8) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<i16> for Operand {
    fn from(v: i16) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<u16> for Operand {
    fn from(v: u16) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<i32> for Operand {
    fn from(v: i32) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<u32> for Operand {
    fn from(v: u32) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<f32> for Operand {
    fn from(v: f32) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<f64> for Operand {
    fn from(v: f64) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<bool> for Operand {
    fn from(v: bool) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl From<&str> for Operand {
    fn from(v: &str) -> Self {
        Operand::LiteralOperand(LiteralOperand { value: Variant::from(v) })
    }
}

impl TryFrom<&ExtensionObject> for Operand {
    type Error = StatusCode;

    fn try_from(v: &ExtensionObject) -> Result<Self, Self::Error> {
        let object_id = v.object_id().map_err(|_| StatusCode::BadFilterOperandInvalid)?;
        let decoding_limits = DecodingLimits::default();
        let operand = match object_id {
            ObjectId::ElementOperand_Encoding_DefaultBinary =>
                Operand::ElementOperand(v.decode_inner::<ElementOperand>(&decoding_limits)?),
            ObjectId::LiteralOperand_Encoding_DefaultBinary =>
                Operand::LiteralOperand(v.decode_inner::<LiteralOperand>(&decoding_limits)?),
            ObjectId::AttributeOperand_Encoding_DefaultBinary =>
                Operand::AttributeOperand(v.decode_inner::<AttributeOperand>(&decoding_limits)?),
            ObjectId::SimpleAttributeOperand_Encoding_DefaultBinary =>
                Operand::SimpleAttributeOperand(v.decode_inner::<SimpleAttributeOperand>(&decoding_limits)?),
            _ => {
                return Err(StatusCode::BadFilterOperandInvalid);
            }
        };
        Ok(operand)
    }
}

impl From<&Operand> for ExtensionObject {
    fn from(v: &Operand) -> Self {
        match v {
            &Operand::ElementOperand(ref op) => ExtensionObject::from_encodable(ObjectId::ElementOperand_Encoding_DefaultBinary, op),
            &Operand::LiteralOperand(ref op) => ExtensionObject::from_encodable(ObjectId::LiteralOperand_Encoding_DefaultBinary, op),
            &Operand::AttributeOperand(ref op) => ExtensionObject::from_encodable(ObjectId::AttributeOperand_Encoding_DefaultBinary, op),
            &Operand::SimpleAttributeOperand(ref op) => ExtensionObject::from_encodable(ObjectId::SimpleAttributeOperand_Encoding_DefaultBinary, op),
        }
    }
}

impl From<(FilterOperator, Vec<Operand>)> for ContentFilterElement {
    fn from(v: (FilterOperator, Vec<Operand>)) -> ContentFilterElement {
        ContentFilterElement {
            filter_operator: v.0,
            filter_operands: Some(v.1.iter().map(|op| op.into()).collect()),
        }
    }
}

impl Operand {
    pub fn operand_type(&self) -> OperandType {
        match self {
            &Operand::ElementOperand(_) => OperandType::ElementOperand,
            &Operand::LiteralOperand(_) => OperandType::LiteralOperand,
            &Operand::AttributeOperand(_) => OperandType::AttributeOperand,
            &Operand::SimpleAttributeOperand(_) => OperandType::SimpleAttributeOperand
        }
    }

    pub fn is_element(&self) -> bool {
        self.operand_type() == OperandType::ElementOperand
    }

    pub fn is_literal(&self) -> bool {
        self.operand_type() == OperandType::LiteralOperand
    }

    pub fn is_attribute(&self) -> bool {
        self.operand_type() == OperandType::AttributeOperand
    }

    pub fn is_simple_attribute(&self) -> bool {
        self.operand_type() == OperandType::SimpleAttributeOperand
    }
}