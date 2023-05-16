use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct UrlBuilder {
    scheme: String,
    subdomains: Vec<String>,
    host: String,
    port: Option<u16>,
    subdirs: Vec<String>,
    params: BTreeMap<String, String>, //ordered keys for determinism
}

impl UrlBuilder {
    pub fn new() -> Self {
        Self {
            scheme: "https".to_owned(),
            subdomains: Vec::new(),
            host: String::new(),
            port: None,
            subdirs: Vec::new(),
            params: BTreeMap::new(),
        }
    }

    pub fn scheme<S>(&mut self, scheme: S) -> &mut Self
        where S: Into<String>
    {
        self.scheme = scheme.into();
        self
    }

    pub fn host<S>(&mut self, host: S) -> &mut Self
        where S: Into<String>
    {
        let host_s: String = host.into();
        while host_s.ends_with("/") {
            self.host.pop();
        }
        self.host = host_s;
        self
    }

    pub fn append_subdomain<S>(&mut self, subdomain: S) -> &mut Self
        where S: Into<String>
    {
        self.subdomains.push(subdomain.into());
        self
    }

    pub fn port(&mut self, port: u16) -> &mut Self
    {
        self.port = Some(port);
        self
    }

    pub fn subdir<S>(&mut self, subdir: S) -> &mut Self
        where S: Into<String>
    {
        let mut subdir_s: String = subdir.into();
        while subdir_s.ends_with("/") {
            subdir_s.pop();
        }
        self.subdirs.push(subdir_s);
        self
    }

    pub fn param<S1, S2>(&mut self, key: S1, value: S2) -> &mut Self
        where
            S1: Into<String>,
            S2: Into<String>
    {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn build(&self) -> String {
        let subdomains_s = self
            .subdomains
            .iter()
            .cloned()
            .reduce(|a, b| a + "." + &b)
            .map(|s| s + ".")
            .or(Some("".to_owned()))
            .unwrap();

        let port_s = match self.port {
            Some(num) => ":".to_owned() + &num.to_string(),
            None => "".to_owned(),
        };

        let subdirs_s = self
            .subdirs
            .iter()
            .cloned()
            .reduce(|a, b| a + "/" + &b)
            .map(|s| "/".to_owned() + &s)
            .or(Some("".to_owned()))
            .unwrap();

        let params_s = self
            .params
            .clone()
            .into_iter()
            .map(|(k, v)| k + "=" + &v)
            .reduce(|p1, p2| p1 + "&" + &p2)
            .map(|s| "?".to_owned() + &s)
            .or(Some("".to_owned()))
            .unwrap();

        self.scheme.clone()
            + "://"
            + &subdomains_s
            + &self.host
            + &port_s
            + &subdirs_s
            + &params_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static HOST: &str = "google.com";

    #[test]
    fn builder_normal_usage() {
        let actual = UrlBuilder::new()
            .append_subdomain("api")
            .append_subdomain("v2")
            .host(HOST)
            .port(400)
            .subdir("s1")
            .subdir("s2")
            .param("k1", "v1")
            .param("k2", "v2")
            .param("k3", "v4")
            .build();
        let expected = format!("https://api.v2.{}:400/s1/s2?k1=v1&k2=v2&k3=v4", HOST.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_only_host_returns_ok() {
        let actual = UrlBuilder::new()
            .host(HOST)
            .build();
        let expected = format!("https://{}", HOST.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_different_scheme_returns_ok() {
        let actual = UrlBuilder::new()
            .scheme("ssh")
            .host(HOST)
            .build();
        let expected = format!("ssh://{}", HOST.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_only_subdirs_returns_ok() {
        let actual = UrlBuilder::new()
            .host(HOST)
            .subdir("s1")
            .subdir("s2")
            .build();
        let expected = format!("https://{}/s1/s2", HOST.to_owned());
        assert_eq!(expected, actual);
    }

    #[test]
    fn builder_only_params_returns_ok() {
        let actual = UrlBuilder::new()
            .host(HOST)
            .param("k1", "v1")
            .param("k2", "v2")
            .param("k3", "v4")
            .build();
        let expected = format!("https://{}?k1=v1&k2=v2&k3=v4", HOST.to_owned());
        assert_eq!(expected, actual);
    }
}
