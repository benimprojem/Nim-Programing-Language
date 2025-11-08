# 🚀 NIMBLE Dili: `gpu` Modülü Detaylandırması

## BÖLÜM 11.11: `gpu` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`gpu`** | **GPU Hesaplama ve Paralel İşleme.** Grafik İşlem Birimlerinin (GPU) yüksek paralel hesaplama gücünden yararlanmak için araçlar ve yapılar sunar. Veri paralel algoritmaların tanımlanmasını ve GPU üzerinde çalıştırılmasını sağlar. |

### 1. Temel Yapılar ve Tipler

GPU hesaplaması, CPU belleğinden ayrı olan GPU belleği ve çekirdek fonksiyonları (kernels) etrafında döner.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Device`** | Sistemdeki fiziksel bir GPU cihazını temsil eder. | `gpu.select_device()` ile elde edilir. |
| **`Kernel`** | GPU üzerinde paralel olarak çalıştırılacak derlenmiş fonksiyonu (çekirdek) temsil eder. | `gpu.compile_kernel()` ile oluşturulur. |
| **`GpuArray<T>`** | GPU belleğinde tahsis edilmiş, belirlenen tipte (`T`) bir dizi. Veri transferi için kullanılır. | CPU'daki dinamik dizilerin (`arr<T>`) GPU karşılığıdır. |
| **`GpuError`** | GPU başlatma, bellek tahsisi veya kernel yürütme sırasında oluşabilecek hataları listeler. | `ERR_NO_DEVICE`, `ERR_COMPILATION_FAILED` |

### 2. Yönetim ve Derleme Fonksiyonları

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`select_device()`** | Sistemdeki kullanılabilir bir GPU cihazını seçer. | `fn select_device(index: i32): Result<Device, GpuError>` |
| **`compile_kernel()`** | Bir NIMBLE fonksiyonunu GPU üzerinde çalıştırılabilir bir `Kernel`'a derler. | `fn compile_kernel(func_ref): Result<Kernel, GpuError>` |
| **`sync()`** | GPU işlemlerinin bitmesini bekler ve CPU'ya kontrolü verir. | `fn sync(device: Device): void` |
| **`error_code()`** | En son GPU işlemi hatasını döndürür. | `fn error_code(): GpuError` |

### 3. Bellek Yönetimi ve Veri Transferi

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`alloc_array()`** | GPU belleğinde yeni bir `GpuArray` tahsis eder. | `fn alloc_array<T>(device: Device, size: i32): Result<GpuArray<T>, GpuError>` | `var gpu_data = gpu.alloc_array<f64>(dev, 1000);` |
| **`to_gpu()`** | CPU belleğindeki bir diziyi (`arr<T>`) GPU belleğine kopyalar. | `fn to_gpu<T>(device: Device, cpu_array: arr<T>): Result<GpuArray<T>, GpuError>` | |
| **`from_gpu()`** | GPU belleğindeki bir diziyi CPU belleğine geri kopyalar. | `fn from_gpu<T>(gpu_array: GpuArray<T>): Result<arr<T>, GpuError>` | |
| **`free_array()`** | GPU belleğinde tahsis edilmiş diziyi serbest bırakır. | `fn free_array(gpu_array: GpuArray): void` | Manuel bellek yönetimi gerektirir. |

### 4. Kernel Tanımlama ve Yürütme

GPU üzerinde çalışacak fonksiyonlar özel bir `kernel` anahtar kelimesi ile tanımlanır.

| Yapı/Fonksiyon | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`kernel fn`** | GPU'da paralel çalışacak fonksiyonun bildirimi. | Sadece `GpuArray` ve temel tipleri parametre olarak alabilir. |
| **`launch()`** | Derlenmiş bir `Kernel`'ı GPU üzerinde yürütür (başlatır). Grid/Blok boyutları belirtilir. | `fn launch(k: Kernel, grid_size: i32, block_size: i32, ...args): void` |
| **`gpu.thread_id()`** | Çalışan çekirdek içindeki mevcut iş parçacığının paralel indeksini döndürür. | Kernel fonksiyonları içinde kullanılır. |

### 5. Örnek Kullanım: İki Diziyi Toplama

```nim
Nim

// GPU üzerinde paralel olarak çalışacak Kernel fonksiyonu
kernel fn add_arrays(a: GpuArray<i32>, b: GpuArray<i32>, result: GpuArray<i32>) {
    // Mevcut iş parçacığının indeksini al
    var i = gpu.thread_id();
    
    // Paralel işlem: result[i] = a[i] + b[i]
    if i < a.count() {
        result[i] = a[i] + b[i];
    }
}

void fn GpuExample() {
    var deviceResult = gpu.select_device(0); // İlk GPU'yu seç
    
    match deviceResult {
        Ok(dev) => {
            var size = 1024;
            var cpu_a: i32[] = array.new_filled(size, 1);
            var cpu_b: i32[] = array.new_filled(size, 2); 
            
            // 1. Veriyi GPU'ya aktar
            var gpu_a = gpu.to_gpu(dev, cpu_a).unwrap();
            var gpu_b = gpu.to_gpu(dev, cpu_b).unwrap();
            var gpu_result = gpu.alloc_array<i32>(dev, size).unwrap();

            // 2. Kernel'ı Derle
            var kernel = gpu.compile_kernel(add_arrays).unwrap();

            // 3. Kernel'ı Başlat (Örn: 1024 iş parçacığı)
            gpu.launch(kernel, size, 256, gpu_a, gpu_b, gpu_result);
            
            // 4. İşlemin Bitmesini Bekle ve Veriyi Geri Al
            gpu.sync(dev); 
            var final_result = gpu.from_gpu(gpu_result).unwrap();

            print("GPU İşlemi Bitti. İlk Eleman: {final_result[0]}"); // 3 olmalı
            
            // 5. Belleği Serbest Bırak (Çok Önemli!)
            gpu.free_array(gpu_a);
            gpu.free_array(gpu_b);
            gpu.free_array(gpu_result);
        },
        Err(e) => {
            io.err_print("GPU cihazı bulunamadı: {e}");
        }
    }
}