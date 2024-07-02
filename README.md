# socks5_server

基于tokio使用Rust语言实现的一个简单的socks5协议服务器。

功能尚未完善，目前仅供学习使用。

## 功能实现

已经实现的指令：

* 建立TCP Stream并转发数据(CMD 0x01)

未实现的功能：
* Bind (CMD 0x02)
* UDP Associate (CMD 0x03)



认证方法目前仅支持“不需要认证的方式”，即METHOD的值为0x00。

## 参考

* [SOCKS-wiki](https://en.wikipedia.org/wiki/SOCKS)