#  NIMBLE PROGRAMLAMA DİLİ MİMARİSİ DOKÜMANI

## BÖLÜM I: GİRİŞ VE DİLİN TEMELİ

| Bölüm Numarası | Konu Başlığı | Kaynak Dosya(lar) | Odak Noktası |
| :--- | :--- | :--- | :--- |
| **BÖLÜM 1** | **Giriş ve Temel Söz Dizimi** | `NIMdoc.md` (Başlangıç) | Felsefe, `var`/`const` Tanımlamaları, Temel Blok Yapısı (`{}`) |
| **BÖLÜM 2** | **Temel Veri Tipleri ve Dönüşümler** | `types.md` | `i32`, `f64`, `str`, `char` gibi tipler. Güvenli/Güvensiz Dönüşümler (`types.to_i32()`) |

## BÖLÜM II: MANTIK VE KONTROL AKIŞI

| Bölüm Numarası | Konu Başlığı | Kaynak Dosya(lar) | Odak Noktası |
| :--- | :--- | :--- | :--- |
| **BÖLÜM 3** | **Modülerlik ve Erişim Kontrolü** | `group_relations.md` | `group` Yapısı, `export`, `use`, `pub` ve Kapsülleme Kuralları |
| **BÖLÜM 4** | **Hata Yönetimi ve Güvenlik** | `error.md` | **Kritik:** `Result<T, E>`, `Option<T>`, `match`, `panic()`, **`rolling`** Mekanizması |
| **BÖLÜM 5** | **Jenerik Tipler (Generics)** | `generics.md` | `<T>` Söz Dizimi ve Jenerik Fonksiyonlar/Yapılar |

## BÖLÜM III: DÜŞÜK SEVİYE ERİŞİM VE ENTEGRASYON

| Bölüm Numarası | Konu Başlığı | Kaynak Dosya(lar) | Odak Noktası |
| :--- | :--- | :--- | :--- |
| **BÖLÜM 6** | **Derleme Direktifleri ve Makrolar** | `DerlemeDirektifleri.md` | `#ifdef`, `#define`, `#include` ve Derleme Öncesi Kontrol |
| **BÖLÜM 7** | **Dış Fonksiyon Arayüzü (FFI)** | `ffi.md` | **Kritik:** `extern`, `#[link]`, C Tipleriyle Eşleştirme |
| **BÖLÜM 8** | **Inline Assembly ve Mikro Optimizasyon** | `asm.md`, `cpu.md` (İlgili Kısımlar) | **Kritik:** `asm: ETIKET { ... }`, `%değişken_adı`, `fast_exec` |
| **BÖLÜM 9** | **Manuel Bellek Yönetimi** | `memory.md` | **Kritik:** `memory.alloc()`, `memory.free()`, `ptr` İşaretçileri |

## BÖLÜM IV: STANDART KÜTÜPHANE (TEMEL MODÜLLER)

| Bölüm Numarası | Konu Başlığı | Kaynak Dosya(lar) | Odak Noktası |
| :--- | :--- | :--- | :--- |
| **BÖLÜM 10** | **Girdi/Çıktı (I/O) ve Loglama** | `io.md`, `log.md` | Konsol (`io.println()`), Hata Çıktısı (`io.err_print()`), Yapılandırılmış Loglama |
| **BÖLÜM 11** | **Temel Veri Yapıları** | `string.md`, `array.md`, `map.md` | Dize Manipülasyonları, Dinamik Diziler (`T[]`), Anahtar-Değer Haritaları (`map<K, V>`) |
| **BÖLÜM 12** | **Sistem ve Zamanlama** | `os.md`, `time.md`, `rand.md` | OS Etkileşimi, Dosya Yolu, Süre Ölçümü, Rastgele Sayı Üretimi |
| **BÖLÜM 13** | **Dosya Sistemi ve Gelişmiş I/O** | `file.md` | `file.open()`, Okuma/Yazma İşlemleri, `FileHandle` |

## BÖLÜM V: UYGULAMA VE PARALEL İŞLEME

| Bölüm Numarası | Konu Başlığı | Kaynak Dosya(lar) | Odak Noktası |
| :--- | :--- | :--- | :--- |
| **BÖLÜM 14** | **Eşzamanlılık ve İş Parçacıkları** | `thread.md` | `spawn()`, `join()`, `Mutex`, `Atomic<T>`, Kanal Tabanlı İletişim |
| **BÖLÜM 15** | **Ağ İletişimi** | `net.md` | TCP/UDP Soketleri, `net.tcp_connect()`, Veri Gönderme/Alma |
| **BÖLÜM 16** | **Veri Formatları ve Kriptografi** | `json.md`, `regex.md`, `crypto.md` | JSON Serileştirme, Düzenli İfadeler, Güvenli Hashleme (`SHA256`) |
| **BÖLÜM 17** | **Donanım Hızlandırma** | `gpu.md`, `cpu.md` (İlgili Kısımlar) | GPU Hesaplama (`Kernel`, `GpuArray`), CPU Performans Sayacı (`cpu.rdtsc()`) |

---

