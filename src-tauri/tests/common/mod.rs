// 通用测试工具模块
// 位置：src-tauri/tests/common/mod.rs

use tauri::{AppHandle, Emitter, Manager, WebviewWindowBuilder};

/// 创建测试用的 Tauri 应用实例
pub fn setup_test_app() -> AppHandle {
    let builder = tauri::Builder::default();
    
    let app = builder
        .setup(|app| {
            // 创建主窗口
            let _window = WebviewWindowBuilder::new(app, "main", Default::default())
                .build()?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri app");
    
    let handle = app.handle().clone();
    
    // 在后台运行应用
    std::thread::spawn(move || {
        app.run(|_app_handle, _event| {});
    });
    
    handle
}

/// 清理测试环境
pub fn cleanup_test_app(_handle: AppHandle) {
    // 清理测试资源
    // TODO: 实现具体的清理逻辑
}
