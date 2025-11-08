Projeyi Derleme
Komut satırında projenizin ana dizininde (içinde Cargo.toml dosyanızın bulunduğu yerde) çalıştırın:

Bash

cargo build
Derleme Sonucu: Bu komut, hata ayıklama (debug) modu için optimize edilmiş bir ikili dosya oluşturur. 
Oluşan dosyanın adı (Linux/macOS'te) genellikle ./target/debug/main olacaktır.

2. İkili Dosyayı Çalıştırma (Örnek Kullanım)
Derleme başarılı olduktan sonra, derleyicinizi çalıştırmak için daha önce oluşturduğumuz 
compile_examples.sh betiğinde belirtilen formatı kullanabilirsiniz 
(Ancak dosya yolunu belirten -- işaretini eklemeyi unutmayın):

Bash

# program.nim adlı bir kaynak dosyanız olduğunu varsayarak:

cargo run -- program.nim

Veya belirli argümanlarla:

Bash

# LLVM IR çıktısını göster ve optimizasyonu O3 yap

cargo run -- --emit-ir -O 3 program.nim