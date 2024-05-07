use crate::application::controllers;
use crate::container::Container;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::Error;
use actix_web::{web, App};
pub fn create_app(
    container: Container,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .app_data(web::Data::from(container.user_service.clone()))
        .wrap(Logger::default())
        .service(
            web::scope("/auth")
                // ...so this handles requests for `GET /app/index.html`
                .route(
                    "/confirm_email",
                    web::post().to(controllers::user::confirm_email),
                ),
        )
}
