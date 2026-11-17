use eframe::egui::{self, Color32, FontFamily, FontId, RichText, Stroke, Vec2};

#[derive(Clone, Copy)]
pub struct Palette {
    pub window: Color32,
    pub card: Color32,
    pub muted: Color32,
    pub text: Color32,
    pub secondary: Color32,
    pub dim: Color32,
    pub border: Color32,
    pub divider: Color32,
    pub accent: Color32,
    pub selected: Color32,
    pub alternate: Color32,
    pub hover: Color32,
    pub favorite: Color32,
    pub hover_border: Color32,
    pub pressed: Color32,
    pub primary_hover: Color32,
    pub primary_pressed: Color32,
    pub header_hover: Color32,
}
fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}
impl Palette {
    // Values mirror ThemeService.cs and App.xaml.
    pub fn new(dark: bool) -> Self {
        let c = if dark {
            [
                0x111827, 0x1F2937, 0x273449, 0xF1F5F9, 0xCBD5E1, 0x94A3B8, 0x475569, 0x344256,
                0x60A5FA, 0x1E4976, 0x243044, 0x30415A, 0xFACC15,
            ]
        } else {
            [
                0xF5F7FB, 0xFFFFFF, 0xF8FAFD, 0x0F172A, 0x475569, 0x7C8798, 0xD8E0EC, 0xE5EAF3,
                0x2563EB, 0xDDEBFF, 0xFBFCFE, 0xF1F6FF, 0xCA8A04,
            ]
        };
        Self {
            window: rgb(c[0]),
            card: rgb(c[1]),
            muted: rgb(c[2]),
            text: rgb(c[3]),
            secondary: rgb(c[4]),
            dim: rgb(c[5]),
            border: rgb(c[6]),
            divider: rgb(c[7]),
            accent: rgb(c[8]),
            selected: rgb(c[9]),
            alternate: rgb(c[10]),
            hover: rgb(c[11]),
            favorite: rgb(c[12]),
            hover_border: rgb(if dark { 0x5B708D } else { 0xBFD0EA }),
            pressed: rgb(if dark { 0x1E3A5F } else { 0xE8F1FF }),
            primary_hover: rgb(if dark { 0x3B82F6 } else { 0x1D4ED8 }),
            primary_pressed: rgb(if dark { 0x1D4ED8 } else { 0x1E40AF }),
            header_hover: rgb(if dark { 0x30415A } else { 0xF1F5FB }),
        }
    }
    pub fn apply(self, ctx: &egui::Context, dark: bool) {
        let mut style = (*ctx.style()).clone();
        style.visuals = if dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        let v = &mut style.visuals;
        v.override_text_color = Some(self.text);
        v.panel_fill = self.window;
        v.window_fill = self.card;
        v.extreme_bg_color = self.card;
        v.faint_bg_color = self.alternate;
        v.selection.bg_fill = self.selected;
        v.selection.stroke = Stroke::new(1.0_f32, self.accent);
        v.window_stroke = Stroke::new(1.0_f32, self.divider);
        v.window_corner_radius = 12.into();
        for w in [&mut v.widgets.inactive, &mut v.widgets.noninteractive] {
            w.bg_fill = self.muted;
            w.weak_bg_fill = self.muted;
            w.bg_stroke = Stroke::new(1.0_f32, self.border);
            w.fg_stroke = Stroke::new(1.0_f32, self.text);
            w.corner_radius = 8.into();
        }
        for w in [
            &mut v.widgets.hovered,
            &mut v.widgets.active,
            &mut v.widgets.open,
        ] {
            w.bg_fill = self.hover;
            w.weak_bg_fill = self.hover;
            w.bg_stroke = Stroke::new(1.0_f32, self.accent);
            w.fg_stroke = Stroke::new(1.0_f32, self.text);
            w.corner_radius = 8.into();
        }
        v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, self.hover_border);
        v.widgets.active.bg_fill = self.pressed;
        v.widgets.active.weak_bg_fill = self.pressed;
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        style.spacing.button_padding = Vec2::new(14.0, 8.0);
        style.spacing.interact_size = Vec2::new(38.0, 34.0);
        style
            .text_styles
            .insert(egui::TextStyle::Body, FontId::proportional(12.0));
        style
            .text_styles
            .insert(egui::TextStyle::Button, FontId::proportional(12.0));
        style
            .text_styles
            .insert(egui::TextStyle::Small, FontId::proportional(11.0));
        style
            .text_styles
            .insert(egui::TextStyle::Heading, FontId::proportional(22.0));
        ctx.set_style(style);
    }
    pub fn card(self, padding: i8) -> egui::Frame {
        egui::Frame::new()
            .fill(self.card)
            .stroke(Stroke::new(1.0_f32, self.divider))
            .corner_radius(12)
            .inner_margin(padding)
    }
}
pub fn icon(glyph: char, size: f32) -> RichText {
    RichText::new(glyph.to_string()).font(FontId::new(size, FontFamily::Name("icons".into())))
}
pub fn tool(ui: &mut egui::Ui, p: Palette, glyph: char, label: &str) -> egui::Response {
    let r = ui.add_sized(
        Vec2::new(ui.available_width(), 34.0),
        egui::Button::new("").frame(false),
    );
    let rect = r.rect;
    r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label));
    let enabled = ui.is_enabled();
    let color = |c: Color32| if enabled { c } else { c.gamma_multiply(0.55) };
    ui.painter().rect(
        rect,
        8.0,
        if r.is_pointer_button_down_on() && enabled {
            p.pressed
        } else if r.hovered() && enabled {
            p.hover
        } else {
            p.muted
        },
        Stroke::new(
            1.0_f32,
            if r.hovered() && enabled {
                p.hover_border
            } else {
                p.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    let center = rect.center().y;
    if glyph != '\0' {
        ui.painter().text(
            egui::pos2(rect.left() + 13.0, center),
            egui::Align2::LEFT_CENTER,
            glyph,
            FontId::new(16.0, FontFamily::Name("icons".into())),
            color(p.accent),
        );
    }
    ui.painter().text(
        egui::pos2(rect.left() + if glyph == '\0' { 12. } else { 39. }, center),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(12.0),
        color(p.text),
    );
    ui.painter().text(
        egui::pos2(rect.right() - 13.0, center),
        egui::Align2::RIGHT_CENTER,
        "\u{e76c}",
        FontId::new(11.0, FontFamily::Name("icons".into())),
        color(p.dim),
    );
    r.on_hover_text(tool_tip(label))
}
pub fn section(
    ui: &mut egui::Ui,
    p: Palette,
    glyph: char,
    label: &str,
    body: impl FnOnce(&mut egui::Ui),
) {
    p.card(14).show(ui, |ui| {
        let id = ui.make_persistent_id(("section", label));
        let mut open = ui.data(|d| d.get_temp::<bool>(id)).unwrap_or(true);
        #[cfg(feature = "ui-qa")]
        if ui.ctx().data(|d| {
            d.get_temp::<bool>(egui::Id::new(("qa-collapse", label)))
                .unwrap_or(false)
        }) {
            open = false;
        }
        let r = ui
            .scope(|ui| {
                let widgets = &mut ui.visuals_mut().widgets;
                for w in [
                    &mut widgets.inactive,
                    &mut widgets.hovered,
                    &mut widgets.active,
                ] {
                    w.weak_bg_fill = Color32::TRANSPARENT;
                    w.bg_stroke = Stroke::NONE;
                }
                ui.spacing_mut().interact_size.y = 0.;
                ui.spacing_mut().button_padding = Vec2::ZERO;
                ui.add_sized(
                    Vec2::new(ui.available_width(), 23.),
                    egui::Button::new("").frame(false),
                )
            })
            .inner;
        if r.clicked() {
            open = !open;
            ui.data_mut(|d| d.insert_temp(id, open));
        }
        r.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::CollapsingHeader,
                ui.is_enabled(),
                open,
                label,
            )
        });
        let y = r.rect.center().y;
        ui.painter().text(
            egui::pos2(r.rect.left(), y),
            egui::Align2::LEFT_CENTER,
            glyph,
            font(15., "icons"),
            p.text,
        );
        ui.painter().text(
            egui::pos2(r.rect.left() + 23., y),
            egui::Align2::LEFT_CENTER,
            label,
            font(14., "semibold"),
            p.text,
        );
        ui.painter().text(
            egui::pos2(r.rect.right(), y),
            egui::Align2::RIGHT_CENTER,
            if open { "\u{e70d}" } else { "\u{e76c}" },
            font(11., "icons"),
            if r.hovered() { p.accent } else { p.dim },
        );
        if open {
            ui.add_space(2.);
            body(ui);
            ui.add_space(0.);
        }
    });
}

pub fn primary(
    ui: &mut egui::Ui,
    p: Palette,
    label: &str,
    enabled: bool,
    size: Vec2,
) -> egui::Response {
    ui.scope(|ui| {
        let v = ui.visuals_mut();
        v.disabled_alpha = 1.;
        v.widgets.inactive.weak_bg_fill = p.accent;
        v.widgets.inactive.bg_fill = p.accent;
        v.widgets.hovered.weak_bg_fill = p.primary_hover;
        v.widgets.hovered.bg_fill = p.primary_hover;
        v.widgets.active.weak_bg_fill = p.primary_pressed;
        v.widgets.active.bg_fill = p.primary_pressed;
        v.override_text_color = Some(Color32::WHITE);
        let content: egui::WidgetText = if label == "Обновить" {
            caption('\u{e72c}', label, Color32::WHITE, true).into()
        } else {
            RichText::new(label).font(font(12., "semibold")).into()
        };
        ui.add_enabled(enabled, egui::Button::new(content).min_size(size))
    })
    .inner
}

pub fn font(size: f32, family: &str) -> FontId {
    FontId::new(size, FontFamily::Name(family.into()))
}
pub fn menu_item(ui: &mut egui::Ui, p: Palette, label: &str) -> egui::Response {
    let mut text = egui::text::LayoutJob::default();
    text.append(
        label,
        37.,
        egui::TextFormat {
            font_id: FontId::proportional(12.),
            color: p.text,
            ..Default::default()
        },
    );
    ui.button(text)
}

pub fn table_header(ui: &mut egui::Ui, p: Palette, text: &str) -> egui::Response {
    let r = ui
        .scope(|ui| {
            ui.spacing_mut().interact_size.y = 0.;
            ui.spacing_mut().button_padding = Vec2::ZERO;
            ui.add_sized(
                [ui.available_width(), ui.available_height()],
                egui::Button::new("")
                    .frame(false)
                    .sense(egui::Sense::CLICK | egui::Sense::DRAG),
            )
        })
        .inner;
    r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), text));
    ui.painter().rect_filled(
        r.rect,
        0.,
        if r.hovered() { p.header_hover } else { p.muted },
    );
    for (a, b) in [
        (r.rect.left_bottom(), r.rect.right_bottom()),
        (r.rect.right_top(), r.rect.right_bottom()),
    ] {
        ui.painter()
            .line_segment([a, b], Stroke::new(1.0_f32, p.divider));
    }
    let (label, arrow) = if let Some(s) = text.strip_suffix(" ▲") {
        (s, Some("▲"))
    } else if let Some(s) = text.strip_suffix(" ▼") {
        (s, Some("▼"))
    } else {
        (text, None)
    };
    ui.painter().text(
        egui::pos2(r.rect.left() + 8., r.rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        font(12., "semibold"),
        p.text,
    );
    if let Some(arrow) = arrow {
        ui.painter().text(
            egui::pos2(r.rect.right() - 8., r.rect.center().y),
            egui::Align2::RIGHT_CENTER,
            arrow,
            FontId::proportional(9.),
            p.secondary,
        );
    }
    r
}

pub fn caption(glyph: char, label: &str, color: Color32, emphasis: bool) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &glyph.to_string(),
        0.,
        egui::TextFormat {
            font_id: font(16., "icons"),
            color,
            ..Default::default()
        },
    );
    job.append(
        label,
        10.,
        egui::TextFormat {
            font_id: if emphasis {
                font(12., "semibold")
            } else {
                FontId::proportional(12.)
            },
            color,
            ..Default::default()
        },
    );
    job
}
fn tool_tip(label: &str) -> &str {
    match label {
        "Очистить журнал" => "Удалить текущие записи журнала из окна",
        "Remote Assistance" => "Открыть Windows Remote Assistance для выбранного клиента",
        "Ping" => "Проверить доступность выбранного клиента",
        "Tracert" => "Открыть трассировку маршрута до выбранного клиента",
        "Открыть C$" => "Открыть административную шару C$ выбранного клиента",
        "Event Viewer" => "Открыть просмотр событий выбранного удаленного ПК",
        "Computer Management" => "Открыть управление компьютером для выбранного клиента",
        "MSTSC" => "Запустить подключение через Remote Desktop",
        "WinRS cmd" => "Открыть удаленную командную строку через WinRS",
        "Отправить сообщение…" => {
            "Отправить сообщение пользователям выбранного ПК"
        }
        "Локальные администраторы…" => {
            "Добавить или удалить пользователя из локальных администраторов выбранного ПК"
        }
        "gpupdate /force" => "Запустить обновление групповых политик на выбранном клиенте",
        "Проверить всех" => "Параллельно проверить ping для всех клиентов из списка",
        "Экспорт логов" => "Сохранить журнал действий в TXT или CSV",
        _ => label,
    }
}
