use std::{fmt::Display, ops::Deref};

pub enum Scope {
    UserReadChat,
    UserWriteChat,
    Other(&'static str),
}
impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
            Scope::UserReadChat => "user:read:chat",
            Scope::UserWriteChat => "user:write:chat",
            Scope::Other(scope) => scope,
        }
    }
}

impl From<&Scope> for String {
    fn from(value: &Scope) -> Self {
        value.as_str().to_owned()
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.into();
        write!(f, "{string}")
    }
}

pub struct Scopes<'a> {
    values: &'a [Scope],
}

impl<'a> Scopes<'a> {
    pub fn new(values: &'a [Scope]) -> Scopes<'a> {
        Self { values }
    }
}

impl Deref for Scopes<'_> {
    type Target = [Scope];
    fn deref(&self) -> &Self::Target {
        self.values
    }
}

impl From<&Scopes<'_>> for String {
    fn from(value: &Scopes) -> Self {
        let values = &value.values;
        let first = match values.first() {
            Some(f) => f.to_string(),
            None => return "".to_owned(),
        };
        values
            .iter()
            .skip(1)
            .fold(first, |acc, e| acc + "+" + e.as_str())
    }
}

impl Display for Scopes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.into();
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod tests {
    use super::Scope;
    use crate::models::scope::Scopes;

    #[test]
    pub fn test_scope() {
        let scopes: Vec<Scope> = vec![];
        let scopes = Scopes::new(&scopes);
        let scopes: String = scopes.to_string();
        assert_eq!(scopes, "".to_owned());

        let scopes = vec![Scope::UserReadChat];
        let scopes = Scopes::new(&scopes);
        let scopes: String = scopes.to_string();
        assert_eq!(scopes, "user:read:chat".to_owned());

        let scopes = vec![Scope::UserReadChat, Scope::UserWriteChat];
        let scopes = Scopes::new(&scopes);
        let scopes: String = scopes.to_string();
        assert_eq!(scopes, "user:read:chat+user:write:chat".to_owned())
    }
}
