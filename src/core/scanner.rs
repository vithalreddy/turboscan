use dns_lookup::lookup_host;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::ErrorKind;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use threadpool::ThreadPool;

const SOCKET_TIMEOUT_DEFAUT: Duration = Duration::from_secs(1);

pub fn scan_for_ports(host: String, start_port: u16, end_port: u16, concurrency: usize) {
    let ip_opt = lookup_host(&host)
        .ok()
        .and_then(|ips| ips.into_iter().next())
        .or_else(|| {
            println!("Error: Failed to lookup host or no IP address found.");
            None
        });

    if ip_opt.is_none() {
        return;
    }

    let ip = ip_opt.unwrap();
    println!("Target IP:: {:?} \n", ip);

    let queue_size = (end_port - start_port + 1) as usize;

    let pb = ProgressBar::new(queue_size as u64)
        .with_style(
            ProgressStyle::with_template(
                "{prefix:.yellow.bold} {msg:.cyan.dim} [{elapsed_precise:.green}] {wide_bar:.cyan/blue} {pos:>7}/{len:7}",
            )
            .unwrap()
            .progress_chars("=> "),
        )
    .with_prefix("⚡️TurboScan")
    .with_message("Scanning ports...");

    let num_threads = concurrency;
    let pool = ThreadPool::new(num_threads);

    let discovered_ports: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(Vec::new()));

    for port in start_port..=end_port {
        let pb = pb.clone();
        let discovered_ports = Arc::clone(&discovered_ports);

        pool.execute(move || {
            portscan(port, ip, discovered_ports);
            pb.inc(1);
        });
    }

    pool.join();
    pb.finish();

    let values = discovered_ports.lock().unwrap();
    println!("Discovered ports: {:?}", *values);
}

fn portscan(port: u16, target_ip: IpAddr, discovered_ports: Arc<Mutex<Vec<u16>>>) {
    let addr = SocketAddr::new(target_ip, port);

    match TcpStream::connect_timeout(&addr, SOCKET_TIMEOUT_DEFAUT) {
        Ok(_stream) => {
            // println!("Port {} is open", port);
            let mut values = discovered_ports.lock().unwrap();
            values.push(port);
        }
        Err(ref e) if e.kind() == ErrorKind::ConnectionRefused => (),
        Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
        Err(e) => eprintln!("Error: P: {}, {:?}", port, e),
    }
}
