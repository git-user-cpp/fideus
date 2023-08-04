![logo_round_mini](https://github.com/git-user-cpp/fideus/assets/61907955/5e8edaf9-2549-4bbd-89b3-45376ae71bd2)

![FiDeus](https://github.com/git-user-cpp/fideus/assets/61907955/c2870b6a-ad23-45a1-9657-046d6261491d)

| Linux | Windows | macOs|
|---|---|---|
| [![Build Ubuntu](https://github.com/git-user-cpp/finance_manager/actions/workflows/CI_Ubuntu.yml/badge.svg?branch=main)](https://github.com/git-user-cpp/finance_manager/actions/workflows/CI_Ubuntu.yml) | [![Build Windows](https://github.com/git-user-cpp/finance_manager/actions/workflows/CI_Windows.yml/badge.svg?branch=main)](https://github.com/git-user-cpp/finance_manager/actions/workflows/CI_Windows.yml) | [![Build macOS](https://github.com/git-user-cpp/finance_manager/actions/workflows/CI_macOS.yml/badge.svg?branch=main)](https://github.com/git-user-cpp/finance_manager/actions/workflows/CI_macOS.yml) |

| Downloads | Stars |
|---|---|
| ![GitHub all releases](https://img.shields.io/github/downloads/git-user-cpp/finance_manager/total?color=00FF00&label=Downloads&logo=GitHub&logoColor=00FF00&style=plastic) | ![GitHub Repo stars](https://img.shields.io/github/stars/git-user-cpp/finance_manager?color=FFFF00&label=Stars&logo=GitHub&logoColor=FFFF00&style=plastic) |

| Stable version | Release date | License | Languages | Main language | Size |
|---|---|---|---|---|---|
| ![GitHub release (latest by date)](https://img.shields.io/github/v/release/git-user-cpp/finance_manager?color=ff0000&label=Release&logo=GitHub&logoColor=ff0000&style=plastic) | ![GitHub Release Date](https://img.shields.io/github/release-date/git-user-cpp/finance_manager?color=ff4500&label=Release%20date&logo=GitHub&logoColor=ff4500&style=plastic) | ![GitHub](https://img.shields.io/github/license/git-user-cpp/finance_manager?color=FFD700&label=License&logo=GitHub&logoColor=FFD700&style=plastic) | ![GitHub language count](https://img.shields.io/github/languages/count/git-user-cpp/finance_manager?color=7FFFD4&label=Languages&logo=GitHub&logoColor=7FFFD4&style=plastic) | ![GitHub top language](https://img.shields.io/github/languages/top/git-user-cpp/finance_manager?color=red&label=Rust&logo=GitHub&logoColor=red&style=plastic) | ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/git-user-cpp/finance_manager?color=00BFFF&label=Code%20size&logo=GitHub&logoColor=00BFFF&style=plastic) |

| Contributors | Watchers | Followers | Sponsors |
|---|---|---|---|
| ![GitHub contributors](https://img.shields.io/github/contributors-anon/git-user-cpp/finance_manager?color=ff0000&label=Contributors&logo=GitHub&logoColor=ff0000&style=plastic) | ![GitHub watchers](https://img.shields.io/github/watchers/git-user-cpp/finance_manager?color=DC143C&label=Watchers&logo=GitHub&logoColor=DC143C&style=plastic) | ![GitHub followers](https://img.shields.io/github/followers/git-user-cpp?color=7FFF00&label=Followers&logo=GitHub&logoColor=7FFF00&style=plastic) | ![GitHub Sponsors](https://img.shields.io/github/sponsors/git-user-cpp?color=00FFFF&label=Sponsors&logo=GitHub&logoColor=00FFFF&style=plastic) |

| Activity | Last commit|
|---|---|
| ![GitHub commit activity](https://img.shields.io/github/commit-activity/y/git-user-cpp/finance_manager?color=98FB98&label=Commit%20activity&logo=GitHub&logoColor=98FB98&style=plastic) | ![GitHub last commit](https://img.shields.io/github/last-commit/git-user-cpp/finance_manager?color=98FB98&label=Last%20commit&logo=GitHub&logoColor=98FB98&style=plastic) |

---

:heavy_dollar_sign: An easy-to-use financial manager written in Rust :heavy_dollar_sign:

---

## ⚠️ LICENSE ⚠️

Copyright 2023 Andrew Kushyk

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

---

## 🤝 How to contribute to FiDeus 🤝

📘 All the rules are here: [Contribution](https://github.com/git-user-cpp/fideus/blob/main/CONTRIBUTING.md).

---

## 💻 How to use this app 💻

1) ⚠️ **The application is designed for Linux, so avoid using other systems or remake the program to avoid bugs.** ⚠️

2) 🗂️ Download *all* files and put them in a one directory.

3) ⚒️ Compile and run the program with *cargo*.
    - Compilation example for desktop version
    ```
    cargo run --features desktop
    ```
    - Compilation example for console version
    ```
    cargo run --features console
    ```
4) :heavy_dollar_sign: Enjoy =)

---

## :question: How to use tests :question:

1) :small_red_triangle: When you've already downloaded the program and you wanna test it, use this command:

```
cargo test -- --nocapture
```

- ⚠️ Note that while using **cargo test** without **-- --nocapture** you won't see standart output.

---
