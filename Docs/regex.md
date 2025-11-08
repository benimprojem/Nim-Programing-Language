# 🔍 NIMBLE Dili: `regex` Modülü Detaylandırması

## BÖLÜM 11.7: `regex` Modülü

| Modül Adı | Açıklama |
| :--- | :--- |
| **`regex`** | **Düzenli İfadeler (Regular Expressions).** Karmaşık metin desenlerini aramak, eşleştirmek, yakalamak ve değiştirmek için fonksiyonlar ve yapılar sağlar. Yüksek performanslı ve güvenli desen eşleştirme üzerine kuruludur. |

### 1. Temel Yapılar ve Tipler

Düzenli ifade işlemleri genellikle iki aşamalıdır: desenin derlenmesi ve desenin kullanılması.

| Tip Adı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`Regex`** | Derlenmiş (compiled) bir düzenli ifade desenini temsil eden opak yapı. Desen, tekrar tekrar kullanılmak üzere bellekte optimize edilir. | `regex.compile()` fonksiyonunun başarılı dönüş tipidir. |
| **`Match`** | Başarılı bir eşleşmenin sonucunu ve yakalanan grupların (captures) listesini tutan yapı. | `regex.find()` ve `regex.match()` fonksiyonlarının dönüş tipidir. |
| **`RegexError`** | Desen derlenirken (compile) oluşabilecek söz dizimi hatalarını listeler. | `ERR_SYNTAX`, `ERR_INVALID_FLAG` |

### 2. Temel Fonksiyonlar

| Fonksiyon | Amaç | Söz Dizimi |
| :--- | :--- | :--- |
| **`compile()`** | Verilen dize desenini optimize edilmiş bir `Regex` yapısına derler. | `fn compile(pattern: str): Result<Regex, RegexError>` |
| **`match()`** | Dizenin **başından itibaren** desenle eşleşip eşleşmediğini kontrol eder. Tam eşleşme (full match) gerektirmez. | `fn match(r: Regex, text: str): Option<Match>` |
| **`find()`** | Dizenin herhangi bir yerinde desenin **ilk** eşleşmesini arar. | `fn find(r: Regex, text: str): Option<Match>` |
| **`replace()`** | Desenin ilk eşleşmesini verilen dizeyle değiştirir ve yeni dizeyi döndürür. | `fn replace(r: Regex, text: str, replacement: str): str` |
| **`replace_all()`** | Desenin dizedeki **tüm** eşleşmelerini verilen dizeyle değiştirir ve yeni dizeyi döndürür. | `fn replace_all(r: Regex, text: str, replacement: str): str` |

### 3. `Match` Yapısının Metotları (Yakalamalara Erişim)

`Match` yapısı, eşleşme başarılı olduğunda sonuç verilerine erişim sağlar.

| Fonksiyon | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`is_success()`** | Eşleşmenin başarılı olup olmadığını kontrol eder. | `if myMatch.is_success(): ...` |
| **`text()`** | Eşleşen metnin tamamını döndürür. | `myMatch.text()` |
| **`group()`** | Belirtilen indeksteki (0: Tam eşleşme, 1+: Yakalama grupları) yakalama grubunun dizesini döndürür. | `myMatch.group(1)` |
| **`start()`**, **`end()`** | Eşleşmenin dizedeki başlangıç ve bitiş indekslerini döndürür. | `print(myMatch.start())` |

### 4. Örnek Kullanım: E-posta Doğrulama ve Yakalama

```nim
Nim

void fn RegexExample() {
    // Desen: Kullanıcı adı, @, alan adı (.uzantı)
    var email_pattern: str = "^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+)\\.([a-zA-Z]{2,})$";
    var test_email: str = "kullanici.adi@nimble.dev";
    
    // 1. Deseni Derleme (Compile)
    var regexResult = regex.compile(email_pattern);
    
    match regexResult {
        Ok(email_regex) => {
            // 2. Eşleşme Arama (Find)
            var matchOption = email_regex.find(test_email);
            
            match matchOption {
                Some(match_data) => {
                    print("Eşleşme Başarılı.");
                    // Yakalama gruplarına erişim
                    var user = match_data.group(1); 
                    var domain = match_data.group(2);

                    print("Kullanıcı Adı: {user}"); // kullanici.adi
                    print("Alan Adı: {domain}");    // nimble
                },
                None => {
                    print("E-posta formatı hatalı.");
                }
            }

            // 3. Değiştirme
            var old_text: str = "Tarih 2024-01-01";
            var date_regex = regex.compile("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
            var new_text = date_regex.replace(old_text, "YENİ TARİH");
            print("Değiştirilen: {new_text}"); // Tarih YENİ TARİH
        },
        Err(error) => {
            print("Desen Derleme Hatası: {error}");
        }
    }
}