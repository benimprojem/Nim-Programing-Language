# 💻 NIMBLE Dili: `os` Modülü Detaylandırması

## BÖLÜM 11.14: `os` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`os`** | **İşletim Sistemi Etkileşimi.** Çevre (Ortam) değişkenlerini yönetme, dosya yolu manipülasyonu, dizin işlemleri, komut satırı argümanlarına erişim ve harici işlem (process) yönetimi gibi işletim sistemine özgü görevleri gerçekleştirmek için fonksiyonlar sağlar. |

### 1. Komut Satırı ve Çevre (Ortam) Değişkenleri

Programın başlatılma şekli ve ortam yapılandırması hakkında bilgi sağlar.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`args()`** | Programın çalıştırıldığı **komut satırı argümanlarını** bir dize dizisi (`str[]`) olarak döndürür. (İlk eleman program adıdır.) | `fn args(): str[]` | |
| **`get_env()`** | Belirtilen isimdeki ortam (çevre) değişkeninin değerini dize olarak döndürür. Değişken tanımlı değilse `Option<str>` döner. | `fn get_env(name: str): Option<str>` | |
| **`set_env()`** | Yeni bir ortam değişkeni tanımlar veya mevcut olanı günceller. | `fn set_env(name: str, value: str): void` | |
| **`pid()`** | Mevcut çalışan işlemin (process) kimliğini (ID) döndürür. | `fn pid(): i32` | |
| **`exec_path()`** | Çalışan programın tam yürütülebilir dosya yolunu döndürür. | `fn exec_path(): str` | |

### 2. Dizin (Directory) ve Dosya Yolu İşlemleri

`file` modülü dosya içeriğiyle ilgilenirken, `os` modülü dosya yolları ve dizin hiyerarşisiyle ilgilenir.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`cwd()`** | Programın mevcut çalışma dizinini (Current Working Directory) döndürür. | `fn cwd(): str` | |
| **`chdir()`** | Programın çalışma dizinini belirtilen yola değiştirir. | `fn chdir(path: str): Result<void, OsError>` | |
| **`mkdir()`** | Yeni bir dizin oluşturur. | `fn mkdir(path: str): Result<void, OsError>` | |
| **`rmdir()`** | Boş bir dizini siler. | `fn rmdir(path: str): Result<void, OsError>` | |
| **`join_path()`** | İşletim sistemine uygun ayırıcıları kullanarak yol segmentlerini birleştirir. | `fn join_path(segments: str[]): str` | `os.join_path({"user", "data", "file.txt"})` |
| **`list_dir()`** | Belirtilen dizindeki tüm dosya ve dizin adlarını bir dize dizisi olarak döndürür. | `fn list_dir(path: str): Result<str[], OsError>` | |

### 3. İşlem (Process) Yönetimi ve Yürütme

Harici programları çalıştırmak ve kontrol etmek için kullanılır.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`execute()`** | Harici bir programı (komutu) çalıştırır ve **işlemin bitmesini bekler** (Bloklar). İşlemin çıkış kodunu döndürür. | `fn execute(command: str, args: str[]): Result<i32, OsError>` | `os.execute("git", {"clone", "repo_url"})` |
| **`spawn()`** | Harici bir programı arka planda başlatır ve hemen kontrolü geri verir (Bloklamaz). | `fn spawn(command: str, args: str[]): Result<ProcessHandle, OsError>` | Arka plan servisleri için kullanılır. |
| **`exit()`** | Programı belirtilen çıkış koduyla sonlandırır. | `fn exit(code: i32): void` | |

### 4. Örnek Kullanım: Komut Satırı Argümanları ve Dizin İşlemleri

```nim
Nim

// Bu programı çalıştırmak için: ./programim.nim --input data.txt

void fn OsExample() {
    // 1. Komut Satırı Argümanlarına Erişim
    var args = os.args();
    io.println("Program Adı: {args[0]}");

    if args.count() > 1 {
        io.println("İlk Argüman: {args[1]}"); // --input
    }

    // 2. Çevre Değişkenine Erişim
    var userEnv = os.get_env("USER"); 
    match userEnv {
        Some(user) => {
            io.println("Kullanıcı: {user}");
        },
        None => {
            io.err_print("USER ortam değişkeni bulunamadı.");
        }
    }
    
    // 3. Harici Komut Çalıştırma
    var exitCodeResult = os.execute("ls", {"-l"}); // Unix/Linux için
    // Veya os.execute("cmd", {"/c", "dir"}); // Windows için
    
    if exitCodeResult.is_ok() && exitCodeResult.unwrap() == 0 {
        io.println("Dizin listeleme komutu başarıyla çalıştı.");
    }
    
    // 4. Dizin Yaratma
    var newDir = "test_data";
    if os.mkdir(newDir).is_ok() {
        io.println("'{newDir}' dizini oluşturuldu.");
    }
}