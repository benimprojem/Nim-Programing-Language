# 🧮 NIMBLE Dili: `math` Modülü Detaylandırması

## BÖLÜM 11.5: `math` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`math`** | **Matematiksel Fonksiyonlar ve Sabitler.** Temel trigonometrik, logaritmik, üs alma ve sayı yuvarlama işlemleri için yüksek hassasiyetli fonksiyonlar ve sabit değerler (constants) sağlar. Tüm fonksiyonlar `f64` (çift hassasiyetli float) tipi ile çalışacak şekilde tasarlanmıştır. |

### 1. Matematiksel Sabitler

`math` modülü, yaygın kullanılan matematiksel sabitleri `const` olarak sunar:

| Sabit Adı | Tip | Değer |
| :--- | :--- | :--- |
| **`PI`** | `f64` | Pi sayısı ($\pi$) |
| **`E`** | `f64` | Euler sayısı ($e$) |
| **`INF`** | `f64` | Pozitif Sonsuzluk (Infinity) |
| **`NAN`** | `f64` | Sayı Değil (Not a Number) |

### 2. Temel Aritmetik ve Üs Alma

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`abs()`** | Sayının mutlak değerini döndürür. | `fn abs(x: f64): f64` |
| **`pow()`** | Bir sayının (taban) başka bir sayıya (üs) yükseltilmiş değerini döndürür ($x^y$). | `fn pow(x: f64, y: f64): f64` |
| **`sqrt()`** | Sayının karekökünü döndürür ($\sqrt{x}$). | `fn sqrt(x: f64): f64` |
| **`exp()`** | $e$ sabitinin verilen üsse yükseltilmiş değerini döndürür ($e^x$). | `fn exp(x: f64): f64` |
| **`log()`** | Sayının doğal logaritmasını ($e$ tabanında) döndürür ($\ln x$). | `fn log(x: f64): f64` |
| **`log10()`** | Sayının 10 tabanında logaritmasını döndürür ($\log_{10} x$). | `fn log10(x: f64): f64` |

### 3. Yuvarlama (Rounding) ve Kırpma

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`round()`** | Sayıyı en yakın tamsayıya yuvarlar. (0.5 durumunda en yakın çift sayıya yuvarlama gibi standart yöntemler uygulanır). | `fn round(x: f64): f64` |
| **`floor()`** | Sayıyı kendisinden küçük veya eşit en büyük tamsayıya yuvarlar (aşağı yuvarlama). | `fn floor(x: f64): f64` |
| **`ceil()`** | Sayıyı kendisinden büyük veya eşit en küçük tamsayıya yuvarlar (yukarı yuvarlama). | `fn ceil(x: f64): f64` |
| **`trunc()`** | Sayının ondalık kısmını keser (sıfıra doğru kırpar). | `fn trunc(x: f64): f64` |

### 4. Trigonometri

Tüm trigonometrik fonksiyonlar **radyan** cinsinden giriş kabul eder.

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`sin()`** | Sinüs değerini hesaplar. | `fn sin(x: f64): f64` |
| **`cos()`** | Kosinüs değerini hesaplar. | `fn cos(x: f64): f64` |
| **`tan()`** | Tanjant değerini hesaplar. | `fn tan(x: f64): f64` |
| **`atan2()`** | İki değişkenli ArkTanjantı hesaplar ($atan2(y, x)$). Koordinat düzleminde açıyı bulmak için kullanılır. | `fn atan2(y: f64, x: f64): f64` |
| **`deg_to_rad()`** | Dereceyi radyana dönüştürür. | `fn deg_to_rad(deg: f64): f64` |
| **`rad_to_deg()`** | Radyanı dereceye dönüştürür. | `fn rad_to_deg(rad: f64): f64` |

### 5. Örnek Kullanım: Matematiksel Hesaplamalar

```nim
Nim

void fn MathExample() {
    var yaricap: f64 = 5.0;
    
    // Sabit Kullanımı
    var alan = math.PI * math.pow(yaricap, 2.0);
    print("Alan: {alan}"); // Örn: 78.5398...

    // Trigonometri
    var derece: f64 = 45.0;
    var radyan = math.deg_to_rad(derece);
    var sin_degeri = math.sin(radyan);
    print("Sin(45): {sin_degeri}"); // Örn: 0.707...

    // Karekök ve Mutlak Değer
    var val1: f64 = math.sqrt(81.0); // 9.0
    var val2: f64 = math.abs(-15.5); // 15.5
    print("Karekök: {val1}, Mutlak: {val2}");

    // Yuvarlama
    var sayi: f64 = 4.7;
    print("Yuvarlama (ceil): {math.ceil(sayi)}"); // 5.0
    print("Yuvarlama (floor): {math.floor(sayi)}"); // 4.0
}