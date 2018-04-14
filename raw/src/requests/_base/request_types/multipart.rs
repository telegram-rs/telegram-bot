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
macro_rules! multipart_field {
    ($result:expr, $field:ident($type:ident) => $val:expr, skip_if $cond:expr) => {{
        if $cond {
            multipart_field!($result, $field ($type) => $val);
        }
    }};

    ($result:expr, $field:ident($type:ident) => $val:expr, optional) => {{
        if $val.is_some() {
            multipart_field!($result, $field ($type) => $val.as_ref().unwrap());
        }
    }};

    ($result:expr, $field:ident(text) => $val:expr) => {{
        let value = MultipartValue::Text(format!("{}", $val));
        $result.push((stringify!($field).into(), value));
    }};

    ($result:expr, $field:ident(file) => $val:expr) => {{
        use std::ffi::OsStr;
        use std::path::Path;
        let filename = Path::new(&format!("{}", $val))
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
            .into();
        let value = MultipartValue::File {
            filename,
            path: format!("{}", $val),
        };
        $result.push((stringify!($field).into(), value));
    }};
}

#[macro_export]
macro_rules! multipart_type {
    ($c:ident => { $($t:tt)* }) => {
        impl ToMultipart for $c {
            multipart_type!(__fn_def, $($t)*);
        }
    };

    ($c:ident<$($params:tt),*> => { $($t:tt)* }) => {
        impl<$($params),*> ToMultipart for $c<$($params),*> {
            multipart_type!(__fn_def, $($t)*);
        }
    };

    (__fn_def, $( $field:ident : $e:tt $(, $k:ident = $v:expr )* ; )* ) => {
        fn to_multipart(&self) -> Multipart {
            use std::path::Path;
            let mut result = Vec::new();
            $(
                multipart_type!(__field, result, self, $field : $e $(, $k = $v )* );
            )*
            result
        }
    };

    (__field, $result:expr, $self:expr, $field:ident : text ,) => {{
        multipart_type!(__field, $result, $field : text, val = $self.$field);
    }};

    (__field, $result:expr, $self:expr, $field:ident : text, val = $e:expr , ) => {{
        let value = MultipartValue::Text(format!("{}", $e));
        multipart_type!(__set_val, $result, $field, value);
    }};

    (__set_val, $result:expr, $field:ident, $value:expr , ) => {{
        $result.push((stringify!($field).into(), $value));
    }};

    (__field, $result:expr, $self:expr, $field:ident : $t:ident, transform = $f:expr , $( $rest:tt )* ) => {{
        multipart_type!(__field, $result, $self, $field : $t, val =  ($f(&$self.$field)) $($rest)*);
    }};

    (__field, $result:expr, $self:expr, $field:ident : $t:ident, skip_if = $f:expr , $( $rest:tt )* ) => {{
        if !$f(&$self.$field) {
            multipart_type!(__field, $result, $self, $field : $t $($rest)*);
        }
    }};

    // (__field, $result:expr, $self:expr, $field:ident : text, transform = $func:expr) => {{
    //     let value = $func($self.$field);
    //     multipart_type!(__set_val, $result, $field, value);
    // }};

    // (__field, $result:expr, $self:expr, $field:ident : file) => {{
    //     let file_name = Path::new(self.$field).file_name.unwrap_or("").into();
    //     let value = MultipartValue::File(file_name, self.$field);
    //     multipart_type!(__set_val, $result, $field, value);
    // }};

    // (__field, $result:expr, $self:expr, $field:ident : calc $func:expr) => {{
    //     let value = MultipartValue::Text(format!("{}", $func($self)));
    //     multipart_type!(__set_val, $result, $field, value);
    // }};

    // (__field, $result:expr, $self:expr, $field:ident : $($def:tt)+, optional) => {{
    //     multipart_type!(__field, $result, $self, $field : $(def:tt)*, skip_if = Option::is_none);
    // }};

}
