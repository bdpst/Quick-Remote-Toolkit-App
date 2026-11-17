use crate::columns::{Columns, DraggedColumn};
use crate::{
    design::{self, Palette},
    models::*,
    services,
};
use eframe::egui::{self, RichText};
use egui_extras::TableBuilder;
const CLIENT_HEADERS: [&str; 8] = [
    "★",
    "№",
    "Компьютер",
    "IP",
    "Сотрудник",
    "Статус",
    "Домен",
    "Последняя проверка",
];
const LOG_HEADERS: [&str; 4] = ["Время", "Компьютер", "Действие", "Результат"];
use std::{
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

pub struct Toolkit {
    settings: Settings,
    clients: Vec<Client>,
    logs: Vec<Log>,
    search: String,
    selected: Option<usize>,
    tx: Sender<Event>,
    rx: Receiver<Event>,
    probing: bool,
    generation: u64,
    window_icon: Option<egui::TextureHandle>,
    csv_loaded_at: String,
    messaging: bool,
    administering: bool,
    message_open: bool,
    message_sized: bool,
    admin_open: bool,
    message: String,
    seconds_text: String,
    dialog_error: String,
    dialog_focus: bool,
    recipient: String,
    settings_writable: bool,
    member: String,
    add: bool,
    client_sort: Option<(usize, bool)>,
    log_sort: Option<(usize, bool)>,
    log_selection: crate::selection::Selection,
    log_focused: bool,
    log_height: f32,
    log_resize_start: Option<LogResize>,
    client_columns: Columns<8>,
    log_columns: Columns<4>,
}
struct LogResize {
    start_height: f32,
    start_y: f32,
    preview_height: f32,
}
impl Toolkit {
    #[cfg(feature = "ui-qa")]
    pub fn qa_state(&self) -> serde_json::Value {
        serde_json::json!({"message_open":self.message_open,"admin_open":self.admin_open,"messaging":self.messaging,"administering":self.administering,"dialog_error":self.dialog_error,"clients":self.clients.len(),"logs":self.logs.len(),"client_columns":self.client_columns.order,"log_columns":self.log_columns.order})
    }
    fn main_body(&mut self, ctx: &egui::Context, p: Palette) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(p.window)
                    .inner_margin(egui::Margin {
                        left: 16,
                        right: 16,
                        top: 0,
                        bottom: 16,
                    }),
            )
            .show(ctx, |ui| {
                let body = ui.max_rect();
                let left_right = (body.right() - 332.).max(body.left());
                let log_height = self.log_height.clamp(0., (body.height() - 22.).max(0.));
                let log_top = body.bottom() - log_height;
                let clients = egui::Rect::from_min_max(
                    body.min,
                    egui::pos2(left_right, (log_top - 22.).max(body.top())),
                );
                let journal = egui::Rect::from_min_max(
                    egui::pos2(body.left(), log_top),
                    egui::pos2(left_right, body.bottom()),
                );
                let sidebar =
                    egui::Rect::from_min_max(egui::pos2(left_right + 12., body.top()), body.max);
                // Same focus traversal order as the source Grid: clients, tools, splitter, journal.
                ui.scope_builder(
                    egui::UiBuilder::new().id_salt("clients").max_rect(clients),
                    |ui| {
                        ui.set_clip_rect(clients);
                        self.client_content(ui, p);
                    },
                );
                ui.scope_builder(
                    egui::UiBuilder::new().id_salt("sidebar").max_rect(sidebar),
                    |ui| {
                        ui.set_clip_rect(sidebar);
                        self.sidebar_content(ui, p);
                    },
                );
                let splitter = egui::Rect::from_min_max(
                    egui::pos2(body.left(), log_top - 16.),
                    egui::pos2(left_right, log_top - 6.),
                );
                let response = ui
                    .interact(
                        splitter,
                        ui.id().with("journal-splitter"),
                        egui::Sense::drag(),
                    )
                    .on_hover_cursor(egui::CursorIcon::ResizeVertical)
                    .on_hover_text("Изменить размер журнала");
                if !self.message_open && !self.admin_open {
                    if response.drag_started()
                        && let Some(origin) = ctx.input(|i| i.pointer.press_origin())
                    {
                        self.log_resize_start = Some(LogResize {
                            start_height: log_height,
                            start_y: origin.y,
                            preview_height: log_height,
                        });
                    }
                    if let Some(resize) = &mut self.log_resize_start {
                        if let Some(pointer) = ctx.input(|i| i.pointer.latest_pos()) {
                            resize.preview_height = (resize.start_height
                                - (pointer.y - resize.start_y))
                                .clamp(0., (body.height() - 22.).max(0.));
                        }
                        let height = resize.preview_height;
                        if response.dragged() {
                            let y = body.bottom() - height - 11.;
                            ui.painter().line_segment(
                                [egui::pos2(body.left(), y), egui::pos2(left_right, y)],
                                egui::Stroke::new(2.0_f32, p.border),
                            );
                        }
                        if response.drag_stopped() {
                            self.log_height = height;
                            self.log_resize_start = None;
                        }
                    }
                }
                ui.scope_builder(
                    egui::UiBuilder::new().id_salt("journal").max_rect(journal),
                    |ui| {
                        ui.set_clip_rect(journal);
                        self.log_content(ui, p);
                    },
                );
            });
    }
    #[cfg(test)]
    fn client_table(&mut self, ctx: &egui::Context, p: Palette) {
        egui::CentralPanel::default().show(ctx, |ui| self.client_content(ui, p));
    }
    #[cfg(feature = "ui-qa")]
    pub fn qa_scene(&mut self, scene: &str, ctx: &egui::Context) {
        self.settings_writable = false;
        self.selected = (!self.clients.is_empty() && scene != "main").then_some(0);
        match scene {
            "message" | "message-error" => {
                self.open_message();
                if scene.ends_with("error") {
                    self.message = "Тестовое сообщение".into();
                    self.seconds_text = "0".into();
                }
            }
            "admin" | "admin-error" => {
                self.open_admin();
                if scene.ends_with("error") {
                    self.member.clear();
                    self.dialog_error.clear();
                }
            }
            "search" => self.search = "PC-DEMO-02".into(),
            "empty" => self.clients.clear(),
            "busy" => {
                self.probing = true;
                for c in &mut self.clients {
                    c.status = "Проверка".into();
                    c.domain = "Проверка…".into();
                    c.last_checked = "16.11.2026 12:34:56".into();
                }
            }
            "reordered" => {
                self.client_columns.move_to(4, 2);
                self.log_columns.move_to(3, 0);
            }
            "logs" => {
                self.logs = (0..20)
                    .map(|i| Log {
                        time: "2026-11-16 12:34:56".into(),
                        target: format!("PC-DEMO-{i:02}"),
                        action: "Ping".into(),
                        result: format!("{i} ms"),
                    })
                    .collect();
                self.log_selection
                    .select(&(0..20).collect::<Vec<_>>(), 0, false, false);
                self.log_selection
                    .select(&(0..20).collect::<Vec<_>>(), 2, false, true);
            }
            "long" => {
                if let Some(c) = self.clients.first_mut() {
                    c.computer = "PC-".to_owned() + &"A".repeat(90);
                    c.person = "Сотрудник с длинным именем ".repeat(6);
                    c.ip = "2001:db8:1234:5678:abcd:ef01:2345:6789".into();
                    c.domain = "EXAMPLE-".to_owned() + &"DOMAIN".repeat(20);
                }
            }
            "context" => {
                ctx.data_mut(|d| d.insert_temp(egui::Id::new("qa-context-menu"), true));
            }
            "collapsed" => {
                for label in ["Подключение", "Инструменты", "Операции"]
                {
                    ctx.data_mut(|d| d.insert_temp(egui::Id::new(("qa-collapse", label)), true));
                }
            }
            _ => {}
        }
    }
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let windows =
            std::path::PathBuf::from(std::env::var_os("WINDIR").unwrap_or("C:\\Windows".into()));
        let mut f = egui::FontDefinitions::default();
        let fallback = f.families[&egui::FontFamily::Proportional].clone();
        for (name, file) in [
            ("segoe", "segoeui.ttf"),
            ("bold", "segoeuib.ttf"),
            ("semibold", "seguisb.ttf"),
            ("icons", "segmdl2.ttf"),
        ] {
            let mut names = fallback.clone();
            if let Ok(bytes) = std::fs::read(windows.join("Fonts").join(file)) {
                f.font_data.insert(
                    name.into(),
                    std::sync::Arc::new(egui::FontData::from_owned(bytes)),
                );
                names.insert(0, name.into());
            }
            f.families
                .insert(egui::FontFamily::Name(name.into()), names);
        }
        f.families.insert(
            egui::FontFamily::Proportional,
            f.families[&egui::FontFamily::Name("segoe".into())].clone(),
        );
        cc.egui_ctx.set_fonts(f);
        let (tx, rx) = mpsc::channel();
        let loaded = services::load_settings();
        let error = loaded.as_ref().err().cloned();
        let settings_writable = loaded.is_ok();
        let mut app = Self {
            settings: loaded.unwrap_or_default(),
            clients: vec![],
            logs: vec![],
            search: String::new(),
            selected: None,
            tx,
            rx,
            probing: false,
            generation: 0,
            window_icon: {
                let icon = crate::application_icon();
                Some(cc.egui_ctx.load_texture(
                    "window-icon",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [icon.width as usize, icon.height as usize],
                        &icon.rgba,
                    ),
                    Default::default(),
                ))
            },
            csv_loaded_at: String::new(),
            messaging: false,
            administering: false,
            message_open: false,
            message_sized: false,
            admin_open: false,
            message: String::new(),
            seconds_text: "300".into(),
            dialog_error: String::new(),
            dialog_focus: false,
            recipient: String::new(),
            settings_writable,
            member: String::new(),
            add: true,
            client_sort: None,
            log_sort: None,
            log_selection: Default::default(),
            log_focused: false,
            log_height: 190.,
            log_resize_start: None,
            client_columns: Columns::new([48., 58., 190., 150., 20., 150., 180., 170.], 4),
            log_columns: Columns::new([155., 180., 230., 20.], 3),
        };
        if let Some(e) = error {
            app.log("Локально", "Настройки", &e);
        }
        app.reload();
        app
    }
    fn poll_events(&mut self) {
        while let Ok(e) = self.rx.try_recv() {
            match e {
                Event::Log(t, a, r) => self.log(&t, &a, &r),
                Event::Error(t, a, r) => {
                    self.log(&t, &a, &r.replace(['\r', '\n'], " "));
                    crate::native::notice(
                        if a == "Сообщение" {
                            "Не удалось отправить сообщение"
                        } else {
                            &a
                        },
                        &r,
                        true,
                    );
                }
                Event::ProbeStarted(generation, i, time) => {
                    if generation != self.generation {
                        continue;
                    }
                    if let Some(c) = self.clients.get_mut(i) {
                        c.status = "Проверка".into();
                        c.domain = "Проверка…".into();
                        c.last_checked = time;
                    }
                }
                Event::Probe(generation, i, s, d) => {
                    if generation != self.generation {
                        continue;
                    }
                    if let Some(c) = self.clients.get_mut(i) {
                        c.status = s;
                        c.domain = d;
                    }
                }
                Event::ProbeDone(generation) => {
                    if generation == self.generation {
                        self.probing = false;
                    }
                }
                Event::Done("message") => self.messaging = false,
                Event::Done("admin") => self.administering = false,
                Event::Done(_) => {}
            }
        }
    }
    fn log(&mut self, target: &str, action: &str, result: &str) {
        self.log_selection.prepend();
        self.logs.insert(
            0,
            Log {
                time: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                target: target.into(),
                action: action.into(),
                result: result.into(),
            },
        );
    }
    fn save(&mut self) -> bool {
        let result = if self.settings_writable {
            services::save_settings(&self.settings)
        } else {
            Err("Исходные настройки повреждены: сохранение запрещено. Исправьте settings.json и перезапустите приложение.".into())
        };
        if let Err(e) = result {
            self.log("", "Сохранение настроек", &e);
            crate::native::notice("Ошибка сохранения настроек", &e, true);
            false
        } else {
            true
        }
    }
    fn reload(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.probing = false;
        self.clients.clear();
        self.csv_loaded_at.clear();
        self.selected = None;
        if self.settings.clients_csv_path.trim().is_empty()
            || !std::path::Path::new(&self.settings.clients_csv_path).is_file()
        {
            self.log("", "Load CSV", "CSV не выбран или файл не найден.");
            return;
        }
        match services::load_csv(&self.settings.clients_csv_path) {
            Ok(c) => {
                self.clients = c;
                self.csv_loaded_at = chrono::Local::now().format("%d.%m.%Y %H:%M").to_string();
                self.selected = None;
                self.log(
                    "",
                    "Load CSV",
                    &format!("Загружено клиентов: {}", self.clients.len()),
                );
            }
            Err(e) => {
                self.log("", "Load CSV", &e);
                crate::native::notice("Ошибка чтения CSV", &e, true);
            }
        }
    }
    fn current(&self) -> Option<Client> {
        self.selected.and_then(|i| self.clients.get(i)).cloned()
    }
    fn run_action(&mut self, index: usize) {
        if self.current().is_none() {
            crate::native::notice(
                concat!("Quick Remote Toolkit ", env!("CARGO_PKG_VERSION")),
                "Выберите клиента.",
                false,
            );
            return;
        }
        if index == 1 {
            self.start_probe_selected();
            return;
        }
        if let Some(c) = self.current() {
            let tx = self.tx.clone();
            std::thread::spawn(move || {
                let event = match services::action(&c, index) {
                    Ok(r) => Event::Log(c.computer, services::ACTIONS[index].into(), r),
                    Err(e) => Event::Error(c.computer, services::ACTIONS[index].into(), e),
                };
                let _ = tx.send(event);
            });
        }
    }
    fn start_probe(&mut self) {
        if self.probing {
            return;
        }
        self.probing = true;
        self.generation = self.generation.wrapping_add(1);
        let generation = self.generation;
        let clients = self.clients.iter().cloned().enumerate().collect();
        let settings = self.settings.clone();
        let tx = self.tx.clone();
        std::thread::spawn(move || services::probe(clients, settings, tx, generation));
    }
    fn start_probe_selected(&mut self) {
        if self.probing {
            return;
        }
        if let Some(index) = self.selected
            && let Some(c) = self.clients.get(index).cloned()
        {
            self.probing = true;
            self.generation = self.generation.wrapping_add(1);
            let generation = self.generation;
            let settings = self.settings.clone();
            let tx = self.tx.clone();
            std::thread::spawn(move || services::probe(vec![(index, c)], settings, tx, generation));
        }
    }
    fn export(&mut self) {
        if let Some(p) = rfd::FileDialog::new()
            .add_filter("Text files (*.txt)", &["txt"])
            .add_filter("CSV files (*.csv)", &["csv"])
            .add_filter("All files (*.*)", &["*"])
            .set_file_name(format!(
                "quick-remote-toolkit-log-{}.txt",
                chrono::Local::now().format("%Y%m%d-%H%M%S")
            ))
            .save_file()
        {
            let csv = p.extension().is_some_and(|s| s.eq_ignore_ascii_case("csv"));
            let text = services::export_logs(&self.logs, csv);
            match std::fs::write(&p, text) {
                Ok(_) => self.log("", "Export logs", &p.display().to_string()),
                Err(e) => {
                    self.log("", "Export logs", &e.to_string());
                    crate::native::notice("Ошибка экспорта журнала", &e.to_string(), true);
                }
            }
        }
    }
    fn open_message(&mut self) {
        if self.current().is_none() {
            crate::native::notice(
                concat!("Quick Remote Toolkit ", env!("CARGO_PKG_VERSION")),
                "Выберите клиента.",
                false,
            );
            return;
        }
        if self.messaging {
            crate::native::notice(
                "Отправить сообщение",
                "Дождитесь завершения текущей отправки.",
                false,
            );
            return;
        }
        if let Some(c) = self.current() {
            self.recipient = c.computer;
            self.message.clear();
            self.seconds_text = "300".into();
            self.dialog_error.clear();
            self.dialog_focus = true;
            self.message_open = true;
            self.message_sized = false;
        }
    }
    fn open_admin(&mut self) {
        if self.current().is_none() {
            crate::native::notice(
                concat!("Quick Remote Toolkit ", env!("CARGO_PKG_VERSION")),
                "Выберите клиента.",
                false,
            );
            return;
        }
        if self.administering {
            self.log(
                "",
                "Локальные администраторы",
                "Дождитесь завершения текущей операции.",
            );
            return;
        }
        if let Some(c) = self.current() {
            self.recipient = c.computer;
            self.member.clear();
            self.add = true;
            self.dialog_error.clear();
            self.dialog_focus = true;
            self.admin_open = true;
        }
    }
    fn dialogs(&mut self, ctx: &egui::Context) {
        let p = Palette::new(self.settings.is_dark_theme);
        let center = ctx.input(|i| i.viewport().outer_rect.map(|r| r.center()));
        if self.message_open {
            let mut builder = egui::ViewportBuilder::default()
                .with_title("Отправить сообщение")
                .with_inner_size([544., 421.])
                .with_min_inner_size([444., 381.])
                .with_decorations(true)
                .with_taskbar(false);
            if let Some(c) = center {
                builder = builder.with_position(c - egui::vec2(280., 230.));
            }
            let mut send = false;
            let mut cancel = false;
            ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("send_message"), builder, |ctx, _| {
                #[cfg(feature="ui-qa")]
                crate::qa::inject_invalid_enter(ctx,"message");
                let keyboard_send=ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter));
                cancel = ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape));
                if !self.message_sized && let Some(insets) = viewport_insets(ctx) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(560., 460.) - insets));
                    ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(egui::vec2(460., 420.) - insets));
                    self.message_sized = true;
                }
                p.apply(ctx, self.settings.is_dark_theme);
                egui::CentralPanel::default().frame(dialog_frame(p)).show(ctx, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.;
                    ui.label(RichText::new(format!("Сообщение на {}", self.recipient)).font(design::font(16., "semibold")));
                    ui.add_space(8.);
                    ui.label(RichText::new("Сообщение получат пользователи выбранного ПК. Нажатие ОК, Отмена или истечение времени появится в журнале. Требуются права просмотра сеансов и отправки сообщений.").color(p.secondary));
                    ui.add_space(12.);
                    let status = if self.dialog_error.is_empty() {"От 1 до 3600 секунд. Ctrl+Enter — отправить."} else {&self.dialog_error};
                    let status_height = ui.painter().layout(status.into(), egui::FontId::proportional(12.), p.secondary, ui.available_width()).size().y;
                    let text_height = (ui.available_height() - 118. - status_height).max(40.);
                    let text = ui.add_sized([ui.available_width(), text_height], egui::TextEdit::multiline(&mut self.message).desired_rows(1).char_limit(2000).margin(egui::vec2(12.,7.)).desired_width(f32::INFINITY));
                    field_name(&text,"Текст сообщения");
                    truncate_utf16(&mut self.message, 2000);
                    if self.dialog_focus {text.request_focus(); ctx.send_viewport_cmd(egui::ViewportCommand::Focus); if ctx.input(|i|i.viewport().focused==Some(true)) {self.dialog_focus = false;}}
                    ui.add_space(12.);
                    let seconds_input = ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 12.;
                        ui.label("Время показа (секунды):");
                        ui.add_sized([90.,46.], egui::TextEdit::singleline(&mut self.seconds_text).char_limit(4).margin(egui::vec2(12.,7.)).vertical_align(egui::Align::Center))
                    }).inner;
                    field_name(&seconds_input,"Время показа в секундах");
                    ui.add_space(8.);
                    ui.label(RichText::new(status).color(p.secondary));
                    ui.add_space(12.);
                    ui.allocate_ui_with_layout(egui::vec2(ui.available_width(),40.), egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        send = design::primary(ui, p, "Отправить", !self.message.trim().is_empty(), egui::vec2(130.,40.)).clicked();
                        cancel |= ui.add_sized([110.,40.], egui::Button::new("Отмена")).clicked();
                    });
                    send |= keyboard_send;
                    if send {
                        if self.message.trim().is_empty() {self.dialog_error="Введите текст сообщения.".into(); text.request_focus(); send=false;}
                        else if !self.seconds_text.parse::<u32>().is_ok_and(|n| (1..=3600).contains(&n)) {self.dialog_error="Укажите время показа от 1 до 3600 секунд.".into(); seconds_input.request_focus(); send=false;}
                    }
                });
                #[cfg(feature="ui-qa")]
                crate::qa::capture(ctx,"message");
            });
            if cancel {
                self.message_open = false;
            } else if send {
                self.message_open = false;
                self.messaging = true;
                let pc = self.recipient.clone();
                let text = self.message.trim().to_owned();
                let seconds = self.seconds_text.parse().unwrap_or(300);
                let tx = self.tx.clone();
                self.log(&pc, "Сообщение", "Отправка…");
                std::thread::spawn(move || {
                    if let Err(e) = crate::native::send_message(&pc, &text, seconds, &tx) {
                        let _ = tx.send(Event::Error(pc, "Сообщение".into(), e));
                    }
                    let _ = tx.send(Event::Done("message"));
                });
            }
        }
        if self.admin_open {
            let mut builder = egui::ViewportBuilder::default()
                .with_title("Локальные администраторы")
                .with_inner_size([544., 410.])
                .with_resizable(false)
                .with_decorations(true)
                .with_taskbar(false);
            if let Some(c) = center {
                builder = builder.with_position(c - egui::vec2(280., 205.));
            }
            let mut apply = false;
            let mut cancel = false;
            ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("administrators"), builder, |ctx, _| {
                #[cfg(feature="ui-qa")]
                crate::qa::inject_invalid_enter(ctx,"admin");
                let keyboard_apply=ctx.input(|i| i.key_pressed(egui::Key::Enter));
                cancel = ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape));
                p.apply(ctx, self.settings.is_dark_theme);
                egui::CentralPanel::default().frame(dialog_frame(p)).show(ctx, |ui| {
                    let mut content_ui = ui.new_child(egui::UiBuilder::new().id_salt("admin-content").max_rect(ui.max_rect()));
                    let ui=&mut content_ui;

                    ui.spacing_mut().item_spacing.y = 0.;
                    ui.set_min_height(0.);
                    ui.label(RichText::new(format!("Локальные администраторы — {}",self.recipient)).font(design::font(16.,"semibold")));
                    ui.add_space(16.); ui.label(r"Пользователь (ДОМЕН\логин или ИМЯ-ПК\логин)"); ui.add_space(8.);
                    let member = ui.add_sized([ui.available_width(),46.], egui::TextEdit::singleline(&mut self.member).char_limit(256).desired_width(f32::INFINITY).margin(egui::vec2(12.,7.)).vertical_align(egui::Align::Center));
                    field_name(&member,"Учётная запись пользователя");
                    truncate_utf16(&mut self.member,256);
                    if self.dialog_focus {member.request_focus(); ctx.send_viewport_cmd(egui::ViewportCommand::Focus); if ctx.input(|i|i.viewport().focused==Some(true)) {self.dialog_focus=false;}}
                    ui.add_space(16.); ui.label("Действие"); ui.add_space(8.);
                    ui.scope(|ui| {
                        ui.spacing_mut().interact_size.y = 16.;
                        ui.spacing_mut().icon_width = 14.;
                        ui.spacing_mut().icon_spacing = 4.;
                        let add_option=ui.radio_value(&mut self.add,true,"Добавить в локальные администраторы");
                        ui.add_space(10.);
                        let remove_option=ui.radio_value(&mut self.add,false,"Удалить из локальных администраторов");
                        if ctx.input(|i|[egui::Key::ArrowUp,egui::Key::ArrowDown,egui::Key::ArrowLeft,egui::Key::ArrowRight].into_iter().any(|k|i.key_pressed(k))) {
                            if add_option.has_focus() {self.add=false;remove_option.request_focus();}
                            else if remove_option.has_focus() {self.add=true;add_option.request_focus();}
                        }
                    });
                    ui.add_space(16.);
                    ui.label(RichText::new("Операция изменит права указанной учётной записи на выбранном ПК. Требуются WinRM и административные права текущего пользователя.").color(p.secondary));
                    ui.add_space(12.);
                    ui.label(RichText::new(&self.dialog_error).color(p.secondary));
                    ui.add_space(12.);
                    ui.allocate_ui_with_layout(egui::vec2(ui.available_width(),40.), egui::Layout::right_to_left(egui::Align::Center),|ui| {
                        apply=design::primary(ui,p,"Выполнить",true,egui::vec2(130.,40.)).clicked();
                        cancel |= ui.add_sized([110.,40.],egui::Button::new("Отмена")).clicked();
                    });
                    apply |= keyboard_apply;
                    if apply && !services::valid_member(&self.member) {self.dialog_error=r"Введите учётную запись в формате ДОМЕН\логин или ИМЯ-ПК\логин.".into(); member.request_focus(); apply=false;}
                    let width = 560. - viewport_insets(ctx).map_or(16., |v|v.x);
                    let desired=egui::vec2(width,ui.min_rect().height()+40.);
                    if ctx.input(|i|i.viewport().inner_rect.is_some_and(|r|(r.size()-desired).length()>1.)) {ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(desired));}
                });
                #[cfg(feature="ui-qa")]
                crate::qa::capture(ctx,"admin");
            });
            if cancel {
                self.admin_open = false;
            } else if apply {
                self.admin_open = false;
                self.administering = true;
                let pc = self.recipient.clone();
                let member = self.member.trim().to_owned();
                let add = self.add;
                let tx = self.tx.clone();
                let action = if add {
                    "Добавление администратора"
                } else {
                    "Удаление администратора"
                };
                self.log(&pc, action, &format!("{member}: выполнение…"));
                std::thread::spawn(move || {
                    let event = match services::administrators(&pc, &member, add) {
                        Ok(r) => Event::Log(pc, action.into(), format!("{member}: {r}")),
                        Err(e) => Event::Error(pc, action.into(), format!("{member}: {e}")),
                    };
                    let _ = tx.send(event);
                    let _ = tx.send(Event::Done("admin"));
                });
            }
        }
    }
}

impl Toolkit {
    fn title_bar(&self, ctx: &egui::Context, p: Palette) {
        egui::TopBottomPanel::top("title_bar")
            .show_separator_line(false)
            .exact_height(32.0)
            .frame(egui::Frame::new().fill(p.card).inner_margin(egui::Margin {
                left: 10,
                right: 0,
                top: 0,
                bottom: 0,
            }))
            .show(ctx, |ui| {
                if self.message_open || self.admin_open {
                    ui.disable();
                }
                let full = ui.max_rect();
                let drag = ui.interact(full, ui.id().with("drag"), egui::Sense::click_and_drag());
                if drag.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                if drag.double_clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(
                        !ctx.input(|i| i.viewport().maximized.unwrap_or(false)),
                    ));
                }
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().interact_size.y = 0.;
                    ui.spacing_mut().button_padding = egui::Vec2::ZERO;
                    if let Some(icon) = &self.window_icon {
                        ui.image((icon.id(), egui::vec2(16., 16.)));
                    }
                    ui.label(concat!("Quick Remote Toolkit ", env!("CARGO_PKG_VERSION")));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 0.;
                        for (glyph, tip, cmd) in [
                            ('\u{e8bb}', "Закрыть", egui::ViewportCommand::Close),
                            (
                                if ctx.input(|i| i.viewport().maximized.unwrap_or(false)) {
                                    '\u{e923}'
                                } else {
                                    '\u{e922}'
                                },
                                if ctx.input(|i| i.viewport().maximized.unwrap_or(false)) {
                                    "Восстановить"
                                } else {
                                    "Развернуть"
                                },
                                egui::ViewportCommand::Maximized(
                                    !ctx.input(|i| i.viewport().maximized.unwrap_or(false)),
                                ),
                            ),
                            (
                                '\u{e921}',
                                "Свернуть",
                                egui::ViewportCommand::Minimized(true),
                            ),
                        ] {
                            let response = ui
                                .scope(|ui| {
                                    let v = ui.visuals_mut();
                                    v.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                                    v.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                                    v.widgets.inactive.corner_radius = 0.into();
                                    v.widgets.hovered.corner_radius = 0.into();
                                    v.widgets.active.corner_radius = 0.into();
                                    if glyph == '\u{e8bb}' {
                                        v.widgets.hovered.weak_bg_fill =
                                            egui::Color32::from_rgb(196, 43, 28);
                                        v.widgets.active.weak_bg_fill =
                                            egui::Color32::from_rgb(164, 38, 29);
                                        v.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;
                                        v.widgets.active.fg_stroke.color = egui::Color32::WHITE;
                                    }
                                    ui.add_sized(
                                        [46., 32.],
                                        egui::Button::new(design::icon(glyph, 10.)).frame(true),
                                    )
                                })
                                .inner;
                            response.widget_info(|| {
                                egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tip)
                            });
                            if response.on_hover_text(tip).clicked() {
                                ctx.send_viewport_cmd(cmd);
                            }
                        }
                    });
                });
            });
    }
    fn toolbar(&mut self, ctx: &egui::Context, p: Palette) {
        egui::TopBottomPanel::top("toolbar")
            .show_separator_line(false)
            .frame(
                egui::Frame::new()
                    .fill(p.window)
                    .inner_margin(egui::Margin {
                        left: 16,
                        right: 16,
                        top: 16,
                        bottom: 12,
                    }),
            )
            .show(ctx, |ui| {
                if self.message_open || self.admin_open {
                    ui.disable();
                }
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.;
                    if let Some(icon) = &self.window_icon {
                        ui.image((icon.id(), egui::vec2(42., 42.)));
                    } else {
                        ui.allocate_exact_size(egui::vec2(42., 42.), egui::Sense::hover());
                    }
                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("Quick Remote Toolkit").font(design::font(22.0, "bold")),
                    );
                    ui.add_space(24.0);
                    let csv_caption = design::caption('\u{e8e5}', "Открыть CSV", p.text, false);
                    let csv_width = ui.painter().layout_job(csv_caption.clone()).size().x + 30.;
                    let refresh_width = ui
                        .painter()
                        .layout_job(design::caption(
                            '\u{e72c}',
                            "Обновить",
                            egui::Color32::WHITE,
                            true,
                        ))
                        .size()
                        .x
                        + 30.;
                    let star_width =
                        (ui.available_width() - csv_width - refresh_width - 70.).max(0.);
                    let width = star_width.min(660.);
                    ui.add_space((star_width - width) / 2.);
                    let response = search_box(ui, p, &mut self.search, width);
                    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::F)) {
                        response.request_focus();
                        if let Some(mut state) = egui::TextEdit::load_state(ctx, response.id) {
                            state
                                .cursor
                                .set_char_range(Some(egui::text::CCursorRange::two(
                                    egui::text::CCursor::new(0),
                                    egui::text::CCursor::new(self.search.chars().count()),
                                )));
                            egui::TextEdit::store_state(ctx, response.id, state);
                        }
                    }
                    ui.add_space((star_width - width) / 2. + 12.);
                    if ui
                        .add_enabled(
                            !self.probing,
                            egui::Button::new(csv_caption).min_size(egui::vec2(csv_width, 46.)),
                        )
                        .clicked()
                        && let Some(path) = rfd::FileDialog::new()
                            .set_title("Выберите CSV клиентов")
                            .add_filter("CSV files (*.csv)", &["csv"])
                            .add_filter("All files (*.*)", &["*"])
                            .pick_file()
                    {
                        self.settings.clients_csv_path = path.display().to_string();
                        self.save();
                        self.reload();
                    }
                    ui.add_space(8.);
                    if design::primary(
                        ui,
                        p,
                        "Обновить",
                        !self.probing,
                        egui::vec2(refresh_width, 46.),
                    )
                    .clicked()
                    {
                        self.reload();
                    }
                    ui.add_space(8.);
                    if ui
                        .add_sized(
                            [42.0, 46.0],
                            egui::Button::new(design::icon(
                                if self.settings.is_dark_theme {
                                    '\u{e706}'
                                } else {
                                    '\u{e708}'
                                },
                                16.0,
                            )),
                        )
                        .on_hover_text(if self.settings.is_dark_theme {
                            "Включить светлую тему"
                        } else {
                            "Включить тёмную тему"
                        })
                        .clicked()
                    {
                        self.settings.is_dark_theme = !self.settings.is_dark_theme;
                        self.save();
                    }
                });
            });
    }
    fn sidebar_content(&mut self, ui: &mut egui::Ui, p: Palette) {
        if self.message_open || self.admin_open {
            ui.disable();
        }
        ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
        ui.spacing_mut().scroll.bar_width = 17.;
        ui.spacing_mut().scroll.bar_inner_margin = 0.;
        ui.spacing_mut().scroll.bar_outer_margin = 0.;
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(ui.available_width());
            let enabled = true;
            design::section(ui, p, '\u{e8af}', "Подключение", |ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    for (a, g, label) in [
                        (0, '\u{e8af}', "Remote Assistance"),
                        (1, '\u{e701}', "Ping"),
                        (2, '\u{e774}', "Tracert"),
                        (3, '\u{e8b7}', "Открыть C$"),
                    ] {
                        if design::tool(ui, p, g, label).clicked() {
                            self.run_action(a);
                        }
                    }
                });
            });
            ui.add_space(4.0);
            design::section(ui, p, '\u{e713}', "Инструменты", |ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    for (a, g, label) in [
                        (4, '\u{e8a5}', "Event Viewer"),
                        (5, '\u{e977}', "Computer Management"),
                        (6, '\u{e7f4}', "MSTSC"),
                        (7, '\u{e756}', "WinRS cmd"),
                    ] {
                        if design::tool(ui, p, g, label).clicked() {
                            self.run_action(a);
                        }
                    }
                    ui.add_enabled_ui(true, |ui| {
                        if design::tool(ui, p, '\u{e8f2}', "Отправить сообщение…").clicked()
                        {
                            self.open_message();
                        }
                    });
                    ui.add_enabled_ui(true, |ui| {
                        if design::tool(ui, p, '\u{ea18}', "Локальные администраторы…").clicked()
                        {
                            self.open_admin();
                        }
                    });
                    if design::tool(ui, p, '\u{e713}', "gpupdate /force").clicked() {
                        self.run_action(8);
                    }
                });
            });
            ui.add_space(4.0);
            design::section(ui, p, '\u{e8fd}', "Операции", |ui| {
                ui.add_enabled_ui(!self.probing && !self.clients.is_empty(), |ui| {
                    if design::tool(ui, p, '\u{e716}', "Проверить всех").clicked() {
                        self.start_probe();
                    }
                });
                if design::tool(ui, p, '\u{e74e}', "Экспорт логов").clicked() {
                    self.export();
                }
                if self.probing {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Проверка компьютеров…");
                    });
                }
            });
            ui.add_space(4.0);
            p.card(14).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(design::icon('\u{e8e5}', 16.0).color(p.accent));
                    ui.label(RichText::new("CSV").font(design::font(14.0, "semibold")));
                });
                ui.add_space(2.0);
                egui::Frame::new()
                    .fill(p.muted)
                    .stroke(egui::Stroke::new(1.0_f32, p.divider))
                    .corner_radius(8)
                    .inner_margin(12)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        let response = ui.add(
                            egui::Label::new(
                                RichText::new(if self.settings.clients_csv_path.is_empty() {
                                    "CSV не выбран или файл не найден."
                                } else {
                                    &self.settings.clients_csv_path
                                })
                                .color(p.secondary),
                            )
                            .wrap()
                            .sense(egui::Sense::CLICK),
                        );
                        if response
                            .on_hover_text("Открыть расположение CSV-файла")
                            .clicked()
                        {
                            let path = &self.settings.clients_csv_path;
                            if std::path::Path::new(path).is_file() {
                                if let Err(e) = crate::native::shell_open(
                                    "explorer.exe",
                                    &format!("/select,\"{path}\""),
                                ) {
                                    self.log("", "Open CSV location", &e);
                                    crate::native::notice(
                                        "Не удалось открыть расположение CSV",
                                        &e,
                                        true,
                                    );
                                }
                            } else {
                                crate::native::notice(
                                    concat!("Quick Remote Toolkit ", env!("CARGO_PKG_VERSION")),
                                    "CSV-файл не выбран или больше не существует.",
                                    false,
                                );
                            }
                        }
                    });
                ui.add_space(4.0);
                ui.label(
                    RichText::new(if self.csv_loaded_at.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "Загружено: {}   Клиентов: {}",
                            self.csv_loaded_at,
                            self.clients.len()
                        )
                    })
                    .size(11.0)
                    .color(p.dim),
                );
            });
        });
    }
    fn log_content(&mut self, ui: &mut egui::Ui, p: Palette) {
        let ctx = ui.ctx().clone();

        let column_order = self.log_columns.order;
        let mut dropped_column = None;

        if self.message_open || self.admin_open {
            ui.disable();
        }
        p.card(10).show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.;
            egui::Frame::new()
                .inner_margin(egui::Margin {
                    left: 4,
                    right: 4,
                    top: 2,
                    bottom: 18,
                })
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(design::icon('\u{e8fd}', 16.0).color(p.accent));
                        ui.label(RichText::new("Журнал").font(design::font(14.0, "semibold")));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .allocate_ui(egui::vec2(142., 34.), |ui| {
                                    design::tool(ui, p, '\0', "Очистить журнал")
                                })
                                .inner
                                .clicked()
                            {
                                self.logs.clear();
                                self.log_selection.clear();
                            }
                        });
                    });
                });
            let mut log_indices: Vec<_> = (0..self.logs.len()).collect();
            if let Some((column, desc)) = self.log_sort {
                log_indices.sort_by(|a, b| {
                    let a = &self.logs[*a];
                    let b = &self.logs[*b];
                    let (a, b) = match column {
                        0 => (&a.time, &b.time),
                        1 => (&a.target, &b.target),
                        2 => (&a.action, &b.action),
                        _ => (&a.result, &b.result),
                    };
                    if desc { b.cmp(a) } else { a.cmp(b) }
                });
            }
            let keyboard_rect = ui.available_rect_before_wrap();
            let table_focus = ui.interact(
                keyboard_rect,
                ui.id().with("table-focus"),
                egui::Sense::focusable_noninteractive(),
            );
            if table_focus.has_focus() {
                self.log_focused = true;
            }
            let mut scroll_to_row = None;
            if self.log_focused
                && !ctx.wants_keyboard_input()
                && keyboard_targets(&ctx, keyboard_rect)
                && !self.message_open
                && !self.admin_open
            {
                let mods = ctx.input(|i| i.modifiers);
                if ctx.input(|i| {
                    i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::ArrowDown)
                }) && !log_indices.is_empty()
                {
                    let pos = self
                        .log_selection
                        .current
                        .and_then(|i| log_indices.iter().position(|v| *v == i));
                    let up = ctx.input(|i| i.key_pressed(egui::Key::ArrowUp));
                    let next = match pos {
                        None => 0,
                        Some(p) if up => p.saturating_sub(1),
                        Some(p) => (p + 1).min(log_indices.len() - 1),
                    };
                    self.log_selection.select(
                        &log_indices,
                        log_indices[next],
                        mods.ctrl,
                        mods.shift,
                    );
                    scroll_to_row = Some(next);
                }
                if mods.ctrl && ctx.input(|i| i.key_pressed(egui::Key::A)) {
                    self.log_selection.rows.extend(&log_indices);
                }
                if ctx.input(|i| i.events.iter().any(|e| matches!(e, egui::Event::Copy))) {
                    let mut text = String::new();
                    for index in &log_indices {
                        if self.log_selection.rows.contains(index) {
                            let l = &self.logs[*index];
                            text.push_str(&clipboard_line(
                                [&display_log_time(&l.time), &l.target, &l.action, &l.result],
                                &column_order,
                            ));
                        }
                    }
                    if !self.log_selection.rows.is_empty() {
                        ctx.copy_text(text);
                    }
                }
            }
            egui::Frame::new()
                .fill(p.card)
                .stroke(egui::Stroke::new(1.0_f32, p.divider))
                .corner_radius(8)
                .inner_margin(1)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
                    ui.spacing_mut().scroll.bar_width = 17.;
                    ui.spacing_mut().scroll.bar_inner_margin = 0.;
                    ui.spacing_mut().scroll.bar_outer_margin = 0.;
                    ui.visuals_mut().widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
                    egui::ScrollArea::horizontal()
                        .id_salt("log_horizontal")
                        .show(ui, |ui| {
                            ui.set_min_width(
                                ui.available_width().max(self.log_columns.minimum_width()),
                            );
                            let mut table = TableBuilder::new(ui)
                                .id_salt("log_table")
                                .min_scrolled_height(0.0)
                                .animate_scrolling(false)
                                .auto_shrink([false, false])
                                .striped(false)
                                .sense(egui::Sense::CLICK)
                                .resizable(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
                            for &column in &column_order {
                                table = table.column(self.log_columns.spec(column));
                            }
                            if std::mem::take(&mut self.log_columns.reset_pending) {
                                table.reset();
                            }
                            if let Some(row) = scroll_to_row {
                                table = table.scroll_to_row(row, None);
                            }
                            table
                                .header(26.0, |mut h| {
                                    for &column in &column_order {
                                        let label = LOG_HEADERS[column];
                                        h.col(|ui| {
                                            let suffix = self
                                                .log_sort
                                                .filter(|(c, _)| *c == column)
                                                .map(|(_, d)| if d { " ▼" } else { " ▲" })
                                                .unwrap_or("");
                                            let response = design::table_header(
                                                ui,
                                                p,
                                                &format!("{label}{suffix}"),
                                            );
                                            response.dnd_set_drag_payload(DraggedColumn {
                                                table: "log",
                                                column,
                                            });
                                            if let Some(payload) =
                                                response.dnd_release_payload::<DraggedColumn>()
                                                && payload.table == "log"
                                            {
                                                dropped_column = Some((payload.column, column));
                                            }
                                            if response.clicked() {
                                                self.log_sort = Some((
                                                    column,
                                                    self.log_sort
                                                        .is_some_and(|(c, d)| c == column && !d),
                                                ));
                                            }
                                        });
                                    }
                                })
                                .body(|body| {
                                    self.log_columns
                                        .remember_widths(&column_order, body.widths());
                                    body.rows(26.0, self.logs.len(), |mut row| {
                                        let row_index_is_odd = row.index() % 2 == 1;
                                        let index = log_indices[row.index()];
                                        row.set_selected(self.log_selection.rows.contains(&index));
                                        let l = &self.logs[index];
                                        let display_time = display_log_time(&l.time);
                                        for &column in &column_order {
                                            let values =
                                                [&display_time, &l.target, &l.action, &l.result];
                                            let text = values[column];
                                            row.col(|ui| {
                                                paint_row_cell(
                                                    ui,
                                                    p,
                                                    self.log_selection.rows.contains(&index),
                                                    row_index_is_odd,
                                                );
                                                if self.log_selection.rows.contains(&index) {
                                                    ui.visuals_mut().override_text_color =
                                                        Some(p.accent);
                                                }
                                                plain_cell(ui, text);
                                            });
                                        }
                                        if row.response().clicked() {
                                            table_focus.request_focus();
                                            let mods = ctx.input(|i| i.modifiers);
                                            self.log_selection.select(
                                                &log_indices,
                                                index,
                                                mods.ctrl,
                                                mods.shift,
                                            );
                                            self.log_focused = true;
                                        }
                                    });
                                });
                        });
                });
        });

        if let Some((from, to)) = dropped_column {
            self.log_columns.move_to(from, to);
        }
    }
    fn client_cell(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        p: Palette,
        c: &Client,
        column: usize,
    ) -> bool {
        let row_palette = p;
        let mut cell_action = false;
        match column {
            0 => {
                ui.add_space(((ui.available_width() - 28.) / 2.).max(0.));
                let favorite = self.settings.favorite(c);
                let response = ui.add_enabled(
                    !c.key().is_empty(),
                    egui::Button::new(
                        design::icon(if favorite { '\u{e735}' } else { '\u{e734}' }, 16.)
                            .color(if favorite { p.favorite } else { p.secondary }),
                    )
                    .frame(false)
                    .min_size(egui::vec2(28., 24.)),
                );
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        !c.key().is_empty(),
                        format!("Избранное: {}", c.computer),
                    )
                });
                if response
                    .on_hover_text(if favorite {
                        "Убрать из избранного"
                    } else {
                        "Добавить в избранное"
                    })
                    .clicked()
                {
                    cell_action = true;
                    let before = self.settings.clone();
                    self.settings.toggle(c);
                    if !self.save() {
                        self.settings = before;
                    }
                }
            }
            1 => {
                plain_cell(ui, &c.number.to_string());
            }
            2..=4 => {
                let value = [&c.computer, &c.ip, &c.person][column - 2];

                if copy_cell(ui, ctx, row_palette, value) {
                    cell_action = true;
                    self.log(
                        &c.computer,
                        "Копирование",
                        "Значение скопировано в буфер обмена.",
                    );
                }
            }
            5 => {
                let color = if c.status == "Доступен" {
                    egui::Color32::from_rgb(22, 163, 74)
                } else if c.status == "Проверка" {
                    egui::Color32::from_rgb(37, 99, 235)
                } else if c.status.is_empty() {
                    egui::Color32::from_rgb(138, 148, 163)
                } else {
                    egui::Color32::from_rgb(220, 38, 38)
                };
                let label = if c.status.is_empty() {
                    "Неизвестно"
                } else {
                    &c.status
                };
                let (r, response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 26.),
                    egui::Sense::hover(),
                );
                ui.painter()
                    .circle_filled(egui::pos2(r.left() + 12.5, r.center().y), 4.5, color);
                ui.painter().text(
                    egui::pos2(r.left() + 26., r.center().y),
                    egui::Align2::LEFT_CENTER,
                    label,
                    egui::FontId::proportional(12.),
                    p.text,
                );
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label)
                });
            }
            _ => {
                let value = if column == 6 {
                    &c.domain
                } else {
                    &c.last_checked
                };

                plain_cell(ui, if value.is_empty() { "—" } else { value });
            }
        }
        cell_action
    }
    fn client_content(&mut self, ui: &mut egui::Ui, p: Palette) {
        let ctx = ui.ctx().clone();

        let column_order = self.client_columns.order;
        let mut dropped_column = None;

        if self.message_open || self.admin_open {
            ui.disable();
        }
        p.card(10).show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.;
            ui.set_min_height((ui.available_height() - 2.0).max(0.0));
            let favorites_changed = egui::Frame::new()
                .inner_margin(egui::Margin {
                    left: 4,
                    right: 4,
                    top: 2,
                    bottom: 10,
                })
                .show(ui, |ui| {
                    ui.spacing_mut().interact_size.y = 16.;
                    ui.spacing_mut().icon_width = 14.;
                    ui.spacing_mut().icon_spacing = 4.;
                    let widgets = &mut ui.visuals_mut().widgets;
                    for w in [
                        &mut widgets.inactive,
                        &mut widgets.hovered,
                        &mut widgets.active,
                    ] {
                        w.corner_radius = 0.into();
                    }
                    ui.checkbox(&mut self.settings.show_favorites_only, "Только избранные")
                        .changed()
                })
                .inner;
            if favorites_changed && !self.save() {
                self.settings.show_favorites_only = !self.settings.show_favorites_only;
            }
            let mut indices: Vec<_> = self
                .clients
                .iter()
                .enumerate()
                .filter(|(_, c)| {
                    (!self.settings.show_favorites_only || self.settings.favorite(c))
                        && c.matches(&self.search)
                })
                .map(|(i, _)| i)
                .collect();
            if self.selected.is_some_and(|i| !indices.contains(&i)) {
                self.selected = None;
            }
            if let Some((column, desc)) = self.client_sort {
                indices.sort_by(|a, b| {
                    let a = &self.clients[*a];
                    let b = &self.clients[*b];
                    let ord = match column {
                        0 => self.settings.favorite(a).cmp(&self.settings.favorite(b)),
                        1 => a.number.cmp(&b.number),
                        2 => a.computer.cmp(&b.computer),
                        3 => a.ip.cmp(&b.ip),
                        4 => a.person.cmp(&b.person),
                        5 => a.status.cmp(&b.status),
                        6 => a.domain.cmp(&b.domain),
                        _ => chrono::NaiveDateTime::parse_from_str(
                            &a.last_checked,
                            "%d.%m.%Y %H:%M:%S",
                        )
                        .ok()
                        .cmp(
                            &chrono::NaiveDateTime::parse_from_str(
                                &b.last_checked,
                                "%d.%m.%Y %H:%M:%S",
                            )
                            .ok(),
                        ),
                    };
                    if desc { ord.reverse() } else { ord }
                });
            }
            let keyboard_rect = ui.available_rect_before_wrap();
            let table_focus = ui.interact(
                keyboard_rect,
                ui.id().with("table-focus"),
                egui::Sense::focusable_noninteractive(),
            );
            if table_focus.has_focus() {
                self.log_focused = false;
            }
            let mut scroll_to_row = None;
            if !self.log_focused
                && !ctx.wants_keyboard_input()
                && keyboard_targets(&ctx, keyboard_rect)
                && !self.message_open
                && !self.admin_open
                && ctx.input(|i| i.events.iter().any(|e| matches!(e, egui::Event::Copy)))
                && let Some(c) = self.current()
            {
                ctx.copy_text(clipboard_line(
                    [
                        "",
                        &c.number.to_string(),
                        &c.computer,
                        &c.ip,
                        &c.person,
                        if c.status.is_empty() {
                            "Неизвестно"
                        } else {
                            &c.status
                        },
                        if c.domain.is_empty() {
                            "—"
                        } else {
                            &c.domain
                        },
                        if c.last_checked.is_empty() {
                            "—"
                        } else {
                            &c.last_checked
                        },
                    ],
                    &column_order,
                ));
            }
            if !self.log_focused
                && !ctx.wants_keyboard_input()
                && keyboard_targets(&ctx, keyboard_rect)
                && !self.message_open
                && !self.admin_open
                && !indices.is_empty()
            {
                let up = ctx.input(|i| i.key_pressed(egui::Key::ArrowUp));
                let down = ctx.input(|i| i.key_pressed(egui::Key::ArrowDown));
                if up || down {
                    let pos = self
                        .selected
                        .and_then(|i| indices.iter().position(|v| *v == i));
                    let next = match pos {
                        None => 0,
                        Some(p) if up => p.saturating_sub(1),
                        Some(p) => (p + 1).min(indices.len() - 1),
                    };
                    self.selected = Some(indices[next]);
                    scroll_to_row = Some(next);
                }
            }
            egui::Frame::new()
                .fill(p.card)
                .stroke(egui::Stroke::new(1.0_f32, p.divider))
                .corner_radius(8)
                .inner_margin(1)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
                    ui.spacing_mut().scroll.bar_width = 17.;
                    ui.spacing_mut().scroll.bar_inner_margin = 0.;
                    ui.spacing_mut().scroll.bar_outer_margin = 0.;
                    ui.visuals_mut().widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
                    egui::ScrollArea::horizontal()
                        .id_salt("clients_horizontal")
                        .show(ui, |ui| {
                            ui.set_min_width(
                                ui.available_width()
                                    .max(self.client_columns.minimum_width()),
                            );
                            let mut table = TableBuilder::new(ui)
                                .id_salt("clients_table")
                                .min_scrolled_height(0.0)
                                .animate_scrolling(false)
                                .auto_shrink([false, false])
                                .sense(egui::Sense::CLICK)
                                .striped(false)
                                .resizable(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
                            for &column in &column_order {
                                table = table.column(self.client_columns.spec(column));
                            }
                            if std::mem::take(&mut self.client_columns.reset_pending) {
                                table.reset();
                            }
                            if let Some(row) = scroll_to_row {
                                table = table.scroll_to_row(row, None);
                            }
                            table
                                .header(26.0, |mut h| {
                                    for &column in &column_order {
                                        let label = CLIENT_HEADERS[column];
                                        h.col(|ui| {
                                            ui.visuals_mut().widgets.hovered.weak_bg_fill =
                                                p.header_hover;
                                            ui.visuals_mut().widgets.hovered.bg_fill =
                                                p.header_hover;
                                            let suffix = self
                                                .client_sort
                                                .filter(|(c, _)| *c == column)
                                                .map(|(_, d)| if d { " ▼" } else { " ▲" })
                                                .unwrap_or("");
                                            let response = design::table_header(
                                                ui,
                                                p,
                                                &format!("{label}{suffix}"),
                                            );
                                            response.dnd_set_drag_payload(DraggedColumn {
                                                table: "client",
                                                column,
                                            });
                                            if let Some(payload) =
                                                response.dnd_release_payload::<DraggedColumn>()
                                                && payload.table == "client"
                                            {
                                                dropped_column = Some((payload.column, column));
                                            }
                                            if response.clicked() {
                                                self.client_sort = Some((
                                                    column,
                                                    self.client_sort
                                                        .is_some_and(|(c, d)| c == column && !d),
                                                ));
                                            }
                                        });
                                    }
                                })
                                .body(|body| {
                                    self.client_columns
                                        .remember_widths(&column_order, body.widths());
                                    body.rows(26.0, indices.len(), |mut row| {
                                        let row_index_is_odd = row.index() % 2 == 1;
                                        let index = indices[row.index()];
                                        let c = self.clients[index].clone();
                                        let mut cell_action = false;
                                        row.set_selected(self.selected == Some(index));
                                        let row_palette = if self.selected == Some(index) {
                                            Palette {
                                                text: p.accent,
                                                ..p
                                            }
                                        } else {
                                            p
                                        };
                                        for &column in &column_order {
                                            row.col(|ui| {
                                                paint_row_cell(
                                                    ui,
                                                    p,
                                                    self.selected == Some(index),
                                                    row_index_is_odd,
                                                );
                                                ui.visuals_mut().override_text_color =
                                                    Some(row_palette.text);
                                                ui.spacing_mut().interact_size.y = 0.;
                                                ui.spacing_mut().button_padding = egui::Vec2::ZERO;
                                                cell_action |= self.client_cell(
                                                    ui,
                                                    &ctx,
                                                    row_palette,
                                                    &c,
                                                    column,
                                                );
                                            });
                                        }
                                        let response = row.response();
                                        if response.clicked() || response.secondary_clicked() {
                                            table_focus.request_focus();
                                            self.selected = Some(index);
                                            self.log_focused = false;
                                        }
                                        if response.double_clicked() && !cell_action {
                                            self.selected = Some(index);
                                            self.run_action(0);
                                        }
                                        let menu = egui::Popup::context_menu(&response)
                                            .width(295.2)
                                            .frame(
                                                egui::Frame::new()
                                                    .fill(p.card)
                                                    .stroke(egui::Stroke::new(1.0_f32, p.border))
                                                    .corner_radius(0)
                                                    .inner_margin(4),
                                            )
                                            .style(move |style: &mut egui::Style| {
                                                style.spacing.item_spacing = egui::Vec2::ZERO;
                                                style.spacing.interact_size.y = 0.;
                                                style.spacing.button_padding = egui::vec2(8., 5.);
                                                style.wrap_mode = Some(egui::TextWrapMode::Extend);
                                                let widgets = &mut style.visuals.widgets;
                                                for w in [
                                                    &mut widgets.inactive,
                                                    &mut widgets.hovered,
                                                    &mut widgets.active,
                                                    &mut widgets.open,
                                                ] {
                                                    w.bg_stroke = egui::Stroke::NONE;
                                                    w.corner_radius = 0.into();
                                                }
                                                widgets.inactive.weak_bg_fill =
                                                    egui::Color32::TRANSPARENT;
                                                widgets.hovered.weak_bg_fill = p.hover;
                                                widgets.active.weak_bg_fill = p.pressed;
                                            });
                                        #[cfg(feature = "ui-qa")]
                                        let menu = if ctx.data(|d| {
                                            d.get_temp::<bool>(egui::Id::new("qa-context-menu"))
                                                .unwrap_or(false)
                                        }) && self.selected == Some(index)
                                            && response.rect.is_finite()
                                        {
                                            menu.open(true).at_position(response.rect.left_bottom())
                                        } else {
                                            menu
                                        };
                                        menu.show(|ui| {
                                            ui.set_min_width(285.2);
                                            for (a, label) in [
                                                "Remote Assistance",
                                                "Ping",
                                                "Tracert",
                                                "Открыть C$",
                                                "Event Viewer",
                                                "Computer Management",
                                                "MSTSC",
                                                "WinRS cmd",
                                                "gpupdate /force",
                                            ]
                                            .iter()
                                            .enumerate()
                                            {
                                                if a == 3 {
                                                    ui.separator();
                                                }
                                                if design::menu_item(ui, p, label).clicked() {
                                                    self.selected = Some(index);
                                                    self.run_action(a);
                                                    ui.close();
                                                }
                                            }
                                            if design::menu_item(ui, p, "Локальные администраторы…")
                                                .clicked()
                                            {
                                                self.selected = Some(index);
                                                self.open_admin();
                                                ui.close();
                                            }
                                            if design::menu_item(ui, p, "Отправить сообщение…")
                                                .clicked()
                                            {
                                                self.selected = Some(index);
                                                self.open_message();
                                                ui.close();
                                            }
                                            ui.separator();
                                            for (label, value, action) in [
                                                ("Копировать имя ПК", &c.computer, "Copy computer"),
                                                ("Копировать IP", &c.ip, "Copy IP"),
                                                (
                                                    "Копировать имя сотрудника",
                                                    &c.person,
                                                    "Copy person",
                                                ),
                                            ] {
                                                if design::menu_item(ui, p, label).clicked() {
                                                    ctx.copy_text(value.clone());
                                                    self.log(&c.computer, action, "Запущено.");
                                                    ui.close();
                                                }
                                            }
                                        });
                                    });
                                });
                        });
                });
        });

        if let Some((from, to)) = dropped_column {
            self.client_columns.move_to(from, to);
        }
    }
}
fn search_box(ui: &mut egui::Ui, p: Palette, text: &mut String, width: f32) -> egui::Response {
    let shortcut =
        ui.painter()
            .layout_no_wrap("Ctrl+F".into(), egui::FontId::proportional(11.), p.dim);
    let badge_size = egui::vec2(
        (shortcut.size().x + 14.).ceil(),
        (shortcut.size().y + 4.).max(20.).ceil(),
    );
    let show_shortcut = width > 100.;
    let right_padding = if show_shortcut {
        (badge_size.x + 20.).ceil() as i8
    } else {
        12
    };
    let response = ui.add_sized(
        [width, 46.],
        egui::TextEdit::singleline(text)
            .hint_text(RichText::new("Поиск компьютера, сотрудника или IP...").color(p.dim))
            .margin(egui::Margin {
                left: 12,
                right: right_padding,
                top: 0,
                bottom: 0,
            })
            .vertical_align(egui::Align::Center),
    );
    if show_shortcut {
        let badge = egui::Rect::from_center_size(
            egui::pos2(
                response.rect.right() - 10. - badge_size.x / 2.,
                response.rect.center().y,
            ),
            badge_size,
        );
        let painter = ui.painter().with_clip_rect(response.rect.shrink(1.));
        painter.rect(
            badge,
            6.,
            p.muted,
            egui::Stroke::new(1.0_f32, p.border),
            egui::StrokeKind::Inside,
        );
        painter.galley(badge.center() - shortcut.size() / 2., shortcut, p.dim);
    }
    response
}
fn display_log_time(time: &str) -> String {
    chrono::NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S")
        .map(|t| t.format("%d.%m.%Y %H:%M:%S").to_string())
        .unwrap_or_else(|_| time.to_owned())
}
fn clipboard_line<const N: usize>(values: [&str; N], order: &[usize; N]) -> String {
    order
        .iter()
        .map(|&i| values[i])
        .collect::<Vec<_>>()
        .join("\t")
        + "\r\n"
}
fn keyboard_targets(ctx: &egui::Context, rect: egui::Rect) -> bool {
    ctx.memory(|m| m.focused()).is_none_or(|id| {
        ctx.read_response(id)
            .is_some_and(|r| rect.intersects(r.rect))
    })
}
fn field_name(response: &egui::Response, label: &str) {
    response
        .ctx
        .accesskit_node_builder(response.id, |node| node.set_label(label));
}
fn dialog_frame(p: Palette) -> egui::Frame {
    egui::Frame::new().fill(p.card).inner_margin(20)
}
fn viewport_insets(ctx: &egui::Context) -> Option<egui::Vec2> {
    ctx.input(|i| Some(i.viewport().outer_rect?.size() - i.viewport().inner_rect?.size()))
}
fn paint_row_cell(ui: &egui::Ui, p: Palette, selected: bool, alternate: bool) {
    let r = ui.max_rect();
    ui.painter().rect_filled(
        r,
        0.,
        if selected {
            p.selected
        } else if alternate {
            p.alternate
        } else {
            p.card
        },
    );
    ui.painter().line_segment(
        [r.left_bottom(), r.right_bottom()],
        egui::Stroke::new(1.0_f32, p.divider),
    );
}
fn plain_cell(ui: &mut egui::Ui, text: &str) {
    ui.add_space(8.0);
    ui.add(egui::Label::new(text).extend());
}
fn copy_cell(ui: &mut egui::Ui, ctx: &egui::Context, p: Palette, value: &str) -> bool {
    let rect = ui.available_rect_before_wrap();
    let size = egui::vec2(rect.width(), 26.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    let text_rect = rect.shrink2(egui::vec2(8.0, 0.0));
    let mut job = egui::text::LayoutJob::simple(
        value.to_owned(),
        egui::FontId::proportional(12.0),
        p.text,
        (rect.width() - 40.0).max(0.0),
    );
    job.wrap.max_rows = 1;
    let galley = ui.painter().layout_job(job);
    let text_y = rect.center().y - galley.size().y / 2.0;
    ui.painter()
        .with_clip_rect(egui::Rect::from_min_max(
            text_rect.min,
            egui::pos2(rect.right() - 32.0, text_rect.bottom()),
        ))
        .galley(egui::pos2(text_rect.left(), text_y), galley, p.text);
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), value));
    response.on_hover_text(value);
    if rect.contains(
        ui.input(|i| i.pointer.hover_pos())
            .unwrap_or(egui::Pos2::ZERO),
    ) && !value.is_empty()
        && value != "-"
    {
        let r = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 16.0, rect.center().y),
            egui::vec2(24.0, 22.0),
        );
        let b = ui.put(
            r,
            egui::Button::new(design::icon('\u{e8c8}', 12.0).color(p.dim)).frame(false),
        );
        b.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                true,
                format!("Копировать {value}"),
            )
        });
        if b.on_hover_text("Копировать").clicked() {
            ctx.copy_text(value.into());
            return true;
        }
    }
    false
}
impl eframe::App for Toolkit {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_events();
        ctx.request_repaint_after(Duration::from_millis(100));
        let p = Palette::new(self.settings.is_dark_theme);
        p.apply(ctx, self.settings.is_dark_theme);
        let icon_theme_id = egui::Id::new("application-icon-theme");
        if ctx.data(|d| d.get_temp::<bool>(icon_theme_id)) != Some(self.settings.is_dark_theme) {
            let icon = crate::icon::themed_icon([p.accent.r(), p.accent.g(), p.accent.b()]);
            if let Some(texture) = &mut self.window_icon {
                texture.set(
                    egui::ColorImage::from_rgba_unmultiplied(
                        [icon.width as usize, icon.height as usize],
                        &icon.rgba,
                    ),
                    Default::default(),
                );
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(std::sync::Arc::new(icon))));
            ctx.data_mut(|d| d.insert_temp(icon_theme_id, self.settings.is_dark_theme));
        }
        self.title_bar(ctx, p);
        self.toolbar(ctx, p);
        self.main_body(ctx, p);
        self.dialogs(ctx);
        crate::native::set_dialog_owner(if self.message_open {
            Some("Отправить сообщение")
        } else if self.admin_open {
            Some("Локальные администраторы")
        } else {
            None
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if self.settings_writable {
            self.save();
        }
    }
}

#[cfg(test)]
mod ui_tests {
    use super::*;
    #[test]
    fn search_text_is_centered_and_shortcut_stays_inside_its_badge() {
        for dark in [false, true] {
            for scale in [1., 1.25, 1.5, 2.] {
                for value in [
                    "",
                    "PC-DEMO",
                    "Длинный поисковый запрос ".repeat(20).as_str(),
                ] {
                    let ctx = context();
                    ctx.set_pixels_per_point(scale);
                    let mut text = value.to_owned();
                    let mut field = egui::Rect::NOTHING;
                    let out = ctx.run(input(vec![]), |ctx| {
                        let p = Palette::new(dark);
                        p.apply(ctx, dark);
                        egui::CentralPanel::default().show(ctx, |ui| {
                            field = search_box(ui, p, &mut text, 660.).rect;
                        });
                    });
                    let texts: Vec<_> = out
                        .shapes
                        .iter()
                        .filter_map(|s| {
                            if let egui::Shape::Text(t) = &s.shape {
                                Some((s.clip_rect, t))
                            } else {
                                None
                            }
                        })
                        .collect();
                    let (_, shortcut) = texts
                        .iter()
                        .find(|(_, t)| t.galley.job.text == "Ctrl+F")
                        .expect("shortcut text");
                    let shortcut_bounds =
                        egui::Rect::from_min_size(shortcut.pos, shortcut.galley.size());
                    let badge = out
                        .shapes
                        .iter()
                        .find_map(|s| {
                            if let egui::Shape::Rect(r) = &s.shape {
                                (r.rect.contains_rect(shortcut_bounds)
                                    && r.rect.width() < 100.
                                    && r.rect.height() <= 25.)
                                    .then_some(r.rect)
                            } else {
                                None
                            }
                        })
                        .expect("badge must fully contain shortcut");
                    assert!(field.contains_rect(badge));
                    assert!((shortcut_bounds.center().y - field.center().y).abs() < 1.);
                    assert!(shortcut_bounds.left() - badge.left() >= 6.);
                    assert!(badge.right() - shortcut_bounds.right() >= 6.);
                    let (clip, editable) = texts
                        .iter()
                        .find(|(_, t)| {
                            t.galley.job.text != "Ctrl+F" && !t.galley.job.text.is_empty()
                        })
                        .expect("search text or placeholder");
                    let editable_bounds =
                        egui::Rect::from_min_size(editable.pos, editable.galley.size());
                    assert!(
                        (editable_bounds.center().y - field.center().y).abs() < 1.,
                        "text must be vertically centered"
                    );
                    assert!(
                        clip.right() <= badge.left() - 5.,
                        "search text must not paint under the shortcut"
                    );
                }
            }
        }
    }
    #[test]
    fn journal_splitter_keeps_preview_until_release_then_commits_total_drag() {
        for dark in [false, true] {
            let ctx = context();
            let mut app = fixture();
            let p = Palette::new(dark);
            let frame = |app: &mut Toolkit, events| {
                let _ = ctx.run(input(events), |ctx| {
                    p.apply(ctx, dark);
                    app.main_body(ctx, p);
                });
            };
            frame(&mut app, vec![]);
            frame(&mut app, vec![]);
            let press = egui::pos2(400., 703.);
            frame(
                &mut app,
                vec![
                    egui::Event::PointerMoved(press),
                    egui::Event::PointerButton {
                        pos: press,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        modifiers: Default::default(),
                    },
                ],
            );
            frame(
                &mut app,
                vec![egui::Event::PointerMoved(egui::pos2(400., 623.))],
            );
            assert!(
                app.log_resize_start.is_some(),
                "splitter must capture the drag"
            );
            assert_eq!(app.log_height, 190., "drag must only show a preview");
            frame(
                &mut app,
                vec![egui::Event::PointerMoved(egui::pos2(400., 583.))],
            );
            assert_eq!(app.log_height, 190.);
            frame(
                &mut app,
                vec![egui::Event::PointerButton {
                    pos: egui::pos2(400., 583.),
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: Default::default(),
                }],
            );
            assert_eq!(
                app.log_height, 310.,
                "commit all 120 DIP, including movement in earlier frames"
            );
            assert!(app.log_resize_start.is_none());
            for (destination, expected) in [(633., 260.), (-1000., 882.), (5000., 0.)] {
                frame(&mut app, vec![]);
                let press = egui::pos2(400., 904. - app.log_height - 11.);
                frame(
                    &mut app,
                    vec![
                        egui::Event::PointerMoved(press),
                        egui::Event::PointerButton {
                            pos: press,
                            button: egui::PointerButton::Primary,
                            pressed: true,
                            modifiers: Default::default(),
                        },
                    ],
                );
                let release = egui::pos2(400., destination);
                frame(&mut app, vec![egui::Event::PointerMoved(release)]);
                frame(
                    &mut app,
                    vec![egui::Event::PointerButton {
                        pos: release,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        modifiers: Default::default(),
                    }],
                );
                assert_eq!(
                    app.log_height, expected,
                    "reverse drag and bounds must work"
                );
                frame(&mut app, vec![]);
            }
        }
    }
    fn input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1500., 920.),
            )),
            events,
            ..Default::default()
        }
    }
    #[test]
    fn copy_uses_visible_column_order_and_selected_log_rows_without_headers() {
        let ctx = context();
        let mut app = fixture();
        app.logs = (0..3)
            .map(|i| Log {
                time: "2026-11-16 12:34:56".into(),
                target: format!("PC-{i}"),
                action: "Ping".into(),
                result: format!("{i} ms"),
            })
            .collect();
        app.log_focused = true;
        app.log_selection.select(&[0, 1, 2], 0, false, false);
        app.log_selection.select(&[0, 1, 2], 2, true, false);
        app.log_columns.move_to(3, 0);
        let out = ctx.run(input(vec![egui::Event::Copy]), |ctx| {
            app.main_body(ctx, Palette::new(false))
        });
        let expected =
            "0 ms\t16.11.2026 12:34:56\tPC-0\tPing\r\n2 ms\t16.11.2026 12:34:56\tPC-2\tPing\r\n";
        assert!(
            out.platform_output
                .commands
                .iter()
                .any(|c| matches!(c,egui::OutputCommand::CopyText(text) if text==expected))
        );
    }
    #[test]
    fn keyboard_navigation_scrolls_to_an_offscreen_selected_client() {
        let ctx = context();
        let mut app = fixture();
        app.client_sort = Some((1, false));
        app.selected = Some(998);
        let _ = ctx.run(
            input(vec![egui::Event::Key {
                key: egui::Key::ArrowDown,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            }]),
            |ctx| app.client_table(ctx, Palette::new(false)),
        );
        let out = ctx.run(input(vec![]), |ctx| {
            app.client_table(ctx, Palette::new(false))
        });
        assert_eq!(app.selected, Some(999));

        assert!(
            out.shapes
                .iter()
                .any(|s| matches!(&s.shape,egui::Shape::Text(t) if t.galley.job.text=="PC-0999"))
        );
    }
    fn context() -> egui::Context {
        let ctx = egui::Context::default();
        let mut fonts = egui::FontDefinitions::default();
        fonts.families.insert(
            egui::FontFamily::Name("icons".into()),
            fonts.families[&egui::FontFamily::Proportional].clone(),
        );
        for name in ["semibold", "bold"] {
            fonts.families.insert(
                egui::FontFamily::Name(name.into()),
                fonts.families[&egui::FontFamily::Proportional].clone(),
            );
        }
        ctx.set_fonts(fonts);
        ctx
    }
    fn fixture() -> Toolkit {
        let (tx, rx) = mpsc::channel();
        Toolkit {
            settings: Settings::default(),
            clients: (0..1000)
                .map(|i| Client {
                    number: i,
                    computer: format!("PC-{i:04}"),
                    ip: format!("192.0.2.{}", i % 255),
                    person: "Тестовый сотрудник".into(),
                    ..Default::default()
                })
                .collect(),
            logs: vec![],
            search: String::new(),
            selected: None,
            tx,
            rx,
            probing: false,
            generation: 0,
            window_icon: None,
            csv_loaded_at: String::new(),
            messaging: false,
            administering: false,
            message_open: false,
            message_sized: false,
            admin_open: false,
            message: String::new(),
            seconds_text: "300".into(),
            dialog_error: String::new(),
            dialog_focus: false,
            recipient: String::new(),
            settings_writable: false,
            member: String::new(),
            add: true,
            client_sort: Some((1, true)),
            log_sort: None,
            log_selection: Default::default(),
            log_focused: false,
            log_height: 190.,
            log_resize_start: None,
            client_columns: Columns::new([48., 58., 190., 150., 20., 150., 180., 170.], 4),
            log_columns: Columns::new([155., 180., 230., 20.], 3),
        }
    }
    #[test]
    fn thousand_row_frames_sort_navigation_and_hidden_selection_in_both_themes() {
        let ctx = context();
        let mut app = fixture();
        app.generation = 7;
        app.probing = true;
        app.tx
            .send(Event::Probe(6, 0, "OLD".into(), "OLD".into()))
            .unwrap();
        app.tx.send(Event::ProbeDone(6)).unwrap();
        app.poll_events();
        assert!(app.clients[0].status.is_empty());
        assert!(app.probing);
        app.tx
            .send(Event::Probe(7, 0, "Доступен".into(), "TEST".into()))
            .unwrap();
        app.tx.send(Event::ProbeDone(7)).unwrap();
        app.poll_events();
        assert_eq!(app.clients[0].status, "Доступен");
        assert!(!app.probing);
        for dark in [false, true] {
            app.settings.is_dark_theme = dark;
            app.search.clear();
            app.selected = None;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1500., 920.),
                )),
                events: vec![egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::default(),
                }],
                ..Default::default()
            };
            let out = ctx.run(input, |ctx| {
                let p = Palette::new(dark);
                p.apply(ctx, dark);
                app.client_table(ctx, p);
            });
            assert_eq!(app.selected, Some(999));
            assert!(!out.shapes.is_empty());
            assert!(out.shapes.len() < 10000, "table must virtualize rows");
            app.search = "PC-0000".into();
            let _ = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1500., 920.),
                    )),
                    ..Default::default()
                },
                |ctx| app.client_table(ctx, Palette::new(dark)),
            );
            assert_eq!(app.selected, None);
            assert!(!app.probing);
            assert_eq!(app.clients.len(), 1000);
        }
    }
}
