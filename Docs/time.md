# ⏳ NIMBLE Dili: `time` Modülü Detaylandırması

## BÖLÜM 11.13: `time` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`time`** | **Zaman ve Tarih Yönetimi.** Sistem saatiyle etkileşim, zaman damgası (timestamp) oluşturma, süreleri (duration) hesaplama ve tarih/saat formatlama işlemleri için temel araçları sağlar. |

### 1. Temel Yapılar ve Tipler

Zaman, farklı hassasiyet ve formatlarda tutulmalıdır.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Timestamp`** | Belirli bir anı temsil eden, genellikle Unix Epoch'tan (1 Ocak 1970) bu yana geçen saniye sayısını tutan sayısal değer. | `i64` veya `f64` olarak saklanabilir. |
| **`DateTime`** | Yıl, ay, gün, saat, dakika, saniye gibi bileşenlere ayrılmış yapılandırılmış tarih ve zaman. | Formatlama ve aritmetik işlemler için kullanılır. |
| **`Duration`** | İki an arasındaki süreyi (farkı) temsil eden değer (genellikle saniye veya milisaniye cinsinden). | Performans ölçümleri ve bekleme (sleep) süreleri için kullanılır. |

### 2. Anlık Zaman Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`now()`** | Mevcut anın zaman damgasını (`Timestamp`) döndürür. Yüksek hassasiyetlidir. | `fn now(): Timestamp` | |
| **`utc_now()`** | UTC (Evrensel Zaman Koordinatı) cinsinden mevcut anın `DateTime` yapısını döndürür. | `fn utc_now(): DateTime` | |
| **`local_now()`** | Sistemin yerel saat dilimine göre mevcut anın `DateTime` yapısını döndürür. | `fn local_now(): DateTime` | |
| **`sleep()`** | İş parçacığının (thread) yürütülmesini belirtilen süre (milisaniye) kadar duraklatır. | `fn sleep(ms: i32): void` | Genellikle `thread` modülünde de bir karşılığı olabilir, ancak burada temel gecikme için sunulur. |

### 3. Dönüşüm ve Formatlama Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`from_timestamp()`** | Bir `Timestamp` değerini `DateTime` yapısına dönüştürür. | `fn from_timestamp(ts: Timestamp): DateTime` | |
| **`format()`** | Bir `DateTime` yapısını, belirtilen formata göre dizeye (`str`) dönüştürür. | `fn format(dt: DateTime, format_str: str): str` | Format dizesi `%Y-%m-%d %H:%M:%S` gibi standartları takip eder. |
| **`parse()`** | Belirtilen formattaki bir dizeyi `DateTime` yapısına ayrıştırır. | `fn parse(time_str: str, format_str: str): Result<DateTime, TimeError>` | |

### 4. Süre Ölçümü Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`measure_start()`** | Yüksek çözünürlüklü zamanlayıcıyı başlatır (Süre ölçümü için). | `fn measure_start(): Timestamp` | |
| **`measure_end()`** | `measure_start()` ile başlayan süreyi ölçer ve `Duration` tipinde döndürür. | `fn measure_end(start_time: Timestamp): Duration` | |
| **`duration_ms()`** | Bir `Duration` değerini milisaniye cinsinden `f64` olarak döndürür. | `fn duration_ms(d: Duration): f64` | |

### 5. Örnek Kullanım: Süre Ölçümü ve Formatlama

```nim
Nim

void fn TimeExample() {
    // 1. Süre Ölçümü (Performans Testi)
    var start_time = time.measure_start();
    
    // Simüle Edilmiş Uzun İşlem
    var i: i32 = 0;
    while i < 10000000 {
        i = i + 1;
    }
    
    var sure = time.measure_end(start_time);
    var sure_ms = time.duration_ms(sure);
    io.println("İşlem süresi: {sure_ms} milisaniye.");

    // 2. Anlık Zaman ve Formatlama
    var simdi_utc = time.utc_now();
    
    // Yıl-Ay-Gün Saat:Dakika:Saniye formatı
    var formatli_zaman = time.format(simdi_utc, "%Y-%m-%d %H:%M:%S");
    io.println("UTC Zamanı: {formatli_zaman}");

    // 3. Uyku (Sleep)
    io.println("200 ms bekliyor...");
    time.sleep(200);
    io.println("Bekleme bitti.");
}