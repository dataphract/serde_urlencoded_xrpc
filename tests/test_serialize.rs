use serde_derive::Serialize;

#[derive(Serialize)]
struct NewType<T>(T);

#[test]
fn serialize_newtype_i32() {
    #[derive(Serialize)]
    struct NamedStruct<T> {
        field: T,
    }

    let params = NamedStruct {
        field: Some(NewType(11)),
    };
    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok("field=11".to_owned())
    );
}

#[test]
fn serialize_newtype_u128() {
    #[derive(Serialize)]
    struct NamedStruct<T> {
        field: T,
    }

    let params = NamedStruct {
        field: Some(NewType(u128::MAX)),
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok(format!("field={}", u128::MAX))
    );
}

#[test]
fn serialize_newtype_i128() {
    #[derive(Serialize)]
    struct NamedStruct<T> {
        field: T,
    }

    let params = NamedStruct {
        field: Some(NewType(i128::MIN)),
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok(format!("field={}", i128::MIN))
    );
}

#[test]
fn serialize_option_map_int() {
    #[derive(Serialize)]
    struct NamedStruct {
        first: Option<u32>,
        middle: Option<u32>,
        last: Option<u32>,
    }

    let params = NamedStruct {
        first: Some(23),
        middle: None,
        last: Some(42),
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok("first=23&last=42".to_owned())
    );
}

#[test]
fn serialize_option_map_string() {
    #[derive(Serialize)]
    struct NamedStruct {
        first: Option<&'static str>,
        middle: Option<&'static str>,
        last: Option<&'static str>,
    }

    let params = NamedStruct {
        first: Some("hello"),
        middle: None,
        last: Some("world"),
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok("first=hello&last=world".to_owned())
    );
}

#[test]
fn serialize_option_map_bool() {
    #[derive(Serialize)]
    struct NamedStruct {
        one: Option<bool>,
        two: Option<bool>,
    }

    let params = NamedStruct {
        one: Some(true),
        two: Some(false),
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok("one=true&two=false".to_owned())
    );
}

#[test]
fn serialize_map_bool() {
    #[derive(Serialize)]
    struct NamedStruct {
        one: bool,
        two: bool,
    }

    let params = NamedStruct {
        one: true,
        two: false,
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok("one=true&two=false".to_owned())
    );
}

#[derive(Serialize)]
enum X {
    A,
    B,
    C,
}

#[test]
fn serialize_unit_enum() {
    #[derive(Serialize)]
    struct NamedStruct {
        one: X,
        two: X,
        three: X,
    }

    let params = NamedStruct {
        one: X::A,
        two: X::B,
        three: X::C,
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(params),
        Ok("one=A&two=B&three=C".to_owned())
    );
}

#[derive(Serialize)]
struct Unit;

#[test]
fn serialize_unit_struct() {
    assert_eq!(serde_urlencoded_xrpc::to_string(Unit), Ok("".to_owned()));
}

#[test]
fn serialize_unit_type() {
    assert_eq!(serde_urlencoded_xrpc::to_string(()), Ok("".to_owned()));
}

#[derive(Serialize)]
struct ListStruct<T> {
    x: Vec<T>,
    y: Vec<T>,
}

#[test]
fn serialize_struct_vec() {
    let list_struct = ListStruct {
        x: vec![1, 2, 3],
        y: vec![4, 5, 6],
    };

    assert_eq!(
        serde_urlencoded_xrpc::to_string(list_struct),
        Ok("x=1&x=2&x=3&y=4&y=5&y=6".into()),
    );
}
