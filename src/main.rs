extern crate nickel;
mod routes;
mod config;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, StaticFilesHandler};

fn hello<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  res.send("Hello World??")
}

fn main() {
  let mut server = Nickel::new();
  //archivos estáticos
  server.utilize(StaticFilesHandler::new("public/"));
  //middleware personalizado
  server.utilize(config::middleware::set_headers);
  server.utilize(config::middleware::logger_fn);
  //handlers de rutas
  server.get("/hello", hello);
  server.get("/", routes::home::index);
  //arrancando el servicio
  server.listen("127.0.0.1:6767").unwrap();
}
