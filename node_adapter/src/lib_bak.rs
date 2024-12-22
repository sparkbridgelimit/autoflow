use actix_service::{boxed, fn_service, Service, ServiceFactory};
use futures_util::future::join_all;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

async fn async_task(value: i32) -> i32 {
    value * 2
}

async fn test_fn_service() -> String {
    // 使用 fn_service 包装异步函数
    let service = fn_service(|req: i32| async move { Ok::<_, ()>(req * 2) });

    let result = service.call(10).await.unwrap();
    assert_eq!(result, 20);
    "test_fn_service passed!".to_string()
}

async fn test_boxed_service() -> String {
    // 使用 boxed 包装 service
    let service = boxed::service(fn_service(|req: i32| async move { Ok::<_, ()>(req * 3) }));
    let result = service.call(5).await.unwrap();
    assert_eq!(result, 15);
    "test_boxed_service passed!".to_string()
}

async fn test_service_factory() -> String {
    // 使用 ServiceFactory 创建服务
    let factory = Rc::new(fn_service(|req: i32| async move { Ok::<_, ()>(req * 4) }));
    let service = factory.new_service(()).await.unwrap();

    let result = service.call(7).await.unwrap();
    assert_eq!(result, 28);
    "test_service_factory passed!".to_string()
}

async fn test_join_all() -> String {
    let tasks = vec![async_task(1), async_task(2), async_task(3)];
    let results = join_all(tasks).await;

    assert_eq!(results, vec![2, 4, 6]);
    "test_join_all passed!".to_string()
}

#[wasm_bindgen]
pub async fn run_tests() -> js_sys::Array {
    let mut results = vec![];

    results.push(test_fn_service().await);
    results.push(test_boxed_service().await);
    results.push(test_service_factory().await);
    results.push(test_join_all().await);

    // 将所有测试结果返回给 JavaScript
    results.into_iter().map(JsValue::from).collect()
}