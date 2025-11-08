// src/semantics.rs

use crate::ast::{Node, Type, Parameter, Argument}; // Parameter ve Argument eklenmiştir
use crate::token::Token;
use crate::symbols::{SymbolTable, Symbol, SymbolKind};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
}

pub struct Analyzer {
    pub symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze_program(&mut self, program: &mut Node) -> Result<(), Vec<SemanticError>> {
        if let Node::Program(statements) = program {
            // Programdaki tüm deyimleri gez
            for stmt in statements.iter_mut() {
                self.analyze_statement(stmt);
            }
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(SemanticError { message });
    }

    // --- AST Dolaşım Fonksiyonları ---
    fn analyze_statement(&mut self, stmt: &mut Node) {
        match stmt {
            Node::Declaration { is_const, names, type_hint, initializer } => {
                self.analyze_declaration(*is_const, names, type_hint, initializer.as_mut());
            }
            Node::Assignment { target, operator, value } => {
                self.analyze_assignment(target.as_mut(), operator, value.as_mut());
            }
            Node::FunctionDeclaration { name, params, return_type, is_inline, body: _ } => {
                // Fonksiyon imzasını sembol tablosuna kaydet
                let param_types = params.iter().map(|p| p.type_hint.clone()).collect();
                
                self.symbol_table.define(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Function { 
                        params: param_types, 
                        return_type: return_type.clone(), 
                        is_inline: *is_inline 
                    },
                    type_info: Type::Primitive("fn".to_string()),
                    scope_id: self.symbol_table.current_scope_id,
                    definition_node_idx: 0,
                }).ok();
                // Fonksiyon gövdesi analizi burada başlatılmalıdır (scope giriş/çıkış dahil).
            }
            Node::StructDeclaration { name, members } => {
                self.analyze_struct_declaration(name, members);
            }
            Node::GroupDeclaration { name, params, body } => {
                self.analyze_group_declaration(name, params, body);
            }
            Node::BlockStatement(statements) => {
                self.symbol_table.enter_scope("block");
                statements.iter_mut().for_each(|s| self.analyze_statement(s));
                // SymbolTable'ın exit_scope fonksiyonu Result döndürdüğü varsayılmıştır.
                if let Err(e) = self.symbol_table.exit_scope() { 
                    self.add_error(e); 
                }
            }
            Node::Identifier(name) => { self.check_identifier_usage(name); }
            Node::CallExpression { function, arguments } => { self.analyze_call_expression(function.as_mut(), arguments); }
            Node::MemberAccess { object, member } => { self.analyze_member_access(object.as_mut(), member.as_mut()); }
            // ... Diğer tüm deyim türleri ...
            _ => {}
        }
    }
    
    fn analyze_expression(&mut self, exp: &mut Node) -> Option<Type> {
        match exp {
            Node::Identifier(name) => self.check_identifier_usage(name),
            Node::Literal(token) => self.check_literal_type(token),
            Node::UnaryExpression { operator, right } => self.analyze_unary_expression(operator, right.as_mut()),
            Node::BinaryExpression { left, operator, right } => self.analyze_binary_expression(left.as_mut(), operator, right.as_mut()),
            Node::CallExpression { function, arguments } => self.analyze_call_expression(function.as_mut(), arguments),
            Node::MemberAccess { object, member } => self.analyze_member_access(object.as_mut(), member.as_mut()),
            // ... Diğer tüm ifadeler
            _ => Some(Type::Unknown),
        }
    }
    
    fn check_literal_type(&self, token: &Token) -> Option<Type> {
        match token {
            Token::Int(_) => Some(Type::Primitive("i32".to_string())),
            Token::Float(_) => Some(Type::Primitive("f32".to_string())),
            Token::String(_) => Some(Type::Primitive("str".to_string())),
            Token::True | Token::False => Some(Type::Primitive("bool".to_string())),
            Token::KeywordNull => Some(Type::Primitive("null".to_string())),
            _ => None,
        }
    }

    // --- Değişken Tanımlama (analyze_declaration) ---
    fn analyze_declaration(&mut self, is_const: bool, names: &Vec<String>, type_hint: &Option<Type>, initializer: Option<&mut Node>) {
        
        let mut inferred_type = Type::Unknown;

        // 1. Başlatıcı İfadeyi Analiz Et ve Tipini Çıkar
        if let Some(init) = initializer {
            if let Some(init_type) = self.analyze_expression(init) {
                inferred_type = init_type;
            }
        }

        // 2. Nihai Tipi Belirle (Tip Notu vs. Çıkarılan Tip)
        let final_type = if let Some(ref hint) = type_hint {
            // Tip Notu varsa, çıkan tiple eşleşmeli (Tip Kontrolü)
            if inferred_type != Type::Unknown && *hint != inferred_type {
                self.add_error(format!("Hata: Tip uyuşmazlığı. '{:?}' tipi, başlatıcı tip olan '{:?}' ile eşleşmiyor.", hint, inferred_type));
            }
            hint.clone()
        } else if inferred_type != Type::Unknown {
            inferred_type
        } else {
            // Ne tip notu ne de başlatıcı varsa 
            self.add_error(format!("Hata: Değişken tanımlaması için tip notu veya başlatıcı gereklidir."));
            Type::Unknown
        };
        
        if final_type == Type::Unknown { return; }

        // 3. Sembol Tablosuna Ekleme
        for name in names {
            let symbol = Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable { 
                    is_const, 
                    is_moved: false, // Başlangıçta taşınmadı
                    is_ref: false,   // Başlangıçta referans değil
                },
                type_info: final_type.clone(),
                scope_id: self.symbol_table.current_scope_id,
                definition_node_idx: 0,
            };

            if let Err(e) = self.symbol_table.define(symbol) {
                self.add_error(e);
            }
        }
    }
    
    // --- Atama İşlemi (analyze_assignment) ---
    fn analyze_assignment(&mut self, target: &mut Node, operator: &Token, value: &mut Node) {
        
        // 1. Hedef ve Değerin Tiplerini Analiz Et
        let target_type = self.analyze_expression(target);
        let value_type = self.analyze_expression(value);
        
        // 2. Tip Kontrolü
        if let (Some(t_type), Some(v_type)) = (&target_type, &value_type) {
            if *t_type != *v_type {
                self.add_error(format!("Hata: Tip uyuşmazlığı. '{:?}' tipine '{:?}' atanamaz.", t_type, v_type));
            }
        }
        
        // 3. Değişkenlik (const) Kontrolü
        if let Node::Identifier(ref name) = target {
            if let Some(symbol) = self.symbol_table.resolve(name) { 
                if let SymbolKind::Variable { is_const, .. } = &symbol.kind {
                    if *is_const {
                        self.add_error(format!("Hata: Sabit değişken '{}' yeniden atanamaz.", name));
                    }
                }
                
                // 4. Sahibiyet Taşıma (Move) Kuralı
                // Atama basit atama (=) ise ve move-only bir tip atanıyorsa
                if operator == &Token::Assign && self.is_move_only(&symbol.type_info) {
                    if let Node::Identifier(ref _src_name) = value {
                        // *ÖNEMLİ NOT: Burada kaynak değişkenin (value) is_moved bayrağının 
                        // Sembol Tablosunda True olarak ayarlanması gerekir.
                        // `SymbolTable::resolve` klon döndürdüğü için, bu işlem için SymbolTable API'sinin
                        // (örneğin `update_symbol_move_status` gibi) genişletilmesi gerekmektedir.
                    }
                }
            }
        }
    }

    // --- Sahibiyet Sistemi (Kullanım Kontrolü) ---
    fn check_identifier_usage(&mut self, name: &str) -> Option<Type> {
        // 1. İsim Çözümleme
        if let Some(symbol) = self.symbol_table.resolve(name) {
            
            // 2. Sahibiyet Kontrolü
            if let SymbolKind::Variable { is_moved, .. } = &symbol.kind {
                if *is_moved {
                    // Hata: Taşıma işleminden sonra değişken kullanımı yasaktır.
                    self.add_error(format!("Hata: Sahibiyeti taşınmış değişken '{}' tekrar kullanılamaz.", name));
                    return None;
                }
            }

            // 3. Tip döndür
            Some(symbol.type_info.clone())
        } else {
            self.add_error(format!("Hata: Tanımlanmamış değişken: '{}'", name));
            None
        }
    }
    
    // --- Yapı Tanımlama Kaydı (analyze_struct_declaration) ---
    fn analyze_struct_declaration(&mut self, name: &str, members: &mut Vec<Node>) {
        
        let mut member_types = HashMap::new();
        
        for member_node in members.iter_mut() {
            if let Node::Declaration { is_const: _, names, type_hint, initializer } = member_node {
                
                if initializer.is_some() {
                     self.add_error(format!("Hata: Yapı üyesi '{}' tanımlanırken başlangıç değeri atanamaz.", names.join(", ")));
                }

                let member_type = type_hint.clone().unwrap_or(Type::Unknown);
                if member_type == Type::Unknown {
                    self.add_error(format!("Hata: Yapı üyesi '{}' için tip notu zorunludur.", names.join(", ")));
                    continue;
                }

                for member_name in names {
                    member_types.insert(member_name.clone(), member_type.clone());
                }
            } else {
                 self.add_error("Hata: Yapı içinde sadece 'Declaration' tipinde tanımlamalar bulunabilir.".to_string());
            }
        }

        let struct_symbol = Symbol {
            name: name.to_string(),
            kind: SymbolKind::Struct { members: member_types },
            type_info: Type::Identifier(name.to_string()),
            scope_id: self.symbol_table.current_scope_id,
            definition_node_idx: 0,
        };

        if let Err(e) = self.symbol_table.define(struct_symbol) {
            self.add_error(e);
        }
    }

    // --- Grup Tanımlama Kaydı (analyze_group_declaration) ---
    fn analyze_group_declaration(&mut self, name: &str, params: &Vec<Parameter>, body: &mut Vec<Node>) {
        
        let constructor_params: Vec<Type> = params.iter().map(|p| p.type_hint.clone()).collect();
        
        let group_symbol_base = Symbol {
            name: name.to_string(),
            kind: SymbolKind::Group { members: HashMap::new(), constructor_params: constructor_params.clone() },
            type_info: Type::Identifier(name.to_string()),
            scope_id: self.symbol_table.current_scope_id,
            definition_node_idx: 0,
        };
        
        if let Err(e) = self.symbol_table.define(group_symbol_base) {
            self.add_error(e);
            return;
        }
        
        self.symbol_table.enter_scope("group");
        
        for param in params {
            let param_symbol = Symbol {
                name: param.name.clone(),
                kind: SymbolKind::Variable { is_const: true, is_moved: false, is_ref: false },
                type_info: param.type_hint.clone(),
                scope_id: self.symbol_table.current_scope_id,
                definition_node_idx: 0,
            };
            self.symbol_table.define(param_symbol).ok(); 
        }
        
        let mut group_members = HashMap::new();
        
        for member in body.iter_mut() {
            match member {
                Node::FunctionDeclaration { name: method_name, .. } => {
                    group_members.insert(method_name.clone(), Type::Primitive("fn".to_string()));
                    self.analyze_statement(member);
                }
                Node::LabeledBlock { label, .. } => {
                    group_members.insert(label.clone(), Type::Primitive("label".to_string()));
                    self.analyze_statement(member);
                }
                Node::Declaration { names, type_hint, .. } => {
                    let member_type = type_hint.clone().unwrap_or(Type::Unknown);
                    for name in names {
                        group_members.insert(name.clone(), member_type.clone());
                    }
                    self.analyze_statement(member);
                }
                _ => { 
                    self.add_error(format!("Hata: Grup içinde sadece fonksiyon, etiket veya üye tanımlamaları bulunabilir."));
                }
            }
        }

        if let Err(e) = self.symbol_table.exit_scope() { 
            self.add_error(e); 
        }

        // Global Sembolü Üye bilgileriyle güncelle
        if let Some(scope) = self.symbol_table.scopes.get_mut(&0) {
            if let Some(global_symbol) = scope.symbols.get_mut(name) {
                if let SymbolKind::Group { members, .. } = &mut global_symbol.kind {
                    *members = group_members;
                }
            }
        }
    }
    
    // --- Üye Erişimi ve Metot Çağrısı Çözümlemesi (analyze_member_access) ---
    fn analyze_member_access(&mut self, object: &mut Node, member: &mut Node) -> Option<Type> {
        
        let object_type = self.analyze_expression(object)?;
        
        let type_name = match object_type {
            Type::Identifier(ref name) => name,
            _ => {
                self.add_error(format!("Hata: '.' operatörü sadece kullanıcı tanımlı tipler için kullanılabilir. Tip: {:?}", object_type));
                return None;
            }
        };

        let type_symbol = self.symbol_table.resolve(type_name)?;

        let member_name = if let Node::Identifier(name) = member { name } else {
            self.add_error("Hata: '.' operatöründen sonra üye ismi bekleniyor.".to_string());
            return None;
        };
        
        match &type_symbol.kind {
            SymbolKind::Struct { members } | SymbolKind::Group { members, .. } => {
                if let Some(member_type) = members.get(member_name) {
                    Some(member_type.clone())
                } else {
                    self.add_error(format!("Hata: '{}' yapısının/grubunun '{}' adında bir üyesi/metodu yok.", type_name, member_name));
                    None
                }
            }
            _ => {
                self.add_error(format!("Hata: '{}' tipinde üye erişimi yapılamaz.", type_name));
                None
            }
        }
    }

    // --- Fonksiyon Çağrısı Çözümlemesi (analyze_call_expression) ---
    fn analyze_call_expression(&mut self, function: &mut Node, arguments: &mut Vec<Argument>) -> Option<Type> {
        
        let _func_type = self.analyze_expression(function)?;
        
        let func_symbol = if let Node::Identifier(name) = function {
            self.symbol_table.resolve(name)
        } else {
            None
        };
        
        if let Some(symbol) = func_symbol {
            if let SymbolKind::Function { params, return_type, .. } = &symbol.kind {
                
                if params.len() < arguments.len() {
                    self.add_error(format!("Hata: '{}' fonksiyonu en fazla {} argüman alabilir, {} verildi.", symbol.name, params.len(), arguments.len()));
                }
                
                for (i, arg) in arguments.iter_mut().enumerate() {
                    let arg_type = self.analyze_expression(&mut arg.value)?;
                    
                    if i < params.len() && params[i] != arg_type {
                        self.add_error(format!("Hata: {} pozisyonundaki argüman tipi uyuşmazlığı. Beklenen: {:?}, Verilen: {:?}", i, params[i], arg_type));
                    }
                    
                    // Taşıma Kuralı
                    if self.is_move_only(&arg_type) {
                        if let Node::Identifier(ref _src_name) = arg.value {
                            // *Not: Argüman olarak geçen 'move-only' değişkenin is_moved bayrağı burada True yapılmalıdır.
                        }
                    }
                }
                
                return Some(return_type.clone());
            }
        }
        
        Some(Type::Unknown) 
    }


    // --- İfade Analizleri (Binary, Unary) ---
    fn analyze_binary_expression(&mut self, left: &mut Node, operator: &Token, right: &mut Node) -> Option<Type> {
        let left_type = self.analyze_expression(left)?;
        let right_type = self.analyze_expression(right)?;

        match operator {
            Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                if left_type == Type::Primitive("f32".to_string()) || right_type == Type::Primitive("f32".to_string()) {
                    Some(Type::Primitive("f32".to_string()))
                } else if left_type == Type::Primitive("i32".to_string()) && right_type == Type::Primitive("i32".to_string()) {
                    Some(Type::Primitive("i32".to_string()))
                } else {
                    self.add_error(format!("Hata: Uyumsuz aritmetik işlem: {:?} ve {:?}", left_type, right_type));
                    None
                }
            }
            Token::Equal | Token::NotEqual | Token::LessThan | Token::GreaterThan | 
            Token::LessEqual | Token::GreaterEqual => {
                if left_type == right_type { Some(Type::Primitive("bool".to_string())) }
                else { 
                    self.add_error(format!("Hata: Uyumsuz karşılaştırma: {:?} ve {:?}", left_type, right_type));
                    None
                }
            }
            Token::KeywordAnd | Token::KeywordOr | Token::KeywordXor => {
                if left_type == Type::Primitive("bool".to_string()) && right_type == Type::Primitive("bool".to_string()) {
                    Some(Type::Primitive("bool".to_string()))
                } else {
                    self.add_error(format!("Hata: Mantıksal operatörler sadece bool tipi ile çalışır."));
                    None
                }
            }
            _ => Some(Type::Unknown),
        }
    }

    fn analyze_unary_expression(&mut self, operator: &Token, right: &mut Node) -> Option<Type> {
        let right_type = self.analyze_expression(right)?;

        match operator {
            Token::Bang => { // ! (Mantıksal DEĞİL)
                if right_type == Type::Primitive("bool".to_string()) { Some(Type::Primitive("bool".to_string())) }
                else { 
                    self.add_error("Hata: '!' operatörü sadece bool tipi ile kullanılır.".to_string());
                    None 
                }
            }
            Token::Minus => { // - (Negatifleme)
                if right_type == Type::Primitive("i32".to_string()) || right_type == Type::Primitive("f32".to_string()) { Some(right_type) }
                else {
                    self.add_error("Hata: '-' operatörü sayısal tiplerle kullanılır.".to_string());
                    None
                }
            }
            Token::Ampersand => { // & (Referans Alma)
                Some(Type::Reference(Box::new(right_type)))
            }
            Token::Star => { // * (Dereference - İşaretçi Çözme)
                if let Type::Pointer(inner_type) = right_type {
                    Some(*inner_type)
                } else {
                    self.add_error("Hata: '*' operatörü sadece işaretçi tipleri için kullanılır.".to_string());
                    None
                }
            }
            _ => Some(Type::Unknown),
        }
    }
    
    // --- Yardımcı Fonksiyonlar ---
    fn is_move_only(&self, type_info: &Type) -> bool {
        match type_info {
            Type::Pointer(_) | Type::Identifier(_) => true, 
            _ => false,
        }
    }
}