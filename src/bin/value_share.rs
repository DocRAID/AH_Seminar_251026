use eframe::egui;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    let counter1 = Arc::new(Mutex::new(0i32));
    let counter2 = Arc::new(Mutex::new(0i32));

    let c1 = Arc::clone(&counter1);
    let c2 = Arc::clone(&counter2);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1.0, 1.0])
            .with_visible(false),
        ..Default::default()
    };

    eframe::run_native(
        "Manager",
        options,
        Box::new(move |_cc| Ok(Box::new(DualWindowApp::new(c1, c2)))),
    )
}

struct DualWindowApp {
    counter1: Arc<Mutex<i32>>,
    counter2: Arc<Mutex<i32>>,
    windows_created: bool,
}

impl DualWindowApp {
    fn new(counter1: Arc<Mutex<i32>>, counter2: Arc<Mutex<i32>>) -> Self {
        Self {
            counter1,
            counter2,
            windows_created: false,
        }
    }

    fn show_counter_window(
        ctx: &egui::Context,
        my_counter: Arc<Mutex<i32>>,
        other_counter: Arc<Mutex<i32>>,
        my_name: &str,
        other_name: &str,
    ) {
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(my_name);
                ui.add_space(20.0);

                let my_value = *my_counter.lock().unwrap();
                ui.label(egui::RichText::new(format!("my value: {}", my_value)).size(22.0));

                ui.add_space(10.0);

                let other_value = *other_counter.lock().unwrap();
                ui.label(
                    egui::RichText::new(format!("{} value: {}", other_name, other_value))
                        .size(22.0)
                        .color(egui::Color32::GRAY),
                );

                ui.label(
                    egui::RichText::new(format!("total value: {}", other_value + my_value))
                        .size(22.0)
                        .color(egui::Color32::GRAY),
                );

                ui.add_space(10.0);

                if ui
                    .add_sized([150.0, 40.0], egui::Button::new("my value +1"))
                    .clicked()
                {
                    let mut counter = my_counter.lock().unwrap();
                    *counter += 1;
                }

                ui.add_space(10.0);

                if ui
                    .add_sized(
                        [150.0, 40.0],
                        egui::Button::new(format!("{} value +1", other_name)),
                    )
                    .clicked()
                {
                    let mut counter = other_counter.lock().unwrap();
                    *counter += 1;
                }
            });
        });
    }
}

impl eframe::App for DualWindowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        let window1_id = egui::ViewportId::from_hash_of("window1");
        let window2_id = egui::ViewportId::from_hash_of("window2");

        let c1_for_w1 = Arc::clone(&self.counter1);
        let c2_for_w1 = Arc::clone(&self.counter2);
        let c1_for_w2 = Arc::clone(&self.counter1);
        let c2_for_w2 = Arc::clone(&self.counter2);

        if !self.windows_created {
            // Window 1 생성
            ctx.show_viewport_deferred(
                window1_id,
                egui::ViewportBuilder::default()
                    .with_title("window 1")
                    .with_inner_size([350.0, 350.0])
                    .with_position([100.0, 100.0]),
                move |ctx, _class| {
                    Self::show_counter_window(
                        ctx,
                        Arc::clone(&c1_for_w1),
                        Arc::clone(&c2_for_w1),
                        "window 1",
                        "window 2",
                    );
                },
            );

            // Window 2 생성
            ctx.show_viewport_deferred(
                window2_id,
                egui::ViewportBuilder::default()
                    .with_title("window 2")
                    .with_inner_size([350.0, 350.0])
                    .with_position([500.0, 100.0]),
                move |ctx, _class| {
                    Self::show_counter_window(
                        ctx,
                        Arc::clone(&c2_for_w2),
                        Arc::clone(&c1_for_w2),
                        "window 2",
                        "window 1",
                    );
                },
            );

            self.windows_created = true;
        } else {
            // Window 1 업데이트
            ctx.show_viewport_deferred(
                window1_id,
                egui::ViewportBuilder::default(),
                move |ctx, _class| {
                    Self::show_counter_window(
                        ctx,
                        Arc::clone(&c1_for_w1),
                        Arc::clone(&c2_for_w1),
                        "window 1",
                        "window 2",
                    );
                },
            );

            // Window 2 업데이트
            ctx.show_viewport_deferred(
                window2_id,
                egui::ViewportBuilder::default(),
                move |ctx, _class| {
                    Self::show_counter_window(
                        ctx,
                        Arc::clone(&c2_for_w2),
                        Arc::clone(&c1_for_w2),
                        "window 2",
                        "window 1",
                    );
                },
            );
        }
    }
}
