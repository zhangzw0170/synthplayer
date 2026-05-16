# Changelog

## 2026-05-16

### Bugs Fixed

#### 1. 进度条在双击歌曲后直接满格（MP3 时长归零）

- **现象**：双击 MP3 文件后进度条跳到末尾，右侧时间始终显示 `00:00`
- **根因**：`rodio::Decoder::total_duration()` 对 MP3 返回 `None`，导致 `current_duration` 为 0；UI 用 `.max(1.0)` 防除零，把 0 秒强转为 1 秒，播放超过 1 秒后 position 触顶 → progress = 1.0
- **修复**：`AudioPlayer::play()` 增加 `known_dur` fallback 参数，播放时从 lofty 元数据获取时长；进度条增加 `.clamp(0.0, 1.0)` 兜底
- **影响文件**：`src/core/player.rs`, `src/ui/app.rs`
- **提交**：`98a4823` feat: keyboard shortcuts, play modes, and session persistence

#### 2. 快进/快退图标与上一首/下一首图标画反

- **现象**：控制栏中快退按钮显示竖线+三角（`|<`），上一首显示双三角（`<<`），含义对调
- **根因**：`icon_btn()` 中 `"prev"` 和 `"rewind"`（以及 `"next"` 和 `"forward"`）的绘制代码写反
- **修复**：交换四组 case 的绘制逻辑
- **影响文件**：`src/ui/app.rs`
- **提交**：`d8d0a30` fix: swap rewind/forward icons with prev/next

#### 3. 快进/快退图标只显示上半部分

- **现象**：修复图标的含义映射后，双三角（`<<` / `>>`）只渲染上半区
- **根因**：三角形顶点定在 `cy - s`（图标顶部），底边在 `cy`（中线），只覆盖上半区
- **修复**：底边改为从 `cy - s` 到 `cy + s`（贯穿整个图标高度），顶点居中
- **影响文件**：`src/ui/app.rs`
- **提交**：`191db4b` fix: proper symmetric triangles for rewind/forward icons

#### 4. 播放模式提示未居中 + 编译错误

- **现象**：添加模式悬停提示后 `cargo check` 未执行，`on_hover_text()` 消耗 `Response` 导致后续 `hovered()` 编译失败
- **根因**：`egui::Response::on_hover_text(self)` 按值获取所有权，调用后不能再访问字段
- **修复**：在 `on_hover_text()` 之前将 `mode_resp.hovered()` 存入局部变量
- **影响文件**：`src/ui/app.rs`
- **提交**：`79beb33` fix: mode hover check before on_hover_text move
