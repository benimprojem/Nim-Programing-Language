# 🏛️ NIMBLE Dili: Yapısal İlişkiler ve Erişim Kontrolü

## BÖLÜM 16.0: `group` Yapısının Derinlemesine İncelenmesi

`group` yapısı, NIMBLE'da modülerliği sağlamanın yanı sıra, veri yapıları (`struct`) ve fonksiyonlar arasında sıkı bir erişim kontrolü katmanı oluşturur.

### 1. `group` ve `struct` İçindeki Erişim Belirteçleri

`group` ve `struct` içindeki üyelerin dış dünyaya nasıl açıldığı kesin kurallarla belirlenmiştir.

| Belirteç | Konum | Erişilebilirlik | Açıklama |
| :--- | :--- | :--- | :--- |
| **`pub`** | `struct` üyeleri | `group` dışından erişilebilir. | `user.name` gibi doğrudan alan erişimini sağlar. Varsayılan olarak private'tır. |
| **`pub`** | `group` üyeleri | Başka modüllerden `modul.group.uye` zinciriyle erişilebilir. | Modülün API'sinin parçasıdır. |
| **(Varsayılan)** | `struct` ve `group` üyeleri | Yalnızca tanımlandığı `group` veya `struct` içinde erişilebilir. | Kapsülleme (Encapsulation) sağlar. |

**Örnek: Kapsülleme**

```nim
Nim

// Kullanıcı Veri Modülü
export group UserModule {
    
    // name alanı pub olduğu için erişilebilir, id alanı private'tır.
    pub struct User {
        pub name: str,
        id: i32 // Varsayılan: Sadece UserModule içinden erişilebilir.
    }
    
    // Fabrika fonksiyonu: Yeni kullanıcı oluşturur
    pub fn create_user(name: str): User {
        // id alanı burada ayarlanabilir çünkü group içindeyiz.
        return User { name: name, id: 12345 }; 
    }
    
    // Private yardımcı fonksiyon
    fn generate_id(): i32 {
        return 999;
    }
}

// Başka Bir Modülün Kullanımı:
use UserModule;

void fn TestAccess() {
    var u = UserModule.create_user("Bob");
    
    // Başarılı: name pub
    io.println("Ad: {u.name}"); 
    
    // HATA: id private
    // io.println("ID: {u.id}"); // DERLEME HATASI
    
    // HATA: generate_id private
    // UserModule.generate_id(); // DERLEME HATASI
}

```

2. group ve struct İç İçe Geçme
group ve struct yapıları iç içe tanımlanarak karmaşık modüler yapılar oluşturulabilir.

İlişki,Amaç,Kural
group içinde struct,Modüle özel veri yapısı tanımlama.,struct dışarıdan kullanılacaksa pub veya export edilmelidir.
struct içinde group,Mantıksal olarak ilişkili metotları veri yapısıyla ilişkilendirme (Rust'taki impl bloğu benzeri).,
"group, struct'a ait metotları içerir. self anahtar kelimesi struct örneğini işaret eder."

Örnek: Metotlar (struct içinde group)

Nim

export struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    
    // group, Vector3 yapısının metotlarını içerir.
    pub group Methods {
        // Metotlar 'self' parametresini alır.
        pub fn length(self: Vector3): f32 {
            // math modülüne erişim gerektirir.
            return math.sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
        }
    }
}

// Kullanım:
use Vector3; 

void fn VectorExample() {
    var vec = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    
    // Metotlar, group aracılığıyla çağrılır.
    var len = vec.Methods.length(); 
    io.println("Vektör uzunluğu: {len}");
}





