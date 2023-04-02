use crate::{error::ServiceError, model::UserRepository, SESSION_KEY};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct LoginCredential {
    pub name: String,
    pub password: String,
}

#[utoipa::path(
    request_body = LoginCredential,
    responses(
        (status = 204, description = "Successfully logged in"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[post("/login")]
#[tracing::instrument(skip(pool, session))]
pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginCredential>,
    session: Session,
) -> crate::Result<impl Responder> {
    login_service(pool.as_ref(), body.into_inner(), session).await
}

async fn login_service(
    pool: &PgPool,
    LoginCredential { name, password }: LoginCredential,
    session: Session,
) -> crate::Result<impl Responder> {
    let user = pool
        .get_user_by_name(&name)
        .await
        .map_err(|_| ServiceError::UserNotFound)?;
    info!(user = ?user);
    verify_password(&password, &user.password_hash)?;

    session.renew();
    info!("Renew the session");
    session
        .insert(SESSION_KEY, user.name.clone())
        .expect("user name must be serializable");
    info!("Create a session for the user {}.", user.name);
    Ok(HttpResponse::NoContent().finish())
}

fn verify_password(password: &str, password_hash: &str) -> crate::Result<()> {
    bcrypt::verify(password, password_hash)
        .map_err(|_| ServiceError::InternalServerError)
        .and_then(|is_valid| {
            if is_valid {
                Ok(())
            } else {
                Err(ServiceError::WrongCredential)
            }
        })
}

#[cfg(test)]
mod tests {
    use actix_session::SessionExt;
    use actix_web::test::TestRequest;

    use crate::routes::signup::{signup_service, SignupCredential};

    use super::*;

    #[sqlx::test]
    async fn invalid_password(pool: PgPool) {
        // arrange
        let req = TestRequest::default().to_srv_request();
        let session = req.get_session();
        signup_service(
            &pool,
            SignupCredential {
                name: "ikanago".to_string(),
                password: "password".to_string(),
            },
            session.clone(),
        )
        .await
        .unwrap();

        // act
        let res = login_service(
            &pool,
            LoginCredential {
                name: "ikanago".to_string(),
                password: "xxxx".to_string(),
            },
            session,
        )
        .await;

        // assert
        assert!(matches!(res, Err(ServiceError::WrongCredential)));
    }
}
