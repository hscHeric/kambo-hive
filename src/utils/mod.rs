use std::{net::UdpSocket, time::Duration};

use log::{error, info};

pub fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Logger inicializado");
}

const DISCOVERY_PORT: u16 = 2901;
const DISCOVERY_MESSAGE: &[u8] = b"KAMBO_HIVE_WORKER_DISCOVERY";
const RESPONSE_PREFIX: &[u8] = b"KAMBO_HIVE_HOST_RESPONSE:";

pub fn discovery_host() -> Result<String, Box<dyn std::error::Error>> {
    let udp_socket = UdpSocket::bind("0.0.0.0:0")?;
    let _ = udp_socket.set_broadcast(true);
    udp_socket.set_read_timeout(Some(Duration::from_secs(5)))?;

    info!("Procurando por um host na rede local na porta {DISCOVERY_PORT}...",);

    udp_socket.send_to(DISCOVERY_MESSAGE, ("255.255.255.255", DISCOVERY_PORT))?;

    let mut buf = [0; 1024];
    match udp_socket.recv_from(&mut buf) {
        Ok((amt, src)) => {
            let response = &buf[..amt];
            if response.starts_with(RESPONSE_PREFIX) {
                // 4. Extrai o endereço do host da resposta.
                let host_addr = String::from_utf8(response[RESPONSE_PREFIX.len()..].to_vec())?;
                info!("Host encontrado em: {host_addr} (respondido por {src})");
                Ok(host_addr)
            } else {
                Err("Resposta inválida recebida.".into())
            }
        }
        Err(e) => {
            error!("Nenhum host encontrado na rede: {e}");
            Err(e.into())
        }
    }
}

pub async fn listen_for_workers(host_address_to_advertise: String) {
    let socket = match UdpSocket::bind(("0.0.0.0", DISCOVERY_PORT)) {
        Ok(s) => s,
        Err(e) => {
            error!("Falha ao iniciar o listener de descoberta UDP: {e}");
            return;
        }
    };
    info!("Host aguardando por broadcasts de workers na porta {DISCOVERY_PORT}");

    let mut buf = [0; 1024];
    loop {
        if let Ok((amt, src)) = socket.recv_from(&mut buf) {
            if &buf[..amt] == DISCOVERY_MESSAGE {
                info!("Broadcast de descoberta recebido de {}", src);
                let response = [RESPONSE_PREFIX, host_address_to_advertise.as_bytes()].concat();
                // Responde diretamente ao worker que enviou o broadcast.
                if let Err(e) = socket.send_to(&response, src) {
                    error!("Falha ao responder ao worker {}: {}", src, e);
                }
            }
        }
    }
}
