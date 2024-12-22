use crate::service::ServiceRequest;


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

pub struct TaskRouter<T> {
    routes: Vec<(ResourceDef, T)>,
}

impl<T> TaskRouter<T> {
    pub fn build() -> RouterBuilder<T> {
        RouterBuilder { routes: Vec::new() }
    }
    pub fn recognize(&self, req: &ServiceRequest) -> Option<&T> {
        let ctx: &crate::context::TaskContext = req.ctx();
        let name = ctx.name();
        self.routes.iter().find_map(|(resource_def, service)| {
            if resource_def.is_match(name) {
                Some(service)
            } else {
                None
            }
        })
    }
}

pub struct RouterBuilder<T> {
    routes: Vec<(ResourceDef, T)>,
}

impl<T> RouterBuilder<T> {
    pub fn push(
        &mut self,
        rdef: ResourceDef,
        val: T,
    ) -> (&mut ResourceDef, &mut T) {
        self.routes.push((rdef, val));
        #[allow(clippy::map_identity)] // map is used to distribute &mut-ness to tuple elements
        self.routes
            .last_mut()
            .map(|(rdef, val)| (rdef, val))
            .unwrap()
    }

    pub fn finish(self) -> TaskRouter<T> {
        TaskRouter {
            routes: self.routes,
        }
    }
}
