# ⚙️ NIMBLE Dili: Derleme Direktifleri ve Ön İşlemci

## BÖLÜM 14.0: Ön İşlemci ve Derleme Direktifleri

NIMBLE, kodun derleme aşamasını etkilemek, platforma özgü kod parçalarını dahil etmek veya hariç tutmak ve derleyiciye özel talimatlar vermek için bir dizi direktif (yönerge) kullanır. Bu direktifler, genellikle **`#`** sembolü ile başlar ve derleme öncesi aşamada işlenir.

### 1. Şartlı Derleme Direktifleri

Bu direktifler, kodun belirli koşullar altında derlenmesini sağlar.

| Direktif | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`#ifdef`** | Belirtilen sembol tanımlıysa kodu derler. | `#ifdef <SEMBO L>` | `#define DEBUG` gibi bir sembol varlığını kontrol eder. |
| **`#ifndef`** | Belirtilen sembol tanımlı değilse kodu derler. | `#ifndef <SEMBO L>` | |
| **`#if`** | Belirtilen koşul (derleme zamanı ifadesi) doğruysa kodu derler. | `#if <koşul>` | Koşul, sabit ifadeler, `const` değerler veya tanımlı semboller içerebilir. |
| **`#elif`** | Önceki `#if` başarısız olursa yeni bir koşulu kontrol eder. | `#elif <koşul>` | |
| **`#else`** | Önceki şartlar başarısız olursa kodu derler. | `#else` | |
| **`#endif`** | Şartlı derleme bloğunu sonlandırır. | `#endif` | |

**Örnek: Platforma Özgü Kod ve DEBUG Bayrağı**

```nim
Nim

#ifdef WINDOWS
    io.println("Windows için özel kod çalışıyor.");
#elif LINUX 
    io.println("Linux için özel kod çalışıyor.");
#endif

#ifdef DEBUG 
    log.debug("Debug modu aktif.");
#endif
```
2. Ön İşlemci Makroları ve Metin İkamesi
```#define direktifi, basit metin ikamesi (text substitution) ve parametreli makrolar için kullanılır. Bu işlem, kod analizi ve derlemeden önce gerçekleşir.```

2.1 Sabit Makrolar ve Metin İkamesi

Direktif,Amaç,Açıklama
```#define <AD> <DEĞER>,"Derleme zamanında <AD> sembolünün geçtiği her yerde, <DEĞER> ile değiştirilmesini sağlar.",```


Örnek: Kod Parçası İkamesi
```
Nim

#define APPLICATION_NAME "NIMBLE v1.0"
#define HATA_KAPATMA io.err_print("Hata! Kapatılıyor."); os.exit(1);

void fn Main() {
    io.println(APPLICATION_NAME);
    // ...
    HATA_KAPATMA // Buraya iki satır kod yerleştirilir.
}
```

2.2 Parametreli Makrolar
Makronun argüman alarak daha dinamik metin ikamesi gerçekleştirmesini sağlar.


Söz Dizimi,Amaç
```
"#define <AD>(<arg1>, ...)",Argümanları kullanarak kod parçası üreten makro.
```
Örnek: Parametreli Makro
```
Nim

// Makro, argümanları doğrudan metin olarak değiştirir.
#define MIN(a, b) ( (a) < (b) ? (a) : (b) )

void fn MacroExample() {
    var x = 10;
    var y = 20;
    
    // Derleme öncesi: ( (x) < (y) ? (x) : (y) )
    var kucuk = MIN(x, y);
}
```

3. Dosya Dahil Etme ve Akış Direktifleri
```
Direktif,Amaç,Söz Dizimi,Açıklama
#include,"Belirtilen dosyanın içeriğini, derleme sırasında direktifin bulunduğu yere ekler (Kopyala/Yapıştır).","#include ""baslik.nim""",use (mantıksal modül dahil etme) yerine düşük seviyede kullanılır.
#undef,Daha önce tanımlanmış bir sembolün tanımını kaldırır.,#undef <SEMBO L>,
```

4. Hata ve Optimizasyon Direktifleri
```
Direktif,Amaç,Söz Dizimi,Açıklama
#error,Derleme işlemini durdurur ve belirtilen mesajı hata olarak gösterir.,"#error ""<mesaj>""",
#warning,"Derleme işlemini durdurmaz, ancak uyarı gösterir.","#warning ""<mesaj>""",
#optimize,Fonksiyon veya blok seviyesinde derleyici optimizasyonlarını kontrol eder.,#optimize speed,
#inline,"Derleyiciye, bir fonksiyonu çağrıldığı yerde satır içine almayı (inline) denemesi için talimat verir.",#inline fn my_func() { ... },
```













