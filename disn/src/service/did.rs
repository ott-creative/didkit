use crate::error::{Error, Result};
use did_method_key::DIDKey;
use didkit::{DIDMethod, DIDResolver, Source, JWK};
pub struct DidService;

impl DidService {
    pub async fn did_create() -> Result<String> {
        // create jwk, a static step
        let jwk = JWK::generate_ed25519().map_err(|err| Error::from(err))?;
        // jwk to did-key
        DIDKey
            .generate(&Source::Key(&jwk))
            .ok_or_else(|| Error::DidGenerateError)
    }
}
