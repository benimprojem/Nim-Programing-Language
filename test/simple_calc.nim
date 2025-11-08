// simple_calc.nim
// Bu dosya, temel değişken tanımlama, atama ve koşullu akışı test eder.

// Programın başlangıç fonksiyonu
fn main(): i32 {
// 1. Değişken Tanımlama
var x: i32 = 10;
const y: i32 = 5;

// 2. Basit Aritmetik ve Atama
x = x + y * 2; // x = 10 + 10 = 20

// 3. Kontrol Akışı (If/Else)
if x > 15 {
    x += 100; // x = 120
} elseif x == 15 {
    x = 0;
} else {
    x = -1;
}

// 4. Fonksiyon çağrısı (main dışındaki fonksiyonları çağırmayı test eder)
var result = calculate_something(x, y); // calculate_something(120, 5)

return result; // 120 - 5 = 115 döner


}

// Yeni bir fonksiyon tanımlama
fn calculate_something(a: i32, b: i32): i32 {
return a - b;
}