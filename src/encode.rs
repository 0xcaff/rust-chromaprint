use base64::{CharacterSet, Config};

fn config() -> Config {
    Config::new(CharacterSet::UrlSafe, false)
}

pub fn encode(fingerprint: &[u8]) -> String {
    base64::encode_config(fingerprint, config())
}
