use requests::*;
use types::*;

pub trait RequestType {
    type Options;
    type Request;

    fn serialize(options: Self::Options, request: &Self::Request) -> Result<HttpRequest, Error>;
}

pub trait ResponseType {
    type Type;

    fn deserialize(resp: HttpResponse) -> Result<Self::Type, Error>;
}

pub trait Request {
    type Type: RequestType;
    type Response: ResponseType + 'static;

    fn serialize(&self) -> Result<HttpRequest, Error>;

    fn detach(&self) -> DetachedRequest<Self::Response> {
        DetachedRequest {
            http_request: self.serialize(),
            phantom: ::std::marker::PhantomData,
        }
    }
}

impl<'a, Req: Request> Request for &'a Req {
    type Type = Req::Type;
    type Response = Req::Response;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        (*self).serialize()
    }
}

impl<'a, Req: Request> Request for &'a mut Req {
    type Type = Req::Type;
    type Response = Req::Response;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        (**self).serialize()
    }
}

pub struct DetachedRequest<Resp> {
    http_request: Result<HttpRequest, Error>,
    phantom: ::std::marker::PhantomData<Resp>,
}

impl<Resp: ResponseType + 'static> Request for DetachedRequest<Resp> {
    type Type = DetachedRequestType;
    type Response = Resp;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Ok(Self::Type::serialize((), &self.http_request)?)
    }
}

/// Use this trait to convert a complex type to corresponding request and send it to the chat.
pub trait ToRequest<'b> {
    /// Request type.
    type Request: Request;

    /// Convert type to request and send it to the chat.
    fn to_request<C>(&'b self, chat: C) -> Self::Request
    where
        C: ToChatRef;
}

/// Use this trait to convert a complex type to corresponding request and reply to the message.
pub trait ToReplyRequest<'b> {
    /// Request type.
    type Request: Request;

    /// Convert type to request and reply to the message.
    fn to_reply_request<M>(&'b self, message: M) -> Self::Request
    where
        M: ToMessageId + ToSourceChat;
}
