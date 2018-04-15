use std::error::Error;
use std::path::Path;
use jimiru::HwAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// このワーカーに関する設定
    pub worker: WorkerConfig,
    /// 接続先サーバーの設定（複数設定可能）
    pub servers: Vec<ServerConfig>,
    /// マジックパケットを送るマシンの情報
    pub machines: Vec<MachineConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkerConfig {
    /// このワーカーの表示名
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// 接続先ホストとポート番号
    pub host: String,
    /// TLS を使用するかどうか
    #[serde(default)]
    pub use_tls: bool,
    /// TLS 接続時にドメイン名の検証をするかどうか
    #[serde(default = "default_validate_certificate")]
    pub validate_certificate: bool,
}

fn default_validate_certificate() -> bool { true }

#[derive(Debug, Clone, Deserialize)]
pub struct MachineConfig {
    /// マシンの表示名
    pub name: String,
    /// ホスト名または IP アドレス（ping 用）
    pub host: String,
    /// MAC アドレス（マジックパケット用）
    pub mac_addr: HwAddr,
}

pub fn read_config_file<P: AsRef<Path>>(config_file: P) -> Result<Config, Box<Error>> {
    use std::fs::File;
    use std::io::Read;
    use toml;

    let (buf, bytes_read) = {
        let mut f = File::open(config_file)?;
        // File は read_to_end をオーバーライドしていないので自分で capacity を確保
        let mut buf = Vec::with_capacity(f.metadata()?.len() as usize);
        let bytes_read = f.read_to_end(&mut buf)?;
        (buf, bytes_read)
    };

    // read_to_end は Vec を拡張するだけ拡張してサイズを戻さないので、範囲を指定
    Ok(toml::from_slice(&buf[..bytes_read])?)
}
