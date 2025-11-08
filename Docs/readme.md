NIM (NIMBLE) PROGRAMLAMA DİLİ - NİHAİ TEKNİK ŞARTNAMESİ (README)
====================================================================
-------------------------------------------------------------

BÖLÜM 1: GİRİŞ VE TEMELLER (CORE)
1.0	Felsefe ve Versiyon	Dilin misyonu, V1.0
1.1 Temel Söz Dizimi: 
Bloklar ({}), Satır Sonu (;), Yorumlar (//, /*...*/).

Büyük-küçük harf duyarlıdır. Tüm anahtar kelimeler ve yerel fonksiyonlar küçük harf kullanır.
for() doğru iken For() hata verir.. Derleyici için Makrolarda #IF  doğru iken #if hata verir..

Değişkenler  `bir: i32 = 1;` ile `Bir: f32 = 0.001;` birbirlerinden farklı iki değişkendir. 

1.2	Değişkenler ve Tanımlama var, const.

const a: i32 = 42;// sabit. Sabitlerde const kullanımı zorunludur.

var b: i32 = 27;  // global veya yereldir.
c: f32 = 1.12;    // global veya yereldir. (var) kullanımı zorunlu değildir.

const pi: f32[4] = math.PI; // pi = 3.1415 olur. ondalık ayrımdan sonra 4 sayı alır, 
4'den az sayı varsa sona 0 ekleyerek 4 e tamamlar.  örnek: c: f32[4] = 1.12; // c = 1.1200 olur.



BÖLÜM 2: VERİ TİPLERİ
2.1	Temel Tipler	i32, f64, string, ptr, Tip Aliasları (Örn: int).

i8,i16,i32,i64,i128

u8,u16,u32,u64,u128

f32,f64,f128

int = i32
float = f32
bool
char
bit
byte
str veya string
ptr 

utf8, utf16


2.2	Koleksiyon Tipleri	array veya arr, list, map, tuple (Demetler).
2.3	Yapılar ve Numaralandırmalar	struct, enum.
2.4	Generics (Jenerikler)	Tip parametreleri <T, K> ile genel yapı ve fonksiyon tanımlama.
2.5	Tip Dönüşümü	Açık Tip Dönüşümü (Casting) kuralı.

BÖLÜM 3: FONKSİYONLAR VE KAPSAM
3.1	Fonksiyon Tanımı fn söz dizimi, void dönüşsüz fonksiyonlar, $fonksiyon_adı fonksiyonun döndürdüğü değer değişkeni.
3.2	Metot Tanımlamaları	impl blokları ve self anahtar kelimesi ile struct metotları.
3.3	Parametreler	Opsiyonel Argümanlar (param: tip = değer) ve İsimli Argüman Çağrısı.
3.4	Geri Dönüş Standardı $Fonksiyon_Adı ile anlık geri dönüş değerine erişim ve $Fonksiyon_Adı[indeks] ile Tuple ayrıştırması. 
sahiplik değiştiğinde $Fonksiyon_Adı ulaşılamaz, kısa süreli var olurlar..
3.5	Lambda ve Anonim Fonksiyonlar	fn (x: int): int -> ... söz dizimi.

BÖLÜM 4: OPERATÖRLER VE İFADELER
4.1	Operatör Önceliği	Tam Öncelik ve İlişkilendirme Tablosu.
4.2	Aritmetik ve Mantıksal Operatörler	
4.3	Bit Düzeyi Operatörler	&, |,  <<,  >>
4.4	İşaretçi Operatörleri	& (Adres alma) ve * (Dereference)

BÖLÜM 5: KONTROL AKIŞI
5.1	Koşullu ve Döngü İfadeleri	if, else, while, for, break, continue.
5.2	Desen Eşleştirme	match ifadesi ve def varsayılan durumu.
5.3	Kontrollü Atlama	rolling: ETİKET mekanizması ve $rolling sayacı.
5.4	Grup Yapısı	group tanımı, etiketli giriş noktaları ve zincirleme erişim.

BÖLÜM 6: BELLEK VE GÜVENLİK MODELİ
6.1	Varsayılan Bellek Modeli	Sahibiyet (Ownership) kuralları ve otomatik yönetim.
6.2	Hata Yolu	Option<T>, Result<T, E> kullanımı.
6.3	Modül olarak: Manuel Kontrol (memory Grubu) Düşük seviyeli bellek kontrol fonksiyonları (memory.alloc, memory.free, vb.).

BÖLÜM 7: MODÜLERLİK VE SİSTEM
7.1	Modül Sistemi	export (Dışa aktarma) ve use (Seçici/Tam İçe aktarma) kuralları.
7.2	Yerel G/Ç Fonksiyonları	print, input, len(data) ve değişken interpolasyonu.
7.3	Sistem Modülleri:	os, file, sitring, math, memory, map, vector, array ve diğer çekirdek kütüphaneler.
7.4 Modüller: cpu | cpu.set_reg(id, value), cpu.get_reg(id) | Düşük seviyeli CPU kaydedici (Register) erişimi ve Hızlı Yürütme Kapsamları.

======================================================================

BÖLÜM 1: GİRİŞ VE TEMELLER (CORE)
---------------------------------------
Version: 3.0 (Çalışma Zamanı ve Generics Destekli, Tutarlı Söz Dizimi)
Felsefe: Prosedürel, Fonksiyonel, Modüler, Güvenli Kontrol.

1.1 TEMEL SÖZ DİZİMİ VE DEĞİŞKENLER

NIM, C-stili söz dizimini benimser. 
Yorum satırları tekli: `// tek satır yorum` çoklu:  `/* çok satırlı  yorum */`
Tüm bloklar `{}` ile tanımlanır ve ifadeler noktalı virgül (`;`) ile sonlanır.


1.2	Değişkenler ve Tanımlama
Anahtar Kelime	Açıklama
`var`	Değiştirilebilir yerel veya global değişken. zorunlu değil.
`const`	Sabit değer. zorunlu..

Örnekler:
```
Nim

// Tip çıkarımı ile int/i32 atanır
var count = 10; 

// Açıkça tip belirtme
var name: str = "NIM Language"; 

// Sabit tanımlama
const PI_VALUE: f64 = 3.14159; 

// Tuple tanımlama
var position: (i32, i32) = (100, 250); 
```

BÖLÜM 2: VERİ TİPLERİ VE YAPILAR (TYPES)
-----------------------------------------------
2.1	Temel ve Özel Tipler
Sayısal: `i8`, `i32`, `i64`, `i128`, `f32`, `f64`, `f128` (Alias: `int`, `float`).
		 `u8`, `u16`, `u32`, `u64`,  `u128` gibi...
		 
İşaretçi: `ptr`.

Metin: `str` (Varsayılan UTF-8). Kesin kodlama: `var msg: str[utf16] = "Data";`

Temel tiplerin bit genişliği kesindir. Açık Tip Dönüşümü (Casting) veri kaybı riskine karşı zorunludur.
```
Nim

var large_num: i64 = 9000000000;

// Kayan nokta kullanımı
var ratio: f64 = 1.25; 

// İşaretçi tipi
var ptr_data: ptr = null;
```

2.2	Koleksiyon Tipleri	array, list, map, Tuple (Demetler).
`array`, `arr`, `list: T`, `map: K, V`.
`Tuple` (Demetler): Birden fazla değer döndürmek için kullanılır. (Örn: `(int, str)` tipi).


2.3	Yapılar ve Numaralandırmalar	struct, enum.

```
Nim

// Yapı tanımı
struct User {
	id: i32;
	name: string;
}

// Enum tanımı (Success=200, Forbidden=403)
enum HTTPStatus {
	Success = 200,
	Forbidden = 403,
	NotFound = 404
}

void fn Main() {
	var user1 = new User { id = 1, name = "Ali" };
	var status = HTTPStatus.Success;
}
```

2.4	Generics (Jenerikler)	Tip parametreleri <T, K> ile genel yapı ve fonksiyon tanımlama.
Tip parametreleri <T, K> ile tanımlanarak, kod tekrarı olmadan tip güvenliği sağlanır.
```
Nim

// Genel bir liste öğesini döndüren fonksiyon
T fn GetFirst<T>(data: list: T) {

	// ...
	return data[0]; 
}

// Genel bir veriyi sarmalayan yapı (Örn: Option<T> implementasyonu için)
struct Wrapper<T> {
	value: T;
}
```

2.5	Tip Dönüşümü	Açık Tip Dönüşümü (Casting) kuralı.
```
// Tip Dönüşümü (Açık Casting)
var small_num: i32 = ((i32)large_num); 
```


BÖLÜM 3: FONKSİYONLAR VE KAPSAM
----------------------------------------------------------------------
3.1	Fonksiyon Tanımı	fn söz dizimi, void dönüşsüz fonksiyonlar.
| fn | Fonksiyon tanımı. |
* Fonksiyon Parametreleri ve Dönüş: `<Dönüş Tipi> fn <İsim>(<param: tip>)`.

Örnek: 
```
b: i32 = 55555; // global kapsamda tanımlı

const pi: f32 = 3.1415; // sabit tanımlı


int fn Add(a: i32, b: i32) {

	return a + b;
 
}
 
int fn sqr(a: i32) {

	b: i32 = 2; // yerel kapsamda tanımlı
	
	return a * b;
 
} 

```

3.2	Metot Tanımlamaları (`impl` Blokları)Veri yapıları (`struct`) ve bu veriler üzerinde çalışan fonksiyonları (metotları) mantıksal olarak bir arada tutmayı sağlar.

Anahtar Kelime Açıklama
`impl` Bir yapıya (`struct`) ait metotların tanımlandığı blok.
`self` Metotun çağrıldığı yapının örneğini (`instance`) temsil eder.

Örnek Kullanım:
```
Nim

struct Point {
	x: i32;
	y: i32;
}

// Point yapısına metotlar ekleniyor
impl Point {
	// Statik Metot (Yapının bir örneğine ihtiyaç duymaz)
	Point fn New(x: i32, y: i32) {
		return new Point { x = x, y = y };
	}

	// Instance Metot (self: &Point ile örnek (instance) referansını alır)
	i32 fn GetDistance(self: &Point) {
		// self.x ve self.y değerlerine erişim
		return self.x + self.y; 
	}
}

void fn Main() {
	var p = Point.New(10, 20); // Statik çağrı
	var dist = p.GetDistance(); // Instance çağrı (20)
	Print("Uzaklık: {dist}");
}
```
3.3 Asenkron Programlama: 
async (asenkron fonksiyon tanımı) ve await (asenkron sonucu bekleme) anahtar kelimeleri eklenir. 
Bu, sistem programlama ve ağ işlemlerinde Coroutine tabanlı eşzamanlılığı sağlar.

3.4	Parametreler	
Opsiyonel Argümanlar (param: tip = değer) ve İsimli Argüman Çağrısı.
İsimli ve Opsiyonel Argümanlar
Fonksiyon çağrılarını esnekleştirir ve okunabilirliği artırır.
Opsiyonel Argüman: Fonksiyon tanımında parametreye varsayılan değer atanarak oluşturulur.
Söz Dizimi: `param: tip = <varsayılan_değer>`
İsimli Argümanlar: Fonksiyon çağrısı sırasında parametre isimlerinin açıkça belirtilmesi. 
Bu, pozisyon sırasının bozulmasına izin verir.
Örnek Kullanım:
```
Nim

void fn Render(x: i32, color: string = "blue", size: i32 = 10) { 
	Print("X:{x}, Color:{color}, Size:{size}");
}

void fn Main() {
	// 1. Varsayılanları kullan (Pozisyonel)
	Render(5); // Çıktı: X:5, Color:blue, Size:10

	// 2. Sadece bir varsayılanı değiştir (İsimli)
	Render(10, size: 20); // Çıktı: X:10, Color:blue, Size:20

	// 3. Sırayı karıştır (İsimli)
	Render(color: "red", x: 15, size: 30); // Çıktı: X:15, Color:red, Size:30
}
```

3.5 Fonksiyon Tanımlama (`fn`)
I.Dönüş tipi fonksiyon adından önce gelir. void dönüşsüz fonksiyonlar için kullanılır.

```
Nim

// İki int alır, bir int döndürür
i32 fn Multiply(a: i32, b: i32) {
	return a * b;
}

// void (dönüşsüz) fonksiyon
void fn Greeting(name: string) {
	Print("Merhaba, {name}");
}
```

II.	Geri Dönüş Standardı	$Fonksiyon_Adı ile anlık geri dönüş değerine erişim ve $Fonksiyon_Adı[indeks] ile Tuple ayrıştırması.
Geri Dönüş Değeri Standardının Genellenmesi Bu, dilin en güçlü ve özgün özelliklerinden biri olacaktır. 
Fonksiyonel programlamadaki `Tuple` (Demet) dönüşlerini, C++'daki geçici nesneye erişim gibi kolaylaştırır.
1. Genel Kural: Tekil Dönüş Değeri ve `$Fonksiyon_Adı` Bir fonksiyon normalde tek bir değer döndürür (int, string, struct, vb.) yada (void) döndürmez.
Fonksiyon çağrısının hemen ardından, bu tek değer, `$Fonksiyon_Adı` değişkeninde otomatik olarak saklanır.
void ise değer yoktur ve void tanımlanmış fonksiyon $fonksiyon oluşturmaz. 
Örn: `int fn GetCode();` çağrısı sonrası `$GetCode int` değerini taşır.
Örn: `void fn GetCodec();` çağrısı sonrası `$GetCodec` oluşturulmaz.

2. Çoklu Dönüş Değeri Kuralı (`Tuple` ve `İndeksleme`)Fonksiyonun dönüş tipi bir `Tuple` (Demet) ise, 
bu yapının tamamına `$Fonksiyon_Adı` ile erişilir ve ayrı elemanlara C stili indeksleme ile erişilir.
Kural: `Tuple` içindeki elemanlara sıfır tabanlı indeksleme ile erişim: `$Fonksiyon_Adı[indeks]`.

Örnek Uygulama:
```
Nim

// Fonksiyon Tipi: Tuple (int, string)
(int, string) fn GetUserStatus() {
	return (200, "OK");
}

void fn Main() {
	// 1. Çağrı yapılır, sonuç tuple olarak bellekte tutulur. GetUserStatus(); 
	
	// 2. $GetUserStatus (200, "OK") tuple'ının tamamını temsil eder.
	
	// 3. İndeksleme ile elemanlara erişim:
	
	if ($GetUserStatus[0] == 200) { // İlk eleman (int) kontrolü
		Print("Durum: {$GetUserStatus[0]}");
		Print("Mesaj: {$GetUserStatus[1]}"); // İkinci elemana erişim (string)
	}
}
```
Bu mekanizma, hem $rolling gibi özel yapıları netleştirir hem de `Result<T, E>` gibi 
Tuple tabanlı hata tiplerini kullanmayı son derece pratik ve okunaklı hale getirir.
Özet: Nihai NIM Özellik Güncellemesi `$rolling` Tekrar Deneme Sayacını (0'dan başlayan) temsil eder. 
(Tutarlılık düzeltmesi)Tüm fonksiyon dönüş değerleri, `$Fonksiyon_Adı` ile anlık olarak erişilebilir.
Çoklu dönüş değerleri (`Tuple`), `$Fonksiyon_Adı[indeks]` yapısıyla kolayca ayrıştırılabilir.

3.6	Lambda ve Anonim Fonksiyonlar	fn (x: int): int -> ... söz dizimi.
Lambda/Anonim Fonksiyonlar: `fn (x: int): int -> x * 2;`



BÖLÜM 4: OPERATÖRLER VE İFADELER
--------------------------------------------------
4.1	Operatör Önceliği	Tam Öncelik ve İlişkilendirme Tablosu.
```
Öncelik	Operatörler	İlişkilendirme
En Yüksek		()						
Pointer/Tekli	*, &, !, -			Sağdan Sola
Çarpımsal		*, /, %				Soldan Sağa
Toplamsal		+, -				Soldan Sağa
Karşılaştırma	==, !=, >, <, >=, <=	
Mantıksal VE	&&					Soldan Sağa
Mantıksal VEYA	||					Soldan Sağa
Atama			=					Sağdan Sola

```

4.2	Aritmetik ve Mantıksal Operatörler	+, -, *, /, %, 
```
Operator	Name	Description	Example	Try it
	+	Addition	Adds together two values	x + y	
	-	Subtraction	Subtracts one value from another	x - y	
	*	Multiplication	Multiplies two values	x * y	
	/	Division	Divides one value by another	x / y	
	%	Modulus		Returns the division remainder	x % y	
	++	Increment	Increases the value of a variable by 1	x++
	--	Decrement	Decreases the value of a variable by 1	x--
```

4.3	Bit Düzeyi Operatörler	&, `
Düşük seviyeli dil felsefesini tamamlamak için ikili veri manipülasyonu ve bayrak (flag) kontrolü için gereklidir.
```
Operator	Name	Example	Description	Try it
	&& 		AND		x < 5 &&  x < 10	Returns 1 if both statements are true	
	|| 		OR		x < 5 || x < 4		Returns 1 if one of the statements is true	
	!		NOT		!(x < 5 && x < 10)	Reverse the result, returns 0 if the result is 1
	`<<` 	`var res = 5 << 1;`			Sola Kaydırma (Left Shift)	
	`>>` 	`var res = 5 >> 1;`			Sağa Kaydırma (Right Shift)	
```

4.4	İşaretçi Operatörleri
Düşük seviyeli erişim için kullanılır.

4.4.1 `&` (Adres alma) ve `*` (Dereference)
Operatör	Açıklama
`&`	Adres alma (bir değişkenin hafıza adresini ptr olarak döndürür).
`*`	Dereference (işaretçinin gösterdiği değere erişir).

```
Nim

 value: i32 = 42;
// value'nun adresini al
 ptr_value: ptr = &value; //  ptr_value = 0x7ffe5367e044 

// ptr_value'nun gösterdiği değeri oku
 val_from_ptr: i32 = *ptr_value; // val_from_ptr = 42
```

4.4.2 İşaretçi Aritmatiği.




BÖLÜM 5: KONTROL AKIŞI
---------------------------------------------
5.1	Koşullu ve Döngü İfadeleri	if, else, while, for, break, continue.
* `if`, `else`, `while`, Basitleştirilmiş `for`.
* Döngü Kontrolü: **`break`** ve **`continue`**.
`for`, `break`, `continue`
Basitleştirilmiş for döngüsü ve standart kontrol ifadeleri.
```
Nim

// i=0'den başlar, 3 dahil
for (i, i <= 3, i++) { 
	if (i == 2) {
		continue; // 2'yi atla
	}
	Print("i: {i}"); //
}

// i=1'den başlar, 3 dahil
for (i=1, i <= 3, i++) { 
	if (i == 2) {
		continue; // 2'yi atla
	}
	Print("i: {i}"); //
}

// if
if (i == 5){
	print("sayı 5");
	break;
}

// if else
if (i == 5){
	print("sayı 5");
}else{
	print("Sayı yanlış.")
} 

// if elseif else
if (i < 9){
	print("Sayı 9 dan küçük.");
}elseif(i > 9){
	print("Sayı 9 dan büyük.");
}else{
	print("Sayı: {i}");
}

//	while
 i:i8 = 0;

while (i < 5) {
  print("\n{i}");
  i++;
}

```

5.2	Desen Eşleştirme	match ifadesi ve def varsayılan durumu.
Match İfadesi (Desen Eşleştirme)
Bir değeri birden fazla desene karşı kontrol eder.
```
Nim

void fn CheckStatus(code: int) {
	match (code) {
		200: { 
			print("İşlem Başarılı."); 
		}
		404: {
			// Birden fazla desen tek bir blokta
			print("Sayfa bulunamadı.");
		}
		def: { // Varsayılan durum
			print("Bilinmeyen HTTP kodu.");
		}
	}
}
```

5.3	Kontrollü Atlama `rolling: ETİKET` mekanizması ve `$rolling` sayacı.
Hata durumunda akışı aynı kapsamdaki önceki bir etikete geri döndürür. $rolling otomatik sayacı sağlar (0'dan başlar).
Örnek:
```
Nim

const MAX_RETRIES = 3;

REQUEST_BLOCK: { // İlk girişte $roll_count = 0
	Print("Deneme {$rolling}...");
	
	if ($rolling < MAX_RETRIES) {
		// Sayacı 1 artırır ve başa döner.
		rolling: REQUEST_BLOCK; 
	} else {
		throw new Exception("Kalıcı Bağlantı Hatası.");
	}
	
}
```

Örnek:
```
Nim
REQUEST_BLOCK: { // Burada $rolling = 0
	print("Deneme No: {$rolling}"); 
	
	// ... Hata oluştu ...
	
	if ($rolling < MAX_RETRIES) { 
		rolling: REQUEST_BLOCK; // $rolling'i 1 artırır ve başa döner.
	} 
}
```
I. Rolling Mekanizması Netleştirilmesi
`rolling`: ifadesi bir kontrol akışı komutudur, 
bir değer döndüren fonksiyon değildir. Bu ayrım, dilin tutarlılığı için önemlidir. 
Bu nedenle, `$rolling` değişkeninin sadece sayaç değerini taşıması, yapıyı netleştirir.
Geri Dönüş (Atlatma): `rolling: ETİKET;` komutu, yalnızca program akışını etikete geri atlatır. 
Bunun doğrudan bir geri dönüş değeri yoktur.
Sayaç Değeri Taşıma: `$rolling` sadece tekrar deneme sayısını taşır. 
Bu, değişkenin "sihirli" görünmesini engeller ve `rolling` mekanizması ile doğrudan ilişkilendirir.
Değişken Tanım Yeri Görev `$rolling` `rolling` etiketinin bulunduğu blok içinde örtülü olarak tanımlanır.
Tekrar Deneme Sayısı: Geri dönülen iterasyon sayısını tutar (0'dan başlar).


5.4	Grup Yapısı	group tanımı, etiketli giriş noktaları ve zincirleme erişim.
Etiketli giriş noktalarına sahip modüler kapsayıcıdır. `def` varsayılan etikettir.
```
Nim

group TaskGroup(x: i32,y: i32,z: i32) {
	//
	val: i32 = 10;
	
	// Etiketli fonksiyon
	kare: fn (x: i32): i32 -> x * 2;
	
	cube: { return x * x * x; } 
	// Alt Group
	sub_group: group(x) {
		// Alt Group etiketi
		test: { return x * x; }
		// Alt group default, zorunlu değildir.
		def: { Print("Alt Grup Çalıştı."); }
	}
	// group Default. Zorunlu değildir.
	def: { Print("Ana Grup Varsayılan İş. kayıt: {val}"); }
}

void fn Main() {
	TaskGroup(); // def bloğunu çağırır. // çıktı: "Ana Grup Varsayılan İş. kayıt: 10"
	TaskGroup.cube(3); // cube bloğunu çağırır. // çıktı: "27" 
	TaskGroup.kare(3); // Alt grup kare bloğunu çağırır. // çıktı: "6"
	TaskGroup.sub_group.test(3); // Alt grup bloğunu çağırır. // çıktı: "9"
	TaskGroup.sub_group(); // varsa Alt grup def bloğunu çağırır. // çıktı: "Alt Grup Çalıştı."
}
```

BÖLÜM 6: BELLEK VE GÜVENLİK MODELİ
-------------------------------------------------
6.1	Varsayılan Bellek Modeli	Sahibiyet (Ownership) kuralları ve otomatik yönetim.
* **Otomatik Yönetim:** Dinamik tipler (`list`, `string`) **Sahibiyet (Ownership)** kurallarına göre otomatik olarak serbest bırakılır.
* **Manuel Kontrol:** **`memory`** grubu altındaki fonksiyonlar (`memory.Alloc`, `memory.Free`, vb.) yalnızca gerektiğinde kullanılır.

6.2	Hata Yolu	`Option<T>`, `Result<T, E>` kullanımı.
Hata Yolu: İşlem sonucunu temsil eden tipler:
	* **`Option<T>`**: Değerin var veya yok (null) olduğunu belirtir.
	* **`Result<T, E>`**: Başarılı değer (`T`) veya hata değeri (`E`) döndürür.
Hata fırlatmak yerine (`throw`), sonucu dönüş tipine gömerek hatasız yolu zorlar.
```
Nim

// Başarılı durumda int, hata durumunda string döndürür.

fn SafeDivide:int, str(a: int, b: int) {
	if (b == 0) {
		return (null, "Sıfıra bölme"); // Hata yolu (Result: E)
	}
	return (a / b, null); // Başarı yolu (Result: T)
}
print(SafeDivide(9,0)); // çıktı: Sıfıra bölme
print(SafeDivide(9,3)); // çıktı: 3

```

6.3	Manuel Kontrol (memory Grubu)	
Düşük seviyeli bellek kontrol fonksiyonları.
`memory` Grubu: Manuel bellek kontrolü (`memory.Alloc`, `memory.Calloc`,`memory.Free`, `memory.Copy`, vb.).


BÖLÜM 7: MODÜLERLİK VE SİSTEM
---------------------------------------------------

7.1	Modül Sistemi	export (Dışa aktarma) ve use (Seçici/Tam İçe aktarma) kuralları.
Modül Sisteminin Çalışma Şekli
A. Varsayılan Görünürlük (Gizlilik)
NIM'de, bir modül (genellikle bir dosya veya bir group bloğu) içinde tanımlanan tüm değişkenler, 
sabitler, fonksiyonlar ve yapılar, varsayılan olarak gizlidir (private). 
Başka bir modül, bu öğelere doğrudan erişemez.

B. Dışa Aktarma (`export`)
export anahtar kelimesi, bir modülün diğer modüller tarafından erişilebilmesini istediği öğeleri açıkça işaretler.

Kural: Bir öğenin başka bir yerde kullanılabilmesi için, tanımlandığı modülde export ile işaretlenmesi zorunludur.

Örnek (math.nim dosyası içinde):
```
Nim

// Dışa aktarılmadığı için bu, sadece math modülü içinde kullanılabilir.
var PI_INTERNAL = 3.14; 

// Dışa aktarıldığı için başka bir modülde kullanılabilir.
export const PI = 3.14159;

// Dışa aktarılan fonksiyon
export i32 fn Add(a: i32, b: i32) {
return a + b;
}
```
C. İçe Aktarma (`use`)
use anahtar kelimesi, başka bir modülde export edilmiş öğeleri mevcut modüle dahil etmek için kullanılır. İki ana kullanım şekli vardır:

C.a. Tam Modül İçe Aktarımı (Önekli Kullanım)
Modülün tamamı içe aktarılır. Kullanım sırasında isimlendirme çakışmasını önlemek için modül adı önek olarak kullanılır.

Söz Dizimi: `use <modül_adı>;`

Örnek (`main.nim` dosyası içinde):
```
Nim

use math; // math modülü içe aktarılır.

void fn Main() {
	var result = math.Add(5, 3); // math. öneki zorunludur.
	Print("Sonuç: {result}");
}
```

C.b. Seçici İçe Aktarma (Doğrudan Kullanım)
Modülden sadece belirli öğeler süslü parantezler `{}` içinde belirtilerek içe aktarılır. 
İçe aktarılan öğeler doğrudan, önek almalıdır, group olarak yazılmış modüllerde aynı isim etiketi olan fonksiyonlar, değişkenler olabilir.

Söz Dizimi: `use <modül_adı> { <öğe_1>, <öğe_2> }`

Örnek (main.nim dosyası içinde):
```
Nim

// math modülünden sadece Add fonksiyonu içe aktarılır.
use math { Add, PI }

void fn Main() {
	var result = math.Add(10, 5);
	Print("Sonuç: {result}");
	Print("PI Sabiti: {math.PI}");
}
```

4. İsim Çakışmaları (Namespace Collisions)
Seçici içe aktarma (`use math { Add };`) kullanılırken, mevcut modülde zaten aynı isimde bir Add fonksiyonu varsa, 
derleyici bir hata verir ve geliştiricinin ya önekli kullanımı tercih etmesini ya da içe aktarılan öğenin adını değiştirmesini ister.
Bu yapı, büyük projelerde isimlendirme karmaşasını minimuma indirir ve kodun modüler ve güvenli kalmasını sağlar.


7.2	Yerel G/Ç Fonksiyonları	Print, Input, len(data) ve değişken interpolasyonu.
G/Ç ve Diğer Yerel Fonksiyonlar
`print("Text {var}")`: Konsola çıktı yazdırır, değişken interpolasyonu destekler.
`print("Text { var }")`:  { var }, { var}, {var }, /{var/} değişken değil string dir. 
`input(prompt)`: Kullanıcıdan girdi alır.
`len(data)`: Koleksiyon/string uzunluğunu döndürür.
```
Nim

// Değişken Interpolasyonu Destekler
var user_name = Input("Adınızı girin: ");
Print("Hoş geldin, {user_name}");

list: int nums = [10, 20, 30];
var count = len(nums); // len(data) söz dizimi
```

7.3	Sistem Grupları	file grubu ve diğer çekirdek kütüphaneler.
Bu gruplar, çekirdek işlevsellikleri içerir.

Grup	Fonksiyon (Örn.)	Açıklama
memory		`memory.Alloc(size)`, `memory.Free(ptr)`, `memory.Copy(dest, src, size)`	Manuel bellek yönetimi.

file	`file.Open("data.txt", "r")`, `file.Delete(path)`	Yerel dosya G/Ç işlemleri.

Modül	`use math { Add };`	Kodun içe/dışa aktarılması (export).

7.4 CPU VE PERFORMANS KONTROLÜ
-------------------------------------------------------------------

7.4.1 Kaydedici Erişimi (cpu Grubu)
Programcının kritik performans algoritmalarında hız optimizasyonu yapabilmesi için doğrudan CPU kaydedicilerine erişim sağlar.

Tip Güvenliği: Fonksiyonlar, `Generic (<T>)` olarak uygulanır. Okuma ve yazma işlemlerinde değerin tipi belirtilmelidir.
Mimari Bağımlılık: Kaydedici ID'leri `(id)`, derleme hedefi olan mimariye `(x86, ARM vb.)` göre semantik olarak eşlenir.
Söz Dizimi Açıklama:  `cpu.set_reg(id: i32, value: T)` Belirtilen id (kaydedici numarası) konumuna value değerini yazar.
`T fn cpu.get_reg(id:i32)` Belirtilen kaydedicideki değeri okur ve `T` tipine dönüştürür.

7.4.2 Hızlı Yürütme Kapsamı 
`(fast_exec: ETİKET)` Derleyiciye, belirli bir kod bloğu içindeki değişkenlerin ve işlemlerin kaydedici tahsisine öncelik verilmesi gerektiğini belirtir.
Kural: Bu blok içindeki değişkenler için derleyici, yığın `(stack)` yerine `register`'lari kullanmaya çalışır.
Söz Dizimi:
```
Nim
fast_exec: {
	// Sanal regsiter lar üretir. Derleyici bu register  öncelikli, olarak kod bütünlüğüne göre registerları ayarlar.
    // Bu bloktaki kod, kaydedici optimizasyonunda önceliklidir.
    // Hız kritik algoritmalar (Sıralama, Arama) buraya yazılır.

    cpu.set_reg(0, 0); // R0'a başlangıç değeri ata

    while (cpu.get_reg(0) < 10) {
        // ...
        cpu.set_reg(0, cpu.get_reg(0) + 1); 
    }
}
```

Kural: `cpu.asm: { ... }`: 
Bu blok, kodlayıcının doğrudan Assembly (ASM) komutlarını hedef mimarinin söz dizimine uygun olarak yazmasına izin verir. 
Bu bloklar, otomatik olarak fast_exec kapsamı içinde kabul edilir.
Örnek:
```	
Nim 
cpu.asm: {
	mov rax, [rbx] // rbx adresindeki değeri rax'e taşı 
	add rax, 10 // rax'a 10 ekle 
}
```
ASM bloğu içindeki NIM değişkenleri, özel bir söz dizimi `(%değişken_adı)` kullanılarak 
Kaydediciler veya yığın adresleri olarak referans alınabilir. (Örn: `add rax, %sayac_degeri`)


`cpu.threaded(2): { ... }` 
Bu blok,kodlayıcının doğrudan Paralel işlemler için uygun kod yazmasına izin verir. 
Bu bloklar, otomatik olarak fast_exec kapsamı içinde kabul edilebilir.

Aynı şekilde `gpu`, `tpu`,  modüller...

----------------------------------------------------------------------------





Modül Yazımı:
------------------------------------------------------------------------------------
Uygulama Örneği: `Modül` İçinde `group` Kullanımı
Bir modül tasarlarken, birbiriyle ilişkili tüm işlemleri tek bir group içine almak, 
o modülün API'sini (uygulama programlama arayüzünü) son derece düzenli hale getirir.

Örneğin, `network.nim` adlı bir modül yazıyorsunuz:

Modül İçinde Gruplama: Tüm HTTP ile ilgili işlemleri bir group içine alırsınız.
```
Nim

// network.nim
:network // ön ek olarak kullanılacak modül ismi..
export group HTTP { // group dışarıya aktarılıyor
    connect: { /* Bağlantı kodu */ }
	
    export get_data: { /* ... */ } // Grubun içinden de export edilebilir
	
	dc: { /* Bağlantıyı kapat .... */ }
	
    def: { Print("HTTP Grubu Başlatıldı."); }
}
```
Odaklanmış Erişim: Ana modülünüzde (`main.nim`) bu modülü kullanırken, 
hem modül sisteminin hem de group yapısının odaklanma özelliğinden yararlanırsınız:
```
Nim

// main.nim
use network; // network modülünün tamamını içe aktar

void fn Main() {
    // Doğrudan group ve etiketine odaklanarak erişim
    network.HTTP.get_data(); 

    // Veya varsayılan etiketini çağırabilirsiniz.
    network.HTTP(); 
}
```

Sonuç: group yapısı, modül yazımları için mükemmel bir seçimdir, çünkü büyük ve 
karmaşık API'leri (veri yapısı, I/O, bellek) mantıksal birimlere ayırmanıza ve 
kullanıcının zincirleme erişimle tam olarak istediği fonksiyona odaklanmasına olanak tanır. 
Bu, dilin hem modülerliğini hem de okunabilirliğini üst düzeye çıkarır.
