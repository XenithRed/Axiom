use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    pub fn type_id(&self) -> u8 {
        match self {
            Self::End        => 0,
            Self::Byte(_)    => 1,
            Self::Short(_)   => 2,
            Self::Int(_)     => 3,
            Self::Long(_)    => 4,
            Self::Float(_)   => 5,
            Self::Double(_)  => 6,
            Self::ByteArray(_) => 7,
            Self::String(_)  => 8,
            Self::List(_)    => 9,
            Self::Compound(_) => 10,
            Self::IntArray(_) => 11,
            Self::LongArray(_) => 12,
        }
    }

    pub fn as_str(&self)      -> Option<&str>                    { if let Self::String(s) = self  { Some(s) } else { None } }
    pub fn as_i32(&self)      -> Option<i32>                     { if let Self::Int(n)    = self  { Some(*n) } else { None } }
    pub fn as_i64(&self)      -> Option<i64>                     { if let Self::Long(n)   = self  { Some(*n) } else { None } }
    pub fn as_f32(&self)      -> Option<f32>                     { if let Self::Float(n)  = self  { Some(*n) } else { None } }
    pub fn as_f64(&self)      -> Option<f64>                     { if let Self::Double(n) = self  { Some(*n) } else { None } }
    pub fn as_compound(&self) -> Option<&HashMap<String, Tag>>   { if let Self::Compound(m) = self { Some(m) } else { None } }
    pub fn as_list(&self)     -> Option<&Vec<Tag>>               { if let Self::List(v)   = self  { Some(v) } else { None } }

    pub fn get(&self, k: &str) -> Option<&Tag> {
        self.as_compound()?.get(k)
    }

    pub fn insert(&mut self, k: impl Into<String>, v: Tag) {
        if let Self::Compound(m) = self { m.insert(k.into(), v); }
    }

    pub fn compound() -> Self { Self::Compound(HashMap::new()) }
}
