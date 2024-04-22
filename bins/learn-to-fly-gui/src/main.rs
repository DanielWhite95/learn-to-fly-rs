#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::{pos2, Color32, Pos2, Stroke};
use emath::RectTransform;
use rand;
use rand::RngCore;
use lib_simulation::Simulation;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Learn to Fly",
        options,
        Box::new(|_cc| {
            Box::<LearnToFlyApp>::default()
        }),
    )
}

struct LearnToFlyApp {
    rng: Box<dyn RngCore>,
    simulation: Simulation,
    birds: u32,
    food: u32,
}

impl Default for LearnToFlyApp {
    fn default() -> Self {
        let mut rng = Box::new(rand::thread_rng());
        Self {
            rng: rng.clone(),
            simulation: Simulation::random(&mut rng, 10, 15),
            birds: 10,
            food: 15,
        }
    }
}

impl LearnToFlyApp {
    fn place_food(pos: &Pos2, screen_transform: RectTransform ) -> epaint::Shape {
        epaint::Shape::Circle(epaint::CircleShape{ center: screen_transform.transform_pos_clamped(*pos), radius: 5.0, fill: Color32::BLUE, stroke: Stroke::NONE})
    }

    fn place_bird(pos: &Pos2, screen_transform: RectTransform ) -> epaint::Shape {
        let traingle_shape = epaint::PathShape::convex_polygon(
            vec![pos2(pos.x - 0.01, pos.y), pos2(pos.x, pos.y + 0.01), pos2(pos.x + 0.01, pos.y)].iter().map(|&p| screen_transform.transform_pos_clamped(p)).collect(),
            Color32::GREEN,
            Stroke::NONE
        );
        epaint::Shape::Path(traingle_shape)
    }
}

impl eframe::App for LearnToFlyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("config_panel").show(ctx, |ui| {
            ui.heading("Simulation options");
            ui.horizontal(|ui| {
                let bird_label = ui.label("Number of Birds: "); 
                let bird_slider = ui.add(egui::Slider::new(&mut self.birds, 0..=100)).labelled_by(bird_label.id);
                if bird_slider.changed() {
                    self.simulation = Simulation::random(&mut self.rng, self.birds as usize, self.food as usize);
                }
            });
            ui.horizontal(|ui| {
                let food_label = ui.label("Number of Food: "); 
                let food_slider = ui.add(egui::Slider::new(&mut self.food, 0..=50)).labelled_by(food_label.id);
                if food_slider.changed() {
                    self.simulation = Simulation::random(&mut self.rng, self.birds as usize, self.food as usize);
                }
            });
            ui.add_space(10.0);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Learn To Fly Simulation");
            egui::containers::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.ctx().request_repaint();

                let desired_size = egui::vec2(ui.available_width() * 1.0, ui.available_height() * 1.0);
                let (_id, rect) = ui.allocate_space(desired_size);
                
                
                let to_screen =
                    emath::RectTransform::from_to(egui::Rect::from_x_y_ranges(0.0..=1.0, 0.0..=1.0), rect.shrink(10.0));


                let mut shapes = vec![];
                for i in self.simulation.world().animals() {
                    shapes.push(Self::place_bird(&pos2(i.position().x, i.position().y), to_screen ));
                }

                for i in self.simulation.world().food() {
                    shapes.push(Self::place_food(&pos2(i.position().x, i.position().y), to_screen ));
                }

                ui.painter().extend(shapes);

            });
        });
    }
}
