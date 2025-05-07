pub mod c_language_parser;

pub trait Parser<'a> {
    fn parse(source_code: &'a String) -> anyhow::Result<Vec<AnalyzedCodeData<'a>>>;
}

#[derive(Debug)]
pub struct AnalyzedCodeData<'a> {
    storage_class: Option<StorageClassKinds>,
    type_kind: Option<TypeKinds<'a>>,
    pub identifier: Option<&'a str>,
    pub contents: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
enum StorageClassKinds {
    Auto,
    External,
}

#[derive(Debug, PartialEq)]
enum InvalidContentTypeKinds {
    Comment,
    MultiComment,
    Blank,
}

#[derive(Debug, Clone, PartialEq)]
enum TypeKinds<'a> {
    Enum,
    Function(&'a str),
    Struct,
    TypeDef,
    Union,
}

impl<'a> AnalyzedCodeData<'a> {
    fn new(
        storage_class: StorageClassKinds,
        type_kind: Option<TypeKinds<'a>>,
        identifier: &'a str,
        contents: Vec<&'a str>,
    ) -> Self {
        Self {
            storage_class: Some(storage_class),
            type_kind: type_kind,
            identifier: Some(identifier),
            contents: contents,
        }
    }

    fn create_from_invalid_code_line(contents: Vec<&'a str>) -> Self {
        Self {
            storage_class: None,
            type_kind: None,
            identifier: None,
            contents: contents,
        }
    }

    pub fn is_extern(&self) -> bool {
        let Some(storage_class) = &self.storage_class else {
            return false;
        };

        *storage_class == StorageClassKinds::External
    }

    pub fn is_struct_type(&self) -> bool {
        let Some(type_kind) = &self.type_kind else {
            return false;
        };

        match type_kind {
            TypeKinds::Struct => true,
            _ => false,
        }
    }

    pub fn is_enum_type(&self) -> bool {
        let Some(type_kind) = &self.type_kind else {
            return false;
        };

        match type_kind {
            TypeKinds::Enum => true,
            _ => false,
        }
    }

    pub fn is_function_type(&self) -> bool {
        let Some(type_kind) = &self.type_kind else {
            return false;
        };
        match type_kind {
            TypeKinds::Function(_) => true,
            _ => false,
        }
    }

    pub fn is_function_implementation(&self) -> bool {
        if self.is_extern() != false {
            return false;
        }

        let Some(type_kind) = &self.type_kind else {
            return false;
        };
        match type_kind {
            TypeKinds::Function("Implementation") => true,
            _ => false,
        }
    }

    pub fn is_invalid_code_line(&self) -> bool {
        self.storage_class.is_none() && self.type_kind.is_none() && self.identifier.is_none()
    }

    pub fn is_given_function_included(&self, function_name: &str) -> bool {
        if !&self.is_function_implementation() {
            return false;
        }

        self.contents.iter().any(|x| {
            // *mallocのような関数は定数を引数としたときに，コンパイルが死ぬためextern宣言されているものとは別物とする
            // if function_name.starts_with("*") {
            //     x.contains(function_name.get(1..).unwrap())
            // } else {
            x.contains(function_name)
            // }
        })
    }
}
