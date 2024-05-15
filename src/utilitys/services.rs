#![allow(unused)]
use std::u16;

pub struct ParametrosServices {
    url: &'static str,
    method: &'static str,
}

pub trait AccionesServices {
    fn new(url: &'static str, method: &'static str) -> Self;
    fn get(&mut self) -> Self;
    fn post(&mut self) -> Self;
    fn handle_request(&mut self) -> (u16, &'static str);
    fn empty() -> Self;
}

impl Default for ParametrosServices {
    fn default() -> Self {
        Self {
            url: Default::default(),
            method: Default::default(),
        }
    }
}

impl AccionesServices for ParametrosServices {
    fn get(&mut self) -> Self {
        Self {
            url: self.url,
            method: self.method,
        }
    }

    fn post(&mut self) -> Self {
        Self {
            url: self.url,
            method: self.method,
        }
    }

    fn handle_request(&mut self) -> (u16, &'static str) {
        let (salida, mensaje) = if self.url == "/app/status" && self.method == "GET" {
            (200, "Ok".into())
        } else if self.url == "/create/user" && self.method == "POST" {
            (201, "User Created".into())
        } else {
            (404, "Not Ok".into())
        };

        (salida, mensaje)
    }

    fn new(url: &'static str, method: &'static str) -> Self {
        Self {
            url: &url,
            method: &method,
        }
    }

    fn empty() -> Self {
        Self::default()
    }
}
