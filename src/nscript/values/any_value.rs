use std::fmt::{Display, Error, Formatter};

use crate::nscript::{AnyType, FnName};

use super::*;

#[derive(Clone)]
pub enum AnyValue {
    // Value types
    Null,
    Integer(Integer),
    // Boolean(Boolean),
    // Number(Number),

    // Reference Types
    // Object(Object),
    // String(String),
    Function(Function),
    // Class(Class),
    // Ref(Box<AnyValue>),
    Type(AnyType),
}

impl AnyValue {
    pub fn get_store(&self) -> Store {
        match self {
            AnyValue::Null => Store::Value,
            AnyValue::Integer(value) => value.get_store(),
            AnyValue::Function(value) => value.get_store(),
            AnyValue::Type(type_) => Store::Value,
        }
    }

    pub fn get_type(&self) -> AnyType {
        match self {
            AnyValue::Null => AnyType::Null,
            AnyValue::Integer(_) => AnyType::Integer,
            AnyValue::Function(_) => AnyType::Function,
            AnyValue::Type(type_) => type_.clone(),
        }
    }

    pub fn satisfy(&self, type_: AnyType) -> bool {
        match self {
            AnyValue::Null => type_.is_null(),
            AnyValue::Integer(value) => value.satisfy(type_),
            AnyValue::Function(value) => value.satisfy(type_),
            AnyValue::Type(value) => value.is_assignable_to(type_),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, AnyValue::Null)
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, AnyValue::Integer(_))
    }

    // pub fn is_number(&self) -> bool {
    //     matches!(self, AnyValue::Number(_))
    // }

    // pub fn is_boolean(&self) -> bool {
    //     matches!(self, AnyValue::Boolean(_))
    // }

    pub fn is_function(&self) -> bool {
        matches!(self, AnyValue::Function(_))
    }

    // pub fn is_object(&self) -> bool {
    //     matches!(self, AnyValue::Object(_))
    // }

    // pub fn is_class(&self) -> bool {
    //     matches!(self, AnyValue::Class(_))
    // }

    // pub fn is_ref(&self) -> bool {
    //     matches!(self, AnyValue::Ref(_))
    // }

    pub fn is_type(&self) -> bool {
        matches!(self, AnyValue::Type(_))
    }

    pub fn into_null(self) -> Option<Null> {
        if let AnyValue::Null = self {
            Some(Null)
        } else {
            None
        }
    }

    pub fn into_integer(self) -> Option<Integer> {
        if let AnyValue::Integer(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_function(self) -> Option<Function> {
        if let AnyValue::Function(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_type(self) -> Option<AnyType> {
        if let AnyValue::Type(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl Display for AnyValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            AnyValue::Null => write!(f, "null"),
            AnyValue::Integer(_) => write!(f, "Integer"),
            // AnyValue::Number(_) => write!(f, "Number"),
            // AnyValue::Boolean(_) => write!(f, "Boolean"),
            AnyValue::Function(function) => {
                // write!(f, "fn {}(", function.name().to_string())?;
                if let FnName::Name(name) = function.name() {
                    write!(f, "fn {name}(")?;
                } else {
                    write!(f, "fn (")?;
                }
                let mut first = true;
                for (name, type_) in function.args() {
                    if first {
                        first = false
                    } else {
                        write!(f, ", ")?
                    }

                    write!(f, "{name}: {type_}")?
                }
                write!(f, ") -> {}", function.return_type())
            }
            // AnyValue::Class(class) => write!(f, "Class({})", class.name_or_default()),
            // AnyValue::Object(object) => write!(f, "Object({})", object.class().name_or_default()),
            // AnyValue::Ref(ref_) => write!(f, "ref {:?}", ref_.type_),
            AnyValue::Type(type_) => write!(f, "type {}", type_),
        }
    }
}
