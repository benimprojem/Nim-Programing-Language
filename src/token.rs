// src/token.rs
// --- Token Tanımı (ENUM) ---
// src/token.rs
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // --- Sözcükler ve Anahtar Kelimeler (BÖLÜM 1 & 4 & 5 & 7 & 9 & 10) ---
    Identifier(String), // myVar, addNumbers
    KeywordVar,         // var
    KeywordConst,       // const
    KeywordFn,          // fn
    KeywordIf,          // if
    KeywordElse,        // else
    KeywordElseIf,      // elseif
    KeywordMatch,       // match
    KeywordDef,         // def (match default veya group default)
    KeywordRolling,     // rolling
    KeywordGroup,       // group
    KeywordWhile,       // while
    KeywordFor,         // for
    KeywordBreak,       // break
    KeywordContinue,    // continue
    KeywordReturn,      // return
    KeywordUse,         // use
    KeywordExport,      // export
    KeywordStruct,      // struct
    KeywordExtend,      // extend
    KeywordTypedef,     // typedef
    KeywordEnum,        // enum
    KeywordAsync,       // async
    KeywordAwait,       // await
    KeywordInline,      // inline
    KeywordAnd,         // and
    KeywordOr,          // or
    KeywordXor,         // xor

    // --- Değer Literalleri (EKSİK OLANLAR EKLENDİ) ---
    Int(i64),           // Tamsayı (Örn: 10, 500)
    Float(f64),         // Kayan nokta (Örn: 3.14, 1e-5)
    String(String),     // String (Örn: "merhaba")
    Null,               // null
	// E0599 Fix: Missing Keyword/Literal Tokens
    KeywordNull,        // null (for null literal)
    True,               // true (for boolean literal)
    False,              // false (for boolean literal)


    // --- Tipler (BÖLÜM 2) ---
    TypeI32, TypeF32, TypeStr, TypeArr, // vs. Diğer tüm temel tipler (i8,...
	
	// E0599 Fix: Missing Type Tokens
    TypeVoid,           // void
    TypeBool,           // bool
    TypeChar,           // char
    TypeBit,            // bit
    TypeByte,           // byte
    TypePtr,            // ptr
    TypeRef,            // ref
    TypeAny,            // any
	
	
    // --- Operatörler ve Sınırlayıcılar (EKSİK OLANLAR EKLENDİ/DÜZELTİLDİ) ---
    Assign,             // = (Tek eşittir, atama)
    Equal,              // == (Çift eşittir, karşılaştırma) (Equals yerine Equal kullanıldı)
    NotEqual,           // != (NotEquals yerine NotEqual kullanıldı)
    LessThan,           // <
    GreaterThan,        // >
    LessEqual,          // <=
    GreaterEqual,       // >=
    Plus,               // +
    Minus,              // -
    Star,               // * (Çarpma, Eklendi)
    Slash,              // /
    Modulo,             // %
    
    PlusAssign,         // += (Eklendi)
    MinusAssign,        // -= (Eklendi)
    
    Ampersand,          // & (Referans/Adres alma)
    Bang,               // ! (Mantıksal DEĞİL)
    Tilde,              // ~ (Bitwise NOT)
    Caret,              // ^ (Bitwise XOR)
    VBar,               // | (Bitwise OR)
    
    // --- Sınırlayıcılar ve Diğerleri ---
    Comma,              // ,
    Colon,              // :
    Semicolon,          // ;
    Dot,                // .
    LBracket,           // [
    RBracket,           // ]
    LParen,             // (
    RParen,             // )
    LBrace,             // {
    RBrace,             // }
    Arrow,              // -> (Fonksiyon dönüş tipi)
    Dollar,             // $ (Özel Nimble sözdizimi, örn. $rolling)

    // --- Meta Tokenlar ---
    EOF,                // Dosya sonu
    Illegal,            // Tanınmayan karakter
}

// --- Yardımcı Fonksiyon: lookup_ident ---
// Girdi: Tanımlayıcı (String), Çıktı: İlgili Token
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        // Anahtar Kelimeler
        "fn" => Token::KeywordFn,
        "var" => Token::KeywordVar,
        "const" => Token::KeywordConst,
        "if" => Token::KeywordIf,
        "else" => Token::KeywordElse,
        "elseif" => Token::KeywordElseIf,
        "return" => Token::KeywordReturn,
        "while" => Token::KeywordWhile,
        "for" => Token::KeywordFor,
        "group" => Token::KeywordGroup,
        "struct" => Token::KeywordStruct,
        "use" => Token::KeywordUse,
        "export" => Token::KeywordExport,
        "inline" => Token::KeywordInline,
        "def" => Token::KeywordDef,
        "rolling" => Token::KeywordRolling,
        "match" => Token::KeywordMatch,
        "extend" => Token::KeywordExtend,
        "typedef" => Token::KeywordTypedef,
        "enum" => Token::KeywordEnum,
        "async" => Token::KeywordAsync,
        "await" => Token::KeywordAwait,
        "and" => Token::KeywordAnd,
        "or" => Token::KeywordOr,
        "xor" => Token::KeywordXor,
        
        // Literal/Tip Anahtar Kelimeler
        "true" => Token::True,
        "false" => Token::False,
        "null" => Token::KeywordNull,
        
        // Tipler
        "i32" => Token::TypeI32,
        "f32" => Token::TypeF32,
        "str" => Token::TypeStr,
        "arr" => Token::TypeArr,
        "void" => Token::TypeVoid,
        "bool" => Token::TypeBool,
        "char" => Token::TypeChar,
        "bit" => Token::TypeBit,
        "byte" => Token::TypeByte,
        "ptr" => Token::TypePtr,
        "ref" => Token::TypeRef,
        "any" => Token::TypeAny,

        // Tanımlayıcı
        _ => Token::Identifier(ident.to_string()),
    }
}


// --- Lexer::next_token() Örnek Parçaları (Çift/Üç Karakterli Operatörler) ---
/*
* Bu mantığın Lexer'ın ana döngüsüne entegre edilmesi gerekmektedir.
* (Bu kısım, tam bir Lexer kodu olmadığı için yorum olarak bırakılmıştır.)

match self.ch {
    '=' => {
        // ... (Kodunuzdaki '=' ile ilgili mantık)
    }
    '+' => {
        // ... (Kodunuzdaki '+' ile ilgili mantık)
    }
    '!' => {
        // ... (Kodunuzdaki '!' ile ilgili mantık)
    }
    '.' => {
        // ... (Kodunuzdaki '.' ile ilgili mantık)
    }
    '$' => {
        // ... (Kodunuzdaki '$' ile ilgili mantık)
    }
    _ => { /* ... */ }
}
*/


// --- Lexer::read_number() Örnek Parçası ---
/*
// Lexer yapısının bir metodu olarak düşünün.
fn read_number(&mut self) -> Token {
    let start_pos = self.position;
    let mut is_float = false;

    // Tamsayı kısmını oku
    while self.ch.is_digit(10) { self.read_char(); }

    // Ondalık kısmı kontrol et
    if self.ch == '.' {
        is_float = true;
        self.read_char();
        while self.ch.is_digit(10) { self.read_char(); }
    }

    // Bilimsel gösterim (e/E) kontrolü (BÖLÜM 2.3)
    if self.ch == 'e' || self.ch == 'E' {
        is_float = true;
        self.read_char();
        if self.ch == '+' || self.ch == '-' { self.read_char(); } // İşareti oku
        while self.ch.is_digit(10) { self.read_char(); } // Üs değerini oku
    }

    let literal_str = &self.input[start_pos..self.position];
    
    // Tamsayı veya Kayan Nokta olarak ayrıştır
    if is_float {
        match literal_str.parse::<f64>() {
            Ok(val) => Token::LiteralFloat(val),
            Err(_) => Token::Illegal,
        }
    } else {
        match literal_str.parse::<i64>() {
            Ok(val) => Token::LiteralInteger(val),
            Err(_) => Token::Illegal,
        }
    }
}
*/