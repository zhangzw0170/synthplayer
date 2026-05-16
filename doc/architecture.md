# SynthPlayer 架构

## 项目结构

```
SynthPlayer/
├── Cargo.toml
├── src/
│   ├── main.rs           # 入口，启动 egui 应用
│   ├── core/
│   │   ├── mod.rs         # 核心库模块导出
│   │   ├── scanner.rs     # 本地音乐文件扫描
│   │   ├── metadata.rs    # 音频元数据读取（标题、艺术家、时长）
│   │   └── player.rs      # 音频播放控制
│   └── ui/
│       ├── mod.rs
│       └── app.rs         # egui 主界面
└── doc/                   # 文档
```

## 架构分层

```
┌──────────────────────────┐
│  ui/app.rs               │  egui 界面（可替换）
│  曲目列表 / 控制栏 / 状态栏  │
├──────────────────────────┤
│  core/                   │  核心库（UI 无关）
│  scanner → metadata      │  文件扫描 → 元数据读取
│  player                  │  音频播放（rodio）
└──────────────────────────┘
```

核心库不依赖任何 UI 框架，未来可以替换为 TUI、Tauri 等其他前端。

## 依赖

| crate | 用途 |
|-------|------|
| eframe 0.31 | egui 应用框架（窗口管理、事件循环） |
| egui 0.31 | 即时模式 GUI 渲染 |
| rodio 0.20 | 音频播放（MP3/FLAC/WAV/OGG/AAC） |
| lofty 0.22 | 元数据解析（ID3/Vorbis/FLAC tags） |
| walkdir 2 | 递归目录扫描 |

## 数据流

```
用户点击 Scan
  → scanner::scan_directory(root) → Vec<PathBuf>
  → metadata::read_metadata(path) → TrackInfo
  → 显示在曲目列表

用户双击曲目
  → player.play(path) → rodio 解码播放
  → 状态栏更新

播放结束
  → player.finished() == true
  → 自动播放下一首
```

## 使用

```bash
cargo run
```

启动后在 Music Directory 输入框中填入音乐目录路径，点击 Scan 扫描，双击曲目播放。
