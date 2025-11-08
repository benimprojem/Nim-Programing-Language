# 🔗 NIMBLE Dili: Dış Fonksiyon Arayüzü (FFI)

## BÖLÜM 18.0: Dış Fonksiyon Arayüzü (FFI) ve C Bağlantıları

FFI (Foreign Function Interface), NIMBLE programlarının, C, C++ veya diğer düşük seviyeli dillerde yazılmış harici kütüphanelerle etkileşim kurmasını sağlayan mekanizmadır. Bu, özellikle sistem seviyesi erişim, işletim sistemi API'leri ve yüksek performanslı mevcut kütüphaneleri kullanmak için hayati öneme sahiptir.

### 1. Temel Söz Dizimi: `extern`

`extern` anahtar kelimesi, bir fonksiyonun veya değişkenin o anki modül tarafından değil, dışarıdan (yani C kütüphanesinden) sağlandığını belirtir.

| Söz Dizimi | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`extern fn`** | Dış kütüphanede tanımlı bir C fonksiyonunu NIMBLE'da bildirme. | Fonksiyon imzası, C'deki karşılığıyla birebir uyumlu olmalıdır. |
| **`extern var`** | Dış kütüphanede tanımlı bir global C değişkenini NIMBLE'da bildirme. | |
| **`#[link(name="<kutuphane_adı>")]`** | Derleyiciye, hangi harici kütüphanenin bağlanması gerektiğini belirtme (Bağlayıcıya (Linker) talimat). |

### 2. C Fonksiyonlarını Bildirme

Bir C kütüphanesini kullanmak için, ilgili fonksiyonların ve değişkenlerin NIMBLE'da "gölge" tanımlarının yapılması gerekir.

**Örnek: `math.h` Kütüphanesinden `sqrt` Fonksiyonunu Kullanma**

```nim
Nim

// Bağlayıcıya, 'm' (matematik kütüphanesi) ile bağlanmasını söyle.
#[link(name="m")] 
export group C_Math {
    
    // extern ile C'deki double sqrt(double x) fonksiyonunu bildiriyoruz.
    // NIMBLE'da f64, C'deki double'a karşılık gelir.
    extern pub fn sqrt(x: f64): f64; 
    
    // extern pub fn sin(x: f64): f64; 
}

// Kullanım
use C_Math;

void fn FFIExample() {
    var sayi: f64 = 16.0;
    
    // Çağrı, C kütüphanesindeki fonksiyona yönlendirilir.
    var kok = C_Math.sqrt(sayi); 
    io.println("Karekök: {kok}"); // Çıktı: 4.0
}

```

3. Veri Tipi Eşleştirme (Bridging)
NIMBLE, FFI için güvenli tip eşleştirmesi kullanır. 
Programcının, NIMBLE tiplerini (i32, f64, str, *T) C'nin karşılık gelen tiplerine (int, double, char*, void*) uygun şekilde kullanması beklenir.
```
NIMBLE Tipi,C Karşılığı,Açıklama
"i32, u64","int, unsigned long long",Boyut eşleştirmesi önemlidir.
"f32, f64","float, double",Standart kayan nokta tipleri.
*T,T* (Pointer),Ham bellek adreslerine karşılık gelir.
str,const char*,Okunabilir dize verisi için (NIMBLE string formatı farklıysa dönüşüm gerekebilir).
```

4. Güvensiz Alan (unsafe)
FFI çağrıları, doğal olarak güvenli olmayan işlemlere (ham işaretçilerle çalışma, bellek yönetimi) izin verdiği için, 
FFI çağrısı yapan NIMBLE kodunun unsafe blokları içine alınması gerekebilir.


Nim

// İşaretçi aritmetiği veya manuel bellek işlemleri için zorunlu olabilir.
unsafe {
    var ptr = C_Memory.malloc(1024);
    // ...
}








