use requests::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct MultipartRequestType<Request> {
    phantom: ::std::marker::PhantomData<Request>,
}

pub trait ToMultipart {
    fn to_multipart(&self) -> Multipart;
}

impl<Request: ToMultipart> RequestType for MultipartRequestType<Request> {
    type Options = RequestUrl;
    type Request = Request;

    fn serialize(url: Self::Options, request: &Self::Request) -> Result<HttpRequest, Error> {
        let multipart = request.to_multipart();

        Ok(HttpRequest {
            url: url,
            method: Method::Post,
            body: Body::Multipart(multipart),
        })
    }
}

#[macro_export]
macro_rules! multipart_map {
    ($self:expr, $( ( $($opts:tt)* ) ; )*) => {
        let mut result = Vec::new();
        $(
            multipart_field!($self, result, $($opts)*);
        )*
        result
    }
}

macro_rules! multipart_field {
    ($self:expr, $result:expr, $field:ident($type:ident)) => {{
        let value = &$self.$field;
        multipart_field!($self, $result, $field ($type) => value);
    }};

    ($self:expr, $result:expr, $field:ident($type:ident), $($t:tt)*) => {{
        let value = &$self.$field;
        multipart_field!($self, $result, $field ($type) => value, $($t)*);
    }};

    ($self:expr, $result:expr, $field:ident($type:ident) => $val:expr,skip_if $cond:expr) => {{
        if *$cond {
            multipart_field!($self, $result, $field ($type) => $val);
        }
    }};

    ($self:expr, $result:expr, $field:ident($type:ident) => $val:expr,optional) => {{
        let value = $val.as_ref();
        if value.is_some() {
            multipart_field!($self, $result, $field ($type) => value.unwrap());
        }
    }};

    ($self:expr, $result:expr, $field:ident($type:ident) => $val:expr,when_true) => {{
        let value = $val;
        multipart_field!($self, $result, $field ($type) => value, skip_if value);
    }};

    ($self:expr, $result:expr, $field:ident(text) => $val:expr) => {{
        let value = MultipartValue::Text($val.to_string());
        $result.push((stringify!($field).into(), value));
    }};

    ($self:expr, $result:expr, $field:ident(json) => $val:expr) => {{
        let stringified = ::serde_json::to_string($val).unwrap();
        let value = MultipartValue::Text(stringified);
        $result.push((stringify!($field).into(), value));
    }};

    ($self:expr, $result:expr, $field:ident(file) => $val:expr) => {{
        use std::ffi::OsStr;
        use std::path::Path;
        let file_name = Path::new(&$val.to_string())
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .into();
        let value = MultipartValue::File {
            file_name,
            path: $val.to_string(),
        };
        $result.push((stringify!($field).into(), value));
    }};

    ($self:expr, $result:expr, $field:ident(raw) => $val:expr) => {{
        let value = $val.clone();
        $result.push((stringify!($field).into(), value));
    }};
}
