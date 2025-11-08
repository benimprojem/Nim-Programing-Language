// src/codegen.rs

// Harici kütüphaneler (Gerçek bir projede Inkwell veya llvm-sys kullanılacaktır)
// use inkwell::context::Context; 
// use inkwell::module::Module;
// use inkwell::builder::Builder;

use crate::ast::{Node, Type};
use crate::symbols::SymbolTable;
use crate::token::Token; // Token enum'unu kullanmak için eklendi

// Kod Üretici Hata Tipi
#[derive(Debug, Clone)]
pub struct CodegenError {
    pub message: String,
}

pub struct CodeGenerator {
    pub symbol_table: SymbolTable, 
    errors: Vec<CodegenError>,
    temp_counter: usize, // Geçici LLVM değerlerini saymak için (%1, %2...)
    llvm_ir_output: String, // Üretilen LLVM IR kodunu tutar
}

impl CodeGenerator {
    
    pub fn new(symbol_table: SymbolTable) -> Self {
        CodeGenerator { 
            symbol_table, 
            errors: Vec::new(), 
            temp_counter: 0,
            llvm_ir_output: String::new(), // Başlatıldı
        }
    }
    
    fn emit(&mut self, ir: &str) {
        self.llvm_ir_output.push_str(ir);
    }

    fn get_temp_reg(&mut self) -> String {
        let temp = format!("%{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    fn get_llvm_type(&self, ty: &Type) -> String {
        match ty {
            Type::Primitive(s) if s == "i32" => "i32".to_string(),
            Type::Primitive(s) if s == "f32" => "float".to_string(),
            Type::Primitive(s) if s == "bool" => "i1".to_string(),
            Type::Primitive(s) if s == "void" => "void".to_string(),
            // Tip çıkarımı aşamasında belirlenen pointer tipi
            Type::Pointer(inner) => format!("{}*", self.get_llvm_type(inner)),
            // Basitleştirme için diğer karmaşık tipler
            _ => "i8*".to_string(), // Varsayılan işaretçi
        }
    }
    
    pub fn generate_code(&mut self, program: &Node) -> Result<String, Vec<CodegenError>> {
        // LLVM Modül tanımlaması gibi başlangıç ayarları
        self.emit("; LLVM IR Dili\n");
        self.emit("target triple = \"unknown-unknown-unknown\"\n");
        
        if let Node::Program(statements) = program {
            for stmt in statements {
                self.compile_statement(stmt);
            }
        }
        
        if self.errors.is_empty() {
             Ok(self.llvm_ir_output.clone())
        } else {
             Err(self.errors.clone())
        }
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(CodegenError { message });
    }

    // --- Derleme Yardımcı Fonksiyonları ---

    fn compile_statement(&mut self, node: &Node) {
        match node {
            Node::FunctionDeclaration { name, params: _, return_type: _, is_inline: _, body } => {
                self.compile_function(name, body);
            }
            Node::Declaration { is_const: _, names: _, type_hint: _, initializer } => {
                self.compile_declaration(initializer.as_ref().map(|v| &**v));
            }
            Node::IfStatement { condition, consequence, alternative } => {
                self.compile_if_statement(condition, consequence, alternative.as_ref());
            }
            Node::Assignment { target: _, operator: _, value } => {
                self.compile_expression(value); // Sadece değer kısmını derle
                // Atama (store) mantığı buraya eklenecek
            }
            Node::CallExpression { .. } => {
                 self.compile_expression(node); // Fonksiyon çağrıları ifade olarak kabul edilir
            }
            Node::ReturnStatement (value) => {
                 let (val_reg, val_ty) = self.compile_expression(value);
                 self.emit(&format!("\tret {} {}\n", val_ty, val_reg));
            }
            Node::LabeledBlock { label, body, is_fast_exec } => {
                self.compile_labeled_block(label, body, *is_fast_exec);
            }
            Node::RollingStatement { label } => {
                self.compile_rolling(label);
            }
            Node::BlockStatement(statements) => {
                statements.iter().for_each(|s| self.compile_statement(s));
            }
            _ => { /* Diğer deyimler */ }
        }
    }

    // İfadeleri derler ve geçici kayıt (register) adı ile tipini döndürür.
    fn compile_expression(&mut self, node: &Node) -> (String, String) {
        match node {
            // --- Literaller ---
            Node::Literal(token) => {
                match token {
                    Token::LiteralInteger(i) => (i.to_string(), "i32".to_string()),
                    Token::LiteralFloat(f) => (f.to_string(), "float".to_string()),
                    Token::LiteralString(s) => {
                         // Basitleştirme: String'leri LLVM'de statik global olarak tanımlamak gerekir.
                         // Şimdilik sadece i8* (pointer) olduğunu varsayalım.
                         (format!("@.str_{:?}", self.temp_counter), "i8*".to_string())
                    },
                    Token::LiteralTrue => ("1".to_string(), "i1".to_string()),
                    Token::LiteralFalse => ("0".to_string(), "i1".to_string()),
                    Token::KeywordNull => ("null".to_string(), "i8*".to_string()),
                    _ => {
                        self.add_error(format!("Hata: Desteklenmeyen literal {:?}", token));
                        ("0".to_string(), "i32".to_string())
                    },
                }
            }

            // --- Tanımlayıcı (Değişken Kullanımı) ---
            Node::Identifier(name) => {
                let reg = self.get_temp_reg();
                let ty = self.symbol_table.resolve(name).map(|s| s.type_info).unwrap_or(Type::Primitive("i32".to_string()));
                let llvm_ty = self.get_llvm_type(&ty);
                
                // load <tip> <kaynak>
                let ir = format!("\t{} = load {}, {}* @{}, align 4\n", reg, llvm_ty, llvm_ty, name);
                self.emit(&ir);
                (reg, llvm_ty)
            }
            
            // --- İkili İfadeler ---
            Node::BinaryExpression { left, operator, right } => {
                let (l_reg, l_ty) = self.compile_expression(left);
                let (r_reg, r_ty) = self.compile_expression(right);
                let result_reg = self.get_temp_reg();
                
                let is_float = l_ty == "float" || r_ty == "float";
                let (op_ir, res_ty) = self.get_binary_op_ir(operator, &l_ty, is_float);

                let ir = format!("\t{} = {} {} {}, {}\n", result_reg, op_ir, l_ty, l_reg, r_reg);
                self.emit(&ir);
                
                // Karşılaştırmalar i1 (boolean) döndürür
                let final_ty = if res_ty == "i1" { "i1".to_string() } else { l_ty };
                (result_reg, final_ty)
            }
            
            // --- Tekli İfadeler (Örn: *ptr, &x, -y, !z) ---
            Node::UnaryExpression { operator, right } => {
                let (r_reg, r_ty) = self.compile_expression(right);
                let result_reg = self.get_temp_reg();
                
                let ir = match operator {
                    Token::Minus => { // Negasyon (-x)
                        let op = if r_ty == "float" { "fsub" } else { "sub nsw" };
                        let zero = if r_ty == "float" { "0.0" } else { "0" };
                        format!("\t{} = {} {} {}, {}\n", result_reg, op, r_ty, zero, r_reg)
                    }
                    Token::Bang => { // Mantıksal NOT (!x)
                        format!("\t{} = xor i1 1, {}\n", result_reg, r_reg)
                    }
                    Token::Asterisk => { // Dereference (*ptr) - Pointerdan değeri oku
                        let inner_ty = r_ty.trim_end_matches('*');
                        format!("\t{} = load {}, {} {}, align 4\n", result_reg, inner_ty, r_ty, r_reg)
                    }
                    Token::Ampersand => { // Referans (&x) - Değişkenin adresini döndür
                        // Identifier'ın zaten adres (global veya alloca) olduğunu varsayarsak,
                        // burada sadece tipini pointer'a çevirmemiz gerekir.
                        // (Gerçek LLVM'de daha karmaşık ele alınır.)
                        return (r_reg, format!("{}*", r_ty));
                    }
                    _ => {
                        self.add_error(format!("Hata: Desteklenmeyen tekli operatör {:?}", operator));
                        return (r_reg, r_ty);
                    }
                };
                self.emit(&ir);
                (result_reg, r_ty.trim_end_matches('*').to_string())
            }

            // --- Fonksiyon Çağrıları ---
            Node::CallExpression { function, arguments } => {
                // Fonksiyon adını çöz
                let func_name = if let Node::Identifier(name) = function.as_ref() { name } else { "" };
                
                // Argümanları derle: (reg, ty) listesi
                let compiled_args: Vec<(String, String)> = arguments.iter().map(|arg| {
                    // Argument struct'ındaki .value alanını derle
                    if let Node::Argument { value, .. } = arg {
                        self.compile_expression(value)
                    } else {
                        self.add_error("Hata: Çağrı argümanı bekleniyor.".to_string());
                        ("0".to_string(), "i32".to_string())
                    }
                }).collect();
                
                // LLVM parametre listesini oluştur: <ty> <reg>, <ty> <reg>...
                let args_ir = compiled_args.into_iter().map(|(r, t)| format!("{} {}", t, r)).collect::<Vec<_>>().join(", ");
                
                // Geri dönüş tipini çöz (Sembol Tablosundan alınmalı)
                let ret_ty = self.symbol_table.resolve(func_name)
                    .and_then(|s| if let crate::symbols::SymbolKind::Function { return_type, .. } = s.kind { Some(return_type) } else { None })
                    .map(|ty| self.get_llvm_type(&ty))
                    .unwrap_or("void".to_string());
                
                if ret_ty == "void" {
                    // call void @func(args...)
                    let ir = format!("\tcall void @{}({})\n", func_name, args_ir);
                    self.emit(&ir);
                    ("".to_string(), ret_ty)
                } else {
                    // %reg = call <ret_ty> @func(args...)
                    let reg = self.get_temp_reg();
                    let ir = format!("\t{} = call {} @{}({})\n", reg, ret_ty, func_name, args_ir);
                    self.emit(&ir);
                    (reg, ret_ty)
                }
            }

            // --- Diğer ifadeler ---
            _ => { 
                self.add_error(format!("Hata: Desteklenmeyen ifade düğümü {:?}", node));
                ("0".to_string(), "i32".to_string())
            },
        }
    }

    // İkili Operatör için LLVM IR talimatını döndürür (örneğin "add nsw" veya "fcmp oeq")
    fn get_binary_op_ir(&mut self, operator: &Token, type_str: &str, is_float: bool) -> (String, String) {
        if is_float {
            match operator {
                Token::Plus => ("fadd".to_string(), type_str.to_string()),
                Token::Minus => ("fsub".to_string(), type_str.to_string()),
                Token::Asterisk => ("fmul".to_string(), type_str.to_string()),
                Token::Slash => ("fdiv".to_string(), type_str.to_string()),
                // Karşılaştırmalar i1 döndürür
                Token::Equal => ("fcmp oeq".to_string(), "i1".to_string()), 
                Token::NotEqual => ("fcmp one".to_string(), "i1".to_string()),
                Token::GreaterThan => ("fcmp ogt".to_string(), "i1".to_string()),
                Token::LessThan => ("fcmp olt".to_string(), "i1".to_string()),
                Token::GreaterEqual => ("fcmp oge".to_string(), "i1".to_string()),
                Token::LessEqual => ("fcmp ole".to_string(), "i1".to_string()),
                _ => {
                    self.add_error(format!("Hata: Desteklenmeyen kayan nokta operatörü {:?}", operator));
                    ("fadd".to_string(), type_str.to_string())
                },
            }
        } else {
            match operator {
                Token::Plus => ("add nsw".to_string(), type_str.to_string()),
                Token::Minus => ("sub nsw".to_string(), type_str.to_string()),
                Token::Asterisk => ("mul nsw".to_string(), type_str.to_string()),
                Token::Slash => ("sdiv".to_string(), type_str.to_string()), // İşaretli tamsayı bölme
                Token::Modulo => ("srem".to_string(), type_str.to_string()), // İşaretli tamsayı kalan
                // Karşılaştırmalar i1 döndürür
                Token::Equal => ("icmp eq".to_string(), "i1".to_string()),
                Token::NotEqual => ("icmp ne".to_string(), "i1".to_string()),
                Token::GreaterThan => ("icmp sgt".to_string(), "i1".to_string()), // İşaretli Büyüktür
                Token::LessThan => ("icmp slt".to_string(), "i1".to_string()), // İşaretli Küçüktür
                Token::GreaterEqual => ("icmp sge".to_string(), "i1".to_string()),
                Token::LessEqual => ("icmp sle".to_string(), "i1".to_string()),
                // Mantıksal operatörler (i1'ler üzerinde çalışır)
                Token::LogicAnd => ("and".to_string(), "i1".to_string()),
                Token::LogicOr => ("or".to_string(), "i1".to_string()),
                _ => {
                    self.add_error(format!("Hata: Desteklenmeyen tamsayı operatörü {:?}", operator));
                    ("add nsw".to_string(), type_str.to_string())
                },
            }
        }
    }


    // --- Kontrol Akışı ---
    
    // BÖLÜM 4.2: If/Else/Elseif yapısı
    fn compile_if_statement(&mut self, condition: &Node, consequence: &Node, alternative: Option<&Box<Node>>) {
        let then_label = format!("L_then_{}", self.temp_counter); self.temp_counter += 1;
        let else_label = format!("L_else_{}", self.temp_counter); self.temp_counter += 1;
        let merge_label = format!("L_merge_{}", self.temp_counter); self.temp_counter += 1;

        // 1. Koşulu derle
        let (cond_reg, _) = self.compile_expression(condition);
        
        // 2. Koşullu atlama (br i1 <koşul>, label <doğru>, label <yanlış>)
        let target_else = if alternative.is_some() { else_label.clone() } else { merge_label.clone() };
        self.emit(&format!("\tbr i1 {}, label %{}, label %{}\n", cond_reg, then_label, target_else));

        // 3. 'Then' bloğu
        self.emit(&format!("{}:\n", then_label));
        self.compile_statement(consequence);
        // 'Then' bloğundan birleştirme bloğuna atla (eğer zaten 'return' yoksa)
        self.emit(&format!("\tbr label %{}\n", merge_label));

        // 4. 'Else' veya 'Elseif' bloğu
        if let Some(alt) = alternative {
            self.emit(&format!("{}:\n", else_label));
            self.compile_statement(alt); // Recursively handle else if or else block
            self.emit(&format!("\tbr label %{}\n", merge_label));
        }

        // 5. Birleştirme (Merge) bloğu
        self.emit(&format!("{}:\n", merge_label));
    }


    // --- NIMBLE Özelliklerinin Derlenmesi (Mevcut mantık korunmuştur) ---
    
    // BÖLÜM 10.2: fast_exec ve Etiketli Bloklar
    fn compile_labeled_block(&mut self, label: &str, body: &Node, is_fast_exec: bool) {
        self.emit(&format!("{}:\n", label)); // LLVM Label
        
        if is_fast_exec {
            println!("LLVM Opt Hint: 'fast_exec' bloğu için yüksek optimizasyon ipucu eklendi: {}", label);
        }
        
        self.compile_statement(body);
        
        // Bu bloğun sonlanması 'br label %birleştirme' ile sağlanmalı.
        // Basitlik için burada eklenmemiştir.
    }

    // BÖLÜM 4.3: rolling: ETİKET;
    fn compile_rolling(&mut self, target_label: &str) {
        // rolling -> br label %{target_label}
        self.emit(&format!("\tbr label %{}\n", target_label));
        println!("LLVM IR: Koşulsuz atlama üretildi: br label %{}", target_label);
    }

    // BÖLÜM 5.6: inline fn
    fn compile_function(&mut self, name: &str, body: &Node) {
        // Fonksiyonun giriş etiketi
        self.emit(&format!("define void @{}() {{\n", name)); // Basitleştirme: Parametresiz ve void dönüşlü varsayalım
        
        if self.symbol_table.resolve(name).map_or(false, |s| s.kind.is_inline()) {
            println!("LLVM Opt Hint: Fonksiyon '{}' için 'alwaysinline' niteliği eklendi.", name);
            // Gerçek LLVM'de nitelikler eklenecektir.
        }
        
        self.compile_statement(body);
        
        // LLVM'de her fonksiyonun bir 'ret' komutu ile bitmesi gerekir.
        self.emit("\tret void\n");
        self.emit("}\n\n");
    }
    
    fn compile_declaration(&mut self, initializer: Option<&Node>) {
        // Gerçekte her değişken için 'alloca' yapılmalı ve sembol tablosuna adres kaydedilmelidir.
        // Örn: @x = global i32 0 (Global değişken için)
        // Örn: %x = alloca i32 (Yerel değişken için)

        if let Some(init) = initializer {
            let (init_value_reg, init_value_ty) = self.compile_expression(init);
            // Değeri ayrılan belleğe kaydet (Örneğin, ilk bildirilen değişkene)
            // LLVM IR: store i32 10, i32* @degisken_adi
            // (Atama mantığı burada basitçe geçilmiştir.)
            self.emit(&format!("; Değişken başlatma: store {} {}, {}* @<degisken_adi>\n", 
                               init_value_ty, init_value_reg, init_value_ty));
        }
    }
}