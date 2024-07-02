mod greeting;
mod reqeust;
mod field;

use std::error::Error;

use greeting::{ClientGreeting, GreetingResponse};
use reqeust::{ClientRequest, ServerResponse};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1080").await.unwrap();
    println!("listening on 127.0.0.1:1080");

    // accept incoming
    loop{
        let (mut tcp_stream, _) = listener.accept().await.unwrap();
        println!("Accepted connection from: {}", tcp_stream.peer_addr().unwrap());

        task::spawn(async move{
            if let Err(e) = handle_client(&mut tcp_stream).await{
                eprintln!("Error handling client connection: {:?}", e);
            }
        });
    }
}


async fn handle_client(tcp_stream: &mut TcpStream)->Result<(), Box<dyn Error>>{
    let greeting = 
        ClientGreeting::read_from_buf(tcp_stream).await?;

    if greeting.version != field::Version::V5 as u8{
        return Err("Invalid version.".into());
    }

    let response = GreetingResponse::new(
        field::Version::V5 as u8,
        field::MethodType::NoAuth as u8
    );
    response.send_as_bytes(tcp_stream).await?;

    let request = 
        ClientRequest::read_from_buf(tcp_stream).await?;
    
    let address = match request.addr_type{
        0x01 =>{
            let ip = format!("{}.{}.{}.{}", 
                request.dst_addr[0], 
                request.dst_addr[1], 
                request.dst_addr[2], 
                request.dst_addr[3]);

            format!("{}:{}",ip,request.dst_port).to_string()  
        }

        0x03 =>{
            let len = request.dst_addr[0] as usize;
            format!("{}:{}",String::from_utf8_lossy(&request.dst_addr[1..1+len]).to_string(),request.dst_port).to_string()
        }

        0x04 =>{
            let buf = request.dst_addr.clone();
            format!("{}:{}",
                std::net::Ipv6Addr::new(
                    ((buf[0x00] as u16) << 8) | (buf[0x01] as u16),
                    ((buf[0x02] as u16) << 8) | (buf[0x03] as u16),
                    ((buf[0x04] as u16) << 8) | (buf[0x05] as u16),
                    ((buf[0x06] as u16) << 8) | (buf[0x07] as u16),
                    ((buf[0x08] as u16) << 8) | (buf[0x09] as u16),
                    ((buf[0x0a] as u16) << 8) | (buf[0x0b] as u16),
                    ((buf[0x0c] as u16) << 8) | (buf[0x0d] as u16),
                    ((buf[0x0e] as u16) << 8) | (buf[0x0f] as u16),
                ).to_string(),
                request.dst_port)
        }

        _ => return Err("Unsupported address type".into())
            
    };

    // connect to remote server
    println!("{}",address);
    let mut target_socket = TcpStream::connect(&address).await?;
    println!("Target address: {}" , address);

    let state = ServerResponse::new(
        field::Version::V5 as u8, 
        field::ResponseState::Success as u8,
        0x01, 
        vec![0x00;4],0);
    
    state.send_as_bytes(tcp_stream).await?;

    // forward data between client and remote server
    let (mut client_reader, mut client_writer) = tcp_stream.split();
    let (mut server_reader, mut server_writer) = target_socket.split();

    let client_to_remote = tokio::io::copy(&mut client_reader, &mut server_writer);
    let remote_to_client = tokio::io::copy(&mut server_reader, &mut client_writer);

    tokio::try_join!(client_to_remote, remote_to_client);

    Ok(())
}