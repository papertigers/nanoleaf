use crate::http_client::HttpClient;
use crate::models::*;
//use failure::Error;
use crate::error::Error;
use futures::Future;
use reqwest::r#async::{Body, ClientBuilder};
use reqwest::RedirectPolicy;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::net::SocketAddr;
use url::Url;

enum EffectsCommand {
    Add,
    Delete,
    Request,
    RequestAll,
    RequestPlugins,
    Display,
    DisplayTemp,
}

pub enum NanoleafState {
    On,
    Off,
}

pub struct Client {
    inner: HttpClient,
}

impl Client {
    pub fn with_socketaddr(sa: SocketAddr) -> Result<Self, Error> {
        let base_url = Url::parse(&format!("http://{}/api/v1/", sa)).unwrap();
        let client = ClientBuilder::new()
            .redirect(RedirectPolicy::none())
            .build()?;
        let http = HttpClient::new(base_url, client);
        Ok(Client { inner: http })
    }

    // ====================
    // Users
    // ====================

    pub fn add_user(&self) -> impl Future<Item = Authorization, Error = Error> {
        self.post_value("new", "")
    }

    pub fn delete_user(&self, token: &str) -> impl Future<Item = (), Error = Error> {
        self.delete_value(token)
    }

    // ====================
    // Panel Info
    // ====================

    pub fn get_panels(&self, token: &str) -> impl Future<Item = PanelInfo, Error = Error> {
        self.get_value(token)
    }

    pub fn identify(&self, token: &str) -> impl Future<Item = (), Error = Error> {
        self.put_value(&format!("{}/identify", token), "", "")
    }

    // ====================
    // Panel State
    // ====================

    pub fn get_state(&self, token: &str) -> impl Future<Item = On, Error = Error> {
        self.get_value(&format!("{}/state/on", token))
    }

    pub fn set_state(
        &self,
        token: &str,
        state: NanoleafState,
    ) -> impl Future<Item = (), Error = Error> {
        let val = match state {
            NanoleafState::On => true,
            NanoleafState::Off => false,
        };
        let on = On { value: val };
        self.put_value(&format!("{}/state", token), "on", on)
    }

    // ====================
    // Brightness
    // ====================
    pub fn get_brightness(&self, token: &str) -> impl Future<Item = Range, Error = Error> {
        self.get_value(&format!("{}/state/brightness", token))
    }

    pub fn set_brightness(
        &self,
        token: &str,
        brightness: Brightness,
    ) -> impl Future<Item = (), Error = Error> {
        self.put_value(&format!("{}/state", token), "brightness", brightness)
    }

    // ====================
    // Hue
    // ====================
    pub fn get_hue(&self, token: &str) -> impl Future<Item = Range, Error = Error> {
        self.get_value(&format!("{}/state/hue", token))
    }

    pub fn set_hue(&self, token: &str, hue: SetRange) -> impl Future<Item = (), Error = Error> {
        self.put_value(&format!("{}/state", token), "hue", hue)
    }

    // ====================
    // Saturation
    // ====================
    pub fn get_saturation(&self, token: &str) -> impl Future<Item = Range, Error = Error> {
        self.get_value(&format!("{}/state/sat", token))
    }

    pub fn set_saturation(
        &self,
        token: &str,
        sat: SetRange,
    ) -> impl Future<Item = (), Error = Error> {
        self.put_value(&format!("{}/state", token), "sat", sat)
    }

    // ====================
    // Color Temperature
    // ====================
    pub fn get_ct(&self, token: &str) -> impl Future<Item = Range, Error = Error> {
        self.get_value(&format!("{}/state/ct", token))
    }

    pub fn set_ct(&self, token: &str, ct: SetRange) -> impl Future<Item = (), Error = Error> {
        self.put_value(&format!("{}/state", token), "ct", ct)
    }

    // ====================
    // Color Mode
    // ====================

    pub fn get_color_mode(&self, token: &str) -> impl Future<Item = String, Error = Error> {
        self.get_value(&format!("{}/effects/select", token))
    }

    // ====================
    // Effects
    // ====================

    pub fn get_effect(&self, token: &str) -> impl Future<Item = String, Error = Error> {
        self.get_value(&format!("{}/effects/select", token))
    }

    pub fn list_effects(&self, token: &str) -> impl Future<Item = Vec<String>, Error = Error> {
        self.get_value(&format!("{}/effects/effectsList", token))
    }

    pub fn get_all_effects(&self, token: &str) -> impl Future<Item = Animations, Error = Error> {
        // XXX turn this into an enum
        let command = json!({ "command": "requestAll"});
        self.put_value(&format!("{}/effects", token), "write", command)
    }

    pub fn set_effect(&self, token: &str, effect: &str) -> impl Future<Item = (), Error = Error> {
        self.put_value(&format!("{}/effects", token), "select", effect.to_owned())
    }

    // ====================
    // Helpers
    // ====================
    fn get_value<T: DeserializeOwned>(&self, path: &str) -> impl Future<Item = T, Error = Error> {
        self.inner
            .get(path)
            .and_then(|res| res.error_for_status().map_err(Error::from))
            .and_then(|mut res| res.json::<T>().map_err(Error::from))
    }

    fn delete_value(&self, path: &str) -> impl Future<Item = (), Error = Error> {
        self.inner
            .delete(path)
            .and_then(|res| res.error_for_status().map_err(Error::from))
            .and_then(|mut res| res.json::<()>().map_err(Error::from))
    }

    fn post_value<B, T>(&self, path: &str, body: B) -> impl Future<Item = T, Error = Error>
    where
        T: DeserializeOwned,
        B: Into<Body>,
    {
        self.inner
            .post(path, body)
            .and_then(|res| res.error_for_status().map_err(Error::from))
            .and_then(|mut res| res.json::<T>().map_err(Error::from))
    }

    fn put_value<'de, T: Serialize, D: DeserializeOwned>(
        &self,
        path: &str,
        key: &str,
        value: T,
    ) -> impl Future<Item = D, Error = Error> {
        let body = json!({ key: value });
        self.inner
            .put(path, body.to_string())
            .and_then(|res| res.error_for_status().map_err(Error::from))
            .and_then(|mut res| res.json::<D>().map_err(Error::from))
        // API returns 204 No Content or a 4xx error only
        //.map(|_| ())
    }
}
