// loop_array.nim
// Bu dosya, döngüleri, kayan nokta sayılarını ve basit dizi kullanımını test eder.

fn main(): f64 {
// Kayan nokta değişkeni
var total: f64 = 0.0;

// Sabit dizi
// Dizi sözdiziminin doğru ayrıştırıldığını varsayıyoruz.
const FACTOR: f64 = 0.5;

// Basit bir While döngüsü (BÖLÜM 4)
var i: i32 = 0;
var max_iter: i32 = 4;

while i < max_iter {
    total = total + (i as f64) * FACTOR;
    i += 1; // i: 0, 1, 2, 3
}
// total = (0 * 0.5) + (1 * 0.5) + (2 * 0.5) + (3 * 0.5)
// total = 0.0 + 0.5 + 1.0 + 1.5 = 3.0

// Match ifadesinin (BÖLÜM 4) basit bir kullanımı
var check: str = "Total is ";

match max_iter {
    4 => check = "Loop ran four times.",
    _ => check = "Unexpected loop count.", // KeywordDef'in match varsayılanı olarak kullanıldığını varsayarak
}

// return statement tipinin f64 olduğunu kontrol eder.
return total; // 3.0 döner


}