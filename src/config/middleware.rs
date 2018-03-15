use nickel::{Request, Response, MiddlewareResult};

pub fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  println!("logging request from logger fn: {:?}", req.origin.uri);
  res.next_middleware()
}