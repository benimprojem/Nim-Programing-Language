⚙️ NIM DERLEYİCİSİ TEKNİK TASARIMI (COMPILER_DESIGN.md)

BÖLÜM 1: GENEL MİMARİ VE FELSEFE
----------------------------------------------------------

1.1 Felsefe: Minimum Çıktı, Maksimum Optimizasyon
NIM derleyicisinin temel amacı, C veya Assembly gibi düşük seviyeli dillere yakın performans ve kontrol sağlayarak, 
nihai çıktıyı (executable) sadece kullanılan kod ile sınırlamaktır (Minimum Kod Çıktısı).

1.2 Mimari Yaklaşım
Derleyici, özellikle Sahibiyet (Ownership) ve ileriye dönük referanslar nedeniyle İki Aşamalı (Two-Pass) bir yapı kullanır:

Analiz Geçişi: Lexer, Parser, Sembol Toplama (Ön geçiş).

Üretim Geçişi: Semantik Analiz, Tip Kontrolü, Optimizasyonlar, Kod Üretimi.

BÖLÜM 2: GİRİŞ VE SÖZDİZİMİ ANALİZİ (LEXER/PARSER)
-----------------------------------------------------

2.1 Lexer Kuralı: 
Lexer, tüm tanımlayıcıları ve anahtar kelimeleri standart bir forma (örneğin küçük harf) dönüştürmeye devam eder. 
Kural, sadece yerel fonksiyon isimleri değil, tüm kod bazında geçerlidir.
Lexer, tanımlayıcıları (değişken, fn adları) okurken, onları Sembol Tablosuna kaydetmeden önce 
standart bir forma (örneğin tamamen küçük harfe) dönüştürmelidir. 
Ancak string sabitleri içerisindeki verilerde bu kural uygulanmaz.

2.2 F-String Interpolasyon Kuralı
Parser, 
Print("Text { var }") 
gibi ifadelerde, f ön eki olmadığı sürece {} parantezleri içindeki boşlukları (örn. { val }) dahili değişken olarak değil, 
sıradan dize karakterleri olarak ele almalıdır.

BÖLÜM 3: SEMANTİK ANALİZ VE KRİTİK KONTROLLER
----------------------------------------------------

3.1 Sahibiyet (Ownership) Sistemi ve Otomatik Bellek Yönetimi

Derleyici, string, list, map gibi dinamik tiplerin yaşam döngüsünü çalışma zamanı yerine derleme zamanında yönetir. 
Bu kontrol, Yaşam Döngüsü Analizcisi (Lifetime Analyzer) adı verilen Semantik Analizci modülü tarafından yapılır.

3.1.1 Temel Sahibiyet KurallarıKuralAçıklamaHata Durumu

Kural 1: Tek SahiplikHer dinamik değere ait bellek bloğunun sadece bir sahibi (Owner) olabilir.

Kural 2: Taşıma Kontrolü (Move)Sahibiyet transferi (atama, fonksiyon argümanı olarak geçme) yapıldığında, eski değişken hemen geçersiz (Invalid) olarak işaretlenmelidir.
Kullanımı halinde: UseAfterMoveError

Kural 3: Metot ile TaşımaBir metot, self: Point (değer ile) imzasıyla tanımlanmışsa, çağrıyı yapan kapsamdaki 
Point değişkeni çağrıdan hemen sonra geçersiz işaretlenmelidir.
İhlali durumunda: UseAfterMoveError

3.1.2 Ödünç Alma (Borrowing) Kuralları
Ödünç alma (& operatörü) ile, sahibiyet devredilmeden dinamik bir değere erişim sağlanır.

Kural 4: Kapsam Sınırı: Tüm ödünç alımların süresi, sahip (Owner) kapsam dışına çıkmadan önce sona ermiş olmalıdır.

Kural 5: Tek Değiştirilebilirlik: Bir değer aktif olarak değiştirilebilir (&mut T) ödünç alındıysa, o anda ne başka bir değiştirilebilir ne de salt okunur ödünç alma (&T) olamaz.

3.1.3 Otomatik Serbest Bırakma Mekanizması (Kapsam Tabanlı Temizlik)
Dinamik tipler için bellek yönetimi, hataları önlemek ve geliştirici yükünü azaltmak amacıyla otomatiktir.

Mekanizma: Sahibiyet Kurallarına tabi olan dinamik değerler, kapsam dışına çıktıklarında (Scope-Based Resource Management veya RAII benzeri) otomatik olarak serbest bırakılır.

Derleyici Rolü: Yaşam Döngüsü Analizcisi, bir değişkenin yaşam döngüsünün bittiği yeri (kapsam dışına çıkış anını) tespit eder ve o noktaya bellek serbest bırakma talimatlarını otomatik olarak enjekte eder.

Sonuç: Programcıların çoğunluğu için mem.Alloc/mem.Free kullanma zorunluluğu ortadan kalkar.

3.2 Manuel Bellek Yönetimi Kontrolü (ptr Tipi Güvenliği)
Bu bölüm, sadece düşük seviyeli kontrol gerektiren ve Sahibiyet sistemi tarafından yönetilmeyen ptr tipi (işaretçiler) kullanıldığında zorunlu kılınan güvenlik kontrollerini kapsar.

3.2.1 Bellek İzleme MekanizmasıSemantik Analizci, her kapsam için aşağıdaki bilgileri tutan hiyerarşik bir Bellek İzleyici Tablosu kullanır:
allocated_ptrs: mem.Alloc() ile tahsis edilen, henüz serbest bırakılmamış işaretçilerin listesi.
freed_ptrs: mem.Free() ile serbest bırakılmış işaretçilerin listesi.

3.2.2 Kritik Hata Kontrolleri
Kapsam Çıkışı Kontrolü (Sızıntı Tespiti): 
Bir kapsam dışına çıkıldığında, allocated_ptrs listesinde olup freed_ptrs listesinde olmayan her işaretçi için MemoryLeakError üretilir.

Çift Serbest Bırakma Kontrolü: 
Zaten freed_ptrs listesinde bulunan bir işaretçi için tekrar mem.Free() çağrısı yapılması durumunda DoubleFreeError üretilir.



BÖLÜM 4: KONTROL AKIŞI VE ÖZGÜN MEKANİZMALAR
----------------------------------------------------

4.1 Fonksiyon Dönüş Değeri Yönetimi (Geçici Değer Ömrü)
Bu mekanizma, fonksiyon dönüş değerlerinin (void değilse) bir değişkene atanmasına gerek kalmadan, 
zincirleme çağrılarda (fn1(fn2())) doğrudan ve güvenli bir şekilde kullanılabilmesini sağlar.

Optimizasyon Kuralı: Fonksiyonlardan dönen değerler, kod üretim aşamasında derleyici tarafından 
otomatik olarak yüksek öncelikli geçici değerler olarak ele alınır ve CPU Kaydedicilerinde (Register) tutulmaya çalışılır.

Güvenlik Kuralı (Ömür Kontrolü): Dinamik tip içeren dönüş değerlerinin sahipliği, 
açıkça bir değişkene taşınmadığı sürece sadece çağrıldığı ifadenin ömrüyle sınırlıdır. Bu sınırın aşılması durumunda (örneğin bir sonraki ifadede dinamik elemanına erişim) UnassignedOwnerError üretilir.

Amaç: Bu kural, process(read_file(...)) gibi zincirleme kullanımların kolayca ve bellek güvenliği ihlal edilmeden yapılmasını garanti eder.


4.2 Kaydediciye Öncelikli Yürütme (cpu Grubu)
Derleyici, programcıya kritik performans bloklarında kaydedicilere doğrudan erişim ve öncelik verme yeteneği sunar.
Söz Dizimi: cpu.set_reg(id, value), cpu.get_reg(id) fonksiyonları.
Kapsam: 
fast_exec: { ... } 
bloğu içinde kullanılan değişkenlere ve işlemlere, derleyici tarafından Kaydedici Tahsisinde yüksek öncelik verilmelidir.

4.3 Kontrollü Tekrar Deneme Mekanizması (rolling: ETİKET)
rolling: ETİKET mekanizması, programcının belirli bir kod bloğunu hata durumunda veya koşullu olarak sınırlı sayıda tekrar denemesini sağlayan özel bir kontrol akışı komutudur.

4.3.1 Semantik Analiz: Gizli Sayaç ($rolling)
Semantik Analizci, ETİKET: { ... } yapısıyla tanımlanmış her rolling bloğu için:

Örtülü Değişken Tanımı: 
Bloğun yerel kapsamına, başlangıç değeri 0 olan i32 tipinde, salt okunur bir $rolling değişkeni tanımlanır. 
Bu değişken, o bloğun kaç kez denendiğini takip eder.

Maksimum Deneme Sınırı: 
Programcı tarafından bir limit (örneğin rolling: MyLabel (limit: 3)) belirtilmezse, 
varsayılan bir maksimum deneme sayacı (MAX_ROLLING_ATTEMPTS = 5 gibi) derleyici tarafından atanır.

4.3.2 Kod Üretimi: rolling: ETİKET; Komutunun Dönüşümü
rolling: ETİKET; komutu, derleyici tarafından daha temel bir döngü ve koşullu atlama yapısına (C dilinde while ve continue veya goto) dönüştürülür.

NIM Kodu (Kaynak)Derleyici Dönüşümü (Düşük Seviye IR/Sözde Kod)Giriş: 

 MyLabel: { 
   if (condition) { 
     rolling: MyLabel; // Tekrar dene 
   } 
 }
 
 Dönüşüm: 
 LABEL_MyLabel: 
   if ($rolling > MAX_ATTEMPTS) { goto LABEL_EXIT; } 
   // ... (Blok Kodu) ... 
   if (condition) { 
     $rolling = $rolling + 1; 
     goto LABEL_MyLabel; 
   } 
 LABEL_EXIT:

4.3.3 Optimizasyon: Sayaç Ataması
$rolling değişkeni, $Fonksiyon_Adı gibi geçici bir değer olduğu için, derleyici tarafından CPU Kaydedicisine (Register) atanmaya öncelik verilmelidir (BÖLÜM 4.2).
Avantaj: rolling döngüsünün her iterasyonunda $rolling sayacını artırmak, bellek erişimi yerine doğrudan CPU kaydedici manipülasyonu ile gerçekleştirilir. 
Bu, tekrar deneme döngüsünün yönetim maliyetini minimuma indirir.

4.4 Asenkron Mekanizması: async ile işaretli fonksiyonların, kod üretimi aşamasında Coroutine yapısına dönüştürülmesi kuralı eklenir. 
await çağrılarının, Coroutine'in askıya alma (yield) noktaları olarak işlenmesi sağlanır.
4.5 Gömülü Assembly (cpu.asm: { ... })

Parser/Semantik Analiz: 
cpu.asm bloğu içindeki tüm içerik bir dize sabit bloğu olarak ayrıştırılır, 
ancak bu bloğun mimariden bağımsızlığı yoktur ve doğrudan hedef mimariye (x86_64, ARM) bağlıdır.

Kapsam Kuralı: 
cpu.asm bloğu, otomatik olarak Kaydedici Tahsisinde en yüksek önceliğe sahip bir fast_exec kapsamı içinde sayılır.

Güvenlik Sınırlaması: 
ASM bloğu, bellek yönetimi (Ownership/Borrowing) kurallarını bypass edebilir. 
Bu nedenle, derleyici, bu blok içinde dinamik tiplerin (string, list) yaşam döngüsünün tehlikeye atıldığı durumlarda
 bir ciddi uyarı (Severe Warning) yayınlamalıdır.
 
 
4.6 Modül Çözümleme ve İsim Çakışmaları
Derleyicinin, use ile içe aktarılan öğeleri Sembol Tablosunda doğru şekilde çözmesi ve olası isim çakışmalarını yönetmesi gerekir.

4.6.1 İçe Aktarma Çeşitleri ve Sembol Kaydı
İçe Aktarma Tipi

NIM Kodu
Sembol Tablosu Kaydı
Tam İçe Aktarma
use network;

network modülündeki tüm export edilmiş öğeler, tam nitelikli isimle (network.HTTP.connect) Sembol Tablosuna kaydedilir.
Seçici İçe Aktarmause math { Add, PI };Add ve PI isimleri yerel kapsamda öncelikli olarak kaydedilir. 
Bunlar için tam nitelikli isme gerek kalmaz.
Grup İçe Aktarma

use network { HTTP };
network.HTTP grubu, yerel kapsamda sadece HTTP olarak kaydedilir. Erişim HTTP.get_data şeklinde olur.

4.6.2 İsim Çakışması KuralıSemantik Analizci, seçici içe aktarma sırasında (örneğin use math { Add }) yerel kapsamda veya 
daha önce içe aktarılmış bir modülde aynı ada sahip bir sembol bulursa:
Çözüm: Derleme hatası (NameConflictError) verir.
Tavsiye: Hata mesajında, programcıya tam nitelikli isim kullanmasını (math.Add) veya içe aktarırken bir takma ad (alias) kullanmasını önerir.

Nim
// main.nim
use math { Add };
use array { Add }; // HATA: array.Add ile math.Add çakışıyor!


BÖLÜM 5: OPTİMİZASYON VE KOD ÜRETİMİ
----------------------------------------------------

5.1 Ağaç Sarsma (Tree Shaking) ve Ölü Kod Elemesi
Derleyici, tüm modülleri bağlama (Linking) aşamasında analiz eder ve sadece ana programdan erişilebilen fonksiyonları, tipleri ve değişkenleri nihai çıktıya dahil eder.

Kullanılmayan modül öğeleri (use math; ile içe aktarılsa bile kullanılmayan math.Subtract) çıktıda yer almaz.

5.2 $Fonksiyon_Adı Optimizasyonu
Kayıtçı Kullanımı: $Fonksiyon_Adı değeri, yığın yerine CPU Kaydedicilerinde tutulmaya çalışılır.

Gecikmeli Materyalleştirme: Tuple dönüşlerinde, $Fonksiyon_Adı ile sadece bir alt elemana erişiliyorsa, diğer elemanların hesaplanması (eğer maliyetliyse) atlanabilir.

5.3 Derleyici Script Desteği
Derleyici, derleme sürecini yapılandırmak ve özelleştirmek için NIM dilinde yazılmış bir yapılandırma/build scriptini desteklemelidir.

Mekanizma: derleyici grubu üzerinden API'ler sunulur (Örn: derleyici.Optimize(Seviye.Aggressive);).

Koşullu Derleme: Script içinde NIM'in if ve group yapıları kullanılabilir.

BÖLÜM 6: HATA YÖNETİMİ VE GÜVENLİK 
----------------------------------------------------

6.1 Tip Tabanlı Hata Yönetimi (Result<T, E>)
NIM, çalışma zamanı istisna mekanizması yerine, fonksiyonların başarı değerini veya hata değerini açıkça beyan ettiği Result<T, E> Generic tipini kullanır.
Tanım: Result<T, E> iki olası durumdan birini tutabilen bir tiptir: başarılı bir değer (T) veya bir hata değeri (E).

Nim
Result<i32, string> fn DosyaOku(path: string);

6.1.1 Semantik Analiz: Zorunlu Kontrol
Semantik Analizci, bir Result<T, E> döndüren fonksiyon çağrısının sonucunun mutlaka kullanıldığından veya işlendiğinden emin olmalıdır.

NIM Kodu (Hata Senaryosu)Semantik Analiz İşlemiDerleyici Hata Mesajı
DosyaOku("/a.txt"); 
// Sonuç kullanılmadı!

Çağrı sonucu bir Result tipidir ancak T veya E elemanlarına erişim yapılmadı. 
HATA!UnusedResultError: DosyaOku sonucu potansiyel bir hata içeriyor. Lütfen sonucu bir değişkene atayın veya hata işlemini yapın.

Bu kural, programcıyı hataları göz ardı etmeye zorlayan bir güvenlik mekanizmasıdır.

6.2 Hata Yayılımı Operatörü (? Operatörü)

Hata yönetimini kolaylaştırmak için NIM, ? operatörünü desteklemelidir. 
Bu operatör, bir Result değerini açmaya çalışır ve eğer hata (E) içeriyorsa, 
bu hatayı mevcut fonksiyonun dönüş tipiyle sararak hemen döndürür.

6.2.1 Derleyici Dönüşümü ve Kod ÜretimiDerleyici,
 ? operatörünü C'deki if/return yapısına dönüştürerek çalışma zamanı maliyetini minimumda tutar.
 
 NIM Kodu (Kaynak)Derleyici Dönüşümü (IR/Sözde Kod)Örnek: 
 
 var veri = DosyaOku(path)?;
 Dönüşüm: 
 Result<i32, string> tmp_res = DosyaOku(path); 
 if (tmp_res.is_error) { 
   return tmp_res.error_value; // Hatayı hemen döndür 
 } 
 var veri = tmp_res.ok_value;
 
 Ön Koşul: ? operatörünün kullanıldığı fonksiyonun dönüş tipi, Result tipindeki hata tipi (E) ile uyumlu olmalıdır. 
 Uyumsuzluk durumunda (örneğin fonksiyon Result<i32, u64> döndürürken DosyaOku Result<i32, string> döndürüyorsa), 
 Semantik Analizci TypeMismatchError üretmelidir.
 
 6.3 Optimizasyon: Hata Kontrolü
 Result<T, E> yapısı, Başarılı bir senaryoda (Result.Ok durumu), 
 ekstra bir performans yükü oluşmaz (yani istisna tablolarına erişim, yığın açma (unwinding) vb. yoktur).
 Öngörülebilir Kod Akışı: Hata akışı (is_error kontrolü), standart koşullu dallanma (if) olarak kodlandığı için 
 CPU'nun tahmin edicisi (branch predictor) tarafından daha iyi yönetilebilir.
 
 6.4 Manuel Hata Sinyali (Panic/Hata Fırlatma)Eğer programcı geri döndürülemeyecek, kritik bir hatayı belirtmek isterse, 
 NIM'in manuel ve genellikle son çare olan bir Panic mekanizması olmalıdır.
 Fonksiyon: void fn sistem.Panic(msg: string)
 Derleyici Rolü: Bu fonksiyon, derleme sırasında özel olarak işaretlenir. 
 Kod üretimi aşamasında, bu bir sistem çağrısı (syscall) veya abort() çağrısına dönüştürülerek programın 
 kontrollü bir şekilde sonlandırılmasını ve hata mesajının yazdırılmasını sağlar.
 
 BÖLÜM 7: GELİŞMİŞ ENTEGRASYON VE SCRİPTLEME
 
7.1 Derleyici Scripti Desteği (Build Scripting)
Derleyici, karmaşık projeleri yönetmek ve derleme sürecini hedef platforma göre özelleştirmek için NIM dilinde yazılmış yapılandırma (build) scriptlerini destekler.

7.1.1 Script Mekanizması ve Yürütme
Script Dosyası: Derleyici, proje kök dizininde bulunan ve export group BuildScript() { ... } yapısını içeren bir build.nim dosyasını otomatik olarak tanır.
İzole Yürütme: Script, ana kod derlenmeden önce, derleyicinin kendi sanal ortamında (Sandbox) yürütülür. 
Bu, script'in ana uygulamanın güvenliğini tehlikeye atmasını engeller.

Derleyici API: Script, derleyici çekirdeği tarafından sağlanan özel derleyici grubunu kullanarak derleme parametrelerini değiştirebilir.

7.1.2 Örnek Derleyici API Fonksiyonları (derleyici Grubu)
FonksiyonAçıklama
derleyici.Compile(path: string)Belirtilen yolu ana uygulama olarak derlemeye başlar. 
derleyici.Optimize(seviye: enum) Çıktı için optimizasyon seviyesini ayarlar 
(Örn: Seviye.Aggressive, Seviye.Size).derleyici.Define(flag: string)
Koşullu derleme için global bir bayrak tanımlar 
(Örn: derleyici.Define("DEBUG_MODE");).derleyici. Target(mimari: enum)
Hedef CPU mimarisini ayarlar (Örn: Mimari.x86_64, Mimari.ARM_v7).

7.1.3 Koşullu Derleme (Conditional Compilation)
Derleyici, script içinde ayarlanan bayraklara göre ana NIM kodunda hangi blokların derleneceğini belirleyebilir:

Nim
// main.nim
if defined("DEBUG_MODE") {
    // Sadece 'DEBUG_MODE' tanımlıysa derlenecek kod.
    Print("Hata ayıklama modunda."); 
}
Bu, kodun farklı ortamlar (Geliştirme, Üretim) için minimum çıktı hedefine uygun olarak özelleştirilmesini sağlar.

BÖLÜM 8: GELİŞTİRME ÇEVRESİ ENTEGRASYONU 
(IDE Desteği)Derleyici, kod yazma deneyimini iyileştirmek için editörlere ve IDE'lere (Notepad++, VS Code, PyCharm vb.) bilgi sağlamalıdır.

8.1 Sözdizimi Ağacı (AST) ve Hata Mesajı Biçimi
Ayrıştırma (Parsing) Sonucu: Derleyici, kaynak kodu ayrıştırdıktan sonra oluşan Soyut Sözdizimi Ağacını (AST) dışa aktarılabilir bir formatta (örneğin JSON) sunabilmelidir.

Hata Konumu: Tüm derleyici hataları (MemoryLeakError, NameConflictError, TypeMismatchError), dosya adı, satır numarası ve sütun numarası dahil olmak üzere tam konum bilgisi içermelidir. 
Bu, editörlerin hata yapılan satırın altını çizebilmesini sağlar.

8.2 Dil Sunucusu Protokolü (Language Server Protocol - LSP) 
DesteğiDerleyicinin kendisi veya bir yardımcı aracı (Language Server), modern IDE'lerin gerektirdiği LSP standardını uygulamalıdır.
LSP Fonksiyonu Derleyici İçin Uygulama Tamamlama (Completion)
Sembol Tablosundaki değişken, fonksiyon, grup ve metod adlarının (büyük-küçük harf duyarsızlığına uygun olarak) otomatik tamamlanması.
Tanıma Git (Go to Definition)Bir fonksiyon veya tip adı üzerinde iken, Sembol Tablosu üzerinden doğru modül ve dosyadaki tanımına atlama.
Linting/Uçan Kontrol (On-the-fly Check)Kod yazılırken, MemoryLeakError veya UseAfterMoveError gibi kritik hataların Semantik Analiz ile anında tespit edilip gösterilmesi.
Biçimlendirme (Formatting)PEP 8 benzeri bir standart formatlama kuralı uygulayan bir araç sağlamak. 
(Girintileme için her zaman 4 boşluk kullanılması gibi).

BÖLÜM 9: MODÜL ÇÖZÜMLEMESİ VE YAPISAL KAPSAM 
-------------------------------------------------------------

NIM'in group ve export yapısı, derleyicinin Sembol Tablosunu hiyerarşik (iç içe) ve iki aşamalı olarak inşa etmesini gerektirir.

9.1 İki Aşamalı Modül Tarama (Two-Pass Module Scanning)Derleyici, bir modülü derlemeye başlamadan önce (veya modül use ile içe aktarıldığında) tüm bağımlılıkları çözmek için iki aşamalı bir tarama yapmalıdır:
Ön Tarama (Header Scan): Yalnızca export ile işaretlenmiş tüm ana yapılar (fn, struct, const, group) tespit edilir.
Derleme Tarama (Full Scan): Tüm modülün kodu derlenir, içeriği ve bağımlılıkları Sembol Tablosuna kaydedilir.

9.2 Sembol Tablosu Hiyerarşisi 
(group Yönetimi)Sembol Tablosu, bir modülün tamamını temsil eden ana bir kök (root) düğümü ile başlar. group yapıları, bu kök düğümün altında yeni ve iç içe geçmiş (nested) kapsamlar oluşturur.

9.2.1 Kapsam (Scope) Kuralı:
Modül Kapsamı (Level 0): Modülün en üst seviyesindeki tüm export edilmiş öğeler bu kapsamdadır.
Grup Kapsamı (Level 1+): group yapısı, kendi içinde yeni bir kapsam tanımlar. Bu kapsam içindeki öğeler, dışarıdan yalnızca grup adı önekiyle erişilebilir.

NIM Kodu
Sembol Tablosu Hiyerarşisi

export group HTTP { ... }
ModülA -> HTTP (Group) -> (Alt Kapsam)export fn get_data { ... }
ModülA -> HTTP (Group) -> get_data (fn)void fn Main() { ... }
ModülA -> Main (fn) (Export edilmediyse görünmez)

9.3 Erişim ve Çözümleme KurallarıDerleyici, içe aktarma yöntemine göre sembol araması yapar.

9.3.1 Tam Nitelikli Erişim (Qualified Access)
use network; ile içe aktarılan bir modülün öğelerine erişim her zaman tam nitelikli olmalıdır.
Çözümleme Sırası: network.HTTP.get_data() 
ifadesinde Semantik Analizci sırasıyla şunu arar:
Kök Kapsamda (ModülA) network adında bir modül var mı?
network modülünün Sembol Tablosunda HTTP adında bir group var mı?
HTTP grubunun içinde export edilmiş get_data adında bir fonksiyon var mı?

9.3.2 Seçici İçe Aktarma 
(use {})Seçici içe aktarma (use network { HTTP };), bir group'u yerel kapsama taşıyarak erişimi kolaylaştırır.
Yerel Kayıt: Semantik Analizci, HTTP adını ana modülün yerel Sembol Tablosuna kaydeder ve bunun network modülündeki HTTP grubuna bir referans olduğunu belirtir.
Basitleştirilmiş Çözümleme: HTTP.get_data() ifadesinde, artık doğrudan yerel kapsamdaki HTTP referansı kullanılır ve arkasından get_data aranır.

9.3.3 Varsayılan Etiket Çağrısı (def)
Bir group tanımı içindeki def: { ... } etiketine, grup adı kullanılarak doğrudan erişilebilir.
Söz Dizimi: network.HTTP();
Derleyici Dönüşümü: Semantik Analizci, bu söz dizimini network.
HTTP.def(); olarak çözümler ve kod üretim aşamasında def etiketine atlama (jump) kodu üretilir.

9.4 Optimizasyon: 
Modül Sınırında Ölü Kod Eleme (LTO)Bu hiyerarşik yapı, BÖLÜM 5'teki (Optimizasyon) Ağaç Sarsma (Tree Shaking) stratejisini destekler.
Eleme Noktası: Derleyici, network modülünün Sembol Tablosunda export edilmiş ancak nihai programın AST'sinde 
hiçbir zaman kullanılmamış tüm group'ları ve fonksiyonları (get_data dışındaki tüm etiketleri) tespit eder.
Minimum Çıktı Garanti Edilmesi: Kullanılmayan bu yapılar, derleme çıktısına (IR veya makine koduna) asla dahil edilmez, böylece çıktı boyutu hedeflenen minimum seviyede tutulur.


BÖLÜM 10: DÜŞÜK SEVİYELİ KONTROL VE OPTİMİZASYON (PTR, CPU VE FAST_EXEC)
----------------------------------------------------------------------------------

10.1 İşaretçi (Pointer - ptr) Tipi ve Bellek Güvenliği
NIM'in ptr tipi, düşük seviyeli bellek erişimine izin verir. Derleyici, bu mekanizmaların hızını korurken, manuel bellek yönetiminin risklerini minimize etmelidir.

10.1.1 Manuel Bellek Yönetimi ve ptr Kuralı
Yaşam Döngüsü: 
ptr değişkenleri Sahibiyet sistemi tarafından yönetilmez. Bu nedenle, derleyici BÖLÜM 3.2'deki 
Bellek İzleyici mekanizmasını kullanarak mem.Alloc() ile ayrılan her belleğin, kapsam dışına çıkmadan önce mem.Free() ile serbest bırakıldığını kontrol etmekle yükümlüdür.

Dereference Güvenliği: 
Dereference (*ptr_value) işlemi sırasında, Semantik Analizci işaretçinin null olup olmadığını veya serbest bırakılmış 
bir bölgeye işaret edip etmediğini (Dangling Pointer) statik analiz yetenekleri dahilinde en iyi çabayla kontrol etmelidir. 
Kesin olarak belirlenemeyen durumlarda uyarı (Warning) verilmelidir.

Metot Sahibiyet Kuralı: 
self: Point ile çağrılan metotlardan sonra, 
sahibin UseAfterMoveError hatası verecek şekilde geçersiz ilan edilmesi kuralı eklenir.

10.2 Agresif Kaydedici Tahsisi (fast_exec Bloğu)
fast_exec: { ... } bloğu (BÖLÜM 4.2), derleyicinin Kaydedici Tahsisi ve optimizasyon algoritmalarını bu kod bloğu için en yüksek seviyeye çıkarmasını sağlar.

10.2.1 Alias Analizinde Kısıtlamalar
Agresif Analiz: Normal kodda temkinli olan derleyici, fast_exec bloğu içinde daha agresif bir İşaretçi Alias Analizi (Alias Analysis) uygular. 
Eğer işaretçiler arasında çakışma riski minimumsa, işaretçinin gösterdiği değer bile geçici olarak Kaydedicilerde (Registers) tutulmaya çalışılır.

Kaydedici Garantisi: Blok içindeki tüm yerel değişkenler (özellikle temel tipler), derleyici tarafından Kaydedici Tahsisçisine en yüksek öncelikli olarak iletilmelidir. 
Derleyici, bu blokta zorunlu bellek erişimi gerektirmeyen her işlemi Kaydediciler üzerinde yapmalıdır.

10.3 Doğrudan CPU Kontrolü (cpu Grubu)
cpu.set_reg ve cpu.get_reg (BÖLÜM 4.2), programcıya özel Kaydedici manipülasyonu sağlayan bir arayüzdür.

10.3.1 Kullanım Kısıtlaması (Safety Fence)
Bu düşük seviyeli çağrıların kararsızlığa yol açmaması için, Semantik Analizci cpu grubuna yapılan çağrıların yalnızca fast_exec bloğu içinde yapıldığını kontrol etmelidir.

Kural: fast_exec dışında cpu fonksiyonları kullanılırsa, derleyici ContextError: cpu grubu sadece fast_exec bloğu içinde kullanılabilir. hatası üretmelidir.

10.3.2 IR Dönüşümü ve Mimariden Bağımsızlık
Haritalama: cpu.set_reg(id, value) çağrısı, derleyicinin arka ucu tarafından, id parametresini hedef 
mimarinin gerçek kaydedici numarasına (Örn: x86'da RAX veya EAX) çözen mimariye özel bir haritalama tablosu üzerinden çözülmelidir.

Tip Dönüşümü: cpu.set_reg ve cpu.get_reg kullanılırken, değerin tipi (T) ile kaydedicinin doğal boyutu arasındaki tip dönüşümü (casting) otomatik olarak yapılmalıdır. 
Yapı (struct) gibi büyük tiplerin doğrudan kaydedicilere atanması ise yasaklanmalı veya yığına taşıma (stack spilling) mekanizması ile çözülmelidir.

10.4 Değişken ve Kaydedici Eşlemesi:

Eşleme Mekanizması: ASM bloğu içindeki %değişken\_adı referansları, derleyicinin Kaydedici Tahsisçisi (Register Allocator) tarafından çözülür.

Eğer sayac_degeri değişkeni bir Kaydediciye atanmışsa (örn. RSI), %sayac\_degeri ifadesi doğrudan RSI olarak değiştirilir.

Eğer değişken Kaydedicilere sığmıyorsa (Stack Spilling), derleyici onun yığındaki (Stack) adres ofsetini (örn. [rbp - 16]) ASM kodu içine enjekte eder.



BÖLÜM 11: JENERİKLER VE MONOMORPHIZATION 
-----------------------------------------------------------------------

11.1 Jenerik Tanım ve Semantik Analiz

11.1.1 Tip Parametrelerinin Kaydı
Semantik Analizci, struct List<T> veya fn Add<T>(a: T, b: T) gibi jenerik tanımları gördüğünde:
Soyut Kayıt: Sembol Tablosuna List ve Add isimlerini, yanlarında taşıdıkları jenerik parametrelerin (<T>) sayısını belirten soyut tipler olarak kaydeder.

11.1.2 Kısıt Kontrolü (Constraint Check)NIM'in jenerik sistemi, tip güvenliğini sağlamak için kullanılacak T tipinin gerekli operasyonları desteklediğinden emin olmalıdır.
Örtülü Kısıtlama: Eğer Add<T> fonksiyonu a + b işlemini yapıyorsa, derleyici, bu fonksiyon çağrıldığında kullanılan somut tipin (örneğin i32 veya f64) + operatörünü desteklediğini kontrol eder. Desteklemiyorsa, ConstraintError hatası üretilir.

Açık Kısıtlama: (Gerekli olası bir eklenti) 
İleride implements Comparable gibi açık kısıtlamalar tanımlanırsa, Semantik Analizci, jenerik kullanıldığında somut tipin bu arayüzü uyguladığını kontrol etmelidir.

11.2 Kod Üretimi: Monomorphization (Somutlaştırma)
Jenerik kodun nihai makine koduna dönüştürülmesinde kullanılan temel teknik, Monomorphization'dur.

11.2.1 Monomorphization PrensibiDerleyici, bir jenerik yapının (örneğin List<T>) 
her somut tip ile kullanıldığı yerde (örneğin List<i32>, List<string>), 
o somut tipe özel yeni ve benzersiz bir kopya (spesifik versiyon) oluşturur.

NIM Kodu 
(Jenerik Çağrı)Derleyici Dönüşümü (Monomorphization)
var int_list: List<i32>;
struct List_i32 { /* ... */ }
var str_list: List<string>;
struct List_string { /* ... */ }
Add<i32>(5, 3);
fn Add_i32(a: i32, b: i32) { return a + b; }

11.2.2 Sembol Tablosu Güncellemesi
List<i32> ilk kez kullanıldığında, Sembol Tablosuna List_i32 adında yeni bir somut tip eklenir.
İkinci kez List<i32> kullanıldığında, derleyici tekrar kopya oluşturmaz, var olan List_i32 somut tipini kullanır.

11.3 Optimizasyon Faydası (Minimum Çıktı)Monomorphization, çalışma zamanı (Runtime) performansını maksimize ederken, çıktıyı minimumda tutma hedefini de destekler:
Sıfır Çalışma Zamanı Maliyeti: Jenerik kod, çalışma zamanında herhangi bir tip kontrolü veya sanal çağrı (Virtual Call) gerektirmez. 
Derleyici, hangi somut fonksiyonu çağırdığını tam olarak bildiği için doğrudan çağırma talimatını üretir (Statik Çağrı). 
Bu, maksimum hız ve kaydedici optimizasyonu sağlar.
Ölü Kod Elemesi (Tree Shaking) Desteği: Monomorphization yalnızca kullanılan somut versiyonları oluşturur. Eğer List<f64> hiç kullanılmamışsa, derleyici List_f64'ü oluşturmaz veya nihai çıktıya dahil etmez. Bu, BÖLÜM 5'teki minimum çıktı hedefini güçlendirir.

11.4 cpu ve Jenerik EntegrasyonuDüşük seviyeli cpu grubuna yapılan çağrılar da jenerik olarak ele alınır (BÖLÜM 4.2).
Tip Güvenliği: cpu.set_reg<i32>(0, 10) çağrısı, derleyicinin kaydediciye atanacak değerin boyutunu (i32 = 4 bayt) 
tam olarak bilmesini ve kaydedicinin sadece o kısmını kullanmasını sağlar. 
Bu, doğrudan kaydediciye özel talimatların (register-specific instructions) üretilmesine olanak tanır.


BÖLÜM 12: GELİŞTİRME YOL HARİTASI VE TESTLER
------------------------------------------------
12.1 Aşama 1: Temel Çerçeve ve Söz Dizimi (Bootstrapping)
Hedef: NIM kaynak kodunu okuyabilen, söz dizimini analiz edebilen ve basit bir AST oluşturabilen minimum çalışır bir derleyici iskeleti oluşturmak.

Odak Noktaları:

Lexer/Parser: NIM'in C-stili söz dizimini ({}, ;) ve büyük-küçük harf duyarsızlığını uygulayın (BÖLÜM 2).

Basit Sembol Tablosu: var, const ve fn tanımlarını kaydedin.

Temel Kod Üretimi: i32 ve f64 gibi temel tiplerle "Hello World" ve basit matematiksel işlemleri C kodu (Transpiler) veya LLVM IR'a çevirin.

12.2 Aşama 2: Kritik Özelliklerin Uygulanması
Hedef: Dilin benzersiz ve performansı en çok etkileyen özelliklerini stabil hale getirmek.

Odak Noktaları:

Modül ve İçe Aktarma: use, export ve group yapılarını Sembol Tablosuna hiyerarşik olarak entegre edin. İsim çakışması hatalarını uygulamaya başlayın (BÖLÜM 9).

$Fonksiyon_Adı: Bu mekanizmayı uygulayın ve değeri bir CPU Kaydedicisine atama (veya simüle etme) mantığını IR'a geçirin (BÖLÜM 4).

Kontrol Akışı: rolling: ETİKET yapısını dahili while/goto mekanizmasına dönüştürün (BÖLÜM 4).

12.3 Aşama 3: Güvenlik ve Gelişmiş Tipler
Hedef: Dilin güvenlik mekanizmalarını ve tip sistemini sağlamlaştırmak.

Odak Noktaları:

Sahibiyet Sistemi: string ve list gibi dinamik tiplerde Move ve Borrow kurallarını uygulayın. UseAfterMoveError hatasını zorunlu kılın (BÖLÜM 3).

Manuel Bellek Kontrolü: mem.Alloc/mem.Free çağrılarını izleyen Bellek İzleyiciyi uygulayın. MemoryLeakError ve DoubleFreeError hatalarını ekleyin (BÖLÜM 3).

Hata Yönetimi: Result<T, E> tipini tanımlayın ve ? operatörünü sıfır maliyetli if/return yapısına çevirin (BÖLÜM 6).

12.4 Aşama 4: Optimizasyon ve Geliştirici Deneyimi
Hedef: Minimum kod çıktısı ve maksimum hız hedefini gerçekleştirmek ve IDE/Editör desteğini eklemek.

Odak Noktaları:

Monomorphization: Jeneriklerin (List<T>) kullanımını izleyin ve her somut tip için ayrı fonksiyon kopyaları oluşturun (BÖLÜM 11).

Ağaç Sarsma (Tree Shaking): Bağlama (Linking) aşamasında kullanılmayan tüm fonksiyon ve tipleri nihai çıktıdan çıkarın (BÖLÜM 5).

CPU Kontrolü: fast_exec bloğunu ve cpu grubunu Kaydedici Tahsisçisine entegre edin. Kullanım kısıtlamalarını kontrol edin (BÖLÜM 10).

Geliştirici Araçları: Hata ve uyarı mesajlarının IDE'lerin anlayacağı formatta (LSP) çıktılanmasını sağlayın (BÖLÜM 8).

12.5 Test Stratejisi
Tüm benzersiz özellikler için özel test senaryoları zorunludur:

Kritik Hata Testleri: MemoryLeakError, UseAfterMoveError, DoubleFreeError, UnusedResultError hatalarının her senaryoda doğru konumda ve doğru mesajla yakalandığından emin olun.

Performans Testleri: Özellikle fast_exec blokları ve rolling döngüleri içeren kod bloklarının, C/C++ karşılıklarına göre rekabetçi veya daha iyi performans gösterdiğini doğrulayın.

