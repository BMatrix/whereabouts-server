#![warn(rust_2018_idioms)]
#![warn(nonstandard_style)]

use confy::{self, ConfyError};

mod people;
use people::{People, Person};
mod request;
use request::{Update};

use serde_derive::{Serialize, Deserialize};
use serde_json;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::   env;
use std::error::Error;
use std::str::from_utf8;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    ipaddr: String,
    port: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { ipaddr: "172.0.0.1".to_string(), port: "34464".to_string() } }
}

fn get_config() -> Result<MyConfig, ConfyError> {
    let cfg: MyConfig = confy::load("whereabouts")?;
    Ok(cfg)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: MyConfig = get_config().unwrap();
    let mut ip = "".to_string();
    ip.push_str(&config.ipaddr);
    ip.push(':');
    ip.push_str(&config.port);

    println!("{}", ip);

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| ip);

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                
                let json = from_utf8(&buf[0..n]).unwrap();
                println!("{}", json);
                let model: Update = serde_json::from_str(&json).unwrap();
                print!("{}", model.id);

                if model.lat == None {   
                    let mockdata = mockdata().unwrap();
                    println!("{}", mockdata);

                    socket
                        .write_all(mockdata.as_bytes())
                        .await
                        .expect("failed to write data to socket");
                    
                    return;
                }
            }
        });
    }
}

fn mockdata() -> serde_json::Result<String> {
    let mut a: Vec<Person> = Vec::new();
    a.push(Person {
        id: "123".to_string(),
        lat: 11.1111,
        lon: 22.2222,
        time: "2022-03-17T22:34:25".to_string()
    });
    a.push(Person {
        id: "456".to_string(),
        lat: 33.3333,
        lon: 44.4444,
        time: "2022-03-16T22:34:25".to_string()
    });
    a.push(Person {
        id: "789".to_string(),
        lat: 55.5555,
        lon: 66.6666,
        time: "2022-03-15T22:34:25".to_string()
    });
    let people = People {
        people: a
    };

    Ok(serde_json::to_string(&people)?)
}