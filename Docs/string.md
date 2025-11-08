# ✍️ NIMBLE Dili: `string` Modülü Detaylandırması

## BÖLÜM 11.4: `string` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`string`** | **Dize İşlemleri.** Metin (string) değişkenleri üzerinde arama, değiştirme, formatlama, büyük/küçük harf dönüştürme ve parçalama (splitting) gibi yaygın işlemleri gerçekleştirmek için yardımcı fonksiyonlar sağlar. |

### 1. Temel Bilgi ve Dönüştürme Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`len()`** | Dizedeki karakter sayısını (uzunluğunu) döndürür. | `fn len(s: str): i32` |
| **`is_empty()`** | Dizenin boş olup olmadığını kontrol eder. | `fn is_empty(s: str): bool` |
| **`to_upper()`** | Dizedeki tüm harfleri büyük harfe dönüştürür. | `fn to_upper(s: str): str` |
| **`to_lower()`** | Dizedeki tüm harfleri küçük harfe dönüştürür. | `fn to_lower(s: str): str` |
| **`trim()`** | Dizenin başındaki ve sonundaki boşluk karakterlerini kaldırır. | `fn trim(s: str): str` |
| **`word_count()`** | Dizedeki kelime sayısını (boşluklarla ayrılmış) döndürür. | `fn word_count(s: str): i32` |

### 2. Arama ve Karşılaştırma Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`contains()`** | Dizenin, verilen alt dizeyi içerip içermediğini kontrol eder. | `fn contains(s: str, substring: str): bool` |
| **`starts_with()`** | Dizenin, verilen alt dize ile başlayıp başlamadığını kontrol eder. | `fn starts_with(s: str, prefix: str): bool` |
| **`ends_with()`**** | Dizenin, verilen alt dize ile bitip bitmediğini kontrol eder. | `fn ends_with(s: str, suffix: str): bool` |
| **`find()`** | Verilen alt dizeyi dizede arar ve bulursa **ilk indeksini** döndürür. Bulamazsa `Option<i32>` döndürür. | `fn find(s: str, substring: str): Option<i32>` |

### 3. Manipülasyon ve Parçalama Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`replace()`** | Dizedeki tüm eşleşen alt dizeleri yeni bir dize ile değiştirir. | `fn replace(s: str, old: str, new: str): str` |
| **`substring()`** | Belirtilen başlangıç indeksinden (dahil) ve uzunluktan (veya bitiş indeksinden) bir alt dize döndürür. | `fn substring(s: str, start: i32, length: i32): str` |
| **`split()`** | Dizeyi, verilen ayırıcıya (delimiter) göre parçalar ve bir dinamik `array<str>` döndürür. | `fn split(s: str, delimiter: str): str[]` |
| **`join()`** | Bir dize dizisindeki elemanları, verilen ayırıcı ile birleştirerek tek bir dize oluşturur. | `fn join(arr: str[], delimiter: str): str` |

### 4. Örnek Kullanım: Dize İşlemleri

```nim
Nim

void fn StringExample() {
    var metin: str = "Merhaba Dünya, NIMBLE harika!";
    
    // Temel Bilgiler
    print("Uzunluk: {metin.len()}"); // 28
    
    // Dönüştürme
    var upper = metin.to_upper(); 
    print("Büyük Harf: {upper}"); // MERHABA DÜNYA, NIMBLE HARIKA!

    // Arama ve Değiştirme
    if metin.contains("NIMBLE") {
        print("NIMBLE bulundu.");
        metin = metin.replace("harika", "mükemmel"); 
        print("Yeni Metin: {metin}");
    }

    // Parçalama
    var kelimeler = metin.split(" ");
    print("Kelime Sayısı: {kelimeler.count()}"); // 4
    
    // Birleştirme
    var yeniMetin = string.join(kelimeler, "_");
    print("Birleştirilmiş: {yeniMetin}"); // Merhaba_Dünya,_NIMBLE_mükemmel!
}