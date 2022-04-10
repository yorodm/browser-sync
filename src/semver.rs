use axum::extract::RequestParts;
use semver::Version;
use axum::{extract::FromRequest, http::StatusCode};
use axum::async_trait;


pub(crate) struct VersionParam(Version);

#[async_trait]
impl<B> FromRequest<B> for VersionParam
where B:Send,
{
    type Rejection= (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        todo!()
    }
}
