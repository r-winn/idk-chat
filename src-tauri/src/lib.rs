#[cfg(any(target_os = "macos", windows))]
use tauri_plugin_updater::UpdaterExt;

#[cfg(any(target_os = "macos", windows))]
async fn install_available_update(app: tauri::AppHandle) {
    let Ok(updater) = app.updater_builder()
        .timeout(std::time::Duration::from_secs(30))
        .build() else { return; };
    let Ok(Some(update)) = updater.check().await else { return; };
    if update.download_and_install(|_, _| {}, || {}).await.is_ok() {
        app.restart();
    }
}

pub fn run() {
    let builder = tauri::Builder::default();

    #[cfg(any(target_os = "macos", windows))]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .setup(|app| {
            #[cfg(any(target_os = "macos", windows))]
            {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(4)).await;
                    install_available_update(handle).await;
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running IDK Chat desktop");
}
