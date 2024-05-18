mod custom_errors;
use custom_errors::Errors;
use std::{io};
use std::thread::sleep;
use std::time::Duration;
use dns_lookup::{lookup_host};
fn ping_website(website: &str) -> Result<(), Errors> {
    println!("Пинговка: {}", website);
    let ips = lookup_host(website).map_err(|e|{
        eprintln!("Ошибка подключения к серверу - {}", e);
        Errors::Error
    })?;
    for ip in ips {
        println!("{} желаемый айпи", ip);
    }
    Ok(())
}
fn main() -> Result<(), Errors> {
    println!("Впишите ссылку");
    let website = {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).map_err(|e|{
            eprintln!("Ошибка ввода пользователя - {}", e);
            Errors::Error
        })?;
        buffer.trim().to_string()
    };
    loop {
        println!("-------------------------------------------");
        let _ = ping_website(&website).map_err(|_| Errors::Error);
        sleep(Duration::from_secs(10));
    }
}