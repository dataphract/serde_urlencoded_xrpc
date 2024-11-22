use crate::ser::Error;

use form_urlencoded::Serializer as UrlEncodedSerializer;
use form_urlencoded::Target as UrlEncodedTarget;
use serde::ser::{Impossible, Serialize, SerializeSeq};
use serde::Serializer;
use std::str;

pub struct ValueSerializer<'input, 'key, 'target, Target>
where
    Target: UrlEncodedTarget,
{
    urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
    allow_seq: bool,
    key: &'key str,
}

impl<'input, 'key, 'target, Target>
    ValueSerializer<'input, 'key, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    pub fn new(
        urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
        key: &'key str,
    ) -> Self {
        ValueSerializer {
            urlencoder,
            allow_seq: true,
            key,
        }
    }
}

impl<'input, 'key, 'target, Target> Serializer
    for ValueSerializer<'input, 'key, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
    'target: 'key,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ValueSeqSerializer<'input, 'key, 'target, Target>;
    type SerializeTuple = Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;
    type SerializeMap = Impossible<Self::Ok, Error>;
    type SerializeStruct = Impossible<Self::Ok, Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Error> {
        self.serialize_str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Error> {
        self.serialize_floating(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Error> {
        self.serialize_floating(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Error> {
        self.collect_str(&v)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Error> {
        self.urlencoder.append_pair(self.key, value);
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Error> {
        match str::from_utf8(value) {
            Ok(value) => self.serialize_str(value),
            Err(err) => Err(Error::Utf8(err)),
        }
    }

    fn serialize_unit(self) -> Result<Self::Ok, Error> {
        Err(self.unsupported("unit"))
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> Result<Self::Ok, Error> {
        self.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Error> {
        Err(self.unsupported("newtype variant"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(self)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Error> {
        if self.allow_seq {
            Ok(ValueSeqSerializer { inner: self })
        } else {
            Err(self.unsupported("sequence"))
        }
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Error> {
        Err(self.unsupported("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Error> {
        Err(self.unsupported("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(self.unsupported("tuple variant"))
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Error> {
        Err(self.unsupported("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Err(self.unsupported("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(self.unsupported("struct variant"))
    }
}

impl<'input, 'key, 'target, Target>
    ValueSerializer<'input, 'key, 'target, Target>
where
    Target: UrlEncodedTarget,
{
    fn serialize_integer<I>(self, value: I) -> Result<(), Error>
    where
        I: itoa::Integer,
    {
        let mut buf = itoa::Buffer::new();
        let part = buf.format(value);
        Serializer::serialize_str(self, part)
    }

    fn serialize_floating<F>(self, value: F) -> Result<(), Error>
    where
        F: ryu::Float,
    {
        let mut buf = ryu::Buffer::new();
        let part = buf.format(value);
        Serializer::serialize_str(self, part)
    }

    fn unsupported(self, type_str: &'static str) -> Error {
        Error::Custom(format!("unsupported value type: {type_str}").into())
    }
}

/// Sequence value serializer.
pub struct ValueSeqSerializer<'input, 'key, 'target, Target: UrlEncodedTarget> {
    inner: ValueSerializer<'input, 'key, 'target, Target>,
}

impl<'input, 'output, 'target, Target> SerializeSeq
    for ValueSeqSerializer<'input, 'output, 'target, Target>
where
    Target: 'output + UrlEncodedTarget,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        let ser = ValueSerializer {
            urlencoder: self.inner.urlencoder,
            allow_seq: false,
            key: self.inner.key,
        };

        value.serialize(ser)
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(())
    }
}
