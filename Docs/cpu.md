cpu	İşlemci Kontrol ve Performans Araçları. 
Çekirdekler, kaydediciler (registers) ve işlemci komutları ile doğrudan etkileşim kurmak için fonksiyonlar ve araçlar sağlar. 
Yüksek performanslı sistem programlama, gömülü sistemler ve kritik algoritmalar için tasarlanmıştır.

cpu modülü, NIMBLE dilinin yüksek performans ve düşük seviye sistem programlama hedefleri için kritik öneme sahiptir. 
Dokümanınızdaki fast_exec ve asm blokları zaten bu alana işaret ediyor, bu yüzden modülün işlevi bu blokların 
davranışlarını ve yeteneklerini daha sistematik hale getirmek olmalıdır.

cpu Modülü Detaylandırması
cpu modülü, doğrudan donanımla (CPU) etkileşim kurmayı, performansı ince ayar yapmayı ve 
düşük seviyeli Assembly (ASM) programlama ile dilin yüksek seviye özelliklerini birleştirmeyi sağlar.

cpu İşlemci Kontrol ve Performans Araçları. Çekirdekler, kaydediciler (registers) ve işlemci komutları ile doğrudan etkileşim kurmak için 
fonksiyonlar ve araçlar sağlar. Yüksek performanslı sistem programlama, gömülü sistemler ve kritik algoritmalar için tasarlanmıştır.

1. Performans ve Kontrol FonksiyonlarıBu fonksiyonlar, yazılımcının kodun çalıştığı CPU ortamını manipüle etmesini ve sorgulamasını sağlar.
FonksiyonAmaçSöz Dizimi ve Açıklama

reg_get()Belirtilen kaydedicinin (register) mevcut değerini döndürür.

var rax_value = cpu.reg_get(RAX); Not: RAX, RBX, RCX gibi kaydediciler cpu modülünde sabit olarak tanımlanır.
reg_set()Belirtilen kaydedicinin değerini ayarlar.
cpu.reg_set(RBX, 500);

core_count()Mevcut sistemdeki fiziksel veya mantıksal çekirdek sayısını döndürür.
var num_cores = cpu.core_count();

pause()İşlemciye kısa bir duraklama (NOP benzeri) komutu göndererek, hyper-threading (HT) ve spin-lock durumlarında performansı optimize eder.
cpu.pause();

clflush()Belirtilen bellek adresindeki veriyi CPU önbelleğinden (cache) temizler. 
Donanım senkronizasyonu için kritiktir.
cpu.clflush(ptr_adres);

cpu modülü, NIMBLE dilinin yüksek performans ve düşük seviye sistem programlama hedefleri için kritik öneme sahiptir. 
Dokümanınızdaki fast_exec ve asm blokları zaten bu alana işaret ediyor, bu yüzden modülün işlevi 
bu blokların davranışlarını ve yeteneklerini daha sistematik hale getirmek olmalıdır.

2. Zamanlama ve Ölçüm AraçlarıKritik kod bloklarının performansını ölçmek için doğrudan CPU'dan veri alır.

rdtsc()Zaman Damgası Sayacı (RDTSC - Read Time-Stamp Counter) değerini okur. Çok hassas, ancak frekans değişikliklerinden etkilenebilen bir zaman ölçümü sağlar.
var t1 = cpu.rdtsc();

get_freq()Mevcut CPU çalışma frekansını (MHz cinsinden) döndürür.
var freq = cpu.get_freq();


3. Assembly (ASM) Entegrasyonu
Dokümanda zaten bulunan asm: ETIKET { ... } yapısı, cpu modülünün doğal bir uzantısı olarak kabul edilmelidir.

asm: ETIKET { ... }Inline Assembly Bloğu. Doğrudan Assembly kodu yazılmasına izin verir. 
Bu blokların performansı artırmak için cpu modülü kapsamı içinde olduğu varsayılır.
ASM Değişken Erişimi: NIM değişkenlerine özel söz dizimi (%değişken_adı) ile kaydedici/yığın adresi olarak referans verilir.

fast_execMikro Optimizasyon Bloğu. Derleyiciye, bu kapsamdaki kod için en agresif hız optimizasyonlarını uygulaması talimatını verir. 
Genellikle asm bloklarını çevreleyen kodlar için kullanılır.Bu yapının modül içindeki fonksiyonlar için otomatik olarak uygulandığı varsayılabilir.




