use bytes::Bytes;

use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InputFile(InputFileImpl);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InputFileRef(Text);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InputFileUpload(InputFileUploadImpl);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum InputFileImpl {
    Ref(Text),
    Path { path: Text, file_name: Option<Text> },
    Data { file_name: Text, data: Bytes },
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum InputFileUploadImpl {
    Path { path: Text, file_name: Option<Text> },
    Data { file_name: Text, data: Bytes },
}

impl InputFileRef {
    pub fn new(r: impl Into<Text>) -> Self {
        InputFileRef(r.into())
    }
}

impl InputFileUpload {
    pub fn with_path(path: impl Into<Text>) -> Self {
        InputFileUpload(InputFileUploadImpl::Path {
            path: path.into(),
            file_name: None,
        })
    }

    pub fn with_data(data: impl Into<Bytes>, file_name: impl Into<Text>) -> Self {
        InputFileUpload(InputFileUploadImpl::Data {
            file_name: file_name.into(),
            data: data.into(),
        })
    }

    pub fn file_name(&self, new_file_name: impl Into<Text>) -> Self {
        let mut this = self.clone();
        match &mut this.0 {
            InputFileUploadImpl::Path { file_name, .. } => *file_name = Some(new_file_name.into()),
            InputFileUploadImpl::Data { file_name, .. } => *file_name = new_file_name.into(),
        };
        this
    }
}

impl From<FileRef> for InputFile {
    fn from(value: FileRef) -> Self {
        InputFile::from(InputFileRef::from(value))
    }
}

impl<'a> From<&'a FileRef> for InputFile {
    fn from(value: &'a FileRef) -> Self {
        InputFile::from(InputFileRef::from(value))
    }
}

impl<'a> From<&'a mut FileRef> for InputFile {
    fn from(value: &'a mut FileRef) -> Self {
        InputFile::from(InputFileRef::from(value))
    }
}

impl From<FileRef> for InputFileRef {
    fn from(value: FileRef) -> Self {
        InputFileRef::new(value.inner)
    }
}

impl<'a> From<&'a FileRef> for InputFileRef {
    fn from(value: &'a FileRef) -> Self {
        InputFileRef::new(value.inner.as_str())
    }
}

impl<'a> From<&'a mut FileRef> for InputFileRef {
    fn from(value: &'a mut FileRef) -> Self {
        InputFileRef::new(value.inner.as_str())
    }
}

impl<'a> From<&'a InputFile> for InputFile {
    fn from(value: &'a InputFile) -> Self {
        value.clone()
    }
}

impl<'a> From<&'a mut InputFile> for InputFile {
    fn from(value: &'a mut InputFile) -> Self {
        value.clone()
    }
}

impl From<InputFileRef> for InputFile {
    fn from(value: InputFileRef) -> Self {
        InputFile(InputFileImpl::Ref(value.0))
    }
}

impl<'a> From<&'a InputFileRef> for InputFile {
    fn from(value: &'a InputFileRef) -> Self {
        InputFile(InputFileImpl::Ref(value.0.clone()))
    }
}

impl<'a> From<&'a mut InputFileRef> for InputFile {
    fn from(value: &'a mut InputFileRef) -> Self {
        InputFile(InputFileImpl::Ref(value.0.clone()))
    }
}

impl From<InputFileUpload> for InputFile {
    fn from(value: InputFileUpload) -> Self {
        InputFile(match value.0 {
            InputFileUploadImpl::Path { path, file_name } => {
                InputFileImpl::Path { path, file_name }
            }
            InputFileUploadImpl::Data { data, file_name } => {
                InputFileImpl::Data { data, file_name }
            }
        })
    }
}

impl<'a> From<&'a InputFileUpload> for InputFile {
    fn from(value: &'a InputFileUpload) -> Self {
        InputFile(match &value.0 {
            InputFileUploadImpl::Path { path, file_name } => InputFileImpl::Path {
                path: path.clone(),
                file_name: file_name.clone(),
            },
            InputFileUploadImpl::Data { data, file_name } => InputFileImpl::Data {
                data: data.clone(),
                file_name: file_name.clone(),
            },
        })
    }
}

impl<'a> From<&'a mut InputFileUpload> for InputFile {
    fn from(value: &'a mut InputFileUpload) -> Self {
        InputFile(match &value.0 {
            InputFileUploadImpl::Path { path, file_name } => InputFileImpl::Path {
                path: path.clone(),
                file_name: file_name.clone(),
            },
            InputFileUploadImpl::Data { data, file_name } => InputFileImpl::Data {
                data: data.clone(),
                file_name: file_name.clone(),
            },
        })
    }
}

impl<'a> From<&'a InputFileRef> for InputFileRef {
    fn from(value: &'a InputFileRef) -> Self {
        value.clone()
    }
}

impl<'a> From<&'a mut InputFileRef> for InputFileRef {
    fn from(value: &'a mut InputFileRef) -> Self {
        value.clone()
    }
}

impl<'a> From<&'a InputFileUpload> for InputFileUpload {
    fn from(value: &'a InputFileUpload) -> Self {
        value.clone()
    }
}

impl<'a> From<&'a mut InputFileUpload> for InputFileUpload {
    fn from(value: &'a mut InputFileUpload) -> Self {
        value.clone()
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
