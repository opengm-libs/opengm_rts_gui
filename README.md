# 随机性检测
随机性检测程序, 遵循GM/T 0005-2021 随机性检测规范.

## Run & Build
```
cargo tauri build [--target=<target>]
```

## 性能
1000样本,每个样本100万比特,在M1 pro max下50秒完成.
[image](./perf.jpg)
![performace](/perf.jpg)