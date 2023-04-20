use std::{net::IpAddr, str::FromStr};
use url::Url;

pub fn valid_host(s: &str) -> Result<String, String> {
    let mut url_str = s.to_string();

    if url_str == "localhost" || url_str == "." {
        return Ok("127.0.0.1".to_string());
    };

    if !s.starts_with("http") {
        url_str = format!("http://{}", s);
    }

    let ip = IpAddr::from_str(&url_str);

    if ip.is_ok() {
        return Ok(ip.unwrap().to_string());
    }

    let url = Url::parse(&url_str);

    if url.is_err() {
        return Err(format!("Invalid target ip or url",));
    }

    let url = url.unwrap().host_str().unwrap().to_owned();

    return Ok(url);
}
