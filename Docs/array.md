# 🔢 NIMBLE Dili: `array` Modülü Detaylandırması

## BÖLÜM 11.3: `array` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`array`** | **Dinamik Dizi (Array) İşlevleri.** Çalışma zamanında boyutu değiştirilebilen, aynı tipteki elemanları sıralı olarak tutan koleksiyonlar için yardımcı metotlar ve ileri düzey işlemler sağlar. |

### 1. Tanımlama Söz Dizimi (Basitleştirilmiş)

Dinamik diziler için özel `array.new()` fonksiyonuna gerek yoktur. Dizi tipi, eleman tipinin arkasına gelen `[]` ile tanımlanır ve başlangıçta boş kabul edilir.

| Söz Dizimi | Açıklama |
| :--- | :--- |
| `var myArray: i32[];` | `i32` tipinde, boş ve dinamik bir dizi tanımlar. |
| `var stringList: str[];` | `string` tipinde, boş ve dinamik bir dizi tanımlar. |
| **İlklendirme:** `var nums: i32[] = {1, 2, 3};` | Elemanlarla birlikte ilklendirme. |

### 2. Temel Yönetim Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`count()`** | Dizideki mevcut eleman sayısını döndürür. | `fn count<T>(list: arr<T>): i32` |
| **`capacity()`** | Dizinin yeniden bellek tahsis etmeden tutabileceği maksimum eleman sayısını döndürür. (Optimizasyon aracıdır.) | `fn capacity<T>(list: arr<T>): i32` |
| **`new_with_capacity()`** | Optimizasyon için belirtilen kapasiteye sahip boş bir dizi oluşturur. | `fn new_with_capacity<T>(size: i32): arr<T>` |
| **`clear()`** | Dizinin tüm elemanlarını siler. | `fn clear<T>(list: arr<T>): void` |
| **`is_empty()`** | Dizinin boş olup olmadığını kontrol eder. | `fn is_empty<T>(list: arr<T>): bool` |

### 3. Eleman Ekleme ve Çıkarma

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`push()`** | Elemanı dizinin **sonuna** ekler. | `fn push<T>(list: arr<T>, item: T): void` |
| **`pop()`** | Dizinin **sonundaki** elemanı çıkarır ve döndürür. (Hata durumunda `Result<T, Error>`) | `fn pop<T>(list: arr<T>): Result<T, Error>` |
| **`add()` / `insert()`** | Elemanı belirtilen **indekse** ekler, sonraki elemanları kaydırır. | `fn add<T>(list: arr<T>, index: i32, item: T): void` |
| **`remove()`** | Belirtilen **indeksteki** elemanı siler ve diziyi sıkıştırır. (Silinen elemanı döndürür.) | `fn remove<T>(list: arr<T>, index: i32): Result<T, Error>` |

### 4. Arama ve Sıralama

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`find()`** | Verilen değeri arar ve bulursa **ilk indeksini** döndürür. | `fn find<T>(list: arr<T>, item: T): Option<i32>` |
| **`sort()`** | Dizinin elemanlarını varsayılan veya özel bir fonksiyonla sıralar. | `fn sort<T>(list: arr<T>, [compare_fn]): void` |
| **`reverse()`** | Dizinin elemanlarının sırasını tersine çevirir. | `fn reverse<T>(list: arr<T>): void` |

### 5. Örnek Kullanım: Dinamik Dizi İşlemleri

```
Nim

void fn ArrayExample() {
    var sayilar: i32[] = {10, 20, 30}; // Dinamik dizi ilklendirme
    
    // push ile eleman ekleme
    sayilar.push(40); // [10, 20, 30, 40]
    
    print("Eleman Sayısı: {sayilar.count()}"); // 4
    
    // add ile araya ekleme
    sayilar.add(1, 15); // [10, 15, 20, 30, 40]

    // pop ile çıkarma
    var sonEleman = sayilar.pop(); 
    if sonEleman.is_ok(): {
        print("Çıkarılan: {sonEleman.unwrap()}"); // 40
    }

    // Arama
    var index = sayilar.find(20);
    if index.is_some(): {
        print("20'nin indeksi: {index.unwrap()}"); // 2
    }
}

```