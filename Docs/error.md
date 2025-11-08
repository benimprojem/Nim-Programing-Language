# ❌ NIMBLE Dili: Hata Yönetimi ve İstisnalar

## BÖLÜM 12.0: Hata Yönetimi Felsefesi

NIMBLE, çalışma zamanında oluşabilecek sorunları yönetmek için iki ana mekanizma kullanır:
1.  **Güvenli Hata Yönetimi (Result/Option):** Geri döndürülebilir ve beklenen hatalar için kullanılır. Hata kontrolü derleyici tarafından zorunludur.
2.  **Kritik İstisnalar (Exception/Panic):** Kurtarılamaz sistem hataları veya programcı mantık hataları için kullanılır.

### 1. Güvenli Hata Yönetimi: `Result<T, E>` ve `Option<T>`

NIMBLE, hatalı bir sonucun olası olduğu her fonksiyonda `Result` veya `Option` tiplerini kullanmayı teşvik eder.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Result<T, E>`** | Başarılı bir değer (`T`) veya bir hata değeri (`E`) tutar. | G/Ç, ağ ve dönüşüm gibi başarısız olma olasılığı olan tüm işlemler için kullanılır. |
| **`Option<T>`** | Ya bir değer (`T`) ya da "Hiçbir Şey" (`None`) tutar. | Arama işlemleri (örn: `array.find()`, `os.get_env()`) veya `null` yerine güvenli bir alternatif olarak kullanılır. |

#### 1.1 Hata Akışı Kontrolü: `match` ve `unwrap`

`Result` ve `Option` tipleri, `match` ifadesi veya `unwrap_or` gibi güvenli metotlar kullanılarak ele alınmalıdır.

```nim
Nim

// Örnek: Dosya Açma
var fileResult = file.open("config.ini", file.READ);

match fileResult {
    Ok(handle) => {
        // Başarılı: İşleme devam et
        ...
    },
    Err(error) => {
        // Hata: Hata tipine özel işlem yap
        io.err_print("Dosya Hatası: {error}"); 
    }
}

// Güvenli Unwrapping: Değer yoksa varsayılan döndür
var kullanıcı_adi = os.get_env("USER").unwrap_or("Anonim");
```
#### 1.2 Hata Tipi Hiyerarşisi (Ana Kategori)
Tüm modül hataları tek bir ana Error enum/struct altında toplanır.
```
Hata Tipi Adı,Kategori,Kapsadığı Alanlar
IO_Error,G/Ç Hataları,"file.FileError, io.IoError"
NET_Error,Ağ Hataları,net.NetError
CONVERT_Error,Dönüşüm Hataları,"types.ConvertError, json.JsonError, regex.RegexError"
SYSTEM_Error,Sistem Hataları,"os.OsError, memory.MemError, gpu.GpuError"
RUNTIME_Error,Çalışma Zamanı Hataları,"Dizin sınır aşımı, Sıfıra bölme vb."
```
#### 1.3 Kritik İstisnalar (Panics)
NIMBLE, kurtarılamaz durumlar veya mantık hataları için panic mekanizmasını kullanır. panic çağıran tüm çağrı yığınını (stack) sarar ve programı sonlandırır.
```
Fonksiyon/Yapı,Amaç,Söz Dizimi,Açıklama
panic(),Kritik bir hata mesajıyla programı hemen sonlandırır.,fn panic(msg: str): void,Genellikle dahili kütüphane hataları için kullanılır.
try_catch,Kritik istisnaları yakalamak için opsiyonel bir bloktur.,try { ... } catch (e) { ... },Kurtarmanın mümkün olduğu nadir durumlar için.
```

```
Nim

void fn process_data(data: i32[]) {
    if data.is_empty() {
        // Boş bir diziyi işlemeye çalışmak bir panic gerektirir.
        panic("process_data: Girdi dizisi boş olamaz."); 
    }
    // ...
}
```


