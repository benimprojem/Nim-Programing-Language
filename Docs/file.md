# NIMBLE Dili: `file` Modülü Detaylandırması

## BÖLÜM 11.2: `file` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`file`** | **Dosya Sistemi ve G/Ç (I/O) İşlemleri.** Dosya açma, okuma, yazma, silme ve kopyalama gibi temel dosya sistemi işlemlerini gerçekleştirmek için araçlar sağlar. Tüm I/O fonksiyonları, güvenli hata yönetimi için genellikle **`Result<T, E>`** tipi döndürür. |

### 1. Temel Yapılar ve Tipler

`file` modülü, işlemleri yönetmek için özel tiplere ihtiyaç duyar:

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`FileHandle`** | Açık bir dosyayı temsil eden işaretçi (handle). | `file.open()` fonksiyonunun başarılı dönüş tipidir. |
| **`FileMode`** | Dosyanın nasıl açılacağını belirleyen sabitler (constants). | `file.READ`, `file.WRITE`, `file.APPEND`, `file.READ_WRITE` |
| **`FileError`** | Dosya işlemlerinde meydana gelebilecek olası hataları (hata kodlarını) listeler. | `ERR_NOT_FOUND`, `ERR_ACCESS_DENIED`, `ERR_IO_FAILED` |

### 2. Dosya Yönetim Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`open()`** | Belirtilen yoldaki dosyayı istenen modda açar. | `fn open(path: str, mode: FileMode): Result<FileHandle, FileError>` |
| **`close()`** | Açık `FileHandle`'ı kapatır ve kaynakları serbest bırakır. | `fn close(handle: FileHandle): void` |
| **`read_all()`** | Dosyanın tüm içeriğini bir string olarak okur. | `fn read_all(handle: FileHandle): Result<str, FileError>` |
| **`write()`** | Belirtilen veriyi dosyaya yazar. Başarılı yazılan byte sayısını döndürür. | `fn write(handle: FileHandle, data: str): Result<i32, FileError>` |
| **`delete()`** | Belirtilen yoldaki dosyayı kalıcı olarak siler. | `fn delete(path: str): Result<void, FileError>` |
| **`copy()`** | Bir dosyayı belirtilen kaynaktan hedefe kopyalar. | `fn copy(src_path: str, dest_path: str): Result<void, FileError>` |
| **`exists()`** | Belirtilen yolda bir dosyanın olup olmadığını kontrol eder. | `fn exists(path: str): bool` |

### 3. Örnek Kullanım: Güvenli Dosya Okuma

```
Nim

void fn Main() {
    var fileResult = file.open("ayarlar.cfg", file.READ);
    
    // Hata kontrolü için match ifadesi kullanımı
    match fileResult {
        Ok(handle) => {
            var contentResult = file.read_all(handle);
            
            match contentResult {
                Ok(content) => {
                    print("Dosya İçeriği:\n{content}");
                },
                Err(error) => {
                    print("Okuma Hatası: {error}");
                }
            }
            file.close(handle); // Dosyayı kapat
        },
        Err(error) => {
            // Dosya bulunamadı, erişim reddedildi vb.
            print("Dosya Açma Hatası: {error}");
        }
    }
}
```

```
Nim

void fn WriteExample() {
    var fileResult = file.open("log.txt", file.WRITE);
    
    match fileResult {
        Ok(handle) => {
            var writeResult = file.write(handle, "Bu log mesajıdır.\n");
            
            if writeResult.is_ok() {
                 print("Başarıyla yazılan byte sayısı: {writeResult.unwrap()}");
            } else {
                 print("Yazma Hatası!");
            }
            file.close(handle);
        },
        Err(error) => {
            print("Dosya Açma Hatası: {error}");
        }
    }
}
```
