use crate::core::metadata;
use crate::core::player::AudioPlayer;
use crate::core::scanner;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, RichText, Sense, Stroke, Vec2};
use std::path::PathBuf;
use std::time::Duration;

struct C;
impl C {
    const BG: Color32 = Color32::from_rgb(14, 14, 20);
    const SURFACE: Color32 = Color32::from_rgb(24, 24, 32);
    const HOVER: Color32 = Color32::from_rgb(42, 42, 56);
    const BORDER: Color32 = Color32::from_rgb(48, 48, 62);
    const TEXT: Color32 = Color32::from_rgb(235, 235, 245);
    const DIM: Color32 = Color32::from_rgb(155, 155, 175);
    const ACCENT: Color32 = Color32::from_rgb(72, 170, 255);
    const PLAYING: Color32 = Color32::from_rgb(88, 216, 172);
    const PLAYING_BG: Color32 = Color32::from_rgb(18, 55, 42);
    const SELECT_BG: Color32 = Color32::from_rgb(20, 48, 82);
    const FMT_MP3: Color32 = Color32::from_rgb(180, 120, 60);
    const FMT_FLAC: Color32 = Color32::from_rgb(120, 180, 100);
    const FMT_WAV: Color32 = Color32::from_rgb(100, 140, 200);
    const FMT_AAC: Color32 = Color32::from_rgb(160, 100, 180);
    const FMT_DEFAULT: Color32 = Color32::from_rgb(100, 100, 120);
}

fn fmt_color(ext: &str) -> Color32 {
    match ext {
        "mp3" => C::FMT_MP3,
        "flac" => C::FMT_FLAC,
        "wav" => C::FMT_WAV,
        "aac" | "m4a" => C::FMT_AAC,
        _ => C::FMT_DEFAULT,
    }
}

fn format_dur(d: std::time::Duration) -> String {
    let s = d.as_secs();
    format!("{:02}:{:02}", s / 60, s % 60)
}

// ─── Font & Style ────────────────────────────────────────────────

fn tri(painter: &egui::Painter, a: egui::Pos2, b: egui::Pos2, c: egui::Pos2, color: Color32) {
    painter.add(egui::Shape::convex_polygon(vec![a, b, c], color, Stroke::NONE));
}

fn icon_btn(ui: &mut egui::Ui, kind: &str, tooltip: &str) -> bool {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(28.0, 28.0), Sense::click());
    let painter = ui.painter();
    let c = if resp.hovered() { C::ACCENT } else { C::TEXT };
    let cx = rect.center();
    let s = 5.0;

    match kind {
        "rewind" => {
            let x = cx.x + 1.0;
            tri(painter, egui::pos2(x + s, cx.y - s), egui::pos2(x + s, cx.y + s), egui::pos2(x, cx.y), c);
            tri(painter, egui::pos2(x, cx.y - s), egui::pos2(x, cx.y + s), egui::pos2(x - s, cx.y), c);
        }
        "forward" => {
            let x = cx.x - 1.0;
            tri(painter, egui::pos2(x - s, cx.y - s), egui::pos2(x - s, cx.y + s), egui::pos2(x, cx.y), c);
            tri(painter, egui::pos2(x, cx.y - s), egui::pos2(x, cx.y + s), egui::pos2(x + s, cx.y), c);
        }
        "prev" => {
            tri(painter, egui::pos2(cx.x + 1.0, cx.y - s), egui::pos2(cx.x + 1.0, cx.y + s), egui::pos2(cx.x - s, cx.y), c);
            painter.rect_filled(egui::Rect::from_min_max(egui::pos2(cx.x - s - 2.0, cx.y - s), egui::pos2(cx.x - s, cx.y + s)), 0.0, c);
        }
        "next" => {
            tri(painter, egui::pos2(cx.x - 1.0, cx.y - s), egui::pos2(cx.x - 1.0, cx.y + s), egui::pos2(cx.x + s, cx.y), c);
            painter.rect_filled(egui::Rect::from_min_max(egui::pos2(cx.x + s, cx.y - s), egui::pos2(cx.x + s + 2.0, cx.y + s)), 0.0, c);
        }
        "play" => {
            tri(painter, egui::pos2(cx.x - s, cx.y - s * 1.2), egui::pos2(cx.x - s, cx.y + s * 1.2), egui::pos2(cx.x + s, cx.y), c);
        }
        "pause" => {
            let g = 2.0;
            painter.rect_filled(egui::Rect::from_center_size(egui::pos2(cx.x - g, cx.y), Vec2::new(3.0, s * 2.2)), 0.0, c);
            painter.rect_filled(egui::Rect::from_center_size(egui::pos2(cx.x + g, cx.y), Vec2::new(3.0, s * 2.2)), 0.0, c);
        }
        "stop" => {
            painter.rect_filled(egui::Rect::from_center_size(cx, Vec2::new(s * 1.8, s * 1.8)), 0.0, c);
        }
        "settings" => {
            painter.circle_stroke(cx, s * 0.6, Stroke::new(1.5, c));
            painter.circle_filled(cx, s * 0.15, c);
            for i in 0..6 {
                let a = i as f32 * std::f32::consts::PI / 3.0;
                let r = s * 0.95;
                painter.circle_filled(egui::pos2(cx.x + r * a.cos(), cx.y + r * a.sin()), 1.2, c);
            }
        }
        "vol" => {
            let lx = cx.x - 4.0;
            painter.rect_filled(egui::Rect::from_center_size(egui::pos2(lx, cx.y), Vec2::new(4.0, 5.0)), 0.0, c);
            tri(painter, egui::pos2(lx + 2.0, cx.y - 2.5), egui::pos2(lx + 2.0, cx.y + 2.5), egui::pos2(cx.x + 2.0, cx.y - 5.0), c);
            tri(painter, egui::pos2(lx + 2.0, cx.y + 2.5), egui::pos2(cx.x + 2.0, cx.y + 5.0), egui::pos2(cx.x + 2.0, cx.y - 5.0), c);
        }
        _ => {}
    }

    let clicked = resp.clicked();
    resp.on_hover_text(tooltip);
    clicked
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // CJK proportional: try Source Han Serif SC first, then fallbacks
    let cjk_paths = [
        "C:\\Windows\\Fonts\\SourceHanSerifSC-Regular.ttf",
        "C:\\Windows\\Fonts\\SourceHanSerifSC-Bold.ttf",
        "C:\\Windows\\Fonts\\NotoSerifSC-VF.ttf",
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\msyh.ttc",
    ];
    let mut cjk_loaded = false;
    for p in &cjk_paths {
        if let Ok(data) = std::fs::read(p) {
            fonts
                .font_data
                .insert("cjk".into(), egui::FontData::from_owned(data).into());
            cjk_loaded = true;
            break;
        }
    }
    if cjk_loaded {
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "cjk".into());
    }

    // Symbol font for UI icons (media controls, etc.)
    let symbol_paths = [
        "C:\\Windows\\Fonts\\seguisym.ttf",  // Segoe UI Symbol
        "C:\\Windows\\Fonts\\seguiemj.ttf",  // Segoe UI Emoji
    ];
    for p in &symbol_paths {
        if let Ok(data) = std::fs::read(p) {
            fonts
                .font_data
                .insert("symbol".into(), egui::FontData::from_owned(data).into());
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push("symbol".into());
            break;
        }
    }

    // Monospace: try JetBrains Mono first, then fallback
    let mono_paths = [
        "C:\\Users\\Lenovo\\.local\\share\\fonts\\JetBrainsMono-Regular.ttf",
        "C:\\Users\\Lenovo\\AppData\\Local\\JetBrains\\Toolbox\\fonts\\JetBrainsMono-Regular.ttf",
        "C:\\Windows\\Fonts\\JetBrainsMono-Regular.ttf",
        "C:\\Windows\\Fonts\\Consola.ttf",
        "C:\\Windows\\Fonts\\cascadia.ttf",
    ];
    for p in &mono_paths {
        if let Ok(data) = std::fs::read(p) {
            fonts
                .font_data
                .insert("mono".into(), egui::FontData::from_owned(data).into());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "mono".into());
            break;
        }
    }

    ctx.set_fonts(fonts);
}

fn configure_style(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill = C::BG;
    v.window_fill = C::SURFACE;
    v.override_text_color = Some(C::TEXT);
    v.widgets.inactive.bg_fill = C::SURFACE;
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, C::DIM);
    v.widgets.hovered.bg_fill = C::HOVER;
    v.widgets.active.bg_fill = C::SELECT_BG;
    v.widgets.active.fg_stroke = Stroke::new(1.0, C::ACCENT);
    v.selection.bg_fill = C::SELECT_BG;
    v.selection.stroke = Stroke::new(1.0, C::ACCENT);
    ctx.set_visuals(v);

    let mut s = (*ctx.style()).clone();
    s.spacing.item_spacing = Vec2::new(6.0, 3.0);
    s.spacing.button_padding = Vec2::new(12.0, 5.0);
    s.spacing.indent = 14.0;
    s.visuals.window_corner_radius = CornerRadius::same(6);
    s.visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
    s.visuals.widgets.active.corner_radius = CornerRadius::same(4);
    ctx.set_style(s);
}

// ─── App state ───────────────────────────────────────────────────

pub struct SynthPlayerApp {
    music_dir: String,
    tracks: Vec<TrackEntry>,
    selected_index: Option<usize>,
    player: AudioPlayer,
    status_message: String,
    search_query: String,
    show_dir_input: bool,
    play_mode: PlayMode,
}

#[derive(Clone, Copy, PartialEq)]
enum PlayMode {
    Normal,
    Shuffle,
    RepeatOne,
    RepeatAll,
}

impl PlayMode {
    fn next(self) -> Self {
        match self {
            Self::Normal => Self::Shuffle,
            Self::Shuffle => Self::RepeatOne,
            Self::RepeatOne => Self::RepeatAll,
            Self::RepeatAll => Self::Normal,
        }
    }
}

struct TrackEntry {
    path: PathBuf,
    title: String,
    artist: String,
    duration_display: String,
    duration: Duration,
    format: String,
}

impl SynthPlayerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        configure_style(&cc.egui_ctx);
        let player = AudioPlayer::new().expect("Failed to init audio");

        let storage = cc.storage;
        let saved_dir = storage.and_then(|s| s.get_string("music_dir"));
        let saved_vol = storage
            .and_then(|s| s.get_string("volume"))
            .and_then(|v| v.parse::<f32>().ok());

        let mut app = Self {
            music_dir: saved_dir.unwrap_or_else(|| r"E:\Main\Music".into()),
            tracks: Vec::new(),
            selected_index: None,
            player,
            status_message: "Ready".into(),
            search_query: String::new(),
            show_dir_input: false,
            play_mode: PlayMode::Normal,
        };

        if let Some(vol) = saved_vol {
            app.player.set_volume(vol);
        }

        // Auto-scan on startup
        app.scan_music();
        app
    }

    fn scan_music(&mut self) {
        let root = PathBuf::from(&self.music_dir);
        if !root.exists() {
            self.status_message = format!("Not found: {}", self.music_dir);
            return;
        }
        self.status_message = "Scanning...".into();
        let paths = scanner::scan_directory(&root);
        self.tracks = paths
            .into_iter()
            .map(|path| {
                let info = metadata::read_metadata(&path);
                let dur = info.as_ref().map_or(std::time::Duration::ZERO, |i| i.duration);
                let fmt = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("???")
                    .to_uppercase();
                TrackEntry {
                    title: info.as_ref().map_or_else(
                        || path.file_stem().unwrap_or_default().to_string_lossy().into(),
                        |i| i.title.clone(),
                    ),
                    artist: info.as_ref().map_or_else(String::new, |i| i.artist.clone()),
                    duration_display: format_dur(dur),
                    duration: dur,
                    format: fmt,
                    path,
                }
            })
            .collect();
        self.status_message = format!("{} tracks", self.tracks.len());
    }

    fn filtered_indices(&self) -> Vec<usize> {
        let q = self.search_query.trim().to_lowercase();
        if q.is_empty() {
            (0..self.tracks.len()).collect()
        } else {
            self.tracks
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    t.title.to_lowercase().contains(&q) || t.artist.to_lowercase().contains(&q)
                })
                .map(|(i, _)| i)
                .collect()
        }
    }

    fn play_at(&mut self, idx: usize) {
        let path = self.tracks[idx].path.clone();
        let title = self.tracks[idx].title.clone();
        let dur = self.tracks[idx].duration;
        match self.player.play(path, Some(dur)) {
            Ok(()) => self.status_message = format!("Playing: {}", title),
            Err(e) => self.status_message = format!("Error [{}]: {}", title, e),
        }
    }

    fn play_selected(&mut self) {
        if let Some(idx) = self.selected_index {
            self.play_at(idx);
        }
    }

    fn play_next(&mut self) {
        if self.tracks.is_empty() {
            return;
        }
        match self.play_mode {
            PlayMode::Normal => {
                if let Some(idx) = self.selected_index
                    && idx + 1 < self.tracks.len()
                {
                    self.selected_index = Some(idx + 1);
                    self.play_selected();
                }
            }
            PlayMode::Shuffle => {
                use rand::Rng;
                let next = rand::thread_rng().gen_range(0..self.tracks.len());
                self.selected_index = Some(next);
                self.play_selected();
            }
            PlayMode::RepeatOne => {
                self.play_selected();
            }
            PlayMode::RepeatAll => {
                let next = match self.selected_index {
                    Some(idx) if idx + 1 < self.tracks.len() => idx + 1,
                    _ => 0,
                };
                self.selected_index = Some(next);
                self.play_selected();
            }
        }
    }

    fn play_prev(&mut self) {
        if self.tracks.is_empty() {
            return;
        }
        match self.play_mode {
            PlayMode::Normal => {
                if let Some(idx) = self.selected_index {
                    self.selected_index = Some(idx.saturating_sub(1));
                    self.play_selected();
                }
            }
            PlayMode::Shuffle => {
                use rand::Rng;
                let prev = rand::thread_rng().gen_range(0..self.tracks.len());
                self.selected_index = Some(prev);
                self.play_selected();
            }
            PlayMode::RepeatOne => {
                self.play_selected();
            }
            PlayMode::RepeatAll => {
                let prev = match self.selected_index {
                    Some(idx) if idx > 0 => idx - 1,
                    _ => self.tracks.len().saturating_sub(1),
                };
                self.selected_index = Some(prev);
                self.play_selected();
            }
        }
    }

    fn cycle_play_mode(&mut self) {
        self.play_mode = self.play_mode.next();
    }
}

// ─── UI rendering ────────────────────────────────────────────────

impl eframe::App for SynthPlayerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.player.finished() && self.player.current_path().is_some() {
            self.player.stop();
            self.play_next();
        }

        // ── Bottom: now playing + progress + controls ──
        egui::TopBottomPanel::bottom("player")
            .exact_height(88.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                let now_playing = self
                    .selected_index
                    .and_then(|i| self.tracks.get(i))
                    .filter(|t| self.player.current_path() == Some(&t.path));

                // Row 1: now playing + status message
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    if let Some(track) = &now_playing {
                        ui.label(
                            RichText::new(&track.title)
                                .size(13.0)
                                .strong()
                                .color(C::PLAYING),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(&track.artist)
                                .size(12.0)
                                .color(C::DIM),
                        );
                    } else {
                        ui.label(
                            RichText::new("No track selected")
                                .size(12.0)
                                .color(C::DIM),
                        );
                    }
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add_space(16.0);
                            ui.label(
                                RichText::new(&self.status_message)
                                    .size(10.0)
                                    .color(C::DIM),
                            );
                        },
                    );
                });

                ui.add_space(4.0);

                // Row 2: progress bar with time labels
                ui.horizontal(|ui| {
                    let pad = 16.0;
                    let label_w = 36.0; // "MM:SS" at 11px
                    ui.add_space(pad);

                    let pos = self.player.position();
                    let total = self.player.current_duration();
                    let total_secs = total.as_secs_f64().max(1.0);
                    let pos_secs = pos.as_secs_f64().min(total_secs);
                    let progress = if total_secs > 0.0 { (pos_secs / total_secs).clamp(0.0, 1.0) } else { 0.0 };

                    ui.label(
                        RichText::new(format_dur(pos))
                            .size(11.0)
                            .color(C::DIM),
                    );
                    ui.add_space(6.0);

                    // Custom progress bar
                    let bar_w = (ui.available_width() - label_w - 6.0 - pad).max(40.0);
                    let bar_h = 20.0;
                    let (bar_rect, bar_resp) = ui.allocate_exact_size(
                        Vec2::new(bar_w, bar_h),
                        Sense::click_and_drag(),
                    );

                    let painter = ui.painter();

                    // Track
                    let track_h = 5.0;
                    let track_y = bar_rect.center().y - track_h / 2.0;
                    let track_rect = egui::Rect::from_min_size(
                        egui::pos2(bar_rect.left(), track_y),
                        Vec2::new(bar_rect.width(), track_h),
                    );
                    painter.rect_filled(track_rect, 2.5, C::HOVER);

                    // Played portion
                    let played_w = (bar_rect.width() * progress as f32).max(track_h);
                    let played_rect = egui::Rect::from_min_size(
                        track_rect.left_top(),
                        Vec2::new(played_w, track_h),
                    );
                    painter.rect_filled(played_rect, 2.5, C::PLAYING);

                    // Handle dot
                    let handle_x = bar_rect.left() + played_w;
                    let handle_radius = if bar_resp.hovered() || bar_resp.dragged() {
                        6.0
                    } else {
                        4.5
                    };
                    painter.circle_filled(
                        egui::pos2(handle_x, bar_rect.center().y),
                        handle_radius,
                        if bar_resp.hovered() || bar_resp.dragged() {
                            C::ACCENT
                        } else {
                            C::PLAYING
                        },
                    );

                    // Seek on click / drag
                    if bar_resp.dragged() || bar_resp.clicked() {
                        if let Some(ptr) = bar_resp.interact_pointer_pos() {
                            let frac = ((ptr.x - bar_rect.left()) / bar_rect.width())
                                .clamp(0.0, 1.0);
                            self.player.seek(std::time::Duration::from_secs_f64(
                                frac as f64 * total_secs,
                            ));
                        }
                    }

                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format_dur(total))
                            .size(11.0)
                            .color(C::DIM),
                    );
                    ui.add_space(pad);
                });

                ui.add_space(4.0);

                // Row 3: transport controls (centered)
                ui.horizontal(|ui| {
                    let total_btn_w = 28.0 * 8.0 + 12.0 * 3.0 + 80.0 + 10.0;
                    let pad = (ui.available_width() - total_btn_w).max(8.0) / 2.0;
                    ui.add_space(pad);

                    if icon_btn(ui, "settings", "Settings") {
                        self.show_dir_input = !self.show_dir_input;
                    }
                    ui.add_space(12.0);

                    if icon_btn(ui, "prev", "Previous") {
                        self.play_prev();
                    }

                    let skip = Duration::from_secs(5);
                    if icon_btn(ui, "rewind", "Rewind 5s") {
                        self.player.seek_backward(skip);
                    }

                    let is_playing = self.player.current_path().is_some()
                        && !self.player.is_paused();
                    let play_icon = if is_playing { "pause" } else { "play" };
                    if icon_btn(
                        ui,
                        play_icon,
                        if is_playing { "Pause" } else { "Play" },
                    ) {
                        if is_playing {
                            self.player.pause();
                        } else if self.player.is_paused() {
                            self.player.resume();
                        } else {
                            self.play_selected();
                        }
                    }
                    if icon_btn(ui, "forward", "Forward 5s") {
                        self.player.seek_forward(skip);
                    }
                    if icon_btn(ui, "next", "Next") {
                        self.play_next();
                    }
                    if icon_btn(ui, "stop", "Stop") {
                        self.player.stop();
                    }

                    ui.add_space(10.0);

                    // Play mode toggle
                    let mode_label = match self.play_mode {
                        PlayMode::Normal => ">>",
                        PlayMode::Shuffle => "><",
                        PlayMode::RepeatOne => "R1",
                        PlayMode::RepeatAll => "RA",
                    };
                    let mode_tip = match self.play_mode {
                        PlayMode::Normal => "Normal",
                        PlayMode::Shuffle => "Shuffle",
                        PlayMode::RepeatOne => "Repeat One",
                        PlayMode::RepeatAll => "Repeat All",
                    };
                    let (mr, mode_resp) =
                        ui.allocate_exact_size(Vec2::new(24.0, 28.0), Sense::click());
                    if mode_resp.clicked() {
                        self.cycle_play_mode();
                    }
                    mode_resp.on_hover_text(mode_tip);
                    let mc = if mode_resp.hovered() { C::ACCENT } else { C::DIM };
                    ui.painter().text(
                        mr.center(), Align2::CENTER_CENTER, mode_label,
                        FontId::proportional(11.0), mc,
                    );

                    ui.add_space(8.0);
                    icon_btn(ui, "vol", "Volume");
                    let mut vol = self.player.volume();
                    let vs = ui.add_sized(
                        [80.0, 14.0],
                        egui::Slider::new(&mut vol, 0.0..=1.0).show_value(false));
                    if vs.changed() {
                        self.player.set_volume(vol);
                    }
                });
            });

        // ── Top: header ──
        egui::TopBottomPanel::top("header")
            .exact_height(42.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(14.0);
                    ui.label(
                        RichText::new("SynthPlayer")
                            .size(15.0)
                            .strong()
                            .color(C::ACCENT),
                    );
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add_space(14.0);
                            ui.label(
                                RichText::new(format!("{} tracks", self.tracks.len()))
                                    .size(12.0)
                                    .color(C::DIM),
                            );
                        },
                    );
                });
            });

        // ── Center: track list ──
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_dir_input {
                egui::Frame::new()
                    .fill(C::SURFACE)
                    .inner_margin(Vec2::new(12.0, 6.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Music Dir:").size(12.0).color(C::DIM),
                            );
                            let r = ui.add(
                                egui::TextEdit::singleline(&mut self.music_dir)
                                    .desired_width(ui.available_width() - 160.0),
                            );
                            if r.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                self.scan_music();
                            }
                            if ui.button("Scan").clicked() {
                                self.scan_music();
                            }
                            if ui.button("Close").clicked() {
                                self.show_dir_input = false;
                            }
                        });
                    });
                ui.add_space(4.0);
            }

            // Search bar
            egui::Frame::new()
                .fill(C::SURFACE)
                .inner_margin(Vec2::new(10.0, 6.0))
                .corner_radius(CornerRadius::same(6))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Search").size(12.0).color(C::DIM));
                        let search_resp = ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text("Title or artist...")
                                .desired_width(ui.available_width() - 60.0),
                        );
                        if !self.search_query.is_empty() {
                            if ui.button("Clear").clicked() {
                                self.search_query.clear();
                                search_resp.request_focus();
                            }
                        }
                    });
                });

            ui.add_space(6.0);

            let filtered = self.filtered_indices();
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label(
                    RichText::new(format!("Showing {}/{}", filtered.len(), self.tracks.len()))
                        .size(11.0)
                        .color(C::DIM),
                );
            });

            ui.add_space(2.0);
            ui.painter().line_segment(
                [ui.min_rect().left_top(), ui.min_rect().right_top()],
                Stroke::new(1.0, C::BORDER),
            );

            // Column header
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(
                    RichText::new("FMT").size(10.0).color(C::DIM),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Title").size(10.0).color(C::DIM),
                );
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("Duration").size(10.0).color(C::DIM),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new("Artist").size(10.0).color(C::DIM),
                        );
                    },
                );
            });
            ui.painter().line_segment(
                [ui.min_rect().left_top(), ui.min_rect().right_top()],
                Stroke::new(1.0, C::BORDER),
            );

            let mut action = None;
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(2.0);
                for &i in &filtered {
                    let track = &self.tracks[i];
                    let is_sel = self.selected_index == Some(i);
                    let is_play = self.player.current_path() == Some(&track.path);

                    let h = 26.0;
                    let w = ui.available_width();
                    let (rect, resp) =
                        ui.allocate_exact_size(Vec2::new(w, h), Sense::click());

                    if is_play {
                        ui.painter().rect_filled(rect, 0.0, C::PLAYING_BG);
                    } else if is_sel {
                        ui.painter().rect_filled(rect, 0.0, C::SELECT_BG);
                    } else if resp.hovered() {
                        ui.painter().rect_filled(rect, 0.0, C::HOVER);
                    }

                    if is_play {
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(rect.left_top(), Vec2::new(2.5, h)),
                            0.0,
                            C::PLAYING,
                        );
                    }

                    let tc = if is_play || is_sel {
                        Color32::WHITE
                    } else {
                        C::TEXT
                    };

                    // Format badge
                    let badge_color = fmt_color(&track.format.to_lowercase());
                    let badge_w = track.format.len() as f32 * 6.5 + 10.0;
                    let badge_rect = egui::Rect::from_min_size(
                        rect.left_top() + Vec2::new(10.0, (h - 16.0) / 2.0),
                        Vec2::new(badge_w, 16.0),
                    );
                    ui.painter()
                        .rect_filled(badge_rect, 4.0, badge_color);
                    ui.painter().text(
                        badge_rect.center(),
                        Align2::CENTER_CENTER,
                        &track.format,
                        FontId::proportional(9.0),
                        Color32::WHITE,
                    );

                    // Title
                    let title_x = 10.0 + badge_w + 10.0;
                    ui.painter().text(
                        rect.left_top() + Vec2::new(title_x, h / 2.0),
                        Align2::LEFT_CENTER,
                        &track.title,
                        FontId::proportional(12.5),
                        tc,
                    );

                    // Artist
                    ui.painter().text(
                        rect.right_top() + Vec2::new(-60.0, h / 2.0),
                        Align2::RIGHT_CENTER,
                        &track.artist,
                        FontId::proportional(11.5),
                        C::DIM,
                    );

                    // Duration
                    ui.painter().text(
                        rect.right_top() + Vec2::new(-10.0, h / 2.0),
                        Align2::RIGHT_CENTER,
                        &track.duration_display,
                        FontId::proportional(11.5),
                        C::DIM,
                    );

                    if resp.clicked() {
                        self.selected_index = Some(i);
                    }
                    if resp.double_clicked() {
                        action = Some(i);
                    }
                }
            });

            if let Some(idx) = action {
                self.selected_index = Some(idx);
                self.play_at(idx);
            }
        });

        // ── Keyboard shortcuts ──
        if !ctx.wants_keyboard_input() {
            let skip = Duration::from_secs(5);

            if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                let active = self.player.current_path().is_some() && !self.player.is_paused();
                if active {
                    self.player.pause();
                } else if self.player.is_paused() {
                    self.player.resume();
                } else {
                    self.play_selected();
                }
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                self.player.seek_backward(skip);
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                self.player.seek_forward(skip);
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.selected_index = Some(
                    self.selected_index.map_or(0, |i| i.saturating_sub(1)));
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                let n = self.selected_index
                    .map_or(0, |i| (i + 1).min(self.tracks.len().saturating_sub(1)));
                if !self.tracks.is_empty() {
                    self.selected_index = Some(n);
                }
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.play_selected();
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.player.stop();
            }
        }

        // Persist settings
        if let Some(storage) = frame.storage_mut() {
            storage.set_string("music_dir", self.music_dir.clone());
            storage.set_string("volume", self.player.volume().to_string());
        }

        ctx.request_repaint();
    }
}