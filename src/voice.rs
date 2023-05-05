use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
};

use colored::Colorize;
use hyper::{header::InvalidHeaderValue, http::HeaderValue};
use serde::{Deserialize, Serialize};

use crate::constants::{ORIGIN, TRIAL_VOICE_LIST_URL};

/// Voice information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Voice {
    display_name: String,
    gender: String,
    local_name: String,
    locale: String,
    locale_name: String,
    name: String,
    sample_rate_hertz: String,
    short_name: String,
    status: String,
    voice_type: String,
    words_per_minute: Option<String>,
    style_list: Option<Vec<String>>,
    role_play_list: Option<Vec<String>>,
}

#[non_exhaustive]
pub enum VoiceListAPIEndpoint<'a> {
    Region(&'a str),
    Url(&'a str),
}

impl<'a> VoiceListAPIEndpoint<'a> {
    pub fn get_endpoint_url(&'a self) -> Cow<'a, str> {
        match self {
            Self::Url(url) => (*url).into(),
            Self::Region(r) => Cow::Owned(format!(
                "https://{r}.tts.speech.microsoft.com/cognitiveservices/voices/list"
            )),
        }
    }
}

#[non_exhaustive]
pub enum VoiceListAPIAuth<'a> {
    SubscriptionKey(&'a str),
    AuthToken(&'a str),
}

impl Voice {
    pub async fn request_available_voices(
        endpoint: VoiceListAPIEndpoint<'_>,
        auth: Option<VoiceListAPIAuth<'_>>,
        proxy: Option<&str>,
    ) -> Result<Vec<Self>, VoiceListAPIError> {
        Self::request_available_voices_with_additional_headers(endpoint, auth, proxy, None).await
    }

    pub async fn request_available_voices_with_additional_headers(
        endpoint: VoiceListAPIEndpoint<'_>,
        auth: Option<VoiceListAPIAuth<'_>>,
        proxy: Option<&str>,
        additional_headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<Vec<Self>, VoiceListAPIError> {
        let url = endpoint.get_endpoint_url();
        let mut client = reqwest::ClientBuilder::new().no_proxy(); // Disable default system proxy detection.
        if let Some(proxy) = proxy {
            client = client.proxy(reqwest::Proxy::all(proxy).map_err(|e| VoiceListAPIError {
                kind: VoiceListAPIErrorKind::ProxyError,
                source: Some(e.into()),
            })?);
        }
        let client = client.build().map_err(|e| VoiceListAPIError {
            kind: VoiceListAPIErrorKind::RequestError,
            source: Some(e.into()),
        })?;
        let mut request = client.get(&*url);
        let request_error = |e: InvalidHeaderValue| VoiceListAPIError {
            kind: VoiceListAPIErrorKind::RequestError,
            source: Some(e.into()),
        };
        match auth {
            Some(VoiceListAPIAuth::SubscriptionKey(key)) => {
                request = request.header(
                    "Ocp-Apim-Subscription-Key",
                    HeaderValue::from_str(key).map_err(request_error)?,
                );
            }
            Some(VoiceListAPIAuth::AuthToken(token)) => {
                request = request.header(
                    "Authorization",
                    HeaderValue::from_str(token).map_err(request_error)?,
                );
            }
            None => {}
        }
        if additional_headers.is_some() {
            request = request.headers(additional_headers.unwrap());
        } else if Some(url.as_ref()) == TRIAL_VOICE_LIST_URL {
            // Trial endpoint
            request = request.header("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        }
        let request_error = |e: reqwest::Error| VoiceListAPIError {
            kind: VoiceListAPIErrorKind::RequestError,
            source: Some(e.into()),
        };
        let request = request.build().map_err(request_error)?;
        let response = client.execute(request).await.map_err(request_error)?;
        let response = response.error_for_status().map_err(|e| VoiceListAPIError {
            kind: VoiceListAPIErrorKind::ResponseError,
            source: Some(
                VoiceListAPIResponseStatusError {
                    status: e.status().unwrap(),
                    source: Some(e.into()),
                }
                .into(),
            ),
        })?;
        response
            .json::<Vec<Voice>>()
            .await
            .map_err(|e| VoiceListAPIError {
                kind: VoiceListAPIErrorKind::ParseError,
                source: Some(e.into()),
            })
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn gender(&self) -> &str {
        &self.gender
    }

    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn locale_name(&self) -> &str {
        &self.locale_name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sample_rate_hertz(&self) -> &str {
        &self.sample_rate_hertz
    }

    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn voice_type(&self) -> &str {
        &self.voice_type
    }

    pub fn words_per_minute(&self) -> Option<&str> {
        self.words_per_minute.as_deref()
    }

    pub fn style_list(&self) -> Option<&[String]> {
        self.style_list.as_deref()
    }

    pub fn role_play_list(&self) -> Option<&[String]> {
        self.role_play_list.as_deref()
    }
}

impl Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name.bright_green())?;
        writeln!(f, "Display name: {}", self.display_name)?;
        writeln!(f, "Local name: {} @ {}", self.local_name, self.locale)?;
        writeln!(f, "Locale: {}", self.locale_name)?;
        writeln!(f, "Gender: {}", self.gender)?;
        writeln!(f, "ID: {}", self.short_name)?;
        writeln!(f, "Voice type: {}", self.voice_type)?;
        writeln!(f, "Status: {}", self.status)?;
        writeln!(f, "Sample rate: {}Hz", self.sample_rate_hertz)?;
        writeln!(
            f,
            "Words per minute: {}",
            self.words_per_minute.as_deref().unwrap_or("N/A")
        )?;
        if let Some(style_list) = self.style_list.as_ref() {
            writeln!(f, "Styles: {style_list:?}")?;
        }
        if let Some(role_play_list) = self.role_play_list.as_ref() {
            writeln!(f, "Roles: {role_play_list:?}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct VoiceListAPIResponseStatusError {
    pub status: reqwest::StatusCode,
    source: Option<anyhow::Error>,
}

impl Display for VoiceListAPIResponseStatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to retrieve voice list because of {} side error: status {:?}",
            if self.status.as_u16() >= 500u16 {
                "server"
            } else {
                "client"
            },
            self.status
        )
    }
}

impl Error for VoiceListAPIResponseStatusError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct VoiceListAPIError {
    pub kind: VoiceListAPIErrorKind,
    source: Option<anyhow::Error>,
}

impl Display for VoiceListAPIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "error while retrieving voice list: {:?}", self.kind)
    }
}

impl Error for VoiceListAPIError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[derive(Debug)]
pub enum VoiceListAPIErrorKind {
    ProxyError,
    RequestError,
    ParseError,
    ResponseError,
}
