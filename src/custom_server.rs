use hbb_common::{
    bail,
    base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _},
    sodiumoxide::crypto::sign,
    ResultType,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct CustomServer {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub api: String,
    #[serde(default)]
    pub relay: String,
}

fn get_custom_server_from_config_string(s: &str) -> ResultType<CustomServer> {
    let tmp: String = s.chars().rev().collect();
    const PK: &[u8; 32] = &[
        88, 168, 68, 104, 60, 5, 163, 198, 165, 38, 12, 85, 114, 203, 96, 163, 70, 48, 0, 131, 57,
        12, 46, 129, 83, 17, 84, 193, 119, 197, 130, 103,
    ];
    let pk = sign::PublicKey(*PK);
    let data = URL_SAFE_NO_PAD.decode(tmp)?;
    if let Ok(lic) = serde_json::from_slice::<CustomServer>(&data) {
        return Ok(lic);
    }
    if let Ok(data) = sign::verify(&data, &pk) {
        Ok(serde_json::from_slice::<CustomServer>(&data)?)
    } else {
        bail!("sign:verify failed");
    }
}

pub fn get_custom_server_from_string(s: &str) -> ResultType<CustomServer> {
    // ============================================
    // ✅ ВАШ СЕРВЕР И КЛЮЧ ПО УМОЛЧАНИЮ
    // ============================================
    let default_server = CustomServer {
        host: "192.168.0.144".to_string(),
        key: "GtuvkU7MhXQnUQFqX2cRw7gW8jLp9yZ3sF=".to_string(),
        api: "".to_string(),
        relay: "".to_string(),
    };
    // ============================================

    // Если имя файла пустое или не содержит настроек, возвращаем наш сервер по умолчанию
    if s.is_empty() || !s.contains("host=") {
        return Ok(default_server);
    }

    let s = if s.to_lowercase().ends_with(".exe.exe") {
        &s[0..s.len() - 8]
    } else if s.to_lowercase().ends_with(".exe") {
        &s[0..s.len() - 4]
    } else {
        s
    };
    
    if s.to_lowercase().contains("host=") {
        let stripped = &s[s.to_lowercase().find("host=").unwrap_or(0)..s.len()];
        let strs: Vec<&str> = stripped.split(",").collect();
        let mut host = String::default();
        let mut key = String::default();
        let mut api = String::default();
        let mut relay = String::default();
        let strs_iter = strs.iter();
        for el in strs_iter {
            let el_lower = el.to_lowercase();
            if el_lower.starts_with("host=") {
                host = el.chars().skip(5).collect();
            }
            if el_lower.starts_with("key=") {
                key = el.chars().skip(4).collect();
            }
            if el_lower.starts_with("api=") {
                api = el.chars().skip(4).collect();
            }
            if el_lower.starts_with("relay=") {
                relay = el.chars().skip(6).collect();
            }
        }
        return Ok(CustomServer {
            host,
            key,
            api,
            relay,
        });
    } else {
        let s = s
            .replace("-licensed---", "--")
            .replace("-licensed--", "--")
            .replace("-licensed-", "--");
        let strs = s.split("--");
        for s in strs {
            if let Ok(lic) = get_custom_server_from_config_string(s.trim()) {
                return Ok(lic);
            } else if s.contains("(") {
                for s in s.split("(") {
                    if let Ok(lic) = get_custom_server_from_config_string(s.trim()) {
                        return Ok(lic);
                    }
                }
            }
        }
    }
    // Если ничего не найдено, возвращаем наш сервер по умолчанию
    Ok(default_server)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filename_license_string() {
        // Теперь даже без параметров должен работать наш сервер
        let default = get_custom_server_from_string("rustdesk.exe").unwrap();
        assert_eq!(default.host, "95.165.130.227");
        assert_eq!(default.key, "GtuvkU7MhXQnUQFqX2cRw7gW8jLp9yZ3sF=");
    }
}
