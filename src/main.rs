use std::env;
use std::net::IpAddr;
use std::path::MAIN_SEPARATOR_STR;
use std::process::Command;
use local_ip_address::local_ip;
use local_ip_address::list_afinet_netifas;

fn main() {
    // If print help is set, print the help and exit.
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        return;
    }

    let filter_loop_interfaces = env::args().any(|arg| arg == "-x");
    let print_ipv4_interfaces = env::args().any(|arg| arg == "-4");
    let print_ipv6_interfaces = env::args().any(|arg| arg == "-6");

    // Fetch the current directory.
    let current_dir = match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(_) => String::from("unknown"),
    };

    // Get the last argument after all arguments that have -- or - in front of them have been processed.
    let file_path = env::args().rev().skip_while(|arg| arg.starts_with("-") || arg.starts_with("--")).next();

    let win_dir_sep = "\\";
    let nix_dir_sep = "/";

    // If windows replace \ with /.
    let file_path = file_path.map(|path| {
        if cfg!(windows) {
            path.replace(nix_dir_sep, MAIN_SEPARATOR_STR)
        } else {
            path.replace(win_dir_sep, MAIN_SEPARATOR_STR)
        }
    });

    // Join the current directory with the file path.
    let file_path = file_path.map(|path| {
        if path.starts_with(MAIN_SEPARATOR_STR) {
            format!("{}{}", current_dir, path)
        } else {
            format!("{}{}{}", current_dir, MAIN_SEPARATOR_STR, path)
        }
    });

    // Enclose the file path in quotes if it contains spaces.
    let file_path = file_path.map(|path| {
        if path.contains(char::is_whitespace) {
            format!("\"{}\"", path)
        } else {
            path
        }
    });

    #[cfg(debug_assertions)]
    {
        println!("filter_loop_interfaces: {}", filter_loop_interfaces);
        println!("print_ipv4_interfaces: {}", print_ipv4_interfaces);
        println!("print_ipv6_interfaces: {}", print_ipv6_interfaces);
        if let Some(ref file_path) = file_path {
            println!("file_path: {}", file_path);
        }
    }

    // Fetch the hostname.
    let hostname = match Command::new("hostname").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => String::from("unknown"),
    };

    // Print a scp friendly pwd string.
    println!("{}:{}", hostname, current_dir);

    // Fetch all the network interfaces and their ip addresses.
    let network_interfaces = list_afinet_netifas();

    if print_ipv4_interfaces {
        let ipv4_interfaces: Result<Vec<&(String, IpAddr)>, &local_ip_address::Error> = network_interfaces.as_ref().map(|interfaces| {
            interfaces.into_iter().filter(|(_, ip)| {
                ip.is_ipv4()
            }).collect::<Vec<_>>()
        });

        print_iface_collect(ipv4_interfaces, filter_loop_interfaces, file_path.clone());
    }

    if print_ipv6_interfaces {
        let ipv6_interfaces = network_interfaces.as_ref().map(|interfaces| {
            interfaces.into_iter().filter(|(_, ip)| {
                ip.is_ipv6()
            }).collect::<Vec<_>>()
        });

        print_iface_collect(ipv6_interfaces, filter_loop_interfaces, file_path.clone());
    }

    if !print_ipv4_interfaces && !print_ipv6_interfaces {
        // Print the local ip address.
        if let Ok(ip) = local_ip() {
            println!("{}:{}", ip, current_dir);
        }
    }
}

fn print_help() {
    println!("Usage: printscp [options] [file_path]");
    println!("Options:");
    println!("  -x  Filter out loopback addresses.");
    println!("  -4  Print all ipv4 addresses.");
    println!("  -6  Print all ipv6 addresses.");
    println!("  -h  --help Print this help.");
}

fn print_iface_collect(mut interfaces: Result<Vec<&(String, IpAddr)>, &local_ip_address::Error>, filter_loop_interfaces: bool, path: Option<String>) {
    if filter_loop_interfaces {
        // Filter out loopback addresses.
        interfaces = interfaces.map(|interfaces| {
            interfaces.into_iter().filter(|(_, ip)| {
                !ip.is_loopback()
            }).collect::<Vec<_>>()
        });
    }

    if let Ok(interfaces) = interfaces {
        // If path is set, print the path. Otherwise print the current directory. Don't add quotes around the path.
        let path = path.unwrap_or_else(|| ".".to_string());

        // Sort the interfaces by ip address and print them.
        for (_name, ip) in interfaces.iter() {
            println!("{}:{}", ip, path);
        }
    }
}