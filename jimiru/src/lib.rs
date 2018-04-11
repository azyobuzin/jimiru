extern crate rmp;
extern crate rmp_serde;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::io;

pub const MESSAGE_VERSION: u32 = 1;

pub mod worker_to_server_codes {
    pub const UPDATE: u8 = 1;
}

#[derive(Debug, Clone)]
pub enum WorkerToServerMessage {
    Update(WorkerInfo),
}

impl WorkerToServerMessage {
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            WorkerToServerMessage::Update(ref info) => {
                let serialized = rmp_serde::to_vec(info).map_err(encode_error_to_io_error)?;

                let len = serialized.len();
                writer.write_all(&[
                    worker_to_server_codes::UPDATE,
                    len as u8,
                    (len >> 8) as u8,
                    (len >> 16) as u8,
                    (len >> 24) as u8
                ])?;

                writer.write_all(&serialized)
            }
        }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<Self, ReadError> {
        let code = {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            buf[0]
        };

        match code {
            worker_to_server_codes::UPDATE => {
                let len = {
                    let mut buf = [0u8; 4];
                    reader.read_exact(&mut buf)?;
                    (buf[0] as u32) | (buf[1] as u32) << 8 | (buf[2] as u32) << 16 | (buf[3] as u32) << 24
                } as usize;

                let mut buf = Vec::with_capacity(len);
                unsafe { buf.set_len(len); }
                reader.read_exact(&mut buf)?;

                let info = rmp_serde::from_slice(&buf)?;
                Ok(WorkerToServerMessage::Update(info))
            }
            _ => Err(ReadError::DecodeError(None))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub worker_name: String,
    pub version: u32,
    pub machines: Vec<MachineInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineInfo {
    pub id: u32,
    pub display_name: String,
    pub is_online: bool,
}

pub mod server_to_worker_codes {
    pub const POWER_UP: u8 = 1;
}

pub enum ServerToWorkerMessage {
    PowerUp { id: u32 },
}

impl ServerToWorkerMessage {
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        unimplemented!()
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<Self, ReadError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum ReadError {
    DecodeError(Option<rmp_serde::decode::Error>),
    IoError(io::Error),
}

impl std::error::Error for ReadError {
    fn description(&self) -> &str { "read error" }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ReadError::DecodeError(Some(ref x)) => Some(x),
            ReadError::IoError(ref x) => Some(x),
            _ => None,
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match std::error::Error::cause(self) {
            Some(x) => fmt::Display::fmt(x, f),
            None => f.write_str("decode error"),
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ReadError::IoError(e)
    }
}

impl From<rmp_serde::decode::Error> for ReadError {
    fn from(e: rmp_serde::decode::Error) -> Self {
        use rmp_serde::decode::Error::*;

        match e {
            InvalidMarkerRead(ioe) | InvalidDataRead(ioe) => ReadError::IoError(ioe),
            e => ReadError::DecodeError(Some(e)),
        }
    }
}

fn encode_error_to_io_error(e: rmp_serde::encode::Error) -> io::Error {
    use rmp::encode::ValueWriteError::*;
    use rmp_serde::encode::Error::InvalidValueWrite;

    match e {
        InvalidValueWrite(InvalidMarkerWrite(ioe)) | InvalidValueWrite(InvalidDataWrite(ioe)) => ioe,
        e => io::Error::new(io::ErrorKind::Other, e),
    }
}
