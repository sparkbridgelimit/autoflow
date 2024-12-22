use js_sys::{Function, JsString};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

// 引用你原本的 worker 逻辑
use worker::task::Task;
use worker::worker::{Worker as CoreWorker};
use worker::worker_service::WorkerEntry;

/// 模拟 `web::resource("xxx")`
#[wasm_bindgen]
pub struct Resource {
    name: String,
    callback: Option<Function>,
}

#[wasm_bindgen]
impl Resource {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Resource {
        Resource {
            name,
            callback: None,
        }
    }

    /// 类似 `web::resource("xxx").to(handler)`
    /// - `handler`: JS 回调
    #[wasm_bindgen]
    pub fn to(&mut self, handler: Function) -> Resource {
        self.callback = Some(handler);
        Resource {
            name: self.name.clone(),
            callback: self.callback.clone(),
        }
    }
}

/// Node.js 侧的“App”，内部保留真正的 Worker
#[wasm_bindgen]
pub struct NodeApp {
    inner: Arc<Mutex<CoreWorker<WorkerEntry>>>,
    routes: Arc<Mutex<HashMap<String, Function>>>,
}

#[wasm_bindgen]
impl NodeApp {
    /// 等同于 `Worker::new()`
    #[wasm_bindgen(constructor)]
    pub fn new() -> NodeApp {
        let worker = CoreWorker::<WorkerEntry>::new();
        NodeApp {
            inner: Arc::new(Mutex::new(worker)),
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 等同于 `app.service(...)`
    #[wasm_bindgen]
    pub fn service(&self, resource: Resource) {
        if let Some(handler) = resource.callback {
            let mut routes = self.routes.lock().unwrap();
            routes.insert(resource.name, handler);
        } else {
            // 没有 callback
        }
    }

    /// 等同于 `factory -> srv -> call -> resp`
    /// 这里做了极简封装，直接调用之前注册的 JS handler
    #[wasm_bindgen]
    pub async fn call_task(&self, name: String, args: JsValue) -> Result<JsValue, JsValue> {
        // 1. 查找是否有对应的 JS 回调
        let routes = self.routes.lock().unwrap();
        let handler = match routes.get(&name) {
            Some(f) => f.clone(),
            None => return Err(JsValue::from_str("No route found for given task name")),
        };
        drop(routes);
    
        // 2. 调用 JS handler，传递参数
        let js_result = handler.call1(&JsValue::NULL, &args);
    
        match js_result {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        }
    }
}

/// 为在 JS 中更接近 DSL，增加一些辅助的 free function
#[wasm_bindgen]
pub fn new_app() -> NodeApp {
    NodeApp::new()
}

#[wasm_bindgen]
pub fn resource(name: &str) -> Resource {
    Resource::new(name.to_owned())
}