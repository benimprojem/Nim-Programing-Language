// src/ast.rs (TEMİZLENMİŞ VE BİRLEŞTİRİLMİŞ AST)
use crate::token::Token;

// Tüm ifadelerin/deyimlerin döndürdüğü temel tip (AST Ağacının Yapı Taşı)
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // Programın en üst seviye konteyneri
    Program(Vec<Node>),

    // --- Deyimler (Statements - Değer döndürmez veya kapsam değiştirir) ---
    
    // 1. Değişken Tanımlama
    Declaration {
        is_const: bool,          // var/const
        names: Vec<String>,      // x, y, z (Çoklu tanım için)
        type_hint: Option<Type>, // :i32 (Opsiyonel Tip Notu)
        initializer: Option<Box<Node>>, // = value (Opsiyonel Atama)
    },

    // 2. Atama
    Assignment {
        target: Box<Node>,       // x, arr[i], struct.member
        operator: Token,         // =, +=, -=, vb.
        value: Box<Node>,
    },

    // 3. Kontrol Akışı
    IfStatement {
        condition: Box<Node>,
        consequence: Box<Node>,  // {} içindeki blok
        alternative: Option<Box<Node>>, // else if veya else
    },
    
    // 4. Modül/Kapsam Yönetimi
    UseStatement {
        module_name: String,
        imported_items: Vec<String>, // Seçici içe aktarılan öğeler
    },
    ExportStatement(Box<Node>), // export const PI = 3.14;

    // 5. Fonksiyon ve Grup Tanımı
    FunctionDeclaration {
        name: String,
        params: Vec<Parameter>,
        return_type: Type,
        is_inline: bool,        // inline fn
        body: Box<Node>,
    },
    GroupDeclaration {
        name: String,
        params: Vec<Parameter>, // Kurucu parametreleri
        body: Vec<Node>,        // Group içindeki etiketler (fonksiyonlar/bloklar)
    },
    
    // 6. Döngü ve Akış Kontrolü
    WhileLoop {
        condition: Box<Node>,
        body: Box<Node>,
    },
    ForLoop { // for (i, i =< 5, i++)
        initial: Option<Box<Node>>,
        condition: Option<Box<Node>>,
        update: Option<Box<Node>>,
        body: Box<Node>,
    },
    ForInLoop { // Diziler için for (myArr as val)
        collection: Box<Node>,
        iterator_name: String,
        body: Box<Node>,
    },
    ControlFlow { // break, continue, return
        keyword: Token,
        value: Option<Box<Node>>, // return değeri
    },
    RollingStatement { // rolling:ETİKET
        label: String,
    },
    LabeledBlock { // REQUEST_BLOCK: { ... }
        label: String,
        body: Box<Node>,
        is_fast_exec: bool, // fast_exec mi yoksa normal etiket mi?
    },
    
    // 7. Blok ve Kapsam
    BlockStatement(Vec<Node>), // {} içindeki kod bloğu

    // 8. Yapı ve Tip Tanımları
    StructDeclaration {
        name: String,
        members: Vec<Box<Node>>, // İçinde Declaration düğümleri tutar
    },
    ExtendBlock { // struct'a metot ekleme
        target_type: Type,
        methods: Vec<Node>, // FunctionDeclaration düğümleri
    },
    TypedefDeclaration { // Tip takma adı
        new_name: String,
        base_type: Type,
    },
    EnumDeclaration {
        name: String,
        members: Vec<(String, Option<i64>)>,
    },
    
    // --- İfadeler (Expressions - Değer döndürür) ---
    
    // 9. Temel İfadeler
    Literal(Token),              // 10, "metin", true, null
    Identifier(String),          // Değişken kullanımı
    // Tekli ve İkili İfadeler
    UnaryExpression {
        operator: Token,
        right: Box<Node>,
    },
    BinaryExpression {
        operator: Token,
        left: Box<Node>,
        right: Box<Node>,
    },

    // Fonksiyon Çağrısı ve Üye Erişimi
    CallExpression {
        function: Box<Node>, // Identifier veya MemberAccess
        arguments: Vec<Argument>,
    },
    MemberAccess {
        object: Box<Node>, // Struct, Group, veya Array
        member: Box<Node>, // Identifier (isimli üye) veya IndexExpression (dizin)
    },
    IndexExpression {
        left: Box<Node>, // Array veya Group
        index: Box<Node>,
    },
    ArrayLiteral(Vec<Node>),     // [1, 2, 3]
    TupleLiteral(Vec<Node>),     // (10, "ok")
    TypeCast(Type, Box<Node>),   // i32(value)
    TernaryExpression {          // a < 10 ? "Hello" : "Good Bye"
        condition: Box<Node>,
        if_true: Box<Node>,
        if_false: Box<Node>,
    },
	ReturnStatement(Option<Box<Node>>), // return expr;
    BreakStatement,
    ContinueStatement,

   
    
    // --- Hata İşleme ---
    NoOp, // Boş bir düğüm
    
  
    
    // Eklenmesi gereken diğer ifadeler
    LambdaExpression { // fn (x: i32) -> i32 { x * 2 }
        params: Vec<Parameter>,
        return_type: Type,
        body: Box<Node>,
    },
    MatchExpression { // match x { 1 -> "bir", _ -> "diğer" }
        target: Box<Node>,
        arms: Vec<(Node, Node)>, // (Pattern/Koşul, İfade/Blok)
    },

}

// Fonksiyon Parametrelerini temsil eder
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
	pub name: String,
	pub type_hint: Type,
	pub default_value: Option<Node>, // Opsiyonel argümanlar için
}

// İsimli ve Pozisyonel argümanları temsil eder
#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
	pub name: Option<String>, // İsimli argüman ise Some("color"), pozisyonel ise None
	pub value: Node,
}

// Basitleştirilmiş Tip Temsili (Semantik Analizde detaylanacak)
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	Primitive(String),       // "i32", "str", "void"
	Array(Box<Type>, Option<Box<Node>>), // [] veya arr veya f32[4]
	Pointer(Box<Type>),      // ptr
	Reference(Box<Type>),    // ref
	Tuple(Vec<Type>),        // (i32, str)
	Variadic(Box<Type>),     // i32... (Homojen değişken parametre)
	Any,                     // any
	Identifier(String),      // Kullanıcı tanımlı struct veya typedef
	StringEncoding(String, Option<usize>), // str[utf16] veya str[30]
	Unknown,                 // Tip çıkarımı bekleniyor
}