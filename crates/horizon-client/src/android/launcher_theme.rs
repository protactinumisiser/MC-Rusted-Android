//! MC-Rusted Launcher's locked warm-orange look for the native Android launcher,
//! plus the round navigation icons. Launcher-only: the game's menus and the
//! on-screen keyboard keep `horizon_theme`'s palette.
use horizon_client::ui_theme::Ui;

/// The launcher's palette, under the same names `ui_theme::color` uses so the
/// launcher modules only change their import.
pub mod color {
    use horizon_client::ui_theme::color::hex;
    /// The page: a deep, warm brown-black.
    pub const BG: u32 = hex(0x17110c, 255);
    /// The bottom bar and sheets above the page.
    pub const SURFACE: u32 = hex(0x21170f, 255);
    /// Secondary buttons, fields and inactive round icons.
    pub const RAISED: u32 = hex(0x2d2018, 255);
    /// Pressed plates and a switch that is off.
    pub const HOVER: u32 = hex(0x43301f, 255);
    /// The selected round icon and other tonal orange plates.
    pub const TONAL: u32 = hex(0x5a3315, 255);
    /// Rules between rows and slider tracks.
    pub const BORDER: u32 = hex(0x4a3524, 255);
    /// Text.
    pub const TEXT: u32 = hex(0xfff1e4, 255);
    /// Secondary text, values and inactive icons.
    pub const MUTED: u32 = hex(0xcdae95, 255);
    /// The warm orange accent: the Play button, progress, selection.
    pub const ACCENT: u32 = hex(0xff8a24, 255);
    /// Text and icons on the accent.
    pub const ON_ACCENT: u32 = hex(0x2b1300, 255);
}

/// The launcher's sections, as round icons.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Play,
    Mods,
    Packs,
    Shaders,
    Files,
    Settings,
    Account,
}

fn disc(ui: &mut Ui, cx: f32, cy: f32, d: f32, c: u32) {
    ui.rounded(cx - d / 2.0, cy - d / 2.0, d, d, d / 2.0, c);
}

/// Draws `icon` centred on (`cx`, `cy`) in a box of `s` dp, using only filled
/// shapes so it needs no icon font or rasterizer.
pub fn draw_icon(ui: &mut Ui, icon: Icon, cx: f32, cy: f32, s: f32, c: u32) {
    let (x0, y0) = (cx - s / 2.0, cy - s / 2.0);
    match icon {
        Icon::Play => {
            // A right-pointing triangle built from 1 dp scanlines.
            let (left, height, width) = (cx - s * 0.32, s * 0.9, s * 0.78);
            let rows = height.ceil().max(2.0) as i32;
            for i in 0..rows {
                let t = (i as f32 + 0.5) / rows as f32;
                let w = (1.0 - (2.0 * t - 1.0).abs()) * width;
                ui.rounded(left, cy - height / 2.0 + i as f32 * (height / rows as f32), w, height / rows as f32 + 0.5, 0.0, c);
            }
        }
        Icon::Mods => {
            // Four rounded tiles.
            let (tile, gap) = (s * 0.42, s * 0.16);
            for (ix, iy) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)] {
                ui.rounded(x0 + ix * (tile + gap), y0 + iy * (tile + gap), tile, tile, s * 0.12, c);
            }
        }
        Icon::Packs => {
            // Three stacked layers.
            let (bar, gap) = (s * 0.24, s * 0.14);
            for i in 0..3 {
                let inset = if i == 1 { 0.0 } else { s * 0.1 };
                ui.rounded(x0 + inset, y0 + s * 0.04 + i as f32 * (bar + gap), s - 2.0 * inset, bar, bar / 2.0, c);
            }
        }
        Icon::Shaders => {
            // A sun: a disc and four rays.
            disc(ui, cx, cy, s * 0.42, c);
            let (len, thick) = (s * 0.2, (s * 0.1).max(2.0));
            ui.rounded(cx - thick / 2.0, y0, thick, len, thick / 2.0, c);
            ui.rounded(cx - thick / 2.0, y0 + s - len, thick, len, thick / 2.0, c);
            ui.rounded(x0, cy - thick / 2.0, len, thick, thick / 2.0, c);
            ui.rounded(x0 + s - len, cy - thick / 2.0, len, thick, thick / 2.0, c);
        }
        Icon::Files => {
            // A folder: tab and body.
            ui.rounded(x0, y0 + s * 0.12, s * 0.44, s * 0.3, s * 0.1, c);
            ui.rounded(x0, y0 + s * 0.26, s, s * 0.62, s * 0.14, c);
        }
        Icon::Settings => {
            // Three sliders: a bar either side of a knob.
            let thick = (s * 0.1).max(2.0);
            for (i, knob) in [0.3_f32, 0.7, 0.45].into_iter().enumerate() {
                let y = y0 + s * (0.2 + 0.3 * i as f32);
                let kx = x0 + s * knob;
                let r = s * 0.15;
                disc(ui, kx, y, r * 2.0, c);
                ui.rounded(x0, y - thick / 2.0, (kx - r - 2.0 - x0).max(0.0), thick, thick / 2.0, c);
                ui.rounded(kx + r + 2.0, y - thick / 2.0, (x0 + s - kx - r - 2.0).max(0.0), thick, thick / 2.0, c);
            }
        }
        Icon::Account => {
            disc(ui, cx, y0 + s * 0.3, s * 0.4, c);
            ui.rounded(x0 + s * 0.1, y0 + s * 0.62, s * 0.8, s * 0.34, s * 0.17, c);
        }
    }
}
