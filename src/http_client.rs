//use failure::Error;
use crate::error::Error;
use futures::{Future, IntoFuture};
use log::info;
use reqwest::header::ACCEPT;
use reqwest::r#async::{Body, Client as ReqwestClient, Response};
use url::Url;

#[derive(Debug)]
pub(crate) struct HttpClient {
    base_url: Url,
    inner: ReqwestClient,
}

impl HttpClient {
    pub(crate) fn new(base_url: Url, client: ReqwestClient) -> Self {
        HttpClient {
            base_url,
            inner: client,
        }
    }

    pub(crate) fn post<B: Into<Body>>(
        &self,
        path: &str,
        body: B,
    ) -> impl Future<Item = Response, Error = Error> {
        let request_url = self.base_url.join(path);
        let client = self.inner.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| {
                info!("POST {}", url.as_str());

                client
                    .post(url.as_str())
                    .body(body.into())
                    .send()
                    .map_err(Error::from)
            })
    }

    pub(crate) fn get(&self, path: &str) -> impl Future<Item = Response, Error = Error> {
        let request_url = self.base_url.join(path);
        let client = self.inner.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| {
                info!("POST {}", url.as_str());

                client
                    .get(url.as_str())
                    .header(ACCEPT, "application/json")
                    .send()
                    .map_err(Error::from)
            })
    }

    pub(crate) fn delete(&self, path: &str) -> impl Future<Item = Response, Error = Error> {
        let request_url = self.base_url.join(path);
        let client = self.inner.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| {
                info!("POST {}", url.as_str());

                client.get(url.as_str()).send().map_err(Error::from)
            })
    }

    pub(crate) fn put<B: Into<Body>>(
        &self,
        path: &str,
        body: B,
    ) -> impl Future<Item = Response, Error = Error> {
        let request_url = self.base_url.join(path);
        let client = self.inner.clone();

        request_url
            .map_err(Error::from)
            .into_future()
            .and_then(move |url| {
                info!("POST {}", url.as_str());

                client
                    .put(url.as_str())
                    .body(body.into())
                    .send()
                    .map_err(Error::from)
            })
    }
}
