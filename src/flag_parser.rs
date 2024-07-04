use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "my_tool")]
pub struct Opt {
    /// Print help screen
    #[structopt(short, long)]
    pub help: bool,

    /// Very verbose
    #[structopt(short = "V", long)]
    pub verbose: bool,

    /// Verbose mode
    #[structopt(short = "v", long)]
    pub verbose_mode: bool,

    /// Simplified log without request header
    #[structopt(short = "s", long)]
    pub simplified_log: bool,

    /// Which port to serve HTTP(S)
    #[structopt(short = "p", long)]
    pub port: Option<u16>,

    /// Last port of range of ports to serve HTTP(S)
    #[structopt(short = "P", long)]
    pub last_port: Option<u16>,

    /// Serve files under this directory
    #[structopt(short = "d", long)]
    pub directory: Option<PathBuf>,

    /// Internal - force file to be given file name
    #[structopt(short = "f", long)]
    pub force_file_name: bool,

    /// Internal - bind to ip4 address
    #[structopt(short = "b", long)]
    pub bind_ip4: bool,

    /// Log filename
    #[structopt(short = "l", long)]
    pub log_file: Option<PathBuf>,

    /// Timeout in seconds
    #[structopt(short = "t", long)]
    pub timeout: Option<u64>,

    /// Internal - gp_proto number
    #[structopt(short = "g", long)]
    pub gp_proto: Option<u64>,

    /// Max data row length expected
    #[structopt(short = "m", long)]
    pub max_data_row_length: Option<u64>,

    /// Use O_SYNC when opening files for write
    #[structopt(short = "S", long)]
    pub use_o_sync: bool,

    /// Internal - queue size for listen call
    #[structopt(short = "z", long)]
    pub listen_queue_size: Option<u64>,

    /// SSL - certificates files under this directory
    #[structopt(long = "ssl")]
    pub ssl_cert_dir: Option<PathBuf>,

    /// SSL_verify_peer - enable or disable the authentication for gpdb identity
    #[structopt(long = "ssl_verify_peer")]
    pub ssl_verify_peer: Option<bool>,

    /// Print version number
    #[structopt(long = "version")]
    pub version: bool,

    /// Wait for session timeout in seconds
    #[structopt(short = "w", long)]
    pub wait_timeout: Option<u64>,

    /// Turn on compressed transmission
    #[structopt(long = "compress")]
    pub compress: bool,

    /// Turn on multi-thread and compressed transmission
    #[structopt(long = "multi_thread")]
    pub multi_thread: Option<u8>,

    /// Timeout to clean up sessions in seconds
    #[structopt(short = "k", long)]
    pub cleanup_timeout: Option<u64>,
}

pub fn parse_flags() -> Result<Opt, String> {
    Opt::from_args_safe().map_err(|e| e.to_string())
}