# Turboscan

Turboscan is a blazingly fast and reliable Rust-based port scanner CLI for network administrators, security professionals and Developers. With lightning-fast scanning capabilities and advanced error handling, Turboscan makes it easy to identify open ports and potential security vulnerabilities on any network.

## Usage

```
Usage: turboscan [OPTIONS] [target]

Arguments:
  [target]  target ip address or url [default: 127.0.0.1]

Options:
  -s, --port-start <PORT_START>    port scan start value [default: 1]
  -e, --port-end <PORT_END>        port scan end value [default: 65535]
  -c, --concurrency <CONCURRENCY>  concurrency num of parallel threads, default = #cpus * 10 [default: 200]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Demo

<img src="https://github.com/vithalreddy/turboscan/blob/master/assets/demo.gif?raw=true">

## Features

- Fast and efficient port scanning using Rust concurrency primitives
- Robust error handling and reporting
- Customizable port ranges and target hosts
- Progress bar and real-time status updates
- Easy-to-use CLI interface with support for command-line options and arguments
- Built-in documentation and examples

## Installation

To install Turboscan, first make sure you have the Rust toolchain installed on your system. You can download and install Rust from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once Rust is installed, you can install Turboscan using Cargo, the Rust package manager. Simply run the following command:

```
$ cargo install turboscan
```

This will download and install Turboscan, along with any necessary dependencies.

## Usage

To use Turboscan, simply run the `turboscan` command followed by the hostname or IP address of the target host. For example:

```
$ turboscan example.com
```

This will scan all ports on the host `example.com` and display the results in real-time using a progress bar. You can also specify a custom range of ports to scan using the `--start-port` and `--end-port` options, like so:

```
$ turboscan example.com --start-port 1 --end-port 1024
```

This will scan only ports 1 through 1024 on the host `example.com`.

For a full list of command-line options and arguments, run `turboscan --help`.

## Documentation

For more information on how to use Turboscan, including detailed examples and API documentation, please refer to the official documentation:

[https://docs.rs/turboscan](https://docs.rs/turboscan)

## License

Turboscan is distributed under the MIT license. See [LICENSE](LICENSE) for more information.

## Contributing

If you would like to contribute to Turboscan, please submit a pull request or open an issue on the GitHub repository:

[https://github.com/vithalreddy/turboscan](https://github.com/vithalreddy/turboScan)
