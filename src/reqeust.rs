use std::{error::Error, io::Read};

use tokio::io::{AsyncReadExt, AsyncWriteExt};



pub struct ClientRequest{
    pub version: u8,
    pub command: u8,
    pub rsv: u8,
    pub addr_type: u8,
    pub dst_addr:Vec<u8>,
    pub dst_port:u16
}


impl ClientRequest {
    pub fn new(version:u8, command:u8, addr_type:u8, dst_addr:Vec<u8>, dst_port:u16)->Self{
        Self{
            version: version,
            command: command,
            rsv: 0x00,
            addr_type: addr_type,
            dst_addr: dst_addr,
            dst_port: dst_port
        }
    }

    pub async fn read_from_buf(tcp_stream: &mut tokio::net::TcpStream)->Result<Self, Box<dyn Error>>{
        let mut buf = [0u8; 4096];
        let n = tcp_stream.read(&mut buf).await?;
        
        // addr type
        let addr_type = buf[3];
        let addr = match addr_type {
            // ipv4 address
            0x01 =>{
                buf[4..8].to_vec()
            },
            // domain name
            0x03 =>{
                let len = buf[4] as usize;
                buf[4..(5+len)].to_vec()
            },
            // ipv6 address, 16 bytes
            0x04=>{
                buf[4..20].to_vec()
            },
            _=>{
                return Err("Unsupported address type".into())
            }
        };

        Ok(Self { 
            version:buf[0], 
            command: buf[1], 
            rsv: buf[2], 
            addr_type: buf[3], 
            dst_addr: addr, 
            dst_port: u16::from_be_bytes([
                buf[n-2], buf[n-1]
            ]) 
        })
    }
}

pub struct ServerResponse{
    pub version:u8,
    pub state:u8,
    pub rsv:u8,
    pub addr_type:u8,
    pub src_addr:Vec<u8>,
    pub src_port:u16
}

impl ServerResponse{
    pub fn new(version:u8, state:u8, addr_type:u8, src_addr:Vec<u8>, src_port:u16)->Self{
        let addr = match addr_type {
            // domain name
            0x03 =>{
                let mut res:Vec<u8> = Vec::new();
                res.push(src_addr.len() as u8);
                res.extend(src_addr.iter());
                res
            },
            // ipv4 & ipv6 address
            _ =>{
                src_addr
            }
        };

        Self{
            version:version,
            state:state,
            rsv:0x00,
            addr_type:addr_type,
            src_addr:addr,
            src_port:src_port
        }
    }

    pub async fn send_as_bytes(&self, tcp_stream: &mut tokio::net::TcpStream)->Result<(), Box<dyn Error>>{

        let mut buf:Vec<u8> = Vec::new();
        buf.push(self.version);
        buf.push(self.state);
        buf.push(self.rsv);
        buf.push(self.addr_type);
        buf.extend(self.src_addr.iter());
        buf.extend(self.src_port.to_be_bytes().iter());

        tcp_stream.write_all(&buf).await?;

        Ok(())
    }

}