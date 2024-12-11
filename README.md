# SNTP Client

Este projeto implementa um cliente para o protocolo **SNTP (Simple Network Time Protocol)** em Rust. O objetivo do trabalho é enviar requisições SNTP a um servidor NTP e exibir a data e hora retornadas no formato legível.

---

## **Passos para Rodar o Cliente**

1. Certifique-se de ter o Rust instalado no sistema. Caso não tenha, instale-o com o comando:
```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone o repositório do projeto:
   
3. Compile o programa em modo release:
```bash
   cargo build --release
```
4. Execute o cliente passando o **IP do servidor NTP** como argumento:
   
   ```bash
   ./target/release/sntp_client <IP do servidor>
    ``` 
   Exemplo com o servidor público NTP do NIST:
   ```bash
   ./target/release/sntp_client 129.6.15.28
   ```
5. Saída esperada (caso o servidor responda corretamente):
   Data/hora: Qui Nov 21 23:15:42 2024

6. Caso o servidor não responda após duas tentativas de 20 segundos cada, o programa exibirá:
   Data/hora: não foi possível contactar servidor

---

## **Estrutura do Projeto**

- src/main.rs: Código principal que implementa o cliente SNTP.
- Cargo.toml: Arquivo de configuração do projeto, incluindo dependências.

---

## **Como Funciona**

### 1. Estrutura da Mensagem SNTP

A mensagem enviada ao servidor é um array de 48 bytes estruturado como segue:
- li_vn_mode (1 byte): Configurações de Leap Indicator, Version Number e Mode.
- Outros campos (47 bytes): Configurados como 0, conforme especificação SNTP.

### 2. Comunicação

O cliente cria um socket UDP (std::net::UdpSocket), envia a mensagem ao servidor e aguarda a resposta. A mensagem de resposta contém o timestamp de transmissão (Transmit Timestamp), que é convertido para o formato Unix e formatado para exibição.

### 3. Formatação da Resposta

A biblioteca chrono é usada para:
- Converter o timestamp para uma data e hora legíveis.
- Exibir o resultado em português.

---

## **Referências**

1. **Descrição do Protocolo SNTP:**
   - RFC 1769
2. **Servidor Público NTP do NIST:**
   - IP: 129.6.15.28
   - NIST Internet Time Service
3. **Rust Documentation:**
   - UdpSocket
   - chrono crate

## Membros do grupo

<center>
<table>
  <tr>
       <td align="center"><a href="https://github.com/Amandaaaaabreu"><img style="border-radius: 50%;" src="https://github.com/Amandaaaaabreu.png" width="100px;" alt=""/><br /><sub><b>Amanda Abreu</b></sub></a><br />
    <td align="center"><a href="https://github.com/JoseFilipi"><img style="border-radius: 50%;" src="https://github.com/JoseFilipi.png" width="100px;" alt=""/><br /><sub><b>José Filipe</b></sub></a><br />
    <td align="center"><a href="https://github.com/lramon2001"><img style="border-radius: 50%;" src="https://github.com/lramon2001.png" width="100px;" alt=""/><br /><sub><b>Lucas Ramon</b></sub></a><br />
    <td align="center"><a href="https://github.com/Maiconrq"><img style="border-radius: 50%;" src="https://github.com/Maiconrq.png" width="100px;" alt=""/><br /><sub><b>Maicon Rodrigues</b></sub></a><br />
  </tr>
 </table>
