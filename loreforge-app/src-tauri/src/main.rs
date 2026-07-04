// Windows 릴리스에서 콘솔 창이 뜨지 않도록
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

#[derive(serde::Serialize)]
struct AiResponse {
    status: u16,
    body: String,
}

/// 프론트엔드(Loreforge.html)에서 window.__TAURI__.core.invoke("ai_fetch", ...)로 호출.
/// Rust(reqwest)가 직접 HTTP 요청을 보내므로 웹뷰의 CORS 제약을 받지 않음 →
/// 로컬 LLM(Ollama 등)과 클라우드 API 모두 연결 가능.
#[tauri::command]
async fn ai_fetch(
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: Option<String>,
) -> Result<AiResponse, String> {
    let client = reqwest::Client::new();
    let m = reqwest::Method::from_bytes(method.as_bytes()).map_err(|e| e.to_string())?;
    let mut req = client.request(m, &url);
    for (k, v) in headers {
        req = req.header(k, v);
    }
    if let Some(b) = body {
        req = req.body(b);
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(AiResponse { status, body: text })
}

#[derive(serde::Serialize)]
struct OpenedFile {
    path: String,
    contents: String,
}

/// 저장 위치 선택 창을 띄우고, 고른 경로에 세계 JSON을 씀. 취소하면 None.
#[tauri::command]
async fn save_world_dialog(default_name: String, contents: String) -> Result<Option<String>, String> {
    let file = rfd::AsyncFileDialog::new()
        .set_file_name(&default_name)
        .add_filter("Loreforge world", &["json"])
        .save_file()
        .await;
    match file {
        Some(f) => {
            let path = f.path().to_path_buf();
            std::fs::write(&path, contents).map_err(|e| e.to_string())?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
        None => Ok(None),
    }
}

/// 이미 아는 경로(현재 파일)에 대화상자 없이 바로 씀.
#[tauri::command]
async fn write_world(path: String, contents: String) -> Result<(), String> {
    std::fs::write(&path, contents).map_err(|e| e.to_string())
}

/// 저장 위치 선택 창을 띄우고, 고른 경로에 지도 PNG(base64로 전달받은 바이너리)를 씀. 취소하면 None.
#[tauri::command]
async fn save_png_dialog(default_name: String, data_base64: String) -> Result<Option<String>, String> {
    use base64::Engine;
    let file = rfd::AsyncFileDialog::new()
        .set_file_name(&default_name)
        .add_filter("PNG image", &["png"])
        .save_file()
        .await;
    match file {
        Some(f) => {
            let path = f.path().to_path_buf();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(data_base64)
                .map_err(|e| e.to_string())?;
            std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
        None => Ok(None),
    }
}

/// 저장 위치 선택 창을 띄우고, 고른 경로에 설정집 HTML(텍스트)을 씀. 취소하면 None.
#[tauri::command]
async fn save_html_dialog(default_name: String, contents: String) -> Result<Option<String>, String> {
    let file = rfd::AsyncFileDialog::new()
        .set_file_name(&default_name)
        .add_filter("HTML document", &["html"])
        .save_file()
        .await;
    match file {
        Some(f) => {
            let path = f.path().to_path_buf();
            std::fs::write(&path, contents).map_err(|e| e.to_string())?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
        None => Ok(None),
    }
}

/// 파일 열기 창을 띄우고, 고른 .json 을 읽어 경로+내용을 반환. 취소하면 None.
#[tauri::command]
async fn open_world_dialog() -> Result<Option<OpenedFile>, String> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("Loreforge world", &["json"])
        .pick_file()
        .await;
    match file {
        Some(f) => {
            let path = f.path().to_path_buf();
            let contents = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            Ok(Some(OpenedFile {
                path: path.to_string_lossy().to_string(),
                contents,
            }))
        }
        None => Ok(None),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ai_fetch,
            save_world_dialog,
            write_world,
            open_world_dialog,
            save_png_dialog,
            save_html_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running Loreforge");
}
