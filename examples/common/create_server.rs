use std::io::Error;

use actix_web::dev::Server;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;

use secured_cookie_middleware::SessionMiddleware;

use crate::common::app::Application;
use crate::common::constants::HOST;
use crate::common::constants::PORT;
use crate::common::constants::SESSION_COOKIE_KEY;
use crate::common::session_factory::SessionFactoryAbstract;

pub async fn create_server<F>(
    session_factory: Box<dyn SessionFactoryAbstract>,
    routes: F,
) -> Result<Server, Error>
where
    F: FnOnce(&mut web::ServiceConfig) + Clone + Send + 'static,
{
    let state = web::Data::new(Application::new(session_factory));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::<Application>::new(SESSION_COOKIE_KEY))
            .app_data(state.clone())
            .configure(routes.clone())
    })
    .bind((HOST.to_owned(), PORT))?
    .run();

    Ok(server)
}
