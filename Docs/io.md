# 💬 NIMBLE Dili: `io` Modülü Detaylandırması

## BÖLÜM 11.8: `io` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`io`** | **Standart Girdi/Çıktı (Console I/O).** Programın terminal/konsol aracılığıyla kullanıcıdan veri alması (`stdin`) ve kullanıcıya veri göstermesi (`stdout`, `stderr`) için temel fonksiyonları sağlar. |

### 1. Standart Çıktı Fonksiyonları (`stdout`)

Bu fonksiyonlar, veriyi programın standart çıktı akışına (genellikle terminal) yazar.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`print()`** | Verilen metni veya değişkeni standart çıktıya yazar. **Satır sonu karakteri (newline) eklemez.** | `fn print(data: any): void` | `io.print("Hello");` |
| **`println()`** | Verilen metni veya değişkeni standart çıktıya yazar ve **ardından bir satır sonu karakteri ekler.** | `fn println(data: any): void` | `io.println("World!");` |
| **`flush()`** | Çıktı tamponundaki (buffer) bekleyen tüm verileri hemen ekrana yazılmaya zorlar. | `fn flush(): void` | Performans kritiktir. |
| **`err_print()`** | Verilen metni **Standart Hata Çıktısı** (`stderr`) akışına yazar. Hata mesajları için kullanılır. | `fn err_print(data: any): void` | `io.err_print("Hata oluştu!");` |

### 2. Standart Girdi Fonksiyonları (`stdin`)

Bu fonksiyonlar kullanıcıdan girdi almak için kullanılır.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`input()`** | Kullanıcıdan bir satır metin okur ve bunu dize (`str`) olarak döndürür. Okuma başarısız olursa `Result<str, Error>` döndürür. | `fn input(): Result<str, IoError>` | `var girdi = io.input();` |
| **`prompt()`** | Kullanıcıdan girdi almadan önce bir mesaj (prompt) gösterir ve ardından girdiyi okur. | `fn prompt(message: str): Result<str, IoError>` | `var isim = io.prompt("Adınız: ");` |
| **`read_char()`** | Kullanıcıdan sadece tek bir karakter okur. | `fn read_char(): Result<char, IoError>` | `var secim = io.read_char();` |

### 3. Hata Yönetimi

Standart G/Ç işlemleri de kesintiye uğrayabileceği için hata tipleri tanımlanmıştır.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`IoError`** | Girdi/Çıktı işlemlerinde oluşabilecek hataları listeler. | `ERR_READ_FAILED`, `ERR_EOF` (Dosya Sonu) |

### 4. Örnek Kullanım: Kullanıcıdan Veri Alma

```nim
Nim

void fn ConsoleExample() {
    // 1. Kullanıcıdan prompt ile girdi alma
    var isimResult = io.prompt("Lütfen adınızı girin: ");
    
    match isimResult {
        Ok(isim) => {
            io.println("Merhaba, {isim}!");

            // 2. Sayısal girdi alma ve dönüştürme
            var yasResult = io.prompt("Lütfen yaşınızı girin: ");
            
            match yasResult {
                Ok(yas_str) => {
                    // String'i tamsayıya dönüştürme
                    // NIMBLE'ın string modülünde veya temel dilde parse fonksiyonu olduğu varsayılır.
                    var yas: i32 = string.to_i32(yas_str).unwrap_or(0);
                    io.println("Demek ki {yas} yaşındasınız.");
                },
                Err(error) => {
                    io.err_print("Girdi okuma hatası oluştu.");
                }
            }
        },
        Err(error) => {
            io.err_print("İsim okunamadı.");
        }
    }
}