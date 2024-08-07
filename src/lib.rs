use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

pub struct BmbpContextVars {
    data: RwLock<HashMap<String, String>>,
}

impl BmbpContextVars {
    fn new() -> Self {
        BmbpContextVars {
            data: RwLock::new(HashMap::new()),
        }
    }
    ///
    ///
    pub fn set_value<T>(&self, key: String, value: T)
    where
        T: ToString,
    {
        let value_string = value.to_string();
        self.data.write().unwrap().insert(key, value_string);
    }
    pub fn get_value<T>(&self, key: T) -> String
    where
        T: ToString,
    {
        if let Some(v) = self.data.read().unwrap().get(&key.to_string()) {
            v.to_string()
        } else {
            "".to_string()
        }
    }
}

pub enum ContexVars {
    AppCode,
    AppName,
    AppNavName,
    AppLoginName,
    AppCopyRight,
    AppIconPath,
}
impl ToString for ContexVars {
    fn to_string(&self) -> String {
        match self {
            ContexVars::AppCode => "AppCode".to_string(),
            ContexVars::AppName => "AppName".to_string(),
            ContexVars::AppNavName => "AppNavName".to_string(),
            ContexVars::AppLoginName => "AppLoginName".to_string(),
            ContexVars::AppCopyRight => "AppCopyRight".to_string(),
            ContexVars::AppIconPath => "AppIconPath".to_string(),
        }
    }
}

/// BMBP_CONTEXT_VARS
///
/// # Code
/// ```rust
///  (&*BMBP_CONTEXT_VARS).set_value("A".to_string(), "B".to_string());
/// assert_eq!(
///    &*BMBP_CONTEXT_VARS.get_value("A".to_string()),
///    "B".to_string()
///);
///assert_eq!(
///    &*BMBP_CONTEXT_VARS.get_value("D".to_string()),
///    "".to_string()
///);
/// ```
pub static BMBP_CONTEXT_VARS: LazyLock<BmbpContextVars> = LazyLock::new(|| BmbpContextVars::new());

#[cfg(test)]
mod tests {
    use crate::BMBP_CONTEXT_VARS;
    #[test]
    pub fn test_vars_mut() {
        (&*BMBP_CONTEXT_VARS).set_value("A".to_string(), "B".to_string());
        assert_eq!(
            &*BMBP_CONTEXT_VARS.get_value("A".to_string()),
            "B".to_string()
        );
        assert_eq!(
            &*BMBP_CONTEXT_VARS.get_value("D".to_string()),
            "".to_string()
        );
    }
}
