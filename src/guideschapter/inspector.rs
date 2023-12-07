use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiPlugin, EguiContext};
use bevy_inspector_egui::bevy_inspector;
use bevy_window::PrimaryWindow;


// PLUGIN
pub struct CustomInspectorPlugin;

impl Plugin for CustomInspectorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_systems(
            Update,
            inspector_ui
        );
    }
}

fn inspector_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

        bevy_inspector_egui::bevy_egui::egui::Window::new("Inspector").show(egui_context.get_mut(), |ui| {
            bevy_inspector_egui::bevy_egui::egui::ScrollArea::vertical().show(ui, |ui| {
            
            // equivalent to `WorldInspectorPlugin`
              // bevy_inspector::ui_for_world(world, ui);
             


            // materials
              // bevy_inspector_egui::bevy_egui::egui::CollapsingHeader::new("Materials").show(ui, |ui| {
              //     bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
              // });

            ui.heading("Entities");
            bevy_inspector::ui_for_world_entities(world, ui);

            ui.heading("Reflect variable");
            // works with any `Reflect` value, including `Handle`s
            let mut any_reflect_value: i32 = 5;
            bevy_inspector::ui_for_value(&mut any_reflect_value, ui, world);
        });
    });
}