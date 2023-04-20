use clap::Parser;

use super::validato;
use crate::core::scanner;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// target ip address or url
    #[arg(name="target",default_value="127.0.0.1",value_parser = validato::valid_host)]
    pub target: String,

    /// port scan start value
    #[arg(short = 's', long, default_value_t = 1)]
    pub port_start: u16,

    /// port scan end value
    #[arg(short = 'e', long, default_value_t = 65535)]
    pub port_end: u16,

    /// concurrency num of parallel threads, default = #cpus * 10
    #[arg(short = 'c', long, default_value_t=num_cpus::get() * 10)]
    pub concurrency: usize,
}

pub fn handle_cmds() {
    let cli = Cli::parse();

    println!("Input Target:: {:?}", cli.target);

    scanner::scan_for_ports(cli.target, cli.port_start, cli.port_end, cli.concurrency);
}
