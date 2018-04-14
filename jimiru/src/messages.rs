use std::error::Error;
use std::fmt;
use std::io;
use std::mem;
use ring::digest::SHA256_OUTPUT_LEN;
use rmp_serde;

pub const MESSAGE_VERSION: u32 = 1;

pub mod worker_to_server_codes {
    pub const HELLO: u8 = 1;
    pub const AUTHENTICATE: u8 = 2;
    pub const UPDATE: u8 = 3;
}

#[derive(Debug, Clone)]
pub enum WorkerToServerMessage {
    Hello { version: u32 },
    Authenticate { hash: [u8; SHA256_OUTPUT_LEN] },
    Update(WorkerInfo),
}

impl WorkerToServerMessage {
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            WorkerToServerMessage::Hello { version } => {
                let mut buf: [u8; 5] = unsafe { mem::uninitialized() };
                buf[0] = worker_to_server_codes::HELLO;
                write_u32_little_endian(&mut buf[1..], version);
                writer.write_all(&buf)
            }
            WorkerToServerMessage::Authenticate { hash } => {
                writer.write_all(&[worker_to_server_codes::AUTHENTICATE])?;
                writer.write_all(&hash)
            }
            WorkerToServerMessage::Update(ref info) => {
                let serialized = rmp_serde::to_vec(info).map_err(encode_error_to_io_error)?;
                let len = serialized.len();
                if len > u32::max_value() as usize {
                    return Err(io::Error::from(io::ErrorKind::InvalidData));
                }

                let mut buf: [u8; 5] = unsafe { mem::uninitialized() };
                buf[0] = worker_to_server_codes::UPDATE;
                write_u32_little_endian(&mut buf[1..], len as u32);
                writer.write_all(&buf)?;

                writer.write_all(&serialized)
            }
        }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<Self, ReadError> {
        let code = {
            let mut buf: [u8; 1] = unsafe { mem::uninitialized() };
            reader.read_exact(&mut buf)?;
            buf[0]
        };

        match code {
            worker_to_server_codes::HELLO => {
                let mut buf: [u8; 4] = unsafe { mem::uninitialized() };
                reader.read_exact(&mut buf)?;
                let version = read_u32_little_endian(&buf);
                Ok(WorkerToServerMessage::Hello { version })
            }
            worker_to_server_codes::AUTHENTICATE => {
                let mut buf: [u8; SHA256_OUTPUT_LEN] = unsafe { mem::uninitialized() };
                reader.read_exact(&mut buf)?;
                Ok(WorkerToServerMessage::Authenticate { hash: buf })
            }
            worker_to_server_codes::UPDATE => {
                let len = {
                    let mut buf: [u8; 4] = unsafe { mem::uninitialized() };
                    reader.read_exact(&mut buf)?;
                    read_u32_little_endian(&buf)
                } as usize;

                let buf = read_exact(reader, len)?;
                let info = rmp_serde::from_slice(&buf)?;
                Ok(WorkerToServerMessage::Update(info))
            }
            _ => Err(ReadError::DecodeError(None)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub worker_name: String,
    pub machines: Vec<MachineInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineInfo {
    pub id: u32,
    pub display_name: String,
    pub is_online: bool,
}

pub mod server_to_worker_codes {
    pub const AUTHENTICATION_REQUEST: u8 = 1;
    pub const UPDATE_REQUEST: u8 = 2;
    pub const POWER_UP: u8 = 3;
}

pub enum ServerToWorkerMessage {
    AuthenticationRequest { nonce: Vec<u8> },
    UpdateRequest,
    PowerUp { machine_id: u32 },
}

impl ServerToWorkerMessage {
    pub fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match *self {
            ServerToWorkerMessage::AuthenticationRequest { ref nonce } => {
                let len = nonce.len();
                if len > u32::max_value() as usize {
                    return Err(io::Error::from(io::ErrorKind::InvalidData));
                }

                let mut buf: [u8; 5] = unsafe { mem::uninitialized() };
                buf[0] = server_to_worker_codes::AUTHENTICATION_REQUEST;
                write_u32_little_endian(&mut buf[1..], len as u32);
                writer.write_all(&buf)?;

                writer.write_all(&nonce)
            }
            ServerToWorkerMessage::UpdateRequest => {
                writer.write_all(&[server_to_worker_codes::UPDATE_REQUEST])
            }
            ServerToWorkerMessage::PowerUp { machine_id } => {
                let mut buf: [u8; 5] = unsafe { mem::uninitialized() };
                buf[0] = server_to_worker_codes::POWER_UP;
                write_u32_little_endian(&mut buf[1..], machine_id);
                writer.write_all(&buf)
            }
        }
    }

    pub fn read<R: io::Read>(reader: &mut R) -> Result<Self, ReadError> {
        let code = {
            let mut buf: [u8; 1] = unsafe { mem::uninitialized() };
            reader.read_exact(&mut buf)?;
            buf[0]
        };

        match code {
            server_to_worker_codes::AUTHENTICATION_REQUEST => {
                let len = {
                    let mut buf: [u8; 4] = unsafe { mem::uninitialized() };
                    reader.read_exact(&mut buf)?;
                    read_u32_little_endian(&buf)
                } as usize;
                let nonce = read_exact(reader, len)?;
                Ok(ServerToWorkerMessage::AuthenticationRequest { nonce })
            }
            server_to_worker_codes::UPDATE_REQUEST => Ok(ServerToWorkerMessage::UpdateRequest),
            server_to_worker_codes::POWER_UP => {
                let mut buf: [u8; 4] = unsafe { mem::uninitialized() };
                reader.read_exact(&mut buf)?;
                let machine_id = read_u32_little_endian(&buf);
                Ok(ServerToWorkerMessage::PowerUp { machine_id })
            }
            _ => Err(ReadError::DecodeError(None)),
        }
    }
}

#[derive(Debug)]
pub enum ReadError {
    DecodeError(Option<rmp_serde::decode::Error>),
    IoError(io::Error),
}

impl Error for ReadError {
    fn description(&self) -> &str { "read error" }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ReadError::DecodeError(Some(ref x)) => Some(x),
            ReadError::IoError(ref x) => Some(x),
            _ => None,
        }
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Error::cause(self) {
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

fn write_u32_little_endian(buf: &mut [u8], value: u32) {
    buf[0] = value as u8;
    buf[1] = (value >> 8) as u8;
    buf[2] = (value >> 16) as u8;
    buf[3] = (value >> 24) as u8;
}

fn read_u32_little_endian(buf: &[u8]) -> u32 {
    (buf[0] as u32)
        | (buf[1] as u32) << 8
        | (buf[2] as u32) << 16
        | (buf[3] as u32) << 24
}

fn read_exact<R: io::Read>(reader: &mut R, len: usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(len);
    unsafe { buf.set_len(len); }
    reader.read_exact(&mut buf)?;
    Ok(buf)
}
