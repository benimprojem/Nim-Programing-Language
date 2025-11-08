# 🎲 NIMBLE Dili: `rand` Modülü Detaylandırması

## BÖLÜM 11.16: `rand` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`rand`** | **Rastgele Sayı Üretimi.** Uygulama ihtiyaçlarına göre hem standart (hızlı, deterministik) hem de kriptografik olarak güvenli (non-deterministik) rastgele sayılar üretmek için fonksiyonlar ve yapılar sağlar. |

### 1. Temel Yapılar ve Tipler

Rastgele sayı üreteci (RNG), bir durum (state) tutar ve bu durumun yönetilmesi gerekir.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Rng`** | Standart, hızlı rastgele sayı üreteci örneği. Genellikle bir algoritma (Xoroshiro, Mersenne Twister gibi) ile uygulanır. | `rand.new_rng()` ile oluşturulur. |
| **`SecureRng`** | Kriptografik olarak güvenli rastgele sayı üreteci (CSPRNG). Güvenlik ve anahtar üretimi için işletim sistemi kaynaklarını kullanır. | `rand.new_secure()` ile oluşturulur. |
| **`Seed`** | Üretecin başlangıç durumunu belirleyen değer. Aynı seed, aynı rastgele sayı dizisini üretir. | |

### 2. Üreteç Yönetimi

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`new_rng()`** | Sistemin anlık zamanını kullanarak (veya varsayılan bir yolla) yeni bir hızlı `Rng` örneği oluşturur. | `fn new_rng(): Rng` | |
| **`new_rng_seeded()`** | Belirtilen bir `Seed` değeri kullanarak deterministik bir `Rng` örneği oluşturur. | `fn new_rng_seeded(seed: Seed): Rng` | Tekrarlanabilir simülasyonlar için kullanılır. |
| **`new_secure()`** | Yeni bir kriptografik olarak güvenli `SecureRng` örneği oluşturur. | `fn new_secure(): SecureRng` | |
| **`set_seed()`** | Mevcut bir `Rng` örneğinin durumunu yeni bir seed ile sıfırlar. | `fn set_seed(r: Rng, seed: Seed): void` | |

### 3. Rastgele Değer Üretme Fonksiyonları

Üretim fonksiyonları hem `Rng` hem de `SecureRng` üzerinde çağrılabilir olmalıdır.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`i32()`** | Rastgele bir 32-bit tamsayı (`i32`) döndürür. | `fn i32(r: Rng): i32` | |
| **`f64()`** | 0.0 (dahil) ile 1.0 (hariç) arasında rastgele bir kayan nokta (`f64`) döndürür. | `fn f64(r: Rng): f64` | |
| **`range_i32()`** | Belirtilen `min` (dahil) ve `max` (hariç) sınırları arasında rastgele bir `i32` döndürür. | `fn range_i32(r: Rng, min: i32, max: i32): i32` | |
| **`choice()`** | Verilen diziden rastgele bir eleman seçer. | `fn choice<T>(r: Rng, arr: T[]): T` | |
| **`bytes()`** | Kriptografik amaçlar için istenen uzunlukta rastgele byte dizisi (`u8[]`) döndürür. (Genellikle `SecureRng` ile kullanılır). | `fn bytes(r: SecureRng, size: i32): u8[]` | |

### 4. Örnek Kullanım: Simülasyon ve Güvenlik

```nim
Nim

void fn RandExample() {
    // 1. Standart RNG (Simülasyon/Oyun)
    var fast_rng = rand.new_rng();
    
    // 1'den 100'e (100 hariç) rastgele sayı üret
    var zar_atisi = fast_rng.range_i32(1, 100); 
    io.println("Rastgele Sayı: {zar_atisi}");
    
    // Kayan nokta üretimi
    var oran = fast_rng.f64();
    io.println("Kayan Oran: {oran}"); 

    // Diziden seçim
    var isimler: str[] = {"Ali", "Veli", "Ayşe", "Fatma"};
    var sansli_isim = fast_rng.choice(isimler);
    io.println("Şanslı İsim: {sansli_isim}");

    // 2. Güvenli RNG (Kriptografik)
    var secure_rng = rand.new_secure();

    // 32 byte uzunluğunda güvenli anahtar üretimi
    var anahtar_bytes = secure_rng.bytes(32);
    io.println("Üretilen Güvenli Anahtarın Uzunluğu: {anahtar_bytes.count()}"); 
    // Anahtar, genellikle string olarak değil, byte dizisi olarak işlenir.
}