# Loreforge

세계관(worldbuilding) 구축 데스크톱 도구입니다. 지도를 그리고, 장소·인물·세력·사건·아이템·개념 같은 설정들을 서로 연결하며 정리할 수 있습니다. AI를 연결하면 설명 확장, 관계 제안, 아이디어 제안, 설정 일관성 점검 같은 기능도 쓸 수 있습니다.

[![Build](https://github.com/loreforge-project/loreforge/actions/workflows/build.yml/badge.svg)](https://github.com/loreforge-project/loreforge/actions/workflows/build.yml)
[![Source available, no redistribution](https://img.shields.io/badge/license-source--available%20(no%20redistribution)-orange.svg)](LICENSE)

---

## 다운로드

**[Releases](https://github.com/loreforge-project/loreforge/releases)** 페이지에서 최신 버전을 받을 수 있습니다.

- `Loreforge-portable.exe` — 설치 없이 그냥 실행하는 단독 실행파일
- `SHA256SUMS.txt` — 무결성 확인용 체크섬(`Get-FileHash .\Loreforge-portable.exe -Algorithm SHA256`로 대조)

Windows 10(1803 이상)·11에는 WebView2가 기본 내장되어 있어 별도 설치 없이 바로 실행됩니다.

---

## 이 프로그램이 안전한지 궁금하다면

소스 전체가 이 저장소에 공개되어 있고, 유일한 외부 통신은 사용자가 설정 메뉴에서 직접 입력한 AI 엔드포인트로의 요청뿐입니다(AI 기능 사용 시에만 호출). 실행파일은 [GitHub Actions](.github/workflows/build.yml)가 이 소스로 직접 빌드하며, 데이터는 전부 로컬(`.json` 파일·localStorage)에만 남습니다. 의심되면 소스를 직접 읽어보거나 [VirusTotal](https://www.virustotal.com/)로 검사해보세요.

---

## 만드는 법 (직접 빌드하기)

Rust만 있으면 되고 Node.js는 필요 없습니다. 자세한 절차는 [`loreforge-app/BUILD.md`](loreforge-app/BUILD.md)를 참고하세요. 개인적으로 빌드해서 검증·테스트해보는 것은 자유롭지만, 빌드한 결과물을 다른 사람에게 배포하는 것은 라이선스상 허용되지 않습니다 — 배포는 이 저장소의 공식 [Releases](https://github.com/loreforge-project/loreforge/releases)를 통해서만 이루어집니다.

```powershell
cd loreforge-app
cargo tauri build
```

---

## 주요 기능

- 지도: 장소(점)·영역(면, 구멍 포함)·선(경로)·라벨을 자유곡선으로 그리기, 여러 겹의 하위 지도, 지형 브러시, 배경 이미지/단색, 축척, 지도 자체 크기 조절
- 관계도 / 연표 뷰
- AI 연결(OpenAI 호환 엔드포인트라면 어디든): 설명 확장, 관계 제안, 아이디어 제안, 설정 일관성 심층 점검
- 규칙 기반 일관성 점검(이름 중복, 끊어진 링크, 자기참조 등)
- 한국어/영어 다국어 지원
- 세계관 설정집(자립형 HTML → 인쇄/PDF) 및 지도 PNG 내보내기

## 라이선스

[소스 공개(Source-Available), 재배포 금지](LICENSE) — 소스를 읽고 개인적으로 빌드해보는 것은 자유롭지만, 원본이든 수정본이든 재배포는 저작권자의 사전 허가 없이는 금지됩니다. 배포된 실행파일(Releases)을 그대로 내려받아 쓰는 것은 누구나 자유롭게 할 수 있습니다.
