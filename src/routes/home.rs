use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;

pub fn index<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  let mut data = HashMap::<&str, &str>::new();
  data.insert("name", "Pepis");
  data.insert("title", "Home");
  return res.render("views/home/index.tpl", &data)
  //res.send("Home")
}