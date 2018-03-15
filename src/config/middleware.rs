use nickel::{Request, Response, MiddlewareResult};

pub fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  println!("Log-> uri: {:?}, method: {:?}", req.origin.uri, req.origin.method);
  res.next_middleware()
}