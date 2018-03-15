use nickel::{Request, Response, MiddlewareResult};

pub fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  println!("Log-> uri: {:?}, method: {:?}", req.origin.uri.to_string(), req.origin.method.to_string());
  res.next_middleware()
}

pub fn set_headers<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  res.next_middleware()
}