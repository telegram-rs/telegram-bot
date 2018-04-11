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
