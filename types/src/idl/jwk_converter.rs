use crate::{
    jwks::{jwk::JWKMoveStruct, AllProvidersJWKs, ObservedJWKs, ProviderJWKs},
    move_any::Any,
};

pub fn convert_provider_jwks(
    origin_provider_jwks: api_types::on_chain_config::jwks::ProviderJWKs,
) -> ProviderJWKs {
    let api_types::on_chain_config::jwks::ProviderJWKs {
        issuer,
        version,
        jwks,
    } = origin_provider_jwks;
    ProviderJWKs {
        issuer,
        version,
        jwks: jwks
            .iter()
            .map(|jwk| JWKMoveStruct {
                variant: Any {
                    type_name: jwk.type_name.clone(),
                    data: jwk.data.clone(),
                },
            })
            .collect(),
    }
}

pub fn convert_all_providers_jwks(
    origin_all_providers_jwks: api_types::on_chain_config::jwks::AllProvidersJWKs,
) -> AllProvidersJWKs {
    let api_types::on_chain_config::jwks::AllProvidersJWKs { entries } = origin_all_providers_jwks;
    AllProvidersJWKs {
        entries: entries
            .iter()
            .map(|origin_provider_jwks| convert_provider_jwks(origin_provider_jwks.clone()))
            .collect(),
    }
}

pub fn construct_observed_jwks(
    origin_all_providers_jwks: api_types::on_chain_config::jwks::AllProvidersJWKs,
) -> ObservedJWKs {
    ObservedJWKs {
        jwks: convert_all_providers_jwks(origin_all_providers_jwks),
    }
}
