# 🔄 NIMBLE Dili: `types` Modülü Detaylandırması

## BÖLÜM 11.9: `types` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`types`** | **Tip Dönüşümleri ve Kontrolü.** Farklı veri tipleri (tamsayı, kayan nokta, dize vb.) arasında güvenli (explicit) ve güvensiz (unsafe) dönüşümler için araçlar sağlar. Güvenli dönüşümler veri kaybını önlemek için tasarlanmıştır. |

### 1. Güvenli Dönüşümler (Explicit Casting)

Veri kaybı riskinin yüksek olduğu veya dönüşümün açıkça belirtilmesi gerektiği durumlar için kullanılır. Başarısızlık durumunda (örneğin dizeden sayıya dönüşümde) `Result<T, E>` veya `Option<T>` döndürür.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`to_i32()`** | Bir değeri (örn: `str`, `f64`) 32-bit tamsayıya (`i32`) dönüştürür. | `fn to_i32(val: any): Result<i32, ConvertError>` | String'den sayıya dönüşümde hata kontrolü sağlar. |
| **`to_f64()`** | Bir değeri 64-bit kayan noktaya (`f64`) dönüştürür. | `fn to_f64(val: any): Result<f64, ConvertError>` | |
| **`to_str()`** | Bir değeri (örn: `i32`, `f64`, `bool`) dizeye (`str`) dönüştürür. (Genellikle otomatik olarak yapılır ancak manuel kontrol için sunulur.) | `fn to_str(val: any): str` | Bu dönüşüm her zaman başarılıdır. |
| **`parse()`** | Bir dizeden belirli bir hedef tipe dönüşüm yapar. Tip, Generic `<T>` ile belirtilir. | `fn parse<T>(s: str): Result<T, ConvertError>` | `var num = types.parse<i32>("123");` |

### 2. Güvensiz/Doğrudan Dönüşümler (Unsafe Casting)

Derleyiciye, veri kaybını veya hatalı yorumlamayı göz ardı ederek tipleri doğrudan dönüştürmesi talimatını verir. Yüksek performans gerektiren düşük seviye sistem programlamasında kullanılır.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`as_type()`** | Bir değeri zorla (force) belirtilen hedef tipe dönüştürür. | `fn as_type<T>(val: any): T` | `var val_i32 = val_f64.as_type<i32>();` (Ondalık kısım kesilebilir!) |
| **`cast()`** | Verinin sadece bellekteki bitlerinin farklı bir tip olarak yorumlanmasını sağlar (Reinterpret/Bitwise Cast). Çok tehlikelidir. | `fn cast<T>(val: any): T` | Örneğin, bir `i32`'nin bitlerini `f32` olarak okumak. |

### 3. Tip Kontrolü ve Doğrulama

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`is_type()`** | Bir değişkenin çalışma zamanındaki tipinin, belirtilen tip olup olmadığını kontrol eder. | `fn is_type<T>(val: any): bool` | `if myVar.is_type<str>(): ...` |
| **`name_of()`** | Bir değişkenin çalışma zamanındaki tip adını dize olarak döndürür. | `fn name_of(val: any): str` | `print(types.name_of(myVar)); // "i32"` |

### 4. Hata Yönetimi

Güvenli dönüşüm fonksiyonları için hata tipi:

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`ConvertError`** | Tip dönüşümü sırasında oluşabilecek hataları listeler. | `ERR_INVALID_FORMAT` (Dize sayı değil), `ERR_OVERFLOW` (Hedef tipe sığmıyor) |

### 5. Örnek Kullanım: Güvenli ve Güvensiz Dönüşümler

```nim
Nim

void fn TypesExample() {
    var float_val: f64 = 123.45;
    var string_val: str = "42";
    var bad_string: str = "abc";

    // Güvenli Dönüşüm (f64 -> i32)
    // Veri kaybı riski nedeniyle explicit olmalı, ancak bu durumda Result gereksizdir.
    var int_val_safe: i32 = types.to_i32(float_val).as_type<i32>(); // 123
    print("Güvenli Dönüşüm (int): {int_val_safe}");

    // Güvenli String Ayrıştırma (str -> i32)
    var parseResult = types.parse<i32>(string_val);
    match parseResult {
        Ok(i) => { print("Başarılı parse: {i}"); }, // 42
        Err(e) => { print("Hata: {e}"); }
    }

    // Başarısız String Ayrıştırma
    var badParse = types.parse<i32>(bad_string);
    if badParse.is_err() {
        print("Hata: 'abc' sayıya dönüştürülemedi.");
    }
    
    // Güvensiz/Zorla Dönüşüm (Veri kaybı riski)
    var large_i32: i32 = 500;
    // float_val'dan i32'ye zorla dönüşüm (as_type) ile ondalık kesilir.
    var forced_i32: i32 = float_val.as_type<i32>(); 
    print("Zorla Dönüşüm (kesme): {forced_i32}"); // 123
}