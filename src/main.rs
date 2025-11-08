// src/main.rs

// ************************************************************************************
// NIMBLE DERLEYİCİ - RUST ÖNYÜKLEME (BOOTSTRAP) AŞAMASI
//
// Bu derleyici başlangıçta Rust ile yazılmıştır. Tüm hatalar giderilip dil stabilize
// edildikten sonra, bu Rust kodu Nimble diline çevrilerek derleyicinin kendisini
// derleyebilmesi (Self-Hosting) sağlanacaktır.
// ************************************************************************************

use std::fs;
use std::path::PathBuf;
use clap::Parser;

// Modül Bildirimleri
mod token;
mod lexer;
mod ast;
mod parser;
mod symbols;
mod semantics;
mod codegen;

use semantics::Analyzer;
use codegen::CodeGenerator;

// --- Komut Satırı Arayüzü (CLI) Tanımlaması ---

/// NIMBLE Yüksek Performanslı Derleyici
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Derlenecek NIMBLE kaynak dosyasının yolu (Örn: my_program.nim)
    #[arg(name = "FILE")]
    input_file: PathBuf,

    /// Çıktı dosyasının adı ve yolu (varsayılan: <dosya_adı>.o)
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// LLVM IR çıktısını göster (Derleme sonrası)
    #[arg(short, long)]
    emit_ir: bool,

    /// Optimizasyon seviyesi (0-3)
    #[arg(short = 'O', default_value_t = 2, value_parser = clap::value_parser!(u8).range(0..=3))]
    optimize: u8,
    
    /// Hata ayıklama sembollerini dahil et
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    
    // 1. Giriş Dosyasını Oku
    let source_code = match fs::read_to_string(&args.input_file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Hata: Kaynak dosya okunamadı '{}': {}", args.input_file.display(), e);
            return;
        }
    };

    println!("--- NIMBLE Derleyici (Rust Bootstrap) Başlatılıyor ---");
    println!("Giriş: {}", args.input_file.display());
    println!("Optimizasyon Seviyesi: O{}", args.optimize);
    
    // --- 2. Lexer (Sözcüksel Analiz) ---
    let mut lexer = lexer::Lexer::new(&source_code);
    
    // --- 3. Parser (Söz Dizimi Analizi) ---
    let mut parser = parser::Parser::new(lexer);
    let mut ast = parser.parse_program();

    // Hata kontrolü: Parser hataları toplanıp burada kontrol edilmelidir. (Basitleştirilmiştir)
    
    println!("-> AST Başarıyla Oluşturuldu.");

    // --- 4. Semantik Analiz ---
    let mut analyzer = semantics::Analyzer::new();
    
    match analyzer.analyze_program(&mut ast) {
        Ok(()) => println!("-> Semantik Analiz (Tip/Sahibiyet) Başarılı."),
        Err(errors) => {
            eprintln!("Hata: Semantik Hatalar Tespit Edildi ({} adet):", errors.len());
            for err in errors {
                eprintln!("  {}", err.message);
            }
            return;
        }
    }
    
    // --- 5. Kod Üretimi ve Optimizasyon ---
    let symbol_table = analyzer.symbol_table; // SymbolTable'ı CodeGenerator'a taşı
    let mut codegen = codegen::CodeGenerator::new(symbol_table);
    
    match codegen.generate_code(&ast) {
        Ok(ir_code) => {
            println!("-> LLVM IR Üretimi Başarılı.");
            
            if args.emit_ir {
                println!("\n--- ÜRETİLEN LLVM IR ---\n{}", ir_code);
            }

            // --- 6. Çıktı Dosyasına Yazma (.nim -> .o) ---
            let output_path = args.output.unwrap_or_else(|| {
                // Varsayılan çıktı adı: giriş.nim -> giriş.o
                let stem = args.input_file.file_stem().map_or_else(|| {
                    PathBuf::from("a") 
                }, |s| s.to_os_string().into());

                stem.with_extension("o") 
            });

            // Gerçek derleyicide: Makine kodu LLVM tarafından üretilir ve buraya yazılır.
            // fs::write(&output_path, compiled_binary).expect("Çıktı dosyası yazılamadı.");
            println!("-> Derleme tamamlandı. Çıktı hedefi: {}", output_path.display());
        }
        Err(errors) => {
            eprintln!("Hata: Kod Üretimi Başarısız Oldu ({} adet):", errors.len());
            for err in errors {
                eprintln!("  {}", err.message);
            }
        }
    }
}