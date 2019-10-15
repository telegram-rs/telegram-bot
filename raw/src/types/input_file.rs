use bytes::Bytes;

use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InputFile(InputFileImpl);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum InputFileImpl {
    Ref(Text),
    Path { path: Text, file_name: Option<Text> },
    Data { file_name: Text, data: Bytes },
}

impl InputFile {
    pub fn with_ref(r: impl Into<Text>) -> Self {
        InputFile(InputFileImpl::Ref(r.into()))
    }

    pub fn with_path(path: impl Into<Text>) -> Self {
        InputFile(InputFileImpl::Path {
            path: path.into(),
            file_name: None,
        })
    }

    pub fn with_path_and_name(path: impl Into<Text>, file_name: impl Into<Text>) -> Self {
        InputFile(InputFileImpl::Path {
            path: path.into(),
            file_name: Some(file_name.into()),
        })
    }

    pub fn with_data(data: impl Into<Bytes>, file_name: impl Into<Text>) -> Self {
        InputFile(InputFileImpl::Data {
            file_name: file_name.into(),
            data: data.into(),
        })
    }
}

impl From<FileRef> for InputFile {
    fn from(value: FileRef) -> Self {
        InputFile::with_ref(value.inner)
    }
}

impl<'a> From<&'a FileRef> for InputFile {
    fn from(value: &'a FileRef) -> Self {
        InputFile::with_ref(value.inner.as_str())
    }
}

impl ToMultipartValue for InputFile {
    fn to_multipart_value(&self) -> MultipartValue {
        match &self.0 {
            InputFileImpl::Ref(r) => MultipartValue::Text(r.clone()),
            InputFileImpl::Path { path, file_name } => MultipartValue::Path {
                path: path.clone(),
                file_name: file_name.clone(),
            },
            InputFileImpl::Data { file_name, data } => MultipartValue::Data {
                file_name: file_name.clone(),
                data: data.clone(),
            },
        }
    }
}
