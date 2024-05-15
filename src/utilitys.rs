#![allow(unused)]

pub mod error;
pub mod prelude;
pub mod services;
pub mod web;
pub mod utilitys {
    use super::services::{self, AccionesServices};
    use crate::utilitys::prelude::*;
    use futures_util::future::ok;
    use std::fs::read_dir;

    pub fn get_services(url: &'static str, method: &'static str) -> (u16, &'static str) {
        let param = services::ParametrosServices::default();

        let mut param = services::ParametrosServices::new(url, method);

        let res = services::AccionesServices::handle_request(&mut param);

        return res;
    }

    pub fn handle_error() -> Result<()> {
        for entry in read_dir("./")?.filter_map(|e| e.ok()) {
            // let entry = entry.path().to_str().map(String::from).ok_or_else(|| {
            //     Error::Generic(f!("Invalid Path {entry:?}"));
            // });
            println!("{entry:?}")
        }
        Ok(())
    }
}
