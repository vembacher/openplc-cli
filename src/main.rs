mod runtime;
extern crate clap;
use clap::{App, AppSettings, Arg};
use runtime::{
    Dnp3ServerConfig, EthernetIpServerConfig, ModbusServerConfig, OpcUaServerConfig, OpenPlcConfig,
    OpenplcRuntime,
};

fn main() {
    let config = OpenPlcConfig {
        host: String::from("localhost"),
        runtime_port: 43628,
    };
    let runtime = OpenplcRuntime::new(config);
    let matches = App::new("openplc-cli")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Victor Embacher <victor.embacher@tum.de>")
        .about("CLI to the OpenPLC runtime.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file [NOT IMPLEMENTED]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start")
                .help("Start the OpenPLC service with that name.")
                .long("start")
                .value_name("SERVICE")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stop")
                .help("stop the OpenPLC service with that name.")
                .long("stop")
                .value_name("SERVICE")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("logs")
                .help("Print logs from the OpenPLC runtime.")
                .long("logs")
                .value_name("SERVICE")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("list_services")
                .help("Lists all OpenPLC services by name.")
                .short("l")
                .long("list_services")
                .takes_value(false),
        )
        .get_matches();

    if matches.occurrences_of("list_services") > 0 {
        println!("The following services are supported:");
        for s in runtime.list_services() {
            println!("\t{}", s);
        }
    } else if let Some(service) = matches.value_of("start") {
        match service {
            "modbusslave" => {
                runtime.start_modbus(ModbusServerConfig { port: 502 });
            }
            "opcuaserver" => {
                runtime.start_opcua_server(OpcUaServerConfig { port: 4840 });
            }
            "dnp3s" => {
                runtime.start_dnp3(Dnp3ServerConfig { port: 20000 });
            }
            "enip" => {
                runtime.start_enip(EthernetIpServerConfig { port: 44818 });
            }
            _ => {}
        }
    } else if let Some(service) = matches.value_of("stop") {
        match service {
            "modbusslave" => {
                runtime.stop_modbus();
            }
            "opcuaserver" => {
                runtime.stop_opcuaserver();
            }
            "dnp3s" => {
                runtime.stop_dnp3();
            }
            "enip" => {
                runtime.stop_enip();
            }
            _ => {}
        }
    } else if matches.occurrences_of("logs") > 0 {
        runtime.logs();
    } else {
        println!("{}", matches.usage())
    }
}
