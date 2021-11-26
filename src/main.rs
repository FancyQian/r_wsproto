mod scanner;

use std::env;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, connect};
use prost::Message;
use url::Url;
use std::io::Cursor;

use scanner::*;


fn main () {
    let ws_server = env::var_os("WS_Server");

    match ws_server {
        Some(_) => {
            let server = TcpListener::bind("192.168.40.17:8001").unwrap();

            for stream in server.incoming() {
                spawn (move || {
                    let mut websocket = accept(stream.unwrap()).unwrap();
                    loop {
                        let msg = websocket.read_message().unwrap();
                        if let tungstenite::Message::Binary(msg) = msg {
                            println!("Raw data: {:X?}", msg);
                            let scanner_message = 
                                scanner::ScannerMessages::decode(&mut Cursor::new(msg)).unwrap();
                            for sc_mesg in scanner_message.mesgs.iter() {
                                match ScannerMessageType::from_i32(sc_mesg.r#type) {
                                    Some(ScannerMessageType::Data) => {
                                        println!("Decode QR data: {}", String::from_utf8(sc_mesg.data.clone()).unwrap());
                                    },
                                    Some(ScannerMessageType::ScannerCtl) => {
                                        if let ScannerCtlCmd::Reboot = ScannerCtlCmd::from_i32(sc_mesg.s_cmd).unwrap() {
                                            println!("Scanner contorl command is reboot!!!!");
                                        }
                                    },
                                    _ => println!("Unable to decode data!")
                                }
                            }
                        }
                    }
                });
            }
        },
        _ => {
            let (mut socket, response) =
            connect(Url::parse("ws://127.0.0.1:8001").unwrap()).expect("Can't connect");

            println!("Connected to the server");
            println!("Response HTTP code: {}", response.status());
            println!("Response contains the following headers:");

            for (ref header, _value) in response.headers() {
                println!("* {}", header);
            }
        
            socket.write_message(tungstenite::Message::Binary(scanner_messages_new_and_encode())).unwrap();
            loop {
                let msg = socket.read_message().expect("Error reading message");
                println!("Received: {}", msg);
            }
        }
    }
}

fn scanner_messages_new_and_encode() -> Vec<u8> {

    let mut msg = ScannerMessage::default();

    /* qr data */
    msg.r#type = 0;
    msg.data = prost::alloc::vec::Vec::new();
    msg.data.push(32u8);
    msg.data.push(1u8);

    let mut msg1 = ScannerMessage::default();

    /* scanner ctl */
    msg1.r#type = 1;
    msg1.s_cmd = 12;

    let mut msgs = ScannerMessages::default();
    msgs.mesgs = prost::alloc::vec::Vec::new();
    msgs.mesgs.push(msg);
    msgs.mesgs.push(msg1);

    let mut buf = Vec::new();
    buf.reserve(msgs.encoded_len());
    msgs.encode(&mut buf).unwrap();
    buf
}