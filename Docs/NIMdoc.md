NIM (NIMBLE) PROGRAMLAMA DİLİ

BÖLÜM 1: GİRİŞ VE TEMELLER
-------------------------------------------------------------------------------
Version: 1.0 (Çalışma Zamanı ve Generics Destekli, Tutarlı Söz Dizimi)
Felsefe: Prosedürel, Fonksiyonel, Modüler, Güvenli Kontrol.


1.1 TEMEL SÖZ DİZİMİ
NIM, C-stili söz dizimini benimser. 
Yorum satırları tekli: `// tek satır yorum`. çoklu:  `/* çok satırlı  yorum */`
Tüm bloklar `{}` ile tanımlanır ve ifadeler noktalı virgül (`;`) ile sonlanır.


1.2 DEĞİŞKENLER
Değişkenlere isim vermenin genel kuralları şunlardır :
İsimler harf, rakam ve alt çizgi içerebilir
İsimler bir harf veya alt çizgi (_) ile başlamalıdır
İsimler büyük/küçük harfe duyarlıdır ( myVar ve myvar farklı değişkenlerdir)
İsimler boşluk veya !, #, %, vb. gibi özel karakterler içeremez.
Ayrılmış kelimeler (örneğin int, arr, char) isim olarak kullanılamaz

 tanım variableName:type = value;

`var`	Değiştirilebilir yerel veya global değişken. zorunlu değil.
`const`	Sabit değer. zorunlu..

Örnekler:
```
Nim

// Tip çıkarımı ile int/i32 atanır
	// yerel, global değişken
	count = 10; 	//  count:i32 = 10;

// Açıkça tip belirtme
	// yerel, global değişken
	var name:str = "NIM Language"; 
	var ver:str = "ver: 0.1.5"; 
	
// Sabit tanımlama
	const PI_VALUE:f64 = 3.14159; 

// Ondalık ayrımı
	const PI:f32[4] = 3.14159; 
	// PI = 3.1415 / ondalık ayrımdan sonra 4 rakam alır, 
	// 4 den az ise 0 ekleyerek tamamlar. 
	// örnek a:f32[4]= 1.25; // a = 1.2500 olur.
	
// Tuple tanımlama
	position:(i32, i32) = (100, 250); // i32 tipinde iki değişken
	position:(i32...) = (100, 250); // i32 tipinde birden fazla değişken
	post:any = ("ok", 200); // herhangi bir tipte (örneğin: int, arr, str, char, ptr) değişken 
	
// Çoklu değişken tanımlama
	x,y,z:i32; 				// tanımlama
	x = 5, y = 6, z = 50; 	// atama
	
	(x = 5, y = 6, z = 50):i32; // tanımlama ve atama
	

```

BÖLÜM 2: VERİ TİPLERİ
-------------------------------------------------------------------------------

2.1	Temel ve Özel Tipler
Sayısal: `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`, `f128`
		 `u8`, `u16`, `u32`, `u64`, `u128`

`int`   = `i32`
`float` = `f32`
`void`	= fn geri dönüşsüz.
`bool`  = `true` or `false`
`char`  = 1 karakter  `char[8]` 8 karakter
`bit`	= 1bit        `bit[16]` 16 bit
`byte`  = 8bit        `byte[4]` 4 byte
`null`	= Null / Başarısız fonksiyonlar null döndürür.
`ptr`   = pointer işaretçi
`ref`	= referans 
`[]`	= array		`cars[];`, `cars:arr;` boş array tanımlama.
`arr`	= array		`cars:arr = ("Volvo", "BMW", "Toyota");  cars[0]; // Volvo` 
		`arr` = Genel Array. `cars:arr;` genel bir array tipini ifade eder,  
		`[]` = Dinamik Array. `cars[];` ile boş, dinamik array tanımlanır.
`str`	= Metin: `str` (Varsayılan UTF-8). Kesin kodlama: `var msg:str[utf16] = "Data";`
				 `str[30];`  Bulunduğu blokta Tüm str "string" ler 30 karakterle sınırlandırılır.
`...`	= Değişken Sayıda Parametre Bildirimi (Homojen Tip). 
		`add(a:i32...)` söz dizimi, fonksiyona yalnızca `i32` tipinde, değişken sayıda parametre `(a[0], a[1],...)` aktarılacağını bildirir.
`any`	= Değişken/Karışık Tip (Heterojen). `str`, `int`, `float`, `arr` vb. herhangi biri veya çoklu karışık tipten oluşabilir. 
		Çoklu Geri Dönüş Tipi olarak kullanıldığında `(return myret:any(a:i32, b:str);)`, 
		farklı tiplerden değerleri bir arada döndürür ve bir dizi gibi erişilebilir (`myret[0]` - i32 değerini verir).


2.2 Temel tiplerin bit genişliği kesindir. Açık Tip Dönüşümü (Casting) veri kaybı riskine karşı zorunludur.
```
Nim

var large_num:i64 = 9000000000;

// Kayan nokta kullanımı
var ratio:f64 = 1.25; 

// İşaretçi tipi
var ptr_data:ptr = null;

```

2.3 Sayılar:   Bu , "10 üzeri 10" anlamına gelen (e veya E) harfi kullanılanımını destekler.
`35000 = 35e3`


2.4 Tip Dönüşümü:	Açık Tip Dönüşümü (Casting) kuralı.
`types` Modülü içerisinde tüm sayı tipleri için dönüşüm metotları bulunur.
```
// Tip Dönüşümü
small_num:i32 = i32(large_num); // i32 e dönüştürür. Dönüşümü yapan i32() fonksiyonudur.
```
`i64(dönüştürülecek_value)`vb.  `types` Modülü içerisinde tüm sayı tipleri için aynı şekilde bulunur...


BÖLÜM 3: OPERATÖRLER
-------------------------------------------------------------------------------------------------

3.1 Aritmetik operatörler
`+`		Addition	
`-`		Subtraction	
`*`		Multiplication
`/`		Division	
`%`		Modulus		
`++`	Increment	`x++`
`--`	Decrement	`x--`

3.2 Atama operatörleri
Tüm atama operatörlerinin listesi:
`=`		`x = 5`		`x = 5`	
`+=`	`x += 3`	`x = x + 3`	
`-=`	`x -= 3`	`x = x - 3`	
`*=`	`x *= 3`	`x = x * 3`	
`/=`	`x /= 3`	`x = x / 3`	
`%=`	`x %= 3`	`x = x % 3`	
`&=`	`x &= 3`	`x = x & 3`	
`|=`	`x |= 3`	`x = x | 3`	
`^=`	`x ^= 3`	`x = x ^ 3`	
`>>=`	`x >>= 3`	`x = x >> 3`	
`<<=`	`x <<= 3`	`x = x << 3`

3.3 Karşılaştırma operatörleri
`==`	Equal		
`===`	Identical	
`!=`	Not equal
`<>`	Not equal	
`!==`	Not identica
`>`		Greater than
`<`		Less than	
`>=`	Greater than or equal to
`<=`	Less than or equal to	

3.4 Mantıksal operatörler
`and`	And		
`&&`	And		
`or`	Or		
`||`	Or		
`xor`	Xor		
`!`		Not		

3.5 Bit düzeyinde operatörler 
Bitsel operatörler tam sayıların ayrı bitleri üzerinde çalışır
`&` : VE -
`|` : VEYA -
`^` : XOR -
`~` : NOT -
`<<`: Sola kaydırma
`>>`: Sağa kaydırma


BÖLÜM 4: YERLEŞİK FONKSİYONLAR KONTROL İFADELERİ
---------------------------------------------------------------------------------------------------

4.1 if, if else, if elseif else :
if
```
if (condition) {
  // code to be executed if condition is true;
}
```
if else
```
if (condition) {
  // code to be executed if condition is true;
} else {
  // code to be executed if condition is false;
}
```
if elseif else
```
if (condition) {
  code to be executed if this condition is true;
} elseif (condition) {
  // code to be executed if first condition is false and this condition is true;
} else {
  // code to be executed if all conditions are false;
}
```
if else kısa yazım
`b:str = a < 10 ? "Hello" : "Good Bye";`

4.2 Desen Eşleştirme `match` ifadesi ve `def` varsayılan durumu.
Match İfadesi (Desen Eşleştirme)
Bir değeri birden fazla desene karşı kontrol eder.
```
Nim

fn CheckStatus(code: int):void {
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

4.3 Kontrollü Atlama `rolling: ETİKET` mekanizması ve `$rolling` sayacı.
Hata durumunda akışı aynı kapsamdaki önceki bir etikete geri döndürür. $rolling otomatik sayacı sağlar (0'dan başlar).
Örnek:
```
Nim

const MAX_RETRIES = 3;

REQUEST_BLOCK: {  // Burada $rolling = 0
	print("Deneme {$rolling}...");
	baglan = network.HTTP.connect(url);
	.......
	.......
	// hata denetimleri....
	if (!baglan && $rolling < MAX_RETRIES) {
		// Sayacı 1 artırır ve başa döner.
		rolling:REQUEST_BLOCK;  // $rolling'i 1 artırır ve başa döner.
	} else {
		//new Exception("Kalıcı Bağlantı Hatası.");
		print("Kalıcı Bağlantı Hatası.");
	}
}
```

I. Rolling Mekanizması
`rolling`: ifadesi bir kontrol akışı komutudur, bir değer döndüren fonksiyon değildir.
Bu nedenle, `$rolling` değişkeninin sadece sayaç değerini taşıması, yapıyı netleştirir.
Belirlenen Hata durumunda akışı aynı kapsamdaki önceki bir etikete geri döndürür. 
Bu, klasik `try/catch` yapısı yerine belirlenen sayıda tekrar denemeyi (retry) programcı kontrolüne verir. 
`$rolling` etiketin otomatik sayacını sağlar (0'dan başlar).
Bunun doğrudan bir geri dönüş değeri yoktur.
Sayaç Değeri Taşıma: `$rolling` sadece tekrarlanma sayısını taşır. 
Bu, değişkenin "sihirli" görünmesini engeller ve `rolling` mekanizması ile doğrudan ilişkilendirir.
Değişken Tanım Yeri Görev `$rolling` `rolling` etiketinin bulunduğu blok içinde örtülü olarak tanımlanır.


4.4 Grup Yapısı:	
group tanımı: etiketli giriş noktaları ve zincirleme erişim.
Etiketli giriş noktalarına sahip modüler kapsayıcıdır. `def` varsayılan etikettir.
```
Nim

group TaskGroup(x: i32,y: i32,z: i32) {
	//
	val:i32 = 10;
	
	// Etiketli fonksiyon
	kare: fn (x:i32):i32 -> x * 2;
	
	cube: { return x * x * x; } 
	// Alt Group
	sub_group: group(x) {
		// Alt Group etiketi
		test: { return x * x; }
		// Alt group default, zorunlu değildir.
		def: { print("Alt Grup Çalıştı."); }
	}
	// group Default. Zorunlu değildir.
	def: { print("Ana Grup Varsayılan İş. kayıt: {val}"); }
}

void fn Main() {
	TaskGroup(); // def bloğunu çağırır. // çıktı: "Ana Grup Varsayılan İş. kayıt: 10"
	TaskGroup.cube(3); // cube bloğunu çağırır. // çıktı: "27" 
	TaskGroup.kare(3); // Alt grup kare bloğunu çağırır. // çıktı: "6"
	TaskGroup.sub_group.test(3); // Alt grup bloğunu çağırır. // çıktı: "9"
	TaskGroup.sub_group(); // varsa Alt grup def bloğunu çağırır. // çıktı: "Alt Grup Çalıştı."
}
```
4.5 while Döngüsü:
```
//	while
 i:i8 = 0;

while (i < 5) {
  print("\n{i}");
  i++;
}
```
4.6 for Döngüsü:
`for (i, i =< 5, i++) {...}` // i, 0 dan başlar 
`for (i=1, i =<5, i++) {...}` // i, 1 den başlar

(Dizilerde for işlemi) `for (myArr as val) {...}` // bir dizideki (myArr) elemanları ilk elemandan başlayarak sırayla (x)e atar. Dizideki tüm elemanlar bitince durur.
```
colors:arr = ("red", "green", "blue", "yellow");
for (colors as x) {
  print(x);
}

```
4.7 break, continue, return ifadeleri:
Döngünün sonuna gelinmemiş olsa bile şu ifadeyle `break` döngüyü durdurabiliriz.
```
colors:arr = ("red", "green", "blue", "yellow");
for (colors as val) {
  if (val == "blue") break;
  print("\n{val}");
}
```

Belirtilen bir koşul gerçekleştiğinde ifade `continue` bir yinelemeyi (döngüde) keser ve döngüdeki bir sonraki yinelemeyle devam eder.
```
for (i, i < 10, i++) {
  if (i == 4) {
    continue;
  }
  print("\n{i}");
}
```

4.8 Yerel G/Ç Fonksiyonları	Print, Input, len(data) ve değişken interpolasyonu. Bu fonksiyonlar için harici bir modül gerektirmez. 
G/Ç ve Diğer Yerel Fonksiyonlar
`print("Text {var}")`: Konsola çıktı yazdırır, değişken interpolasyonu destekler. `{val}` süslü parantezler içerisinde sağında ve solunda boşluk bulunamaz.
`print("Text { var }")`:  `{ var }`, `{ var}`, `{var }`, `\{var}` değişken değil string dir. 
`prompt:i32 = input("bir sayı giriniz?")`: Kullanıcıdan girdi alır.
`len(data)`: Koleksiyon/string uzunluğunu döndürür.
Kaçış karakterleri `\` gibi desteklenir.
```
Nim

// Değişken Interpolasyonu Destekler
var user_name:str = input("Adınızı girin: ");
print("Hoş geldin, {user_name}");

list nums:i32 = [10, 20, 30];
var count = len(nums); // len(data) söz dizimi
```


BÖLÜM 5: FONKSİYONLAR VE KAPSAM
----------------------------------------------------------------------

5.1	Fonksiyon Tanımı:	`fn` söz dizimi, void dönüşsüz fonksiyonlar.
fn  Fonksiyon tanımı. 
Fonksiyon Parametreleri ve Dönüş: `fn <İsim>(<param:tip>):<Dönüş Tipi>{...} `
```
Nim:

fn addNumbers(a:f32, b:f32):f32 {
  return a + b;
}

print(addNumbers(1.2, 5.2));

```

`fn func_name(...):i32{//...}` 	//	çoklu paramatre alımı. 
`fn func_name(myarr[]):arr{//...}` //	bir array parametre olarak alır ve array geri döner.
`fn func_name():void{//...}` 	//	void dönüşsüz fonksiyonlar.
`fn func_name(){//...}` 		//	void dönüşsüz fonksiyonlar.
`fn (x:i32):i32 -> x * 2;` 		//	Lambda/Anonim Fonksiyonlar
`print("Sayının Karesi: {kare(input("bir sayı giriniz:"))}");` 		//	parametre için fonksiyon kullanımı.

5.2 Opsiyonel Argümanlar (param:tip = değer) ve İsimli Argüman Çağrısı.
İsimli ve Opsiyonel Argümanlar
Fonksiyon çağrılarını esnekleştirir ve okunabilirliği artırır.
Opsiyonel Argüman: Fonksiyon tanımında parametreye varsayılan değer atanarak oluşturulur.
Söz Dizimi: `param:tip = <varsayılan_değer>`
İsimli Argümanlar: Fonksiyon çağrısı sırasında parametre isimlerinin açıkça belirtilmesi. 
Bu, pozisyon sırasının bozulmasına izin verir.
Örnek Kullanım:
```
Nim

void fn Render:void(x:i32, color:str = "blue", size:i32 = 10) { 
	print("X:{x}, Color:{color}, Size:{size}");
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

5.3 Callback
callback fonksiyonu, başka bir fonksiyona argüman olarak geçirilen bir fonksiyondur .
Parametre alan fonksiyonları da geçirebilirsiniz - sadece fonksiyon işaretçisi tipinin eşleştiğinden emin olun:
```
fn addNumbers(a:int , b:int ) {
  printf("The sum is: {a + b}");
}

fn calculate((*callback)(int, int), x:int , y:int ) {
  callback(x, y);
}

int main() {
  calculate(addNumbers, 5, 3);
  return 0;
}
```

5.4 Asenkron Programlama: 
`async` (asenkron fonksiyon tanımı) ve `await` (asenkron sonucu bekleme) anahtar kelimeleri eklenir. 
Bu, sistem programlama ve ağ işlemlerinde Coroutine tabanlı eşzamanlılığı sağlar.


5.5 Hata Yolu	`Option<T>`, `Result<T, E>` kullanımı.
Hata Yolu: İşlem sonucunu temsil eden tipler:
`Option<T>`: Değerin var veya yok (null) olduğunu belirtir.
`Result<T, E>`: Başarılı değer (`T`) veya hata değeri (`E`) döndürür.
Hata fırlatmak yerine, sonucu dönüş tipine gömerek hatasız yolu zorlar.
```
Nim

// Başarılı durumda int, hata durumunda string döndürür.

fn SafeDivide:any(a:int, b:int) {
	if (b == 0) {
		return (null, "Sıfıra bölme"); // Hata yolu (Result: E)
	}
	return (a / b, null); // Başarı yolu (Result: T)
}

print(SafeDivide(9,0)); // çıktı: Sıfıra bölme
print(SafeDivide(9,3)); // çıktı: 3

```

5.6 Satır içi fonksiyon
Derleyiciden kodunu çağrıldığı yere doğrudan eklemesini isteyen, doğrudan koda atlamasını istemeyen küçük bir fonksiyondur.
```
inline fn square(x:int ):int {
  return x * x;
}
```

5.7 Özyineleme
Özyineleme, bir fonksiyonun çağrısını kendi kendine yapma tekniğidir. 
Bu teknik, karmaşık problemleri, çözülmesi daha kolay olan basit problemlere bölmenin bir yolunu sunar.

Örnek
```

factorial(n:int):int {
  if (n > 1){
    return n * factorial(n - 1);
  }else{
    return 1;
  }
}

int main() {
  print("Factorial of 5 is {factorial(5)}");
  return 0;
}

```
Faktöriyel, bir sayının kendisinden sonra gelen ve 1'e kadar olan tüm sayılarla çarpılması anlamına gelir. 
Örneğin, 5'in faktöriyeli: 5 * 4 * 3 * 2 * 1 = 120'dir.


BÖLÜM 6: BELLEK ADRESİ VE İŞARETÇİ
---------------------------------------------------
6.1 Bellek adresi
Değişkene bir değer atadığımızda bu değer bu bellek adresinde saklanır.
Buna erişmek için referans operatörünü  `&` kullanın ve sonuç değişkenin nerede saklandığını gösterir:

```
myAge:i32 = 42;
print(&myAge); // Outputs 0x7ffe5367e044
```
Ayrıca, genellikle "işaretçi" olarak da adlandırıldığını unutmayın. 
`&myAge` Bir işaretçi, temel olarak bir değişkenin bellek adresini değeri olarak depolar.

6.2 Pointer İşaretçi
bildiriminde kullanıldığında bir işaretçi değişkeni `*ptr:i32` oluşturur.
Bildirimde kullanılmadığında, bir başvuru kaldırma operatörü gibi davranır.

`myAge:i32 = 42;     // Variable declaration`
`*ptr:i32 = &myAge;  // Pointer declaration`

// Reference: Output the memory address of myAge with the pointer (0x7ffe5367e044)
`print(ptr);`

// Dereference: Output the value of myAge with the pointer (42)
`print(*ptr);`

6.3 İşaretçiler ve Diziler
Dizilere erişmek için işaretçileri de kullanabilirsiniz.
```
myNumbers[4]:int = {25, 50, 75, 100};

for (i, i < 4, i++) {
  print("\n{&myNumbers[i]}");
}
```
```
Sonuç:

0x7ffe70f9d8f0
0x7ffe70f9d8f4
0x7ffe70f9d8f8
0x7ffe70f9d8fc
```
Her bir elemanın bellek adresinin son sayısının farklı olduğunu ve 4 eklendiğini unutmayın.
Çünkü bir int(i32) türün boyutu 4 bayttır.

Bir dizinin adı aslında dizinin ilk elemanına işaret eden bir işaretçidir.
İlk elemanın bellek adresi dizinin adıyla aynıdır :
```
myNumbers[4]:i32 = {25, 50, 75, 100};
// Get the memory address of the myNumbers array
print(myNumbers);
// Get the memory address of the first array element
print("\n {&myNumbers[0]}");
```
```
Sonuç:

0x7ffe70f9d8f0
0x7ffe70f9d8f0
```
Bu temel olarak, işaretçiler aracılığıyla dizilerle çalışabileceğimiz anlamına gelir!
`myNumbers`, `myNumbers`'daki ilk öğeye bir işaretçi olduğundan, ona erişmek için şu `*` operatörü kullanabilirsiniz.

6.3 İşaretçi Aritmetiği:
İşaretçi Aritmetiği Türe Bağlıdır. Tüm işaretçiler aynı şekilde hareket etmez.
Bir işaretçiye ekleme yaptığınızda 1, işaretçi işaret ettiği şeyin boyutu kadar ilerler; sadece 1 bayt kadar değil.

Örneğin:

Bir *işaretçi:i32 bir tam sayının boyutu kadar (4 bayt) hareket eder.
Bir *işaretçi:char bir karakterin boyutu kadar (1 bayt) hareket eder.
Bir *işaretçi:str bir karakterin boyutu kadar (4 bayt) hareket eder.  utf8 karakterler 4 bayt yer kaplar.

Yani eğer her iki işaretçi de bellek adresinden başlıyorsa 1000:
int *→  p + 1 adrese taşınacaktı  1004
char*→  p + 1 adrese taşınacaktı  1001
str *→  p + 1 adrese taşınacaktı  1004  
Bu, işaretçi hareketinin eklediğiniz sayıya değil, işaret ettiği veri türüne bağlı olduğunu gösterir:


6.4 Fonksiyon işaretçisi
Bir fonksiyon işaretçisi normal bir işaretçi gibidir , ancak bir değişkene işaret etmek yerine bir fonksiyona işaret eder .
Bu, bir fonksiyonun adresini depoladığı ve bu fonksiyonu işaretçiyi kullanarak çağırmanıza olanak tanıdığı anlamına gelir.
Fonksiyon işaretçileri, program çalışırken hangi fonksiyonun çalıştırılacağına karar vermenizi sağlar 
veya bir fonksiyonu başka bir fonksiyona argüman olarak geçirmek istediğinizde kullanılır.
Geri aramalar, menüler ve esnek program tasarımı için kullanışlıdır.

`(*pointerName):returnType (parameterType1, parameterType2, ...);`
`(*ptr):int(i32...);`
Bir fonksiyonu bir işaretçiye iki şekilde atayabilirsiniz:
`ptr = add;`
`ptr = &add;`
İkisi de aynıdır, çünkü fonksiyonun adı zaten bellekteki adresini temsil eder.

İşaretçi atandıktan sonra, fonksiyonu iki şekilde çağırabilirsiniz:
`ptr(5, 3);`
`(*ptr)(5, 3);`
İkisi de geçerlidir ve aynı işi yaparlar.

İki sayıyı toplayan bir fonksiyona işaretçi:
```
add:int( a:int, b:int) {
  return a + b;
}

int main() {
  (*ptr):int(i32...) = add;
  result:int = ptr(5, 3);
  print("Result: {result}");
  return 0;
}
```
Not: Bir fonksiyon adı, kendi başına bellekteki kodunun başlangıcını gösterir. Yani, bir fonksiyon adı zaten bir işaretçi gibi davranır! 
Bir fonksiyon işaretçisi bildirmek, size yalnızca o adresi tutabilecek bir değişken verir; böylece onu değiştirebilir veya başkalarına aktarabilirsiniz.

Fonksiyon işaretçileri diğer fonksiyonlara aktarılabilir - buna callback denir .
Girdi olarak verdiğiniz bir fonksiyonun başka bir fonksiyonu çağırmasına izin verir.


BÖLÜM 7: YAPILAR (structs)
---------------------------------------------------------------------------------------------
7.1 struct
Yapılar bir biriyle ilişkili birkaç değişkeni tek bir yerde gruplandırmanın bir yoludur.
Yapıdaki her değişkene yapının bir üyesi denir.

Bir dizinin aksine , bir yapı birçok farklı veri türünü ( int, float, char, vb.) içerebilir.
```
// Create a structure called myStructure
struct myStructure { // Yapı adı
  myNum:i32, 	 // 
  myLetter:char //
}

int main() {
  // Create a structure variable of myStructure called s1
  myStructure => s1;

  // Assign values to members of s1
  s1.myNum = 13;
  s1.myLetter = "B";

  // Print values
  print("My number: {s1.myNum}\n");
  print("My letter: {s1.myLetter}");

  return 0;
}
```
Bir yapı, üye olarak başka bir yapıyı da içerebilir. 
Buna iç içe geçmiş yapı denir ve ilgili verileri katmanlar halinde gruplamak istediğinizde kullanışlıdır:
```
// NIM type
struct Owner {
  firstName:str[30],
  lastName:str[30]
}

struct Car {
  brand:str[30],
  year:i32,
  Owner => owner // Nested structure
}

int main() {
  Owner => person = {"John", "Doe"};
  Car => car1 = {"Toyota", 2010, person};

  print("Car: {car1.brand} : {car1.year}\n");
  print("Owner: {car1.owner.firstName} {car1.owner.lastName}\n");

  return 0;
}
```

Metotları tanımlamak için 'extend'

```
// 1. struct tanımı
struct Point {
    x: i32,
    y: i32
}

// 2. Metotları tanımlamak için 'extend' bloğu
extend Point {
    // Kurucu (Constructor) fonksiyonu
    fn new(x: i32, y: i32): Point {
        return { x: x, y: y }; // Yapı literali ile dönüş
    }

    // Değiştirilemez metot (self: referans)
    fn get_x(self: ref): i32 {
        return self.x;
    }

    // Değiştirilebilir metot (self: ptr - işaretçi)
    fn move_to(self: ptr, new_x: i32, new_y: i32): void {
        self.x = new_x;
        self.y = new_y;
    }
}

// Kullanım:
void fn Main() {
    var p1 = Point.new(10, 20);      // Kurucuyu çağırır
    print("X: {p1.get_x()}");        // Metot çağrısı
    p1.move_to(50, 60);              // Değiştirilebilir metot çağrısı
}
```


7.2 typedef
Anahtar sözcük, mevcut bir tür için yeni bir ad (bir takma ad typedef ) oluşturmanıza olanak tanır . 
Bu, karmaşık bildirimlerin okunmasını ve kodunuzun bakımını kolaylaştırır.

Örneğin, her zaman float yazmak yerine , kodu daha anlaşılır hale getirmek için 
şu şekilde adlandırılan yeni bir tür oluşturabiliriz :Temperature
```
typedef Temperature:f32;

int main() {
  today:Temperature = 25.5;
  tomorrow:Temperature = 18.6;

  print("Today: {today}C\n");
  print("Tomorrow: {tomorrow}C\n");

  return 0;
}
```


7.3 Enum Numaralandırma
Enum , sabitlerin (değiştirilemeyen değerlerin) bir grubunu temsil eden özel bir türdür.
Bir enum oluşturmak için enumanahtar kelimeyi kullanın, ardından enum adını yazın ve enum öğelerini virgülle ayırın:
```
enum Level {
  LOW,
  MEDIUM,
  HIGH
}
```
Son maddede virgüle gerek olmadığını unutmayın.
Büyük harf kullanmak zorunlu değildir, ancak çoğu zaman iyi bir uygulama olarak kabul edilir.

Varsayılan olarak, ilk öğe ( LOW) değerine 0, ikinci öğe ( MEDIUM) değerine 1vb. sahiptir.
Şimdi myVar'ı yazdırmaya çalışırsanız, 1şunu temsil eden çıktıyı alırsınız MEDIUM:
```
int main() {
  // Create an enum variable and assign a value to it
  Level myVar = MEDIUM;
  // Print the enum variable
  print(myVar);

  return 0;
}
```

Bildiğiniz gibi, bir enum'un ilk öğesinin değeri 0'dır. İkinci öğenin değeri 1'dir ve bu böyle devam eder.
Değerleri daha iyi anlamak için onları kolayca değiştirebilirsiniz:
```
enum Level {
  LOW = 25,
  MEDIUM = 50,
  HIGH = 75
}

print(myVar); // Now outputs 50
```

Belirli bir öğeye bir değer atarsanız, sonraki öğelerin numaralarının buna göre güncellenir:
```
enum Level {
  LOW = 5,
  MEDIUM, // Now 6
  HIGH // Now 7
}
```


BÖLÜM 8: BELLEK
---------------------------------------------------------------------------------
Varsayılan Bellek Modeli Sahibiyet (Ownership) kuralları ve otomatik yönetim.
Otomatik Yönetim: Dinamik tipler (`list`, `string`) Sahibiyet (Ownership) kurallarına göre otomatik olarak serbest bırakılır.

8.1 Düşük seviyeli bellek kontrol fonksiyonları.
`memory` Grubu: Manuel bellek kontrolü (`memory.Alloc`, `memory.Calloc`, `memory.reAlloc`, `memory.Free`, vb.) programcı tarafından gerektiğinde kullanılır.

Dinamik bellek bir değişkene ait değildir, yalnızca işaretçilerle erişilebilir.

`memory.Alloc` Dinamik bellek ayırmak için veya `memory.Calloc` işlevlerini kullanabilirsiniz . 
Bunları kullanmak için başlığı eklemeniz gerekir . Her ikiside belirtilen miktar bellek ayırır ve adresine bir işaretçi döndürür.
```
*ptr1:int = memory.Alloc(size);
*ptr2:int = memory.Calloc(amount, size);
```
Fonksiyonun , bayt cinsinden ölçülen ne kadar bellek ayrılacağını belirten size adında `memory.Alloc(size)` bir parametresi vardır .

Fonksiyonun `memory.Calloc(amount, size)` iki parametresi vardır:
miktar - Tahsis edilecek öğelerin miktarını belirtir.
boyut - Bayt cinsinden ölçülen her öğenin boyutunu belirtir.


Bir veri türü için doğru miktarda belleği ayırmanın en iyi yolu şu `sizeof` operatörü kullanmaktır:
```
*ptr1, *ptr2:int;
ptr1 = memory.Alloc(sizeof(*ptr1));
ptr2 = memory.Calloc(1, sizeof(*ptr2));
```

Daha önce de belirtildiği gibi, `sizeof` ne kadar bellek ayrıldığını ölçmek için kullanamayız,  
bunu öğelerin miktarını, veri türünün boyutuyla çarparak hesaplamamız gerekir:
Örnek
```
*students:int;
numStudents:int = 12;
students = memory.Calloc(numStudents, sizeof(*students));
print(numStudents * sizeof(*students)); // 48 bytes
```

8.2 Dinamik Belleğe Erişim
Dinamik bellek, veri türü işaretçinin türüyle belirtilen bir dizi gibi davranır.

Dizilerde olduğu gibi, dinamik bellekteki bir öğeye erişmek için, onun dizin numarasına bakın :

`ptr[0] = 12;`
Ayrıca ilk öğeye erişmek için işaretçiyi de referanssızlaştırabilirsiniz:

`*ptr = 12;`

Örnek
Dinamik bellekten okuma ve yazma:
```
// Allocate memory
*ptr:int;
ptr = memory.Calloc(4, sizeof(*ptr));

// Write to the memory
*ptr = 2;
ptr[1] = 4;
ptr[2] = 6;

// Read from the memory
print(*ptr);
print(ptr[1], ptr[2], ptr[3]);
```

8.3 Belleği Yeniden Tahsis Et
Eğer ayırdığınız bellek miktarı yeterli değilse, onu daha büyük hale getirmek için yeniden ayırabilirsiniz .
Yeniden tahsis, içinde saklanan verileri korurken farklı (genellikle daha büyük) miktarda bellek ayırır.

Fonksiyonu kullanarak ayrılan belleğin boyutunu değiştirebilirsiniz `memory.reAlloc()`.
Fonksiyon `memory.reAlloc()` iki parametre alır:

`*ptr2:int = memory.reAlloc(ptr1, size);`
İlk parametre yeniden boyutlandırılan belleğe ait bir işaretçidir.
İkinci parametre, ayrılan belleğin yeni boyutunu bayt cinsinden belirtir.

Tahsis edilen belleğin boyutunu artırın:
Örnek
```
*ptr1, *ptr2, size:int;

// Allocate memory for four integers
size = 4 * sizeof(*ptr1);
ptr1 = memory.Alloc(size);

print("{size} bytes allocated at address {ptr1} \n");

// Resize the memory to hold six integers
size = 6 * sizeof(*ptr1);
ptr2 = memory.reAlloc(ptr1, size);

printf("{size} bytes reallocated at address {ptr2} \n");
```

Not: Bellek ayırırken her zaman hata denetimini (if ptr2 == NULL ise) eklemelisiniz .

Not: Ayrıca, işiniz bittiğinde ayrılmış belleği her zaman serbest bırakmalısınız. 
Bu, programınızın beklendiği gibi davranmasını sağlamak için önemlidir, 
ancak aynı zamanda daha sürdürülebilir ve verimli olmasını da sağlar.

8.4 Belleği boşaltmak için şu `memory.free()` fonksiyonu kullanmanız yeterlidir:

Örnek
Tahsis edilen belleği boşalt:
```
// Free allocated memory
memory.free(ptr1);
ptr1 = null; // Belleği boşalttıktan sonra işaretcisini `null` ayarla.
```



BÖLÜM 9: MODÜLERLİK VE SİSTEM
---------------------------------------------------

9.1	Modül Sistemi	`export` (Dışa aktarma) ve `use` (Seçici/Tam İçe aktarma) kuralları.
Modül Sisteminin Çalışma Şekli
Varsayılan Görünürlük (Gizli)
NIM'de, bir modül (genellikle bir dosya veya bir group bloğu) içinde tanımlanan tüm değişkenler, 
sabitler, fonksiyonlar ve yapılar, varsayılan olarak gizlidir (private). 
Başka bir modül, bu öğelere doğrudan erişemez.

9.2. Dışa Aktarma (`export`)
`export` anahtar kelimesi, bir modülün diğer modüller tarafından erişilebilmesini istediği öğeleri açıkça işaretler.
Kural: Bir öğenin başka bir yerde kullanılabilmesi için, tanımlandığı modülde export ile işaretlenmesi zorunludur.

Örnek (math.nim dosyası içinde):
```
Nim
:math;
// Dışa aktarılmadığı için bu, sadece math modülü içinde kullanılabilir.
var PI_INTERNAL = 3.14; 

// Dışa aktarıldığı için başka bir modülde kullanılabilir.
export const PI = 3.14159;

// Dışa aktarılan fonksiyon
export fn Add:i32(a:i32, b:i32) {
	return a + b;
}
```
9.3. İçe Aktarma (`use`)
`use` anahtar kelimesi, başka bir modülde `export` edilmiş öğeleri mevcut modüle dahil etmek için kullanılır. İki ana kullanım şekli vardır:

9.3 -1  Tam Modül İçe Aktarımı (Önekli Kullanım)
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

9.3 -2 Seçici İçe Aktarma (Doğrudan Kullanım)
Modülden sadece belirli öğeler süslü parantezler `{}` içinde belirtilerek içe aktarılır. 
İçe aktarılan öğeler doğrudan, modül içerisinde en başta tanımllı `:modül_adı;` 'nı ön ek alır. Bu isim çakışmalarını engeller, 
group olarak yazılmış modüllerde aynı isim etiketi olan fonksiyonlar, değişkenler olabilir.

Söz Dizimi: `use <modül_adı> { <öğe_1>, <öğe_2> }`

Örnek (main.nim dosyası içinde):
```
Nim

// math modülünden sadece Add fonksiyonu ve  PI sabiti içe aktarılır.
use math { Add, PI }

void fn Main() {
	var result = math.Add(10, 5);
	Print("Sonuç: {result}");
	Print("PI Sabiti: {math.PI}");
}
```

9.4 Modül Yazımı:
Uygulama Örneği: `Modül` İçinde `group` Kullanımı
Bir modül tasarlarken, birbiriyle ilişkili tüm işlemleri tek bir group içine almak, 
o modülün API'sini (uygulama programlama arayüzünü) son derece düzenli hale getirir.

Örneğin, `network.nim` adlı bir modül yazıyorsunuz:
Modül İçinde Gruplama: Tüm HTTP ile ilgili işlemleri bir group içine alırsınız.

```
Nim

// network.nim
:network; // ön ek olarak kullanılacak modül ismi..
export group HTTP(any...){ // group dışarıya aktarılıyor
    connect: { /* Bağlantı kodu */ }
	
    export get_data: { /* ... */ } // Grubun içinden de export edilebilir, tüm grup yerine tercih edilebilir.
	
	dc: { /* Bağlantıyı kapat .... */ }
	
    def: { HTTP.connect(); print("HTTP Grubu Başlatıldı."); }
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
	
	// tüm group değilde içerisinden tekil aktarım varsa 
	network.get_data(); // vs...

    // Veya varsayılan etiketini çağırabilirsiniz.
    network.HTTP();
}
```

Sonuç: group yapısı, modül yazımları için mükemmel bir seçimdir, çünkü büyük ve 
karmaşık API'leri (veri yapısı, I/O, bellek) mantıksal birimlere ayırmanıza ve 
kullanıcının zincirleme erişimle tam olarak istediği fonksiyona odaklanmasına olanak tanır. 
Bu, dilin hem modülerliğini hem de okunabilirliğini üst düzeye çıkarır.


10 CPU VE PERFORMANS KONTROLÜ
-------------------------------------------------------------------

10.1 Kaydedici Erişimi cpu.nim
Programcının kritik performans algoritmalarında hız optimizasyonu yapabilmesi için doğrudan CPU kaydedicilerine erişim sağlar.

Tip Güvenliği: Fonksiyonlar, `Generic (<T>)` olarak uygulanır. Okuma ve yazma işlemlerinde değerin tipi belirtilmelidir.
Mimari Bağımlılık: Kaydedici ID'leri `(id)`, derleme hedefi olan mimariye `(x86, x64, ARM vb.)` göre semantik olarak eşlenir.
Söz Dizimi Açıklama:  `cpu.set_reg(id: i32, value: T)` Belirtilen id (kaydedici numarası) konumuna value değerini yazar.
`T fn cpu.get_reg(id:i32)` Belirtilen kaydedicideki değeri okur ve `T` tipine dönüştürür.

10.2 Hızlı Yürütme Kapsamı 
`(fast_exec: ETİKET)` Derleyiciye, belirli bir kod bloğu içindeki değişkenlerin ve işlemlerin kaydedici tahsisine öncelik verilmesi gerektiğini belirtir.
Kural: Bu blok içindeki değişkenler için derleyici, yığın `(stack)` yerine `register`'lari kullanmaya çalışır.
Söz Dizimi:
```
Nim
fast_exec: TEST{
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

10.3 thread thread.nim
Bu bloklar, manuel olarak fast_exec kapsamı içinde kabul edilebilir.
`cpu.thread(){ ... }` 
Bu blok, kodlayıcının doğrudan Paralel işlemler için uygun kod yazmasına izin verir. 
`cpu.thread.spawn{ ... }` 

Aynı şekilde `gpu`, `tpu`,  modüller...

10.4 ASEMBLER asm.nim
`asm: ETIKET { ... }`: 
Bu blok, kodlayıcının doğrudan Assembly (ASM) komutlarını hedef mimarinin söz dizimine uygun olarak yazmasına izin verir. 
Bu bloklar, otomatik olarak fast_exec kapsamı içinde kabul edilir.
Örnek:
```	
Nim 
asm: TEST2{
	mov rax, [rbx] // rbx adresindeki değeri rax'e taşı 
	add rax, 10 // rax'a 10 ekle 
}
```
ASM bloğu içindeki NIM değişkenleri, özel bir söz dizimi `(%değişken_adı)` kullanılarak 
Kaydediciler veya yığın adresleri olarak referans alınabilir. (Örn: `add rax, %sayac_degeri`)





11. Yazılacak Modüller:
Modüller:
*`memory`	Modülü: memory.nim  Manuel bellek yönetimi. `memory.alloc(size)`, `memory.calloc(amount,size)`, `memory.realloc(ptr1,size)`, `memory.free(ptr)`, `memory.copy(dest, src, size)`	
*`file`		Modülü: file.nim `file.open("data.txt", "r")`, `file.delete(path)`, `file.copy(path_orj_file,copy_path_file)` vb.
*`array`	Modülü: array.nim `array.count(), array.push(), array.add() ...` vb.
*`string`	Modülü: string.nim `string.len(), string.word_count(), string.toupper(), string.tolower(), string.replace()`... vb.
*`math`		Modülü: math.nim `math.PI`, `math.AU`, `math.sqrr()`, `math.sqr()`... vb.
*`json`		Modülü: json.nim `json.decode`, `json.encode ...`
*`regex`	Modülü: regex.nim `regex.match()`, `regex.replace()` vb.
*`io`		Modülü: io.nim
*`types`	Modülü: types.nim  Tip dönüşümlerini yapan fonksiyonlar. `i32()`, `i64()`, `f32()`, `f64()` vb.  
*`asm`		Modülü:	asm.nim
*`cpu`		Modülü:	cpu.nim
*`thread`	Modülü: thread.nim
*`gpu`		Modülü: gpu.nim
*`map` 		Modülü:	map.nim

...

