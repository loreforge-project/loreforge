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

/// 폴더 선택 창을 띄우고, 고른 폴더 경로를 반환. 취소하면 None. (md 미러 대상 폴더 지정용)
#[tauri::command]
async fn pick_folder_dialog() -> Result<Option<String>, String> {
    let dir = rfd::AsyncFileDialog::new().pick_folder().await;
    Ok(dir.map(|d| d.path().to_string_lossy().to_string()))
}

/// 미러 파일 하나: text(텍스트) 또는 base64(바이너리) 중 하나를 담는다.
#[derive(serde::Deserialize)]
struct MirrorFile {
    path: String,
    text: Option<String>,
    base64: Option<String>,
}

/// 지정된 폴더(dir) 아래에 md 미러 트리를 통째로 씀. 앱이 관리하는 하위폴더
/// (places/entities/events/assets)는 먼저 비워서 이름 변경·삭제로 생긴 고아 파일을 없앤다.
/// 그 외 사용자가 폴더에 둔 파일은 건드리지 않는다. 경로는 dir 밖(..)으로 못 나가게 검증.
#[tauri::command]
async fn write_md_mirror(dir: String, files: Vec<MirrorFile>) -> Result<(), String> {
    use base64::Engine;
    use std::path::{Component, Path};
    let root = std::path::PathBuf::from(&dir);
    if !root.is_dir() {
        return Err("mirror folder not found".into());
    }
    for sub in ["places", "entities", "events", "assets"] {
        let p = root.join(sub);
        if p.is_dir() {
            let _ = std::fs::remove_dir_all(&p);
        }
    }
    for f in files {
        let rel = Path::new(&f.path);
        let unsafe_path = rel.is_absolute()
            || rel.components().any(|c| {
                matches!(
                    c,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            });
        if unsafe_path {
            return Err(format!("unsafe path: {}", f.path));
        }
        let full = root.join(rel);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        if let Some(b64) = f.base64 {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(b64)
                .map_err(|e| e.to_string())?;
            std::fs::write(&full, bytes).map_err(|e| e.to_string())?;
        } else {
            std::fs::write(&full, f.text.unwrap_or_default()).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
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
            save_html_dialog,
            pick_folder_dialog,
            write_md_mirror
        ])
        .run(tauri::generate_context!())
        .expect("error while running Loreforge");
}
