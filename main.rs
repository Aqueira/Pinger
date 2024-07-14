use std::{env, io};
use std::thread::sleep;
use std::time::Duration;
use dns_lookup::{lookup_host};


fn main() -> Result<(), anyhow::Error> {
    initial_logger();

    println!("Write IPv4");
    let website = {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).map_err(|err|{
            log::error!("Error to get user input - {}", err);
            anyhow::anyhow!("Error to get user input - {}", err)
        })?;
        buffer.trim().to_string()
    };
    
    loop {
        println!("-------------------------------------------");
        let _ = ping_website(&website).inspect_err(|err|{
            log::error!("Error to ping website -> {}", err)
        });
        sleep(Duration::from_secs(10));
    }
}


fn ping_website(website: &str) -> Result<(), anyhow::Error> {
    println!("Pinging: {}", website);
    let ips = lookup_host(website).map_err(|err|{
        log::error!("Error to get connect with server! - {}", err);
        anyhow::anyhow!("Error to get connect with server! - {}", err)
    })?;
    
    for ip in ips {
        log::info!("{} желаемый айпи", ip);
    }
    
    Ok(())
}

fn initial_logger() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
}
