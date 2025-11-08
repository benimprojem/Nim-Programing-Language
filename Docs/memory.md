# 🧠 NIMBLE Dili: `memory` Modülü Detaylandırması

## BÖLÜM 11.10: `memory` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`memory`** | **Manuel Bellek Yönetimi.** Dinamik bellek tahsis etme (`malloc`/`calloc` benzeri), yeniden boyutlandırma ve serbest bırakma (`free`) gibi düşük seviyeli işlemleri programcıya sunar. Otomatik bellek yönetiminin (GC) kullanılmadığı veya bypass edildiği senaryolar için kritiktir. |

### 1. Temel Yapılar ve Tipler

Bellek işlemlerinde sadece işaretçiler ve boyutlar kullanılır.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`ptr`** | Herhangi bir türdeki belleği işaret eden güvensiz işaretçi (pointer). | Bellek yönetim fonksiyonlarının dönüş ve giriş tipidir. |
| **`size`** | Bellek boyutunu (byte cinsinden) tutan platforma özgü unsigned tamsayı. | `alloc` ve `copy` gibi fonksiyonlarda kullanılır. |
| **`MemError`** | Bellek tahsisinde oluşabilecek hataları listeler. | `ERR_NO_MEMORY` (Yetersiz sistem belleği), `ERR_INVALID_PTR` |

### 2. Bellek Tahsis ve Serbest Bırakma Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`alloc()`** | İstenen boyutta (byte cinsinden) bellek tahsis eder. Başarılı olursa `ptr` döndürür, başarısız olursa `null` veya hata kodu döner. | `fn alloc(size: size): Result<ptr, MemError>` | `ptr ptr1 = memory.alloc(1024);` |
| **`calloc()`** | İstenen miktarda eleman için bellek tahsis eder ve tahsis edilen tüm belleği **sıfırlarla doldurur.** | `fn calloc(amount: size, element_size: size): Result<ptr, MemError>` | `ptr ptr2 = memory.calloc(10, types.sizeof<i32>());` (10 adet i32 için yer ayır) |
| **`realloc()`** | Daha önce tahsis edilmiş belleğin boyutunu yeniden ayarlar. Veri korunur. | `fn realloc(ptr: ptr, new_size: size): Result<ptr, MemError>` | `ptr1 = memory.realloc(ptr1, 2048);` |
| **`free()`** | Daha önce `alloc` veya `calloc` ile tahsis edilmiş belleği serbest bırakır. | `fn free(ptr: ptr): void` | Serbest bırakıldıktan sonra işaretçi geçersiz hale gelir. |

### 3. Bellek Manipülasyon Fonksiyonları

Bu fonksiyonlar, bloklar halinde belleği taşımak veya doldurmak için kullanılır.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`copy()`** | Kaynak bellek bloğundan hedef bellek bloğuna belirtilen boyutta veri kopyalar. | `fn copy(dest: ptr, src: ptr, size: size): void` | `memory.copy(ptr_hedef, ptr_kaynak, 500);` |
| **`move()`** | `copy()` ile aynıdır ancak örtüşen (overlapping) bellek bloklarında güvenlidir. | `fn move(dest: ptr, src: ptr, size: size): void` | |
| **`set()`** | Bellek bloğunun tamamını belirtilen tek bir byte değeriyle doldurur (Örn: sıfırlama için). | `fn set(ptr: ptr, value: u8, size: size): void` | `memory.set(ptr1, 0, 1024);` (Belleği sıfırla) |

### 4. Örnek Kullanım: Manuel Bellek Yönetimi

```nim
Nim

// i32'nin bellekteki boyutunu almak için dilde varsayılan bir fonksiyon olduğu varsayılır.
const I32_SIZE = types.sizeof<i32>(); 

void fn MemoryExample() {
    var size_bytes: size = 10 * I32_SIZE; // 10 adet i32 için alan
    var data_ptr: ptr;
    
    // 1. Bellek Tahsis Etme
    var allocResult = memory.calloc(10, I32_SIZE);
    
    match allocResult {
        Ok(ptr_val) => {
            data_ptr = ptr_val;
            
            // 2. Belleği kullanma (Pointer aritmetiği ve Güvensiz Erişim)
            // İlk i32'ye 42 değerini ata
            (data_ptr as *i32)[0] = 42; 
            
            // 3. Belleği Yeniden Boyutlandırma (Örn: 20 i32 için)
            var reallocResult = memory.realloc(data_ptr, 20 * I32_SIZE);

            match reallocResult {
                Ok(new_ptr) => {
                    data_ptr = new_ptr;
                    print("Bellek yeniden boyutlandırıldı. İlk değer: { (data_ptr as *i32)[0] }");
                },
                Err(e) => {
                    print("Yeniden boyutlandırma başarısız: {e}");
                }
            }
        },
        Err(e) => {
            print("Bellek tahsisi başarısız: {e}");
        }
    }
    
    // 4. Belleği Serbest Bırakma (Kesinlikle gereklidir!)
    memory.free(data_ptr);
    print("Bellek serbest bırakıldı.");
}