thread	Eşzamanlılık ve Paralel İş Parçacığı Yönetimi. 
Çok çekirdekli sistemlerin gücünden faydalanmayı, işleri ayrı iş parçacıklarında (thread) güvenli bir şekilde çalıştırmayı sağlar.

1. Temel İş Parçacığı Yönetimi (Threads) 
Bu fonksiyonlar, C/C++'daki iş parçacığı (thread) yönetimine benzer, paylaşımlı bellek modelini temel alır.

`spawn()` Yeni bir iş parçacığı başlatır. Verilen fonksiyonu, ana iş parçacığından bağımsız olarak yürütür.
`thread.spawn(worker_function, arg1, arg2);`

`join()` Ana iş parçacığını durdurur ve belirtilen iş parçacığının bitmesini bekler. Kaynakların düzgün serbest bırakılması için kritiktir.
`thread.join(thread_handle);`

`detach()` İş parçacığını ana iş parçacığından ayırır. İş parçacığı bittiğinde kaynaklarını otomatik olarak serbest bırakır (Arka plan görevleri için).
`thread.detach(thread_handle); `

`sleep()` İş parçacığını belirtilen süre kadar askıya alır.
`thread.sleep(milliseconds);`

`id()` Çalışan iş parçacığının benzersiz kimliğini (ID) döndürür.
`var current_id = thread.id();`


2. Güvenli Eşzamanlılık (Senkronizasyon)
Paylaşılan bellek üzerinde veri yarışını (data race) önlemek için senkronizasyon araçları gereklidir.

`Mutex` (Karşılıklı Dışlama)Kritik bölümlere aynı anda yalnızca tek bir iş parçacığının erişmesini garanti eder. Veri yarışını önlemenin temel yoludur.
```
var lock = thread.Mutex.new();
lock.lock();// Kritik kod
lock.unlock();
```

`Semaphore`  Sınırlı sayıda iş parçacığının bir kaynağa erişimine izin verir.
```
var sem = thread.Semaphore.new(max_count);

sem.wait();// Kaynağa erişim
sem.signal();
```


`Atomic<T> `  Basit veri tiplerinde (tamsayılar gibi) okuma ve yazma işlemlerinin bölünemez (atomik) olmasını sağlar. Lock kullanmadan güvenli sayaçlar için idealdir.
```
var counter: thread.Atomic<i32>;
counter.add(1); // Güvenli artırma
```

3. Mesajlaşma Tabanlı Güvenli Eşzamanlılık (Channels) Rust veya Go gibi modern dillerin benimsediği güvenli eşzamanlılık modelidir. 
İş parçacıkları paylaşılan bellek yerine birbirlerine veri gönderir.

`Channel<T>`  İş parçacıkları arasında tip güvenli mesaj iletimi için bir boru hattı oluşturur.
`var ch = thread.Channel<str>.new();`

`send()` Bir kanala veri gönderir. (Bloklayıcı veya Bloklamayan olabilir.)
`ch.send("İş bitti.");`

`receive()`  Kanaldan veri alır. Kanal boşsa iş parçacığı veri gelene kadar bekler (bloklar).
`var msg = ch.receive();`