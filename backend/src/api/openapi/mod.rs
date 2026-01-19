mod modifiers;

use utoipa::OpenApi;

use crate::{
	api::openapi::modifiers::SecurityAddon,
	dtos::{AdminDto, PaginatedResult},
};

use self::modifiers::BaseRoute;

#[derive(OpenApi)]
#[openapi(
    // servers((url = "http://localhost:3000", description = "local server")),
    paths(
        super::routes::admin::create::admin_create,
        super::routes::admin::get_list::admin_get_list,
        super::routes::admin::get_one::admin_get_one,
        super::routes::admin::update::admin_update,
        super::routes::admin::delete::admin_delete,
        super::routes::auth::login::auth_login,
        super::routes::auth::login_web::auth_login_web,
        super::routes::auth::logout::auth_logout,
    ),
    components(
        schemas(
			AdminDto,
			PaginatedResult<AdminDto>,
        ),
        responses(),
    ),
    modifiers(&BaseRoute, &SecurityAddon),
    tags(
        (name = "Auth", description = "Auth"),
        (name = "Admin", description = "Admin"),
    ),
)]
pub struct ApiDoc;
