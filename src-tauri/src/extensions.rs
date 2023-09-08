use crate::errors::AppResult;
use anyhow::{anyhow, Context};
use reqwest::header::CONTENT_DISPOSITION;
use tauri::{AppHandle, Manager};

pub trait ResponseExt {
    fn content_disposition_file_name(&self) -> AppResult<String>;
}

impl ResponseExt for reqwest::Response {
    fn content_disposition_file_name(&self) -> AppResult<String> {
        let header_string_value = self
            .headers()
            .get(CONTENT_DISPOSITION)
            .context("no content-disposition header")
            .and_then(|value| {
                value
                    .to_str()
                    .map(|i| i.to_string())
                    .context("no valid ascii")
            })?;
        let disposition = header_string_value.split(';').skip(1);
        for section in disposition {
            let mut parts = section.splitn(2, '=');
            let key = parts.next().context("no key")?.trim();
            let val = parts.next().context("no val")?.trim();

            if key == "filename" {
                return Ok(val.trim_matches('"').to_string());
            }
        }
        Err(anyhow!("no filename in header").into())
    }
}

pub trait StringExt {
    fn truncate_up_to(&self, n: usize) -> Self;
}

impl StringExt for String {
    fn truncate_up_to(&self, n: usize) -> Self {
        let up_to = self
            .char_indices()
            .map(|(i, _)| i)
            .nth(n)
            .unwrap_or(self.len());
        let mut new_string = self.clone();
        new_string.truncate(up_to);
        new_string
    }
}

pub trait StrOptionExt {
    fn to_maybe_string(&self) -> Option<String>;
}

impl StrOptionExt for Option<&str> {
    fn to_maybe_string(&self) -> Option<String> {
        self.map(|i| i.to_string())
    }
}

pub trait AppHandleExt {
    fn send_invalidate_cache(&self, key: impl Into<String>) -> AppResult<()>;
}

impl AppHandleExt for AppHandle {
    fn send_invalidate_cache(&self, key: impl Into<String>) -> AppResult<()> {
        self.emit_all("invalidate-cache", key.into())?;
        Ok(())
    }
}
