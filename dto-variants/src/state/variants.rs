use syn::{Attribute, Ident, Type, Visibility};

// pub struct StateVariant {
//     pub vis: Visibility,
//     pub mode: VariantMode,
//     pub name: Ident,
// }

#[derive(Debug)]
pub enum StateVariant {
    Include(StateVariantInclude),
    Exclude(StateVariantExclude),
}

#[derive(Debug)]
pub struct StateVariantExclude {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub fields_to_exclude: Vec<Ident>,
}

#[derive(Debug)]
pub struct StateVariantInclude {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub fields_to_include: Vec<StateVariantIncludeField>,
}

#[derive(Debug)]
pub struct StateVariantIncludeField {
    pub overwrite_attrs: bool,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
}
