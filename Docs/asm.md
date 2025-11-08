# 🛠️ NIMBLE Dili: Düşük Seviyeli Mimari Erişimi

## BÖLÜM 19.0: Assembly (ASM) Entegrasyonu

NIMBLE, kritik performans gereksinimleri olan veya donanıma özgü işlemler için Assembly (ASM) kodunun doğrudan dil içine yerleştirilmesine izin verir. Bu mekanizma, dilin soyutlama katmanını atlayarak doğrudan işlemcinin komut setini kullanmaya olanak tanır ve genellikle **`cpu`** modülünün bir uzantısı olarak kabul edilir.

### 1. `asm: ETIKET { ... }` Bloğu

`asm` bloğu, NIMBLE derleyicisine, içerideki metni hedef mimarinin Assembly söz dizimi olarak işlemesi talimatını verir.

| Yapı | Amaç | Açıklama |
| :--- | :--- | :--- |
| **`asm: ETIKET { ... }`** | Inline Assembly Bloğu. Doğrudan Assembly kodu yazılmasına izin verir. | Etiket zorunludur ancak genellikle bir kontrol akışı etiketi olarak kullanılmaz, sadece tanımlama amaçlıdır. |

**Örnek: Basit Assembly İşlemi (x86-64 Varsayımı)**

```nim
Nim

var toplama_sonucu: i64;

asm: TOPLAMA_KODU {
    // rax'a 5 değerini taşı
    mov rax, 5 
    // rax'a 10 ekle
    add rax, 10 
    // rax'taki sonucu NIMBLE değişkenine kaydet (bkz: 19.1)
    mov %toplama_sonucu, rax 
}
```

1.1 ASM Değişken Erişimi
asm bloğu içindeki Assembly kodu, çevreleyen NIMBLE fonksiyonunun yerel değişkenlerine erişebilir. 
Bu, Assembly ve NIMBLE kodları arasında veri alışverişi için temel mekanizmadır.
```
Söz Dizimi,Amaç,Açıklama
%değişken_adı,NIMBLE değişkenine referans verme.,"Bu söz dizimi, Assembly kodunun çalıştığı sırada derleyici tarafından değişkenin kaydedici veya yığın adresi ile değiştirileceğini belirtir."
(%değişken_adı),Değişkenin adresi üzerinden değere erişim.,"Assembly söz dizimine bağlı olarak, değişkenin kendisinin adresini (%değişken_adı) veya adresin içeriğini ((%değişken_adı)) kullanmak gerekebilir."
```

2. Mikro Optimizasyon: fast_exec Kapsamı
asm bloğu, doğası gereği kritik performans amaçlı kullanıldığı için, otomatik olarak fast_exec mikro optimizasyon kapsamı içinde kabul edilir.
```
Yapı,Amaç,Açıklama
fast_exec,Mikro Optimizasyon Bloğu.,"Derleyiciye, bu kapsamdaki kod için en agresif hız optimizasyonlarını uygulaması talimatını verir. Genellikle Assembly bloklarını çevreleyen NIMBLE kodları için kullanılır."
```


Kural: asm bloğunun kendisi her zaman fast_exec kapsamındadır. Eğer bloğu çevreleyen NIMBLE kodunun da aynı optimizasyonu alması isteniyorsa, fast_exec { ... } bloğu ile açıkça sarmalanmalıdır.

Örnek: fast_exec Kullanımı
```
Nim

void fn Hesaplama() {
    // Bu NIMBLE kodu için agresif optimizasyon uygula
    fast_exec {
        var a = 10;
        var b = 20;
        
        // Bu asm bloğu zaten fast_exec kapsamındadır.
        asm: CRITICAL_ADD {
            mov rax, %a
            add rax, %b
            mov %toplam, rax
        }
    }
}

```

