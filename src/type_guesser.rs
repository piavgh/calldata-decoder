use crate::constants::Types;

#[derive(Clone)]
pub struct ParamTypes(Vec<Types>);
impl ParamTypes {
    pub fn new(t: Vec<Types>) -> Self {
        Self(t)
    }
}

impl std::fmt::Debug for ParamTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.0))
    }
}

#[derive(Clone)]
pub struct Params {
    pub selector: String,
    pub params: Vec<String>,
    pub types: Vec<ParamTypes>,
}

impl Params {
    pub fn new(selector: &str, params: Vec<String>) -> Self {
        Self {
            selector: selector.to_string(),
            params,
            types: vec![],
        }
    }
}

impl std::fmt::Debug for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\nMethod ID: {}\nInputs: {:#?}\nTypes: {:#?}",
            self.selector, self.params, self.types
        ))
    }
}
