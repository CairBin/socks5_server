#[repr(u8)]
pub enum Version{
    V5 = 0x05
}


#[repr(u8)]
pub enum MethodType{
    NoAuth = 0x00,
    GssApi = 0x01,
    UsrPwd = 0x02,
    HandshakeChallenge = 0x03,
    ResponseChallenge = 0x05,
    SecurityLayer = 0x06,
    Nds = 0x07,
    MulAuthFramework = 0x08,
    JsonBlock = 0x09,
    NoAccessable = 0xFF
}

#[repr(u8)]
pub enum AddressType{
    Ipv4 = 0x01,
    DomainName = 0x03,
    Ipv6 = 0x04
}

#[repr(u8)]
pub enum CommandType{
    Connect = 0x01,
    Bind = 0x02,
    UdpAssociate = 0x03
}

#[repr(u8)]
pub enum ResponseState{
    Success = 0x00,
    GeneralFailure = 0x01,
    ConnectionNotAllowed = 0x02,
    NetworkUnreachable = 0x03,
    HostUnreachable = 0x04,
    ConnectionRefused = 0x05,
    TtlExpired = 0x06,
    CommandNotSupported = 0x07,
    AddressTypeNotSupport = 0x08,
    UnknownErrorCode = 0xFF
}