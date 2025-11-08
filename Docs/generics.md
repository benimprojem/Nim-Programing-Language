# ✨ NIMBLE Dili: Jenerik Tipler (Generics)

## BÖLÜM 15.0: Generics Kavramı ve Uygulaması

Generics (Jenerik Tipler), kodun yeniden kullanılabilirliğini artırmak amacıyla, 
tip parametreleri ile yazılmış fonksiyonlar ve veri yapıları oluşturmaya olanak tanır. 
NIMBLE'da Jenerikler, **`<T>`** söz dizimi kullanılarak tanımlanır.

### 1. Jenerik Fonksiyonlar

Bir fonksiyonun birden fazla tipte veriyle aynı mantıkta çalışması gerektiğinde kullanılır. Tip parametresi, fonksiyon adından hemen sonra belirtilir.

| Söz Dizimi | Açıklama |
| :--- | :--- |
| `fn <isim><T>(param: T): T` | `T` tipinde bir parametre alır ve `T` tipinde bir değer döndürür. |

**Örnek: İki Değeri Takas Etme (Swap)**

```nim
Nim

// T tipindeki iki değeri takas eden jenerik fonksiyon.
fn swap<T>(a: *T, b: *T): void {
    var temp: T = *a; // a'nın değerini oku
    *a = *b;          // b'nin değerini a'ya yaz
    *b = temp;        // temp'in değerini b'ye yaz
}

void fn GenericSwapExample() {
    var x: i32 = 10;
    var y: i32 = 20;
    
    swap<i32>(&x, &y); // i32 tipi için derlenir
    
    var s1: str = "A";
    var s2: str = "B";
    
    swap<str>(&s1, &s2); // str tipi için derlenir
}