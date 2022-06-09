// Copyright 2015-2018 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// An error that can contain wascap-specific context
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

pub(crate) fn new(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

#[derive(Debug)]
pub enum ErrorKind {
    Serialize(serde_json::error::Error),
    Encryption(nkeys::error::Error),
    Decode(base64::DecodeError),
    UTF8(std::string::FromUtf8Error),
    Token(String),
    InvalidCapability,
    WasmElement(parity_wasm::elements::Error),
    IO(std::io::Error),
    InvalidModuleHash,
    ExpiredToken,
    TokenTooEarly,
    InvalidAlgorithm,
    MissingIssuer,
    MissingSubject,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Error {
        Error(Box::new(ErrorKind::IO(source)))
    }
}

impl From<parity_wasm::elements::Error> for Error {
    fn from(source: parity_wasm::elements::Error) -> Error {
        Error(Box::new(ErrorKind::WasmElement(source)))
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(source: serde_json::error::Error) -> Error {
        Error(Box::new(ErrorKind::Serialize(source)))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(source: base64::DecodeError) -> Error {
        Error(Box::new(ErrorKind::Decode(source)))
    }
}

impl From<nkeys::error::Error> for Error {
    fn from(source: nkeys::error::Error) -> Error {
        Error(Box::new(ErrorKind::Encryption(source)))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Error {
        Error(Box::new(ErrorKind::UTF8(source)))
    }
}
