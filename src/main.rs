mod core;
mod ui;

fn screen_size() -> (f32, f32) {
    #[cfg(target_os = "windows")]
    {
        unsafe extern "system" {
            fn GetSystemMetrics(idx: i32) -> i32;
        }
        let w = unsafe { GetSystemMetrics(0) }; // SM_CXSCREEN
        let h = unsafe { GetSystemMetrics(1) }; // SM_CYSCREEN
        (w as f32, h as f32)
    }
    #[cfg(not(target_os = "windows"))]
    {
        (1920.0, 1080.0)
    }
}

fn app_icon() -> egui::IconData {
    let size = 64;
    let mut rgba = vec![0u8; size * size * 4];

    // Dark background circle
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let r = size as f32 / 2.0 - 2.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            let idx = (y * size + x) * 4;

            if dist <= r {
                // Background: dark blue-grey
                rgba[idx] = 18;
                rgba[idx + 1] = 22;
                rgba[idx + 2] = 36;
                rgba[idx + 3] = 255;

                // Play triangle (pointing right)
                let tx = cx - 6.0;
                let ty = cy - 14.0;
                let tri = point_in_triangle(
                    x as f32, y as f32,
                    tx, ty,
                    tx, cy + 14.0,
                    cx + 14.0, cy,
                );
                if tri {
                    // Accent blue
                    rgba[idx] = 72;
                    rgba[idx + 1] = 170;
                    rgba[idx + 2] = 255;
                }
            } else if dist <= r + 1.5 {
                // Anti-aliased edge
                let alpha = (r + 1.5 - dist) / 1.5 * 255.0;
                rgba[idx] = 18;
                rgba[idx + 1] = 22;
                rgba[idx + 2] = 36;
                rgba[idx + 3] = alpha as u8;
            }
        }
    }

    egui::IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}

fn point_in_triangle(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> bool {
    let d1 = (px - x2) * (y1 - y2) - (x1 - x2) * (py - y2);
    let d2 = (px - x3) * (y2 - y3) - (x2 - x3) * (py - y3);
    let d3 = (px - x1) * (y3 - y1) - (x3 - x1) * (py - y1);
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    !(has_neg && has_pos)
}

fn main() -> eframe::Result {
    let (sw, sh) = screen_size();
    let w = (sw * 0.48).min(sh * 0.7).max(700.0);
    let h = w * 0.667;

    let icon = std::sync::Arc::new(app_icon());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([w, h])
            .with_title("SynthPlayer")
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "SynthPlayer",
        options,
        Box::new(|cc| Ok(Box::new(ui::app::SynthPlayerApp::new(cc)))),
    )
}
