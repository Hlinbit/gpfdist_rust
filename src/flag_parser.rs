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
    pub timeout: Option<usize>,

    /// Internal - gp_proto number
    #[structopt(short = "g", long)]
    pub gp_proto: Option<usize>,

    /// Max data row length expected
    #[structopt(short = "m", long)]
    pub max_data_row_length: Option<usize>,

    /// Use O_SYNC when opening files for write
    #[structopt(short = "S", long)]
    pub use_o_sync: bool,

    /// Internal - queue size for listen call
    #[structopt(short = "z", long)]
    pub listen_queue_size: Option<usize>,

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
    pub wait_timeout: Option<usize>,

    /// Turn on compressed transmission
    #[structopt(long = "compress")]
    pub compress: bool,

    /// Turn on multi-thread and compressed transmission
    #[structopt(long = "multi_thread")]
    pub multi_thread: Option<u8>,

    /// Timeout to clean up sessions in seconds
    #[structopt(short = "k", long)]
    pub cleanup_timeout: Option<usize>,
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            help: false,
            verbose: false,
            verbose_mode: false,
            simplified_log: false,
            port: Some(8090),
            last_port: None,
            directory: Some(std::env::current_dir().expect("Failed to get current directory")),
            force_file_name: false,
            bind_ip4: false,
            log_file: None,
            timeout:  Some(5 * 60),
            gp_proto: None,
            max_data_row_length: Some(1024 * 64),
            use_o_sync: false,
            listen_queue_size: Some(1024),
            ssl_cert_dir: None,
            ssl_verify_peer: None,
            version: false,
            wait_timeout: Some(5 * 60),
            compress: false,
            multi_thread: Some(8),
            cleanup_timeout: Some(5 * 60),
        }
    }
}

pub fn parse_flags() -> Result<Opt, String> {
    // 使用默认值创建 Opt 结构体
    let default_opt = Opt::default();
    println!("Default Opt: {:?}", default_opt);

    // 使用命令行参数创建 Opt 结构体，如果没有提供参数则使用默认值
    let mut opt = Opt::from_args_safe().map_err(|e| e.to_string())?;
    // 提供默认值
    if opt.port.is_none() {
        opt.port = Some(8080);
    }
    if opt.directory.is_none() {
        opt.directory = Some(std::env::current_dir().expect("Failed to get current directory"));
    }
    if opt.timeout.is_none() {
        opt.timeout = Some(5 * 60);
    }
    if opt.max_data_row_length.is_none() {
        opt.max_data_row_length = Some(1024 * 64);
    }
    if opt.listen_queue_size.is_none() {
        opt.listen_queue_size = Some(128);
    }
    if opt.ssl_verify_peer.is_none() {
        opt.ssl_verify_peer = Some(false);
    }
    if opt.wait_timeout.is_none() {
        opt.wait_timeout = Some(60 * 5);
    }
    if opt.multi_thread.is_none() {
        opt.multi_thread = Some(8);
    }
    if opt.cleanup_timeout.is_none() {
        opt.cleanup_timeout = Some(300);
    }
    println!("Opt from args: {:?}", opt);
    return Ok(opt);
}