#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResourceId(pub u16);

#[derive(Clone, Debug)]
pub struct ResourceDef {
    id: u16,
    name: Option<String>,
}

impl ResourceDef {
    pub fn new () -> Self {
        Self {
            id: 0,
            name: None,
        }
    }
}

impl ResourceDef {
    #[inline]
    pub fn is_match(&self, n: &str) -> bool {
        match &self.name {
            Some(name) => name == n,
            None => false,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        let name = name.into();

        assert!(!name.is_empty(), "resource name should not be empty");

        self.name = Some(name)
    }
}
