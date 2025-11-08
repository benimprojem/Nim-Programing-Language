Anahtar-Değer Haritası [Dictionary / İlişkisel Dizi]. 
Farklı tiplerden oluşan verileri, benzersiz anahtarlar [key] aracılığıyla hızlı erişim için saklayan dinamik bir koleksiyondur. 

1. Tanımlama Söz Dizimi [Generics Kullanımı] 
map modülü, hangi tip anahtar [Key] ve hangi tip değer [Value] tutacağını belirtmek için Generics (<K, V>) kullanır.

Söz DizimiAçıklamamap<KeyTip, ValueTip>Genel harita tipini tanımlar. 
KeyTip genellikle str veya tamsayıdır, ValueTip ise any dahil her şey olabilir.

var myMap: map<str, i32>;
Anahtarı string, değeri i32 olan bir harita tanımlar.

var mixedMap: map<any, any>;
En esnek yapı. Anahtarlar ve değerler karışık tipten olabilir.

2. Temel Fonksiyonlar ve Kullanımmap modülü, harita yapısını yönetmek için aşağıdaki temel fonksiyonları sağlar:
FonksiyonAmaçSöz Dizimi ve Açıklama
new()Yeni, boş bir map örneği oluşturur.

var user_data = map.new();

set()Haritaya yeni bir anahtar-değer çifti ekler veya var olanı günceller.

user_data.set["isim", "Alp"];
user_data.set[1001, "ID");

get()Verilen anahtara karşılık gelen değeri getirir. Anahtar bulunamazsa null döndürür veya opsiyonel olarak Result<V, Error> döndürebilir.
var ad = user_data["isim"];

has()Belirtilen anahtarın haritada olup olmadığını kontrol eder.
if user_data.has("age"): ...

remove()Belirtilen anahtara sahip elemanı haritadan siler.
user_data.remove("age");

count()Haritadaki anahtar-değer çifti sayısını döndürür.
var adet = user_data.count();

clear()Haritanın tüm içeriğini temizler.
user_data.clear();

keys()Haritadaki tüm anahtarları içeren bir array<KeyTip> döndürür.
var anahtarlar = user_data.keys();

values()Haritadaki tüm değerleri içeren bir array<ValueTip> döndürür.
var degerler = user_data.values();

3. Kolay Erişim Söz Dizimi 

Kolay Söz Dizimi
Temel Fonksiyon Karşılığı

Değer Atama [Set]
user_data["city"] = "Ankara";


Değer Alma [Get]
var sehir = user_data["city"];
