use std::env;
use std::net::UdpSocket;
use std::time::{Duration, UNIX_EPOCH};
use chrono::{NaiveDateTime, DateTime, Utc, Datelike, Timelike};
use regex::Regex;

const NTP_PACKET_SIZE: usize = 48; // Tamanho do pacote NTP
const NTP_PORT: u16 = 123;         // Porta padrão do protocolo NTP

fn main() {

    let server_ip = get_server_ip();

    if !is_valid_ip(&server_ip) {
        eprintln!("Endereço IP inválido: {}", server_ip);
        std::process::exit(1);
    }

    // Criar o socket UDP
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Erro ao criar o socket");
    socket
        .set_read_timeout(Some(Duration::new(20, 0)))
        .expect("Erro ao configurar timeout");

    // Criar o pacote NTP
    let packet = NtpPacket::new();
    let packet_bytes = packet.to_bytes();

    // Enviar o pacote
    let server_addr = format!("{}:{}", server_ip, NTP_PORT);
    socket
        .send_to(&packet_bytes, &server_addr)
        .expect("Erro ao enviar o pacote");

    // Buffer para receber a resposta
    let mut buffer = [0u8; NTP_PACKET_SIZE];
    match socket.recv_from(&mut buffer) {
        Ok((size, _src)) if size == NTP_PACKET_SIZE => {
            // Interpretar a resposta
            match parse_ntp_response(&buffer) {
                Some(time) => println!("Data/hora: {}", time),
                None => println!("Data/hora: erro ao interpretar a resposta"),
            }
        }
        _ => println!("Data/hora: não foi possível contactar servidor"),
    }
}

/// Obtém o IP do servidor da linha de comando
fn get_server_ip() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: sntp_client <IP do servidor>");
        std::process::exit(1);
    }
    args[1].clone()
}

/// Estrutura do pacote NTP
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct NtpPacket {
    li_vn_mode: u8,
    stratum: u8,
    poll: u8,
    precision: u8,
    root_delay: u32,
    root_dispersion: u32,
    ref_id: u32,
    ref_tm_s: u32,
    ref_tm_f: u32,
    orig_tm_s: u32,
    orig_tm_f: u32,
    rx_tm_s: u32,
    rx_tm_f: u32,
    tx_tm_s: u32,
    tx_tm_f: u32,
}

impl NtpPacket {
    fn new() -> Self {
        NtpPacket {
            li_vn_mode: 0x1B,
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay: 0,
            root_dispersion: 0,
            ref_id: 0,
            ref_tm_s: 0,
            ref_tm_f: 0,
            orig_tm_s: 0,
            orig_tm_f: 0,
            rx_tm_s: 0,
            rx_tm_f: 0,
            tx_tm_s: 0,
            tx_tm_f: 0,
        }
    }

    fn to_bytes(&self) -> [u8; NTP_PACKET_SIZE] {
        unsafe { std::mem::transmute::<NtpPacket, [u8; NTP_PACKET_SIZE]>(*self) }
    }
}

/// Interpreta a resposta do servidor NTP
fn parse_ntp_response(buffer: &[u8]) -> Option<String> {
    if buffer.len() < 48 {
        return None;
    }

    // Extrair o Transmit Timestamp (bytes 40-47)
    let tx_seconds = u32::from_be_bytes(buffer[40..44].try_into().unwrap()) as u64;
    let ntp_to_unix = 2_208_988_800; // Ajuste entre 1900 (NTP) e 1970 (Unix epoch)
    let unix_seconds = tx_seconds.checked_sub(ntp_to_unix)?;

    // Converter para o formato legível usando chrono
    let naive = NaiveDateTime::from_timestamp(unix_seconds as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    Some(format_date_in_portuguese(&datetime))
}

fn format_date_in_portuguese(datetime: &DateTime<Utc>) -> String {
    let days = ["Dom", "Seg", "Ter", "Qua", "Qui", "Sex", "Sáb"];
    let months = [
        "Jan", "Fev", "Mar", "Abr", "Mai", "Jun",
        "Jul", "Ago", "Set", "Out", "Nov", "Dez",
    ];

    let day = days[datetime.weekday().num_days_from_sunday() as usize];
    let month = months[(datetime.month() - 1) as usize];
    format!(
        "{} {} {:02} {:02}:{:02}:{:02} {}",
        day,
        month,
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.year()
    )
}


fn is_valid_ip(ip: &str) -> bool {
    let regex_ipv4 = Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    let regex_ipv6 = Regex::new(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$").unwrap();

    regex_ipv4.is_match(ip) || regex_ipv6.is_match(ip)
}
