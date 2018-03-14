extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, StaticFilesHandler};

fn index<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  res.send("Hello World??")
}

fn main() {
  let mut server = Nickel::new();
  server.utilize(StaticFilesHandler::new("public/"));
  server.get("/", index);
  server.listen("127.0.0.1:6767").unwrap();
}
