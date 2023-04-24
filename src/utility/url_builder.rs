use std::collections::BTreeMap;

#[derive(Debug)]
pub struct UrlBuilder {
    url: String,
    subdirs: Vec<String>,
    params: BTreeMap<String, String>, //ordered keys for determinism
}

impl UrlBuilder {
    pub fn with_url(url: &str) -> Self {
        UrlBuilder {
            url: url.to_owned(),
            subdirs: Vec::new(),
            params: BTreeMap::new(),
        }
    }

    pub fn subdir(&self, subdir: &str) -> Self {
        let mut nsubdirs = self.subdirs.clone();
        nsubdirs.push(subdir.to_string());
        UrlBuilder {
            url: self.url.clone(),
            subdirs: nsubdirs,
            params: self.params.clone(),
        }
    }

    pub fn param(&self, key: &str, value: &str) -> Self {
        let mut nparams = self.params.clone();
        nparams.insert(key.to_string(), value.to_string());
        UrlBuilder {
            url: self.url.clone(),
            subdirs: self.subdirs.clone(),
            params: nparams,
        }
    }

    pub fn build(&self) -> String {
        let subdirs_s = self
            .subdirs
            .iter()
            .cloned()
            .reduce(|a, b| format!("{}/{}", a, b))
            .map(|s| format!("/{}", s))
            .or_else(|| Some("".to_owned()))
            .unwrap();

        let params_s = self
            .params
            .clone()
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .reduce(|s, t| format!("{}&{}", s, t))
            .map(|s| format!("?{}", s))
            .or_else(|| Some("".to_owned()))
            .unwrap();

        format!("{}{}{}", self.url, subdirs_s, params_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static API_URL: &str = "api.google.com";

    #[test]
    fn builder_normal_usage_returns_url() {
        let actual = UrlBuilder::with_url(API_URL)
            .subdir("s1")
            .subdir("s2")
            .param("k1", "v1")
            .param("k2", "v2")
            .param("k3", "v4")
            .build();
        let expected = format!("{}/s1/s2?k1=v1&k2=v2&k3=v4", API_URL.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_only_url_returns_ok() {
        let actual = UrlBuilder::with_url(API_URL).build();
        let expected = format!("{}", API_URL.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_only_subdirs_returns_ok() {
        let actual = UrlBuilder::with_url(API_URL)
            .subdir("s1")
            .subdir("s2")
            .build();
        let expected = format!("{}/s1/s2", API_URL.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_only_params_returns_ok() {
        let actual = UrlBuilder::with_url(API_URL)
            .param("k1", "v1")
            .param("k2", "v2")
            .param("k3", "v4")
            .build();
        let expected = format!("{}?k1=v1&k2=v2&k3=v4", API_URL.to_owned());
        assert_eq!(expected, actual);
    }
}
