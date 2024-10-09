# 随机性检测
随机性检测程序, 遵循GM/T 0005-2021 随机性检测规范.

## Build
- Install rust(https://www.rust-lang.org/tools/install)
- Install tauri cli `cargo install tauri-cli`(https://tauri.app/v1/guides/getting-started/setup/html-css-js)
- Build `cargo tauri build [--target=<target>]`

## 性能
1000样本,每个样本100万比特,在M1 Max下10秒完成.
![performace](/perf.jpg)