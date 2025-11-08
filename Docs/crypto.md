# 🔒 NIMBLE Dili: `crypto` Modülü Detaylandırması

## BÖLÜM 11.17: `crypto` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`crypto`** | **Kriptografi ve Güvenlik.** Veri bütünlüğünü sağlamak için hashleme (özet çıkarma), veri gizliliğini sağlamak için simetrik/asimetrik şifreleme ve güvenli anahtar üretimi gibi temel kriptografik işlevleri sağlar. Yüksek güvenlikli ve endüstri standardı algoritmaları kullanır. |

### 1. Temel Yapılar ve Tipler

Kriptografi modülünde veriler genellikle byte dizileri olarak işlenir.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Digest`** | Hash fonksiyonunun (özet) çıktısını tutan byte dizisi (`u8[]`). | Verinin bütünlüğünü kontrol etmek için kullanılır. |
| **`Cipher`** | Simetrik veya asimetrik şifreleme/deşifreleme anahtarı. | Şifreleme algoritmaları için gereklidir. |
| **`Algorithm`** | Kullanılacak algoritmayı belirten sabitler. | `crypto.SHA256`, `crypto.AES256`, `crypto.RSA_2048` |

### 2. Hashleme ve Öz Alma Fonksiyonları

Bu fonksiyonlar verinin bütünlüğünü kanıtlamak ve parolaları saklamak için kullanılır.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`hash()`** | Verilen algoritmayı kullanarak (örn: SHA256) girdi verisinin kriptografik özetini (`Digest`) döndürür. | `fn hash(alg: Algorithm, data: u8[]): Digest` | |
| **`hmac()`** | Bir veri parçasının bütünlüğünü kontrol etmek için bir anahtar ve bir hash algoritması kullanır (Keyed-Hash Message Authentication Code). | `fn hmac(alg: Algorithm, key: u8[], data: u8[]): Digest` | |
| **`pbkdf2()`** | Parola tabanlı anahtar türetme fonksiyonu. Parolaları saklamadan önce yavaş hashleme (salting ve iterasyonlarla) için idealdir. | `fn pbkdf2(password: str, salt: u8[], iterations: i32, length: i32): u8[]` | |

### 3. Simetrik Şifreleme (AES)

Gizli bir anahtar kullanarak hem şifreleme hem de deşifreleme yapar.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`generate_key()`** | Belirtilen algoritma için güvenli, yeni bir simetrik anahtar (`Cipher`) üretir. | `fn generate_key(alg: Algorithm): Cipher` | Genellikle 128/256 bit uzunluğunda. |
| **`encrypt_aes()`** | AES algoritmasıyla veri şifreler. GCM/CBC gibi bir mod gereklidir. | `fn encrypt_aes(key: Cipher, iv: u8[], data: u8[]): Result<u8[], CryptoError>` | `iv` (Initialization Vector) gereklidir. |
| **`decrypt_aes()`** | AES algoritmasıyla şifrelenmiş veriyi deşifreler. | `fn decrypt_aes(key: Cipher, iv: u8[], encrypted_data: u8[]): Result<u8[], CryptoError>` | |

### 4. Asimetrik Şifreleme ve İmzalar (RSA/ECC)

Genel (public) ve özel (private) anahtar çiftleri kullanılır.

| Fonksiyon | Amaç | Söz Dizimi | Açıklama |
| :--- | :--- | :--- | :--- |
| **`generate_keypair()`** | Asimetrik şifreleme için Genel ve Özel anahtar çifti oluşturur. | `fn generate_keypair(alg: Algorithm): Tuple<Cipher, Cipher>` | `crypto.RSA_2048` gibi. |
| **`sign()`** | Özel anahtar ile veriyi imzalar. | `fn sign(private_key: Cipher, data: u8[]): u8[]` | |
| **`verify()`** | Genel anahtar ve imza ile verinin doğruluğunu kontrol eder. | `fn verify(public_key: Cipher, data: u8[], signature: u8[]): bool` | |

### 5. Örnek Kullanım: Veri Bütünlüğü ve Şifreleme

```nim
Nim

void fn CryptoExample() {
    var password = "gizli_parolam";
    var data: u8[] = string.to_bytes("Gizli mesaj."); // Veriyi byte dizisine dönüştürme
    
    // 1. Hashleme (Veri Bütünlüğü)
    var hash_digest = crypto.hash(crypto.SHA256, data);
    io.println("SHA-256 Özeti: {hash_digest.to_hex()}"); 

    // 2. Parola Güvenliği (PBKDF2)
    var salt = rand.new_secure().bytes(16); // Güvenli, rastgele tuz
    var derived_key = crypto.pbkdf2(password, salt, 100000, 32);
    io.println("Türetilmiş Anahtar (32 byte): {derived_key.count()}");

    // 3. Simetrik Şifreleme (AES-256)
    var aes_key = crypto.generate_key(crypto.AES256);
    var iv = rand.new_secure().bytes(16); // Benzersiz IV

    // Şifreleme
    var cipher_result = crypto.encrypt_aes(aes_key, iv, data);
    
    match cipher_result {
        Ok(encrypted) => {
            io.println("Şifrelenmiş Veri Uzunluğu: {encrypted.count()}");
            
            // Deşifreleme
            var decrypt_result = crypto.decrypt_aes(aes_key, iv, encrypted);
            if decrypt_result.is_ok() {
                var decrypted_data = decrypt_result.unwrap();
                io.println("Deşifrelenen Mesaj: {string.from_bytes(decrypted_data)}"); 
            }
        },
        Err(e) => {
            io.err_print("Şifreleme Hatası: {e}");
        }
    }
}