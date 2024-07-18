use axum::{
    body::Body,
    http::{header::SET_COOKIE, HeaderValue, Response, StatusCode},
};
use bcrypt::{BcryptError, DEFAULT_COST};
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use crate::{
    handlers::user::{
        Claims, SignInError, SignInRequest, SignUpError, SignUpRequest, SignUpRequestModel, User,
    },
    infra::errors::{adapt_infra_error, InfraError},
    schema::users,
};

pub async fn sign_in(
    pool: &deadpool_diesel::postgres::Pool,
    sign_in_json: SignInRequest,
) -> Result<impl Into<Response<Body>>, SignInError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;

    let user: User = conn
        .interact(move |conn| {
            users::table
                .filter(users::username.eq(&sign_in_json.username))
                .select(User::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    let stored_pw_hash = String::from_utf8_lossy(&user.pw_hash).to_string();
    let is_password_correct = bcrypt::verify(&sign_in_json.pw, &stored_pw_hash)?;

    if !is_password_correct {
        return Err(SignInError::NotFound);
    }

    let jwt: String = generate_jwt(&user.id).map_err(|_| InfraError::InternalServerError)?;

    let cookie = format!("session={}; HttpOnly; Secure; Path=/; SameSite=Strict", jwt);
    let value = HeaderValue::from_str(&cookie).unwrap();
    let redirect_url = "/";
    // let location_header = HeaderValue::from_str(redirect_url).unwrap();

    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(SET_COOKIE, value)
        // .header(LOCATION, location_header)
        .header("HX-Redirect", redirect_url)
        .body(Body::empty())
        .unwrap();

    Ok(response)
}

pub async fn sign_up(
    pool: &deadpool_diesel::postgres::Pool,
    sign_up_json: SignUpRequest,
) -> Result<impl Into<Response<Body>>, SignUpError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let username = sign_up_json.username.clone();
    let existing_user = conn
        .interact(move |conn| {
            users::table
                .filter(users::username.eq(username))
                .first::<User>(conn)
        })
        .await
        .map_err(adapt_infra_error)?;

    if existing_user.is_ok() {
        return Err(SignUpError::UsernameAlreadyExists);
    }

    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let pw_hash = match bcrypt::hash(sign_up_json.pw, DEFAULT_COST) {
        Ok(pw_hash) => pw_hash,
        Err(e) => return Err(SignUpError::PasswordHashError(e)),
    };
    let sign_up_json = SignUpRequestModel {
        username: sign_up_json.username,
        pw_hash: pw_hash.as_bytes().to_vec(),
    };

    let result = conn
        .interact(move |conn| {
            diesel::insert_into(users::table)
                .values(&sign_up_json)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;
    let jwt = generate_jwt(&result.id).map_err(|_| InfraError::InternalServerError)?;

    let cookie = format!("session={}; HttpOnly; Secure; Path=/; SameSite=Strict", jwt);
    let value = HeaderValue::from_str(&cookie).unwrap();
    let redirect_value = HeaderValue::from_static("/");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(SET_COOKIE, value)
        .header("HX-Redirect", redirect_value)
        .body(Body::empty())
        .unwrap();

    Ok(response)
}

impl From<InfraError> for SignUpError {
    fn from(error: InfraError) -> Self {
        match error {
            InfraError::InternalServerError => {
                SignUpError::InfraError(InfraError::InternalServerError)
            }
            InfraError::NotFound => SignUpError::InfraError(InfraError::NotFound),
        }
    }
}
impl From<InfraError> for SignInError {
    fn from(err: InfraError) -> Self {
        Self::InfraError(err)
    }
}
impl From<BcryptError> for SignInError {
    fn from(err: BcryptError) -> Self {
        Self::InvalidCredentials(err)
    }
}

pub const JWT_SECRET: &'static str = "your-secret-key";

fn generate_jwt(user_id: &Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: user_id.to_owned().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let key = JWT_SECRET;

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key.as_ref()),
    )?;

    Ok(token)
}
