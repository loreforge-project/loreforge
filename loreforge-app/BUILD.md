# Loreforge 데스크톱 앱 빌드 가이드 (Windows · Tauri v2)

`Loreforge.html`(웹앱)을 **설치형 Windows 데스크톱 앱(.exe)** 으로 만드는 방법입니다.
Rust만 있으면 되고 **Node.js는 필요 없습니다**(정적 HTML이라서).

> 실제 빌드는 **너의 Windows 컴퓨터에서** 진행해. 아래 명령은 전부 **PowerShell**에서 실행하면 돼.

---

## 0. 이 앱이 하는 일 (구조 요약)

- 화면(UI)은 위 폴더의 `Loreforge.html` 하나로 되어 있어.
- 빌드하면 그 HTML을 감싼 가벼운 네이티브 앱이 만들어져(설치본 몇 MB 수준).
- AI 요청(로컬 Ollama·클라우드 API)은 Rust 쪽 `ai_fetch` 명령이 대신 보내서 **CORS 문제 없이** 동작해.

```
loreforge-app/
├─ sync-ui.ps1          ← Loreforge.html 을 ui\index.html 로 복사(빌드 전 실행)
├─ app-icon.png         ← 아이콘 원본(원하면 이걸 교체하고 재생성)
├─ ui/                  ← (자동 생성) 프론트엔드. index.html 이 여기로 복사됨
└─ src-tauri/           ← Tauri(러스트) 프로젝트
   ├─ tauri.conf.json   ← 앱 설정(창 크기·이름·아이콘·번들)
   ├─ Cargo.toml        ← 러스트 의존성
   ├─ build.rs
   ├─ capabilities/default.json
   ├─ icons/            ← 앱 아이콘들(생성됨)
   └─ src/main.rs       ← ai_fetch 명령 + 앱 진입점
```

---

## 1. 사전 준비물 설치 (한 번만)

### 1-1. Microsoft C++ Build Tools
1. https://visualstudio.microsoft.com/visual-cpp-build-tools/ 에서 설치 프로그램 다운로드
2. 설치 시 **"C++를 사용한 데스크톱 개발(Desktop development with C++)"** 항목 체크 후 설치

### 1-2. WebView2
- Windows 10(1803 이상)·11에는 **이미 설치돼 있어** → 보통 건너뛰어도 돼.
- 없다면: https://developer.microsoft.com/microsoft-edge/webview2/ 에서 "Evergreen Bootstrapper" 설치

### 1-3. Rust (MSVC 툴체인)
1. https://www.rust-lang.org/tools/install 에서 `rustup-init.exe` 실행
2. 기본값(`x86_64-pc-windows-msvc`)으로 설치
3. 설치 후 **PowerShell을 새로 열고** 확인:
   ```powershell
   rustc --version
   cargo --version
   ```
   버전이 뜨면 성공.

### 1-4. Tauri CLI
```powershell
cargo install tauri-cli --version "^2.0"
```
확인:
```powershell
cargo tauri --version
```

> 이미 깔려 있는지 모르겠으면 위 확인 명령들을 먼저 쳐봐. 버전이 안 뜨면 그 항목만 설치하면 돼.

---

## 2. 빌드하기

PowerShell에서 이 폴더로 이동:
```powershell
cd "D:\WorkSpace\world engine\loreforge-app"
```

### 2-1. 설치본 빌드
```powershell
cargo tauri build
```
> `build`/`dev`는 이제 **자동으로 `Loreforge.html`을 `ui\index.html`로 복사**한 뒤 진행돼(설정의 `beforeBuildCommand`). 그래서 앱 소스를 고쳐도 따로 복사할 필요 없어.
> (수동으로 하고 싶으면 `.\sync-ui.ps1` 실행. 실행이 막히면 `powershell -ExecutionPolicy Bypass -File .\sync-ui.ps1`)
- 처음엔 러스트 크레이트를 받고 컴파일하느라 **5~15분** 걸릴 수 있어(다음부터는 빨라져).
- 끝나면 결과물이 여기 생겨:
  - **설치 프로그램**: `src-tauri\target\release\bundle\nsis\Loreforge_1.0.0_x64-setup.exe`
  - **단독 실행파일**: `src-tauri\target\release\loreforge.exe`

설치본을 실행하면 시작 메뉴에 **Loreforge**가 등록돼.

---

## 3. 빌드 없이 바로 실행해보기 (개발/테스트용)

```powershell
cargo tauri dev
```
창이 바로 떠(시작 시 최신 `Loreforge.html`이 자동 복사됨). F12(또는 우클릭 → 검사)로 콘솔을 열어 AI payload 로그 등을 볼 수 있어.
`Loreforge.html`을 고쳤으면 **`cargo tauri dev`를 다시 시작**하면 최신본이 반영돼(자동 복사).

---

## 4. 아이콘 바꾸기 (선택)

`app-icon.png`(정사각형, 1024×1024 권장)를 원하는 이미지로 바꾼 뒤:
```powershell
cargo tauri icon app-icon.png
```
→ `src-tauri\icons\` 아이콘들이 다시 생성돼. 그다음 다시 빌드.

---

## 5. 데이터 보관 관련 (안전 모드)

- **자동 저장**은 앱 내부 저장소(localStorage)에만 계속 돼 → 크래시·실수로부터 보호. 앱을 껐다 켜도 유지.
- **저장 (Ctrl+S)**: 현재 열린 파일에 **바로 덮어써(커밋)**. 열린 파일이 없으면 저장 위치를 고르는 창이 뜨고, 그 파일이 "현재 파일"이 돼.
- **내보내기 (Ctrl+Shift+S)**: 항상 저장창을 띄워 **새 `.json`으로 저장(다른 이름으로 저장)**. 새 위치가 현재 파일이 돼.
- **핵심**: 자동 저장은 파일을 건드리지 않아서, **불러온 원본 `.json`은 네가 '저장'을 누르기 전엔 안 바뀌어.** 파일보다 앞선 변경이 있으면 상단에 `파일명 •` 처럼 점으로 표시돼.
- **불러오기**는 파일 탐색기로 `.json`을 열어. **새로 만들기**는 빈 세계(현재 파일 없음).
- 브라우저에서 열면 네이티브 창 대신 다운로드/파일선택으로 폴백돼(개발 테스트용).
- AI 연결(엔드포인트·모델·키)은 설정에 저장돼 있으니 앱에서 한 번 더 넣어주면 돼.

---

## 6. 자주 나는 오류

- **`cargo: 명령을 찾을 수 없음`** → Rust 설치 후 PowerShell을 새로 열지 않은 경우. 창을 새로 열어.
- **`link.exe` / MSVC 관련 오류** → 1-1의 C++ Build Tools에서 "데스크톱 개발(C++)" 체크가 빠진 경우. 다시 설치.
- **`frontendDist ... does not exist`** → `sync-ui.ps1`을 안 돌린 경우. 2-1을 먼저 실행.
- **앱은 뜨는데 AI가 안 됨** → Ollama가 실행 중인지(`http://localhost:11434`), 설정의 엔드포인트/모델이 맞는지 확인. 앱의 **AI 설정 → 연결 테스트**로 점검.
- **빌드가 느림/멈춘 듯** → 첫 빌드는 원래 오래 걸려. 진행 로그가 흐르면 정상.

---

## 7. 다음 단계 (선택)

- **자동 업데이트**: Tauri Updater 플러그인으로 새 버전 자동 배포 가능.
- **코드 서명**: 서명 없이 배포하면 Windows SmartScreen 경고가 떠. 개인 사용엔 문제없고, 배포하려면 서명 인증서가 필요해.
- 궁금한 거 있으면 언제든 물어봐.
