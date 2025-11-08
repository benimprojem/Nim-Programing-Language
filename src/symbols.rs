// src/symbols.rs

use crate::ast::Type;
use std::collections::HashMap;

// --- Sembol Tipi Tanımlaması ---
// Derleyicinin Sembol Tablosunda tuttuğu öğelerin özellikleri
#[derive(Debug, Clone)]
pub enum SymbolKind {
    // Değişken (const mu? taşındı mı? referans mı?)
    Variable { is_const: bool, is_moved: bool, is_ref: bool }, 
    Function { 
        params: Vec<Type>, 
        return_type: Type, 
        is_inline: bool 
    },
    // Grup (Örn: match { def: ... }) veya özel yapı
    Group { 
        members: HashMap<String, Type>, 
        constructor_params: Vec<Type> 
    },
    Struct { 
        members: HashMap<String, Type> 
    },
    TypeAlias(Type), // Typedef
    // LabeledBlock veya fast_exec etiketi
    Label,           
}

// Bir sembolün tüm detayları
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_info: Type, // Sembolün nihai tipi
    pub scope_id: usize, // Hangi kapsama ait olduğu
    pub definition_node_idx: usize, // AST'deki düğümün indeksi
}

// --- Kapsam Yönetimi ---
// Programdaki her kod bloğu (fonksiyon, if, for, { }) bir kapsama sahiptir.
#[derive(Debug)]
pub struct Scope {
    pub id: usize,
    pub parent_id: Option<usize>,
    pub symbols: HashMap<String, Symbol>, // Bu kapsamdaki tanımlayıcılar
    pub scope_type: String, // "global", "function", "block", "group"
}

// Semantik Analizcinin ana yapısı
#[derive(Debug)]
pub struct SymbolTable {
    pub scopes: HashMap<usize, Scope>,
    pub current_scope_id: usize,
    next_scope_id: usize,
}

impl SymbolTable {
    // Sembol tablosunu global kapsam ile başlatır.
    pub fn new() -> Self {
        let global_scope = Scope {
            id: 0,
            parent_id: None,
            symbols: HashMap::new(),
            scope_type: "global".to_string(),
        };
        let mut scopes = HashMap::new();
        scopes.insert(0, global_scope);

        SymbolTable {
            scopes,
            current_scope_id: 0,
            next_scope_id: 1,
        }
    }

    // Yeni bir kapsam oluşturur ve geçerli kapsamı ona ayarlar.
    pub fn enter_scope(&mut self, scope_type: &str) {
        let new_id = self.next_scope_id;
        let new_scope = Scope {
            id: new_id,
            parent_id: Some(self.current_scope_id),
            symbols: HashMap::new(),
            scope_type: scope_type.to_string(),
        };
        self.scopes.insert(new_id, new_scope);
        self.current_scope_id = new_id;
        self.next_scope_id += 1;
    }

    // Geçerli kapsamdan çıkar ve ebeveyne döner.
    // Global kapsamdan çıkış hatasını uygun bir mesajla ele alır.
    pub fn exit_scope(&mut self) -> Result<(), String> {
        let current_scope_id = self.current_scope_id;
        
        let parent_id = self.scopes.get(&current_scope_id)
            .ok_or_else(|| "Hata: Geçerli kapsam bulunamadı.".to_string())?
            .parent_id;

        match parent_id {
            Some(id) => {
                self.current_scope_id = id;
                // Kapsamdan çıkıldığında, bu kapsamı `scopes` map'inden silmek (veya silmemek) 
                // tasarım kararıdır. Semantik analiz sonrası gereksizse silinebilir. 
                // Şimdilik sadece `current_scope_id`'yi güncelliyoruz.
                Ok(())
            },
            None => Err("Hata: Global (Kapsam ID 0) kapsamdan çıkılamaz.".to_string()),
        }
    }

    // Geçerli ve üst kapsamlarda sembol arar (İsim Çözümleme).
    // Bulunan sembolün bir klonunu döndürür.
    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        let mut current_id = Some(self.current_scope_id);
        
        // Kapsam zincirinde yukarı doğru dolaş
        while let Some(id) = current_id {
            if let Some(scope) = self.scopes.get(&id) {
                if let Some(symbol) = scope.symbols.get(name) {
                    return Some(symbol.clone()); // Sembol bulundu
                }
                current_id = scope.parent_id; // Üst kapsama git
            } else {
                break; // Kapsam ID geçersizse döngüyü kır
            }
        }
        None // Sembol bulunamadı
    }

    // YALNIZCA geçerli kapsama sembol ekler.
    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        let name = symbol.name.clone();
        if let Some(scope) = self.scopes.get_mut(&self.current_scope_id) {
            if scope.symbols.contains_key(&name) {
                // Aynı kapsamda yeniden tanımlama hatası
                return Err(format!("Hata: '{}' bu kapsamda zaten tanımlanmış.", name));
            }
            scope.symbols.insert(name, symbol);
            Ok(())
        } else {
            // Bu normalde olmaması gereken bir durumdur (`new` ve `enter_scope` doğru çalışıyorsa)
            Err("Hata: Geçerli kapsam bulunamadı.".to_string())
        }
    }
}