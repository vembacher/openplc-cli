use core::panic;
use std::fmt::format;
use std::io::{BufRead, BufReader, Read as IoRead};
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;
use std::{thread, time};
pub struct OpenPlcConfig {
    pub host: String,
    pub runtime_port: u16,
}

pub struct OpenplcRuntime {
    config: OpenPlcConfig,
}
pub struct ModbusServerConfig {
    pub port: u16,
}

pub struct OpcUaServerConfig {
    pub port: u16,
}
pub struct Dnp3ServerConfig {
    pub port: u16,
}
pub struct EthernetIpServerConfig {
    pub port: u16,
}

pub struct PersistentStorageConfig {
    pub poll_rate: u32,
}

impl OpenplcRuntime {
    pub fn list_services(&self) -> Vec<String> {
        //probably very suboptimal
        vec!["modbusslave", "opcuaserver"]
            .into_iter()
            .map(|s| String::from(s))
            .collect()
    }
    pub fn new(config: OpenPlcConfig) -> OpenplcRuntime {
        OpenplcRuntime { config }
    }
    pub fn start_opcua_server(&self, config: OpcUaServerConfig) {
        self.send_command(format!("start_opcuaserver({})\n", config.port))
    }
    pub fn start_modbus(&self, config: ModbusServerConfig) {
        self.send_command(format!("start_modbusslave({})\n", config.port))
    }

    pub fn stop_opcuaserver(&self) {
        self.send_command(String::from("stop_opcuaserver()\n"))
    }
    pub fn stop_modbus(&self) {
        self.send_command(String::from("stop_modbusslave()\n"))
    }

    pub fn start_dnp3(&self, config: Dnp3ServerConfig) {
        self.send_command(format!("start_dnp3s({})\n", config.port))
    }
    pub fn stop_dnp3(&self) {
        self.send_command(String::from("stop_dnp3s()\n"))
    }

    pub fn start_enip(&self, config: EthernetIpServerConfig) {
        self.send_command(format!("start_enip({})\n", config.port))
    }
    pub fn stop_enip(&self) {
        self.send_command(String::from("stop_enip3s()\n"))
    }

    pub fn start_pstorage(&self, config: PersistentStorageConfig) {
        self.send_command(format!("start_pstorage({})\n", config.poll_rate))
    }
    pub fn stop_pstorage(&self) {
        self.send_command(String::from("stop_pstorage()\n"))
    }
    pub fn logs(&self) {
        let command = String::from("runtime_logs()\n");
        if let Ok(mut stream) =
            TcpStream::connect(format!("{}:{}", self.config.host, self.config.runtime_port))
        {
            if let Err(err) = stream.write(command.as_bytes()) {
                panic!("[Error] Could not send command {}'. {}", command, err)
            };
            if let Err(err) = stream.flush() {
                panic!("{}", &err)
            };
            stream
                .set_read_timeout(Some(time::Duration::from_millis(100)))
                .unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            println!("Logs:");
            loop {
                let mut logs: String = String::with_capacity(1000);
                match reader.read_line(&mut logs) {
                    Err(_) | Ok(0) => break,
                    Ok(_) => print!("{}", logs),
                }
            }
        } else {
            panic!("Could not connect to runtime.");
        }
    }
    // pub fn exec_time(&self) {}
    // pub fn start_runtime(&self) {}
    // pub fn stop_runtime(&self) {}
    // pub fn compile_program(&self, path: &str) {}
    //
    fn send_command(&self, command: String) {
        if let Ok(mut stream) =
            TcpStream::connect(format!("{}:{}", self.config.host, self.config.runtime_port))
        {
            if let Err(err) = stream.write(command.as_bytes()) {
                panic!("[Error] Could not send command {}'. {}", command, err)
            };
            stream
                .set_read_timeout(Some(time::Duration::from_millis(100)))
                .unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            println!("Runtime answered:");
            loop {
                let mut answer: String = String::with_capacity(1000);
                match reader.read_line(&mut answer) {
                    Err(_) | Ok(0) => break,
                    Ok(_) => print!("{}", answer),
                }
            }
        } else {
            panic!("Could not connect to runtime.");
        }
    }
}
