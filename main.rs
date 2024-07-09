use std::{env, io};
use std::thread::sleep;
use std::time::Duration;
use dns_lookup::{lookup_host};


fn main() -> Result<(), anyhow::Error> {
    initial_logger();

    println!("Впишите ссылку");
    let website = {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).map_err(|e|{
            log::error!("Ошибка ввода пользователя - {}", e);
            anyhow::anyhow!("Ошибка ввода пользователя - {}", e)
        })?;
        buffer.trim().to_string()
    };
    loop {
        println!("-------------------------------------------");
        let _ = ping_website(&website).inspect_err(|e|{
            log::error!("Error to ping website -> {}", e)
        });
        sleep(Duration::from_secs(10));
    }
}

fn initial_logger() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
}

fn ping_website(website: &str) -> Result<(), anyhow::Error> {
    println!("Пинговка: {}", website);
    let ips = lookup_host(website).map_err(|e|{
        log::error!("Ошибка подключения к серверу - {}", e);
        anyhow::anyhow!("Ошибка подключения к серверу - {}", e)
    })?;
    for ip in ips {
        log::info!("{} желаемый айпи", ip);
    }
    Ok(())
}
