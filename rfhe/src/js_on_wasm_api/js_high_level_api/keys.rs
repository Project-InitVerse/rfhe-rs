use crate::high_level_api as hlapi;
use crate::js_on_wasm_api::js_high_level_api::config::RfheConfig;
use crate::js_on_wasm_api::js_high_level_api::{catch_panic, catch_panic_result, into_js_error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct RfheClientKey(pub(crate) hlapi::ClientKey);

#[wasm_bindgen]
impl RfheClientKey {
    #[wasm_bindgen]
    pub fn generate(config: &RfheConfig) -> Result<RfheClientKey, JsError> {
        catch_panic(|| Self(hlapi::ClientKey::generate(config.0)))
    }

    #[wasm_bindgen]
    pub fn generate_with_seed(
        config: &RfheConfig,
        seed: JsValue,
    ) -> Result<RfheClientKey, JsError> {
        catch_panic_result(|| {
            let seed =
                u128::try_from(seed).map_err(|_| JsError::new("Value does not fit in a u128"))?;
            let key = hlapi::ClientKey::generate_with_seed(config.0, crate::Seed(seed));
            Ok(Self(key))
        })
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Result<Vec<u8>, JsError> {
        catch_panic_result(|| bincode::serialize(&self.0).map_err(into_js_error))
    }

    #[wasm_bindgen]
    pub fn deserialize(buffer: &[u8]) -> Result<RfheClientKey, JsError> {
        catch_panic_result(|| {
            bincode::deserialize(buffer)
                .map(Self)
                .map_err(into_js_error)
        })
    }
}

// Wasm cannot generate a normal server key, only a compressed one
#[wasm_bindgen]
pub struct RfheCompressedServerKey(pub(crate) hlapi::CompressedServerKey);

#[wasm_bindgen]
impl RfheCompressedServerKey {
    #[wasm_bindgen]
    pub fn new(client_key: &RfheClientKey) -> Result<RfheCompressedServerKey, JsError> {
        catch_panic(|| Self(hlapi::CompressedServerKey::new(&client_key.0)))
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Result<Vec<u8>, JsError> {
        catch_panic_result(|| bincode::serialize(&self.0).map_err(into_js_error))
    }

    #[wasm_bindgen]
    pub fn deserialize(buffer: &[u8]) -> Result<RfheCompressedServerKey, JsError> {
        catch_panic_result(|| {
            bincode::deserialize(buffer)
                .map(Self)
                .map_err(into_js_error)
        })
    }
}

#[wasm_bindgen]
pub struct RfhePublicKey(pub(crate) hlapi::PublicKey);

#[wasm_bindgen]
impl RfhePublicKey {
    #[wasm_bindgen]
    pub fn new(client_key: &RfheClientKey) -> Result<RfhePublicKey, JsError> {
        catch_panic_result(|| {
            let uses_big_params = client_key.0.key.block_parameters().encryption_key_choice()
                == crate::shortint::parameters::EncryptionKeyChoice::Big;
            if uses_big_params {
                return Err(JsError::new(
                    "PublicKey using big parameters not compatible wasm",
                ));
            }
            Ok(Self(hlapi::PublicKey::new(&client_key.0)))
        })
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Result<Vec<u8>, JsError> {
        catch_panic_result(|| bincode::serialize(&self.0).map_err(into_js_error))
    }

    #[wasm_bindgen]
    pub fn deserialize(buffer: &[u8]) -> Result<RfhePublicKey, JsError> {
        catch_panic_result(|| {
            bincode::deserialize(buffer)
                .map(Self)
                .map_err(into_js_error)
        })
    }
}

#[wasm_bindgen]
pub struct RfheCompressedPublicKey(pub(crate) hlapi::CompressedPublicKey);

#[wasm_bindgen]
impl RfheCompressedPublicKey {
    #[wasm_bindgen]
    pub fn new(client_key: &RfheClientKey) -> Result<RfheCompressedPublicKey, JsError> {
        catch_panic(|| Self(hlapi::CompressedPublicKey::new(&client_key.0)))
    }

    #[wasm_bindgen]
    pub fn decompress(&self) -> Result<RfhePublicKey, JsError> {
        catch_panic(|| RfhePublicKey(self.0.decompress()))
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Result<Vec<u8>, JsError> {
        catch_panic_result(|| bincode::serialize(&self.0).map_err(into_js_error))
    }

    #[wasm_bindgen]
    pub fn deserialize(buffer: &[u8]) -> Result<RfheCompressedPublicKey, JsError> {
        catch_panic_result(|| {
            bincode::deserialize(buffer)
                .map(Self)
                .map_err(into_js_error)
        })
    }
}

#[wasm_bindgen]
pub struct RfheCompactPublicKey(pub(crate) hlapi::CompactPublicKey);

#[wasm_bindgen]
impl RfheCompactPublicKey {
    #[wasm_bindgen]
    pub fn new(client_key: &RfheClientKey) -> Result<RfheCompactPublicKey, JsError> {
        catch_panic(|| Self(hlapi::CompactPublicKey::new(&client_key.0)))
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Result<Vec<u8>, JsError> {
        catch_panic_result(|| bincode::serialize(&self.0).map_err(into_js_error))
    }

    #[wasm_bindgen]
    pub fn deserialize(buffer: &[u8]) -> Result<RfheCompactPublicKey, JsError> {
        catch_panic_result(|| {
            bincode::deserialize(buffer)
                .map(Self)
                .map_err(into_js_error)
        })
    }
}

#[wasm_bindgen]
pub struct RfheCompressedCompactPublicKey(pub(crate) hlapi::CompressedCompactPublicKey);

#[wasm_bindgen]
impl RfheCompressedCompactPublicKey {
    #[wasm_bindgen]
    pub fn new(client_key: &RfheClientKey) -> Result<RfheCompressedCompactPublicKey, JsError> {
        catch_panic(|| Self(hlapi::CompressedCompactPublicKey::new(&client_key.0)))
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Result<Vec<u8>, JsError> {
        catch_panic_result(|| bincode::serialize(&self.0).map_err(into_js_error))
    }

    #[wasm_bindgen]
    pub fn deserialize(buffer: &[u8]) -> Result<RfheCompressedCompactPublicKey, JsError> {
        catch_panic_result(|| {
            bincode::deserialize(buffer)
                .map(Self)
                .map_err(into_js_error)
        })
    }

    #[wasm_bindgen]
    pub fn decompress(&self) -> Result<RfheCompactPublicKey, JsError> {
        catch_panic(|| RfheCompactPublicKey(self.0.decompress()))
    }
}
