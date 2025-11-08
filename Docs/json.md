# 🌐 NIMBLE Dili: `json` Modülü Detaylandırması

## BÖLÜM 11.6: `json` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`json`** | **JSON Veri İşleme.** JSON formatındaki dize verilerini NIMBLE'ın dinamik yapılarına dönüştürmek (Parse) ve NIMBLE verilerini JSON formatına geri dönüştürmek (Stringify) için fonksiyonlar sağlar. Web API'leri ile iletişim kurmak ve konfigürasyon dosyalarını yönetmek için kritik öneme sahiptir. |

### 1. Temel Yapılar ve Tipler

JSON verileri yapı olarak dinamik ve karmaşık olabileceğinden, modülün tek bir dinamik tip kullanması ve hata durumlarını `Result` ile yönetmesi gerekir.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`JsonValue`** | JSON içindeki herhangi bir değeri (nesne, dizi, string, sayı, boolean, null) temsil eden dinamik tip. | `json.parse()` fonksiyonunun başarılı dönüş tipidir. İçerik tipi, `as_str()`, `as_i32()` gibi metotlarla doğrulanarak alınmalıdır. |
| **`JsonError`** | JSON ayrıştırma (parsing) veya serileştirme (stringifying) sırasında oluşabilecek olası hataları listeler. | `ERR_SYNTAX`, `ERR_INVALID_TYPE` |

### 2. Dönüştürme (Parse ve Stringify) Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`parse()`** | JSON formatındaki bir dizeyi (str), NIMBLE'ın dinamik `JsonValue` yapısına dönüştürür. | `fn parse(json_string: str): Result<JsonValue, JsonError>` |
| **`stringify()`** | Bir NIMBLE yapısını (genellikle `JsonValue`, `map` veya `struct`) JSON formatında bir dizeye dönüştürür. | `fn stringify(data: any): Result<str, JsonError>` |
| **`format()`** | Serileştirilmiş JSON dizesini daha okunabilir (girintili) hale getirir. | `fn format(json_string: str, indent_spaces: i32): Result<str, JsonError>` |

### 3. `JsonValue` Metotları (Veriye Erişim)

`JsonValue` dinamik bir tip olduğundan, veriye erişim için özel doğrulama metotları (metot zinciri desteği ile) kullanılmalıdır.

| Fonksiyon | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`get()`** | Bir `JsonValue` içindeki anahtarla (key) alt değere erişir. (Nesneler için). | `json_val.get("anahtar").as_str().unwrap()` |
| **`at()`** | Bir `JsonValue` içindeki indeksteki değere erişir. (Diziler için). | `json_val.at(0).as_i32().unwrap()` |
| **`is_object()`**, `is_array()`, `is_str()` vb. | İçeriğin tipini kontrol eder. | `if json_val.is_object(): ...` |
| **`as_str()`**, `as_i32()`, `as_bool()` vb. | İçeriği belirtilen NIMBLE tipine dönüştürür (`Option<T>` döndürerek güvenliği sağlar). | `var isim: str = json_val.get("name").as_str().unwrap_or("Bilinmiyor");` |

### 4. Örnek Kullanım: JSON Ayrıştırma (Parsing)

```nim
Nim

void fn JsonExample() {
    var json_str: str = "{\"ad\":\"NIMBLE\",\"versiyon\":1.0,\"aktif\":true,\"liste\":[10, 20]}";
    
    var parseResult = json.parse(json_str);
    
    match parseResult {
        Ok(data) => {
            // Verilere güvenli erişim
            var ad = data.get("ad").as_str().unwrap_or("?");
            var versiyon = data.get("versiyon").as_f64().unwrap_or(0.0);
            
            print("Adı: {ad}");        // NIMBLE
            print("Versiyon: {versiyon}"); // 1.0

            // Diziye erişim
            var ilk_sayi = data.get("liste").at(0).as_i32().unwrap_or(-1);
            print("Listedeki ilk sayı: {ilk_sayi}"); // 10
        },
        Err(error) => {
            print("JSON Ayrıştırma Hatası: {error}");
        }
    }
}