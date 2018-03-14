use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

pub fn index<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  res.send("Home")
}