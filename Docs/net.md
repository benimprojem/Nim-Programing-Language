# 🌐 NIMBLE Dili: `net` Modülü Detaylandırması

## BÖLÜM 11.12: `net` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`net`** | **Ağ ve Soket İletişimi.** TCP (güvenilir) ve UDP (hızlı, güvensiz) protokolleri kullanarak düşük seviyeli ağ iletişimi sağlamak için araçlar sunar. Tüm fonksiyonlar, bağlantı ve iletim hatalarını yönetmek için **`Result<T, E>`** tipi döndürür. |

### 1. Temel Yapılar ve Tipler

Ağ iletişimi için soketleri ve adresleri temsil eden temel yapılar.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Socket`** | Bir ağ bağlantı noktasını (socket) temsil eden opak yapı. Dosya işleyicisine (`FileHandle`) benzerdir. | `net.tcp_open()` veya `net.udp_open()` ile oluşturulur. |
| **`Protocol`** | Kullanılacak ağ protokolünü belirleyen sabitler. | `net.TCP`, `net.UDP` |
| **`Address`** | Bir IP adresi ve port numarasını temsil eden yapı. | `net.Address { ip: "127.0.0.1", port: 8080 }` |
| **`NetError`** | Ağ başlatma, bağlantı kurma, gönderme veya alma sırasında oluşabilecek hataları listeler. | `ERR_CONNECTION_REFUSED`, `ERR_TIMEOUT`, `ERR_ADDR_IN_USE` |

### 2. Soket Yönetim Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`tcp_connect()`** | Belirtilen uzaktaki adrese (IP ve Port) giden bir TCP bağlantısı kurar. | `fn tcp_connect(address: Address): Result<Socket, NetError>` | Client (istemci) tarafında kullanılır. |
| **`tcp_listen()`** | Belirtilen yerel adreste (Port) yeni bağlantıları dinlemek için bir TCP soketi oluşturur. | `fn tcp_listen(address: Address): Result<Socket, NetError>` | Server (sunucu) tarafında kullanılır. |
| **`tcp_accept()`** | Dinleme soketinde yeni bir gelen bağlantıyı kabul eder ve yeni bir veri soketi döndürür. | `fn tcp_accept(listener: Socket): Result<Socket, NetError>` | Sunucu tarafından çağrıldığında bloklar (bekler). |
| **`udp_open()`** | Belirtilen adreste (Port) veri göndermek/almak için bir UDP soketi açar. | `fn udp_open(address: Address): Result<Socket, NetError>` | |
| **`close()`** | Açık bir soketi kapatır ve sistem kaynaklarını serbest bırakır. | `fn close(socket: Socket): void` | Soket işlemlerinden sonra zorunludur. |

### 3. Veri İletimi Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`send()`** | Bağlı soket üzerinden veri (byte dizisi veya dize) gönderir. Gönderilen byte sayısını döndürür. | `fn send(s: Socket, data: any): Result<i32, NetError>` | |
| **`recv()`** | Soketten veri okur. Okunan veriyi (byte dizisi veya dize) döndürür. Okunacak maksimum boyutu belirtir. | `fn recv(s: Socket, max_size: i32): Result<str, NetError>` | Bloklayan (blocking) bir fonksiyondur. |
| **`send_to()`** | UDP soketi kullanarak belirtilen hedefe veri gönderir. | `fn send_to(s: Socket, address: Address, data: any): Result<i32, NetError>` | |
| **`recv_from()`** | UDP soketinden veri okur ve veriyi gönderen adres bilgisini döndürür. | `fn recv_from(s: Socket, max_size: i32): Result<Tuple<str, Address>, NetError>` | |

### 4. Örnek Kullanım: Basit TCP İstemcisi (Client)

```nim
Nim

void fn TcpClientExample() {
    var server_addr = net.Address { ip: "127.0.0.1", port: 8080 };
    
    // 1. Sunucuya Bağlan
    var connectResult = net.tcp_connect(server_addr);
    
    match connectResult {
        Ok(sock) => {
            io.println("Sunucuya başarıyla bağlandı.");
            
            // 2. Veri Gönder
            var data_to_send: str = "Merhaba Sunucu!";
            var sendResult = net.send(sock, data_to_send);
            
            if sendResult.is_ok() {
                io.println("{sendResult.unwrap()} byte gönderildi.");
            }
            
            // 3. Veri Al
            var recvResult = net.recv(sock, 1024); // Maksimum 1024 byte oku
            
            if recvResult.is_ok() {
                io.println("Sunucudan gelen: {recvResult.unwrap()}");
            } else {
                io.err_print("Veri alma hatası.");
            }

            net.close(sock);
        },
        Err(e) => {
            io.err_print("Bağlantı Hatası: {e}");
        }
    }
}