//! enhanced netcatt for zrpc
//!   only for testing

use tokio::{self, prelude::*};
use parity_tokio_ipc::Endpoint;

use zrpc::{ResultBlob, DrainBlob};

#[tokio::main]
async fn main() {
	let path = std::env::args().nth(1).expect("Run it with server path as argument");

    let mut client = Endpoint::connect(&path).await
        .expect("Failed to connect client.");

    loop {
        let mut next = String::new();

        if let Err(e) = std::io::stdin().read_line(&mut next) {
            println!("std i/o err: {}; quitting..", e);
            break;
        };

        let len = next.len();
        next.truncate(len - if std::cfg!(windows) { 2 } else { 1 }); // remove eol

        if next.len() == 0 {
            println!("should be method id [and params]");
            continue;
        }

        let method_id = match next.split(' ').nth(0).expect("checked above").parse::<u16>() {
            Ok(v) => v,
            Err(e) => {
                println!("Error parsing method num: {:?}", e);
                continue;
            }
        };

        let mut result_blob = ResultBlob::new();
        for arg in next.split(' ').skip(1) {
            if arg.chars().nth(0).expect("0 exist or won't split out") == 'i' {
                let arg = match arg.split_at(1).1.parse::<u64>() {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Error parsing arg: {:?}", e);
                        continue;
                    }
                };
                result_blob.push(arg);
            } else {
                println!("only integer arguments supported atm, for example i35654");
                continue;
            }
        }

        {
            client.write_all(&method_id.to_le_bytes()).await.expect("Unable to write message to client");
            client.write_all(&(result_blob.as_bytes().len() as u32).to_le_bytes()).await.expect("Unable to write message to client");
            client.write_all(result_blob.as_bytes()).await.expect("Unable to write message to client");

            let mut response_len = [0u8; 4];
            client.read_exact(&mut response_len).await.expect("Unable to read message from client");

            let mut response = vec![0; u32::from_le_bytes(response_len) as usize];
            client.read_exact(&mut response[..]).await.expect("Unable to read message from client");

            let mut drain_blob = DrainBlob::new(response);
            let result = match drain_blob.next::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    println!("Error decoding response code: {:?}", e);
                    continue;
                }
            };

            println!("result: {}", result);
        }
    }
}