use crate::errors::TokenError;
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::lookup_host;
use url::Url;

pub fn get_params(request: &hyper::Request<hyper::body::Incoming>) -> HashMap<String, Vec<String>> {
    let query = request.uri().query();
    let mut params: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(query_param) = query {
        let param = url::form_urlencoded::parse(query_param.as_bytes()).into_owned();
        param.for_each(|(key, element_value)| {
            match params.get_mut(&key) {
                Some(value) => {
                    value.push(element_value);
                }
                None => {
                    let value = vec![element_value];
                    params.insert(key, value);
                }
            };
        });
    }

    params
}

pub async fn tcp_addres(url: &str) -> Result<SocketAddr, TokenError> {
    let url = Url::parse(url)?;
    let host_and_port = match (url.host_str(), url.port()) {
        (Some(host), Some(port)) => host.to_owned() + ":" + &port.to_string(),
        _ => return Err(TokenError::InvalidToken),
    };
    let address = lookup_host(host_and_port).await?.next();
    let address = match address {
        Some(address) => address,
        None => {
            return Err(TokenError::InvalidToken);
        }
    };
    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_tcp_address() {
        let address = tcp_addres("http://localhost:3000/probando-para-que-no-se-vea").await;
        if let Err(address) = &address {
            println!("Error: {address}");
        }
        assert!(address.is_ok());
        if let Ok(address) = address {
            assert_eq!("127.0.0.1:3000", address.to_string());
        }
    }
}
