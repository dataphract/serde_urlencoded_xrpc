//! Serialization support for the `application/x-www-form-urlencoded` format.

mod value;

use form_urlencoded::Serializer as UrlEncodedSerializer;
use form_urlencoded::Target as UrlEncodedTarget;
use serde::ser;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::str;

/// Serializes a value into a `application/x-www-form-urlencoded` `String` buffer.
///
/// ```
/// use serde_derive::Serialize;
///
/// #[derive(Serialize)]
/// struct Meal {
///     bread: String,
///     cheese: String,
///     meat: String,
///     fat: String,
/// }
///
/// let meal = Meal {
///     bread: "baguette".into(),
///     cheese: "comté".into(),
///     meat: "ham".into(),
///     fat: "butter".into(),
/// };
///
/// assert_eq!(
///     serde_urlencoded_xrpc::to_string(meal),
///     Ok("bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter".to_owned()));
/// ```
pub fn to_string<T: ser::Serialize>(input: T) -> Result<String, Error> {
    let mut urlencoder = UrlEncodedSerializer::new("".to_owned());
    input.serialize(Serializer::new(&mut urlencoder))?;
    Ok(urlencoder.finish())
}

/// A serializer for the `application/x-www-form-urlencoded` format.
///
/// * Supported top-level inputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Supported keys and values are integers, bytes (if convertible to strings),
///   unit structs and unit variants.
///
/// * Newtype structs defer to their inner values.
pub struct Serializer<'input, 'output, Target: UrlEncodedTarget> {
    urlencoder: &'output mut UrlEncodedSerializer<'input, Target>,
}

impl<'input, 'output, Target: 'output + UrlEncodedTarget>
    Serializer<'input, 'output, Target>
{
    /// Returns a new `Serializer`.
    pub fn new(
        urlencoder: &'output mut UrlEncodedSerializer<'input, Target>,
    ) -> Self {
        Serializer { urlencoder }
    }
}

/// Errors returned during serializing to `application/x-www-form-urlencoded`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Custom(Cow<'static, str>),
    Utf8(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Custom(ref msg) => msg.fmt(f),
            Error::Utf8(ref err) => write!(f, "invalid UTF-8: {}", err),
        }
    }
}

impl error::Error for Error {
    /// The lower-level source of this error, in the case of a `Utf8` error.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Custom(_) => None,
            Error::Utf8(ref err) => Some(err),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg).into())
    }
}

/// Struct serializer.
pub struct StructSerializer<'input, 'output, Target: UrlEncodedTarget> {
    urlencoder: &'output mut UrlEncodedSerializer<'input, Target>,
}

impl<'input, 'output, Target> ser::Serializer
    for Serializer<'input, 'output, Target>
where
    Target: 'output + UrlEncodedTarget,
{
    type Ok = &'output mut UrlEncodedSerializer<'input, Target>;
    type Error = Error;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = StructSerializer<'input, 'output, Target>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    /// Returns an error.
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns `Ok`.
    fn serialize_unit(self) -> Result<Self::Ok, Error> {
        Ok(self.urlencoder)
    }

    /// Returns `Ok`.
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok, Error> {
        Ok(self.urlencoder)
    }

    /// Returns an error.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Serializes the inner value, ignoring the newtype name.
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(self)
    }

    /// Returns an error.
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Error> {
        Err(Error::top_level())
    }

    /// Returns `Ok`.
    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(self.urlencoder)
    }

    /// Serializes the given value.
    fn serialize_some<T: ?Sized + ser::Serialize>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(self)
    }

    /// Serialize a sequence, given length (if any) is ignored.
    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Error::top_level())
    }

    /// Returns an error.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Error::top_level())
    }

    /// Serializes a map, given length is ignored.
    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Error> {
        Err(Error::top_level())
    }

    /// Serializes a struct, given length is ignored.
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Ok(StructSerializer {
            urlencoder: self.urlencoder,
        })
    }

    /// Returns an error.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Error::top_level())
    }
}

impl<'input, 'output, Target> ser::SerializeStruct
    for StructSerializer<'input, 'output, Target>
where
    Target: 'output + UrlEncodedTarget,
{
    type Ok = &'output mut UrlEncodedSerializer<'input, Target>;
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        let value_ser = value::ValueSerializer::new(self.urlencoder, key);
        value.serialize(value_ser)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(self.urlencoder)
    }
}

impl Error {
    fn top_level() -> Self {
        let msg = "top-level serializer supports only structs";
        Error::Custom(msg.into())
    }
}
