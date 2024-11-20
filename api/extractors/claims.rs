use crate::db;
use crate::errors::{ClientError, ServiceError};
use actix_web::{http::Uri, Error, FromRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use awc::Client;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::Deserialize;
use std::{collections::HashSet, future::Future, pin::Pin};

#[derive(Clone, Deserialize)]
pub struct Auth0Config {
    audience: String,
    domain: String,
}

impl Default for Auth0Config {
    fn default() -> Self {
        envy::prefixed("AUTH0_")
            .from_env()
            .expect("Provide missing environment variables for Auth0Client")
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    permissions: Option<HashSet<String>>,
    sub: Option<String>,
}

impl Claims {
    pub fn validate_permissions(
        &self,
        required_permissions: &HashSet<String>,
    ) -> Result<(), ServiceError> {
        
        self.permissions.as_ref().map_or(
            Err(ServiceError::BadRequest(String::from(
                "Unable to parse permissions",
            ))),
            |permissions| {
                if permissions.is_superset(required_permissions) {
                    Ok(())
                } else {
                    Err(ServiceError::Forbidden(String::from(
                        "Insufficient permissions",
                    )))
                }
            },
        )
    }

    pub fn get_auth0_sub(&self) -> Result<String, ServiceError> {
        self.sub.to_owned().ok_or_else(|| {
            ServiceError::BadRequest(String::from("Unable to retrieve sub from claims"))
        })
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let config = req.app_data::<Auth0Config>().unwrap().clone();
        let extractor = BearerAuth::extract(req);

        Box::pin(async move {
            let credentials = extractor.await.map_err(ClientError::Authentication)?;
            let token = credentials.token();
            let header = decode_header(token).map_err(ClientError::Decode)?;
            let kid = header.kid.ok_or_else(|| {
                ClientError::NotFound("Kid not found in token header".to_string())
            })?;
            let domain = config.domain.as_str();
            let jwks: JwkSet = Client::new()
                .get(
                    Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/.well-known/jwks.json")
                        .build()
                        .unwrap(),
                )
                .send()
                .await
                .map_err(|_| ServiceError::InternalServerError)?
                .json()
                .await
                .map_err(|_| ServiceError::InternalServerError)?;

            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| ClientError::NotFound("No JWK found for kid".to_string()))?;
            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[config.audience]);
                    validation.set_issuer(&[Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/")
                        .build()
                        .unwrap()]);
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(ClientError::Decode)?;
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(ClientError::Decode)?;
                    Ok(token.claims)
                }
                algorithm => Err(ClientError::UnsupportedAlgortithm(algorithm).into()),
            }
        })
    }
}
