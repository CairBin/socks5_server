use std::{error::Error, io::{Read, Write}};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct ClientGreeting{
    pub version:u8,
    pub n_method:u8,
    pub methods:Vec<u8>
}

impl ClientGreeting{
    pub fn new(version:u8, methods:Vec<u8>)->Self{
        Self{
            version:version,
            n_method:methods.len() as u8,
            methods:methods
        }
    }

    pub async fn read_from_buf(tcp_stream:&mut tokio::net::TcpStream)->Result<Self,Box<dyn Error>>{
        let mut buf = [0u8;256];
        tcp_stream.read(&mut buf).await?;
        Ok(Self { 
            version: buf[0],
            n_method: buf[1], 
            methods: buf[2..(buf[1] as usize)].to_vec()
        })
    }
}


pub struct GreetingResponse{
    pub version:u8,
    pub method:u8
}

impl GreetingResponse{
    pub fn new(version:u8, method:u8)->Self{
        Self{
            version:version,
            method:method
        }
    }

    pub async fn send_as_bytes(&self, tcp_stream:&mut tokio::net::TcpStream)->Result<(), Box<dyn Error>>{
        tcp_stream.write_all(&[self.version, self.method]).await?;

        Ok(())
    }
}