use std::string::ToString;
use egui::NumExt as _;
use rand::{Rng, thread_rng};
use re_entity_db::TimesPerTimeline;
use re_log_types::{TimeReal, TimeType};
use re_ui::{list_item, UiExt as _};

use re_viewer_context::{Looping, PlayState, TimeControl};

static mut SHOW_ADD_BOOKMARK: bool = false;
static mut ADD_BOOKMARK_NAME: String = String::new();
static mut BOOKMARKS: Vec<(String, f64)> = vec![];
static mut SHOW_BOOKMARKS: bool = false;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct TimeControlUi;

impl TimeControlUi {
    #[allow(clippy::unused_self)]
    pub fn timeline_selector_ui(
        &mut self,
        time_control: &mut TimeControl,
        times_per_timeline: &TimesPerTimeline,
        ui: &mut egui::Ui,
    ) {
        time_control.select_a_valid_timeline(times_per_timeline);

        ui.scope(|ui| {
            ui.spacing_mut().button_padding += egui::Vec2::new(2.0, 0.0);
            ui.visuals_mut().widgets.active.expansion = 0.0;
            ui.visuals_mut().widgets.hovered.expansion = 0.0;
            ui.visuals_mut().widgets.open.expansion = 0.0;

            egui::ComboBox::from_id_salt("timeline")
                .selected_text(time_control.timeline().name().as_str())
                .show_ui(ui, |ui| {
                    for timeline in times_per_timeline.timelines() {
                        if ui
                            .selectable_label(
                                timeline == time_control.timeline(),
                                timeline.name().as_str(),
                            )
                            .clicked()
                        {
                            time_control.set_timeline(*timeline);
                        }
                    }
                })
                .response
                .on_hover_ui(|ui| {
                    list_item::list_item_scope(ui, "tooltip", |ui| {
                        ui.markdown_ui(
                            r"
Select timeline.

Each piece of logged data is associated with one or more timelines.

The logging SDK always creates two timelines for you:
* `log_tick` - a sequence timeline with the sequence number of the log call
* `log_time` - a temporal timeline with the time of the log call

You can also define your own timelines, e.g. for sensor time or camera frame number.
"
                                .trim(),
                        );

                        ui.re_hyperlink(
                            "Full documentation",
                            "https://rerun.io/docs/concepts/timelines",
                        );
                    });
                })
        });
    }

    #[allow(clippy::unused_self)]
    pub fn fps_ui(&mut self, time_control: &mut TimeControl, ui: &mut egui::Ui) {
        if time_control.time_type() == TimeType::Sequence {
            if let Some(mut fps) = time_control.fps() {
                ui.scope(|ui| {
                    ui.spacing_mut().interact_size -= egui::Vec2::new(0., 4.);

                    ui.add(
                        egui::DragValue::new(&mut fps)
                            .suffix(" FPS")
                            .speed(1)
                            .range(0.0..=f32::INFINITY),
                    )
                        .on_hover_text("Frames per second");
                });
                time_control.set_fps(fps);
            }
        }
    }

    pub fn play_pause_ui(
        &mut self,
        time_control: &mut TimeControl,
        times_per_timeline: &TimesPerTimeline,
        ui: &mut egui::Ui,
    ) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0; // from figma
            self.play_button_ui(time_control, ui, times_per_timeline);
            self.follow_button_ui(time_control, ui, times_per_timeline);
            self.pause_button_ui(time_control, ui);
            self.step_time_button_ui(time_control, ui, times_per_timeline);
            self.loop_button_ui(time_control, ui);
            self.add_bookmark_button_ui(time_control, ui, times_per_timeline);
            self.show_bookmarks_button_ui(time_control, ui, times_per_timeline);
        });
    }

    #[allow(clippy::unused_self)]
    fn play_button_ui(
        &mut self,
        time_control: &mut TimeControl,
        ui: &mut egui::Ui,
        times_per_timeline: &TimesPerTimeline,
    ) {
        let is_playing = time_control.play_state() == PlayState::Playing;
        if ui
            .large_button_selected(&re_ui::icons::PLAY, is_playing)
            .on_hover_text(format!("Play.{}", toggle_playback_text(ui.ctx())))
            .clicked()
        {
            time_control.set_play_state(times_per_timeline, PlayState::Playing);
        }
    }

    #[allow(clippy::unused_self)]
    fn add_bookmark_button_ui(
        &mut self,
        time_control: &mut TimeControl,
        ui: &mut egui::Ui,
        _times_per_timeline: &TimesPerTimeline,
    ) {
        // Create a large blue button with "Add Bookmark" text
        let button = egui::Button::new("Add Bookmark")
            .fill(egui::Color32::from_rgb(0, 122, 255)); // Set the background color to blue

        if ui.add(button).on_hover_text("Add bookmark").clicked()
        {
            unsafe { SHOW_ADD_BOOKMARK = true; }
            unsafe { ADD_BOOKMARK_NAME = format!("bookmark {}", thread_rng().gen_range(1..42)); }
        }
        if unsafe { SHOW_ADD_BOOKMARK } {
            // Calculate center position
            let screen_rect = ui.ctx().screen_rect();
            let popup_width = 300.0;
            let popup_height = 100.0;
            let popup_pos = screen_rect.center() - egui::vec2(popup_width / 2.0, popup_height / 2.0);

            egui::Window::new("Bookmark name")
                .collapsible(false)
                .resizable(false)
                // .fixed_size(egui::vec2(popup_width, popup_height))
                // .default_pos(popup_pos)
                .show(ui.ctx(), |ui| {
                    unsafe { ui.text_edit_singleline(&mut ADD_BOOKMARK_NAME); }

                    if ui.button("Save").clicked() {
                        unsafe { SHOW_ADD_BOOKMARK = false; }
                        unsafe { println!("User entered: {}", ADD_BOOKMARK_NAME); }
                        // Handle the collected input here
                        unsafe { BOOKMARKS.push((ADD_BOOKMARK_NAME.clone(), time_control.time().unwrap().as_f64())); }
                    }

                    if ui.button("Cancel").clicked() {
                        unsafe { SHOW_ADD_BOOKMARK = false; }
                    }
                });
        }
    }

    #[allow(clippy::unused_self)]
    fn show_bookmarks_button_ui(
        &mut self,
        time_control: &mut TimeControl,
        ui: &mut egui::Ui,
        _times_per_timeline: &TimesPerTimeline,
    ) {
        let button = egui::Button::new("Add Bookmark")
            .fill(egui::Color32::from_rgb(255, 255, 0));

        if ui.add(button).on_hover_text("Bookmarks").clicked()
        {
            unsafe { SHOW_BOOKMARKS = true; }
        }
        if unsafe { SHOW_BOOKMARKS } {
            // Calculate center position
            let screen_rect = ui.ctx().screen_rect();
            let popup_width = 300.0;
            let popup_height = 100.0;
            let popup_pos = screen_rect.center() - egui::vec2(popup_width / 2.0, popup_height / 2.0);

            egui::Window::new("Bookmarks")
                .collapsible(false)
                .resizable(false)
                .fixed_size(egui::vec2(200.0, 150.0))
                .show(ui.ctx(), |ui| {
                    unsafe {
                        for (name, ts) in BOOKMARKS.iter() {
                            if ui.button(name.clone()).clicked() {
                                println!("Selected item: {}", name);
                                // SHOW_BOOKMARKS = false;
                                time_control.set_time(TimeReal::from(*ts))
                            }
                        }
                    }

                    if ui.button("Close").clicked() {
                        unsafe { SHOW_BOOKMARKS = false; }
                    }
                });
        }
    }

    #[allow(clippy::unused_self)]
    fn follow_button_ui(
        &mut self,
        time_control: &mut TimeControl,
        ui: &mut egui::Ui,
        times_per_timeline: &TimesPerTimeline,
    ) {
        let is_following = time_control.play_state() == PlayState::Following;
        if ui
            .large_button_selected(&re_ui::icons::FOLLOW, is_following)
            .on_hover_text(format!(
                "Follow latest data.{}",
                toggle_playback_text(ui.ctx())
            ))
            .clicked()
        {
            time_control.set_play_state(times_per_timeline, PlayState::Following);
        }
    }

    #[allow(clippy::unused_self)]
    fn pause_button_ui(&mut self, time_control: &mut TimeControl, ui: &mut egui::Ui) {
        let is_paused = time_control.play_state() == PlayState::Paused;
        if ui
            .large_button_selected(&re_ui::icons::PAUSE, is_paused)
            .on_hover_text(format!("Pause.{}", toggle_playback_text(ui.ctx())))
            .clicked()
        {
            time_control.pause();
        }
    }

    #[allow(clippy::unused_self)]
    fn step_time_button_ui(
        &mut self,
        time_control: &mut TimeControl,
        ui: &mut egui::Ui,
        times_per_timeline: &TimesPerTimeline,
    ) {
        if ui
            .large_button(&re_ui::icons::ARROW_LEFT)
            .on_hover_text("Step back to previous time with any new data (left arrow)")
            .clicked()
        {
            time_control.step_time_back(times_per_timeline);
        }

        if ui
            .large_button(&re_ui::icons::ARROW_RIGHT)
            .on_hover_text("Step forwards to next time with any new data (right arrow)")
            .clicked()
        {
            time_control.step_time_fwd(times_per_timeline);
        }
    }

    #[allow(clippy::unused_self)]
    fn loop_button_ui(&mut self, time_control: &mut TimeControl, ui: &mut egui::Ui) {
        let icon = &re_ui::icons::LOOP;

        ui.scope(|ui| {
            // Loop-button cycles between states:
            match time_control.looping() {
                Looping::Off => {
                    if ui
                        .large_button_selected(icon, false)
                        .on_hover_text("Looping is off")
                        .clicked()
                    {
                        time_control.set_looping(Looping::All);
                    }
                }
                Looping::All => {
                    ui.visuals_mut().selection.bg_fill =
                        re_ui::DesignTokens::loop_everything_color();
                    if ui
                        .large_button_selected(icon, true)
                        .on_hover_text("Looping entire recording")
                        .clicked()
                    {
                        time_control.set_looping(Looping::Selection);
                    }
                }
                Looping::Selection => {
                    // ui.visuals_mut().selection.bg_fill = re_ui::ReUi::loop_selection_color(); // we have one color for the button, and a slightly different shade of it for the actual selection :/
                    #[allow(clippy::collapsible_else_if)]
                    if ui
                        .large_button_selected(icon, true)
                        .on_hover_text("Looping selection")
                        .clicked()
                    {
                        time_control.set_looping(Looping::Off);
                    }
                }
            }
        });
    }

    #[allow(clippy::unused_self)]
    pub fn playback_speed_ui(&mut self, time_control: &mut TimeControl, ui: &mut egui::Ui) {
        let mut speed = time_control.speed();
        let drag_speed = (speed * 0.02).at_least(0.01);
        ui.scope(|ui| {
            ui.spacing_mut().interact_size -= egui::Vec2::new(0., 4.);
            ui.add(
                egui::DragValue::new(&mut speed)
                    .speed(drag_speed)
                    .suffix("x"),
            )
                .on_hover_text("Playback speed");
        });

        time_control.set_speed(speed);
    }
}

fn toggle_playback_text(egui_ctx: &egui::Context) -> String {
    if let Some(shortcut) = re_ui::UICommand::PlaybackTogglePlayPause.kb_shortcut() {
        format!(" Toggle with {}", egui_ctx.format_shortcut(&shortcut))
    } else {
        Default::default()
    }
}
