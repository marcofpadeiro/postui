use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct URL {
    pub host: String,
    pub path: Vec<String>
}

impl URL {
    pub fn to_raw(&self) -> String {
        let mut out = self.host.clone();
        for seg in &self.path {
            out.push('/');
            out.push_str(seg);
        }
        out
    }

    pub fn from_raw(s: &str) -> Self {
        let mut parts = s.split('/').filter(|p| !p.is_empty());
        let host = parts.next().unwrap_or_default().to_string();
        let path = parts.map(str::to_string).collect();
        URL { host, path }
    }
}

pub fn serialize_url<S>(url: &URL, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&url.to_raw())
}

pub fn deserialize_url<'de, D>(deserializer: D) -> Result<URL, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(URL::from_raw(&raw))
}
