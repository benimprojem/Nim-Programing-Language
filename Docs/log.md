# 📝 NIMBLE Dili: `log` Modülü Detaylandırması

## BÖLÜM 11.15: `log` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`log`** | **Uygulama İçi Loglama ve İzleme.** Çalışma zamanı olaylarını, hataları ve hata ayıklama bilgilerini yapılandırılmış seviyeler (level) kullanarak kaydetmek için bir arayüz sağlar. Log çıktıları, konsol/terminal, dosya veya ağ gibi çeşitli hedeflere yönlendirilebilir. |

### 1. Temel Yapılar ve Tipler

Loglama, mesajları kategorize etmek için standart seviyelere ihtiyaç duyar.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Level`** | Log mesajının önem derecesini belirleyen sabitler. | `log.DEBUG`, `log.INFO`, `log.WARN`, `log.ERROR`, `log.FATAL` |
| **`Logger`** | Loglama ayarlarını (seviye, format, hedef) tutan yapı. Birden fazla hedefe log atmayı sağlar. | `log.new()` ile oluşturulur. |
| **`Target`** | Logların yazılacağı hedefi (dosya, konsol, ağ) belirten yapı. | `log.TARGET_CONSOLE`, `log.TARGET_FILE` |

### 2. Yapılandırma ve Yönetim Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`new()`** | Yeni bir `Logger` örneği oluşturur. | `fn new(target: Target): Logger` | |
| **`set_level()`** | Logger'ın minimum loglama seviyesini ayarlar. Bu seviyeden daha düşük önemdeki mesajlar göz ardı edilir. | `fn set_level(level: Level): void` | `log.set_level(log.INFO);` |
| **`set_format()`** | Log mesajlarının çıktıda nasıl görüneceğini belirten bir format şablonu ayarlar. | `fn set_format(format_str: str): void` | Örn: `"{time} [{level}] {message}"` |
| **`add_target()`** | Logların yazılacağı ek bir hedef (örneğin bir dosya) ekler. | `fn add_target(l: Logger, target: Target): Result<void, LogError>` | |

### 3. Loglama Fonksiyonları

Bu fonksiyonlar, mesajları ilgili önem seviyesine göre kaydeder.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`debug()`** | Hata ayıklama veya ince taneli izleme bilgisi kaydeder. | `fn debug(msg: str): void` | Yalnızca geliştirme sırasında yararlıdır. |
| **`info()`** | Genel uygulama akışı hakkında bilgi kaydeder. | `fn info(msg: str): void` | `log.info("Kullanıcı oturum açtı.");` |
| **`warn()`** | Potansiyel sorunları veya beklenmeyen durumları kaydeder, ancak programın çalışmasını engellemez. | `fn warn(msg: str): void` | |
| **`error()`** | Uygulama hatası veya kurtarılabilir bir sorun kaydeder. | `fn error(msg: str): void` | `log.error("Dosya açılırken hata oluştu.");` |
| **`fatal()`** | Kritik, programın sonlandırılmasını gerektiren geri döndürülemez hataları kaydeder. | `fn fatal(msg: str): void` | Genellikle `os.exit()` ile sonlanır. |

### 4. Örnek Kullanım: Yapılandırılmış Loglama

```nim
Nim

void fn LogExample() {
    // 1. Varsayılan Logger'ı Yapılandırma
    log.set_level(log.INFO);
    log.set_format("[{time} - {level}] {message}");
    
    // Konsol çıktısı
    log.info("Uygulama başarıyla başlatılıyor..."); 
    // Çıktı: [2025-11-08 02:11:00 - INFO] Uygulama başarıyla başlatılıyor...
    
    // DEBUG mesajı ayar seviyesinden düşük olduğu için göz ardı edilir
    log.debug("Veritabanı bağlantı denemesi."); 
    
    // 2. Özel bir Logger oluşturma (Dosyaya Loglama)
    var fileTarget = log.Target { type: log.TARGET_FILE, path: "app.log" };
    var fileLogger = log.new(fileTarget);
    fileLogger.set_level(log.DEBUG); // Dosyaya her şeyi kaydet
    
    // 3. Özel Logger Kullanımı
    if some_condition.is_false() {
        var hata_mesaji: str = "Yapılandırma dosyası eksik.";
        
        log.warn(hata_mesaji); // Konsola WARN yazar
        fileLogger.error(hata_mesaji); // Dosyaya ERROR yazar
    }
    
    // Kritik Hata
    log.fatal("Bellek tahsis edilemedi. Program sonlandırılıyor.");
}