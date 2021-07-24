use actix::MailboxError;
use actix_web::HttpResponse;
use cobase_core::Error;
use serde::Serialize;
use std::convert::Into;

#[derive(Serialize)]
pub struct Ok {
    pub success: bool,
}

#[derive(Serialize)]
pub struct ResponseError<C: Into<String>, M: Into<String>> {
    code: C,
    message: M,
}

pub struct Response<S: Serialize>(pub Result<Result<S, Error>, MailboxError>);

impl<S: Serialize> Into<HttpResponse> for Response<S> {
    fn into(self) -> HttpResponse {
        let res = self.0.unwrap_or_else(|e| Err(e.into()));

        match res {
            Ok(resp) => HttpResponse::Ok().json(resp),
            Err(err) => match err {
                Error::Unkown(e) => {
                    error!("{}", e);

                    HttpResponse::InternalServerError().json(ResponseError {
                        code: "Internal Server Error",
                        message: "Oops something went wrong, Please try again later.",
                    })
                }
                Error::ValidationErrors(_) => {
                    HttpResponse::BadRequest().json(ResponseError {
                        code: "Bad Request",
                        message: err.to_string(),
                    })
                },
            },
        }
    }
}

pub struct CommandResponse(pub Result<Result<(), Error>, MailboxError>);

impl Into<HttpResponse> for CommandResponse {
    fn into(self) -> HttpResponse {
        Response(self.0.map(|res| res.map(|_| Ok { success: true }))).into()
    }
}
