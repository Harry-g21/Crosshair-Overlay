use std::{process, sync::mpsc, thread};
use eframe::{egui, CreationContext};
use egui::{Color32, Pos2, Stroke, Vec2, WindowLevel, ViewportCommand};
use tray_item::{IconSource, TrayItem};

enum TrayMessage {
    Exit,
    ToggleCrosshair,
}

struct CrosshairApp {
    has_enabled_passthrough: bool,
    rx: mpsc::Receiver<TrayMessage>,
    crosshair_visible: bool,
}

impl CrosshairApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        // Create a channel for communication between the tray and the app
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            // Create tray with icon from embedded resource
            let mut tray = match TrayItem::new("Crosshair Overlay", IconSource::Resource("#101")) {
                Ok(tray) => tray,
                Err(e) => {
                    eprintln!("Failed to create tray icon: {:?}", e);
                    return; // Or fallback to another IconSource if you want
                }
            };
            
            let toggle_tx = tx.clone();
            tray.add_menu_item("Toggle Crosshair", move || {
                if let Err(e) = toggle_tx.send(TrayMessage::ToggleCrosshair) {
                    println!("Tray: Failed to send toggle message: {:?}", e);
                }
            }).expect("Failed to add toggle menu item");

            let exit_tx = tx.clone();
            tray.add_menu_item("Exit", move || {
                if let Err(e) = exit_tx.send(TrayMessage::Exit) {
                    println!("Tray: Failed to send exit message: {:?}", e);
                }
            }).expect("Failed to add menu item");


            // Keep the thread alive
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
        
        Self {
            has_enabled_passthrough: false,
            rx,
            crosshair_visible: true
        }
    }
}

impl eframe::App for CrosshairApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0] // Fully transparent background
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Workaround for click passthrough not working on startup
        if !self.has_enabled_passthrough {
            ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true));
            self.has_enabled_passthrough = true;
        } else {
            match self.rx.recv() {
                Ok(TrayMessage::Exit) => {
                    println!("Exit");
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                    process::exit(0);
                },
                Ok(TrayMessage::ToggleCrosshair) => {
                    // Toggle the crosshair visibility
                    self.crosshair_visible = !self.crosshair_visible;
                    println!("Crosshair visibility: {}", self.crosshair_visible);
                    
                }
                _ => {}
            }
        }


        // Create a completely transparent frame
        let ui_frame = egui::Frame::none().fill(Color32::TRANSPARENT);

        // Add a transparent central panel
        egui::CentralPanel::default()
            .frame(ui_frame)
            .show(ctx, |_ui| {});
        
        if self.crosshair_visible {
            let screen_rect = ctx.screen_rect();
            let center = screen_rect.center();
            
            // Use a top-level painter
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("crosshair"),
            ));
            
            let length = 5.0;
            let thickness = 2.0;
            
            // Draw the crosshair
            painter.line_segment(
                [Pos2::new(center.x, center.y - length), Pos2::new(center.x, center.y + length)],
                Stroke::new(thickness, Color32::GREEN),
            );
            
            painter.line_segment(
                [Pos2::new(center.x - length, center.y), Pos2::new(center.x + length, center.y)],
                Stroke::new(thickness, Color32::GREEN),
            );
        }

        ctx.request_repaint();

    }
}

fn main() -> eframe::Result<()> {
    // Calculate window position
    let window_pos = unsafe {
        use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        let screen_width = GetSystemMetrics(SM_CXSCREEN) as f32;
        let screen_height = GetSystemMetrics(SM_CYSCREEN) as f32;
        
        Pos2::new(
            (screen_width - 200.0) / 2.0,
            (screen_height - 200.0) / 2.0,
        )
    };

    let mut native_options = eframe::NativeOptions::default();
    
    // In eframe 0.27, window options are under the viewport field
    native_options.viewport.window_level = Some(WindowLevel::AlwaysOnTop);
    native_options.viewport.transparent = Some(true);
    native_options.viewport.decorations = Some(false);
    native_options.viewport.resizable = Some(false);
    native_options.viewport.inner_size = Some(Vec2::new(200.0, 200.0).into());
    native_options.viewport.position = Some(window_pos);
    native_options.viewport.taskbar = Some(false);


    eframe::run_native(
        "Crosshair Overlay",
        native_options,
        Box::new(|cc| Box::new(CrosshairApp::new(cc))),
    )
}

