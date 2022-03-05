/*
 * gerb
 *
 * Copyright 2022 - Manos Pitsidianakis
 *
 * This file is part of gerb.
 *
 * gerb is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * gerb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with gerb. If not, see <http://www.gnu.org/licenses/>.
 */

use glib::{clone, ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
use gtk::cairo::{Context, FontSlant, FontWeight};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::glyphs::{Glyph, GlyphDrawingOptions, GlyphKind};
use crate::project::Project;
use crate::unicode::blocks::*;

const GLYPH_BOX_WIDTH: f64 = 110.;
const GLYPH_BOX_HEIGHT: f64 = 140.;

#[derive(Debug, Default)]
pub struct GlyphsArea {
    app: OnceCell<gtk::Application>,
    project: OnceCell<Arc<Mutex<Option<Project>>>>,
    grid: OnceCell<gtk::Grid>,
    cols: Cell<u32>,
    hide_empty: Cell<bool>,
    zoom_factor: Cell<f64>,
    filter_input: RefCell<Option<String>>,
    widgets: OnceCell<Vec<GlyphBoxItem>>,
}

#[glib::object_subclass]
impl ObjectSubclass for GlyphsArea {
    const NAME: &'static str = "GlyphsArea";
    type Type = GlyphsOverview;
    type ParentType = gtk::EventBox;
}

impl ObjectImpl for GlyphsArea {
    // Here we are overriding the glib::Object::contructed
    // method. Its what gets called when we create our Object
    // and where we can initialize things.
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        *self.filter_input.borrow_mut() = None;
        self.cols.set(4);

        let grid = gtk::Grid::builder()
            .expand(true)
            .visible(true)
            .can_focus(true)
            .column_spacing(5)
            .row_spacing(5)
            .build();

        let overlay = gtk::Overlay::builder()
            .expand(true)
            .visible(true)
            .can_focus(true)
            .build();

        let scrolled_window = gtk::ScrolledWindow::builder()
            .expand(true)
            .visible(true)
            .can_focus(true)
            .margin_top(5)
            .margin_start(5)
            .build();

        scrolled_window.set_child(Some(&grid));
        scrolled_window.connect_size_allocate(clone!(@weak obj => move |_scrolled_window, rect| {
            let mut new_cols = rect.width() as u32 / ((obj.imp().zoom_factor.get()) * GLYPH_BOX_WIDTH) as u32;
            if new_cols < 4 {
                new_cols = 4;
            }
            let prev_cols = obj.imp().cols.get();
            if new_cols != prev_cols {
                //println!("grid resized: {} -> {}", prev_cols, new_cols);
                obj.imp().cols.set(new_cols.saturating_sub(1));
                obj.update_grid();
            }
        }));

        let tool_palette = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        tool_palette.set_expand(false);
        tool_palette.set_halign(gtk::Align::End);
        tool_palette.set_valign(gtk::Align::End);
        tool_palette.set_spacing(5);
        tool_palette.set_visible(true);
        tool_palette.set_can_focus(true);
        /*
        let zoom_scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 2.0, 0.05);
        zoom_scale.set_value(1.0);
        zoom_scale.connect_value_changed(clone!(@weak obj => move |_self| {
            let value = _self.value();
            std::dbg!(value);

            let imp = obj.imp();
            imp.zoom_factor.set(value);
            obj.update_grid();
        }));
        let zoom_scale = gtk::ToolItem::builder()
            .expand(true)
            .visible(true)
            .child(&zoom_scale)
            .build();
        */
        let hide_empty_button = gtk::CheckButton::builder()
            .label("Hide empty glyphs")
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Start)
            .visible(true)
            .active(false)
            .build();
        hide_empty_button.connect_toggled(clone!(@weak obj => move |_| {
            let imp = obj.imp();
            let hide_empty = !imp.hide_empty.get();
            imp.hide_empty.set(hide_empty);
            obj.update_grid();
            imp.grid.get().unwrap().queue_draw();
        }));

        tool_palette.pack_start(&hide_empty_button, false, false, 0);

        let add_glyph_button = gtk::Button::builder()
            .label("Add glyph")
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Start)
            .visible(true)
            .build();

        add_glyph_button.connect_clicked(clone!(@weak obj => move |_| {
        }));

        tool_palette.pack_start(&add_glyph_button, false, false, 0);

        let search_entry = gtk::Entry::builder()
            .expand(true)
            .visible(true)
            .placeholder_text("Filter glyph name")
            .build();

        search_entry.connect_changed(clone!(@weak obj => move |_self| {
            let imp = obj.imp();
            let filter_input = if _self.buffer().length() == 0 {
                None
            } else {
                Some(_self.buffer().text())
            };
            let mut cur_input = imp.filter_input.borrow_mut();
            if cur_input.as_ref() != filter_input.as_ref() {
                *cur_input = filter_input;
                drop(cur_input);
                obj.update_grid();
                imp.grid.get().unwrap().queue_draw();
            }
        }));

        tool_palette.pack_start(&search_entry, false, false, 0);

        /*let filter_pop = gtk::Popover::builder()
            .expand(true)
            .visible(true)
            .modal(true)
            .child(&tree)
            .build();
        tool_palette.pack_start(&filter_pop, false, false, 0);
        */

        tool_palette
            .style_context()
            .add_class("glyphs_area_toolbar");

        overlay.set_child(Some(&scrolled_window));
        overlay.add_overlay(
            &gtk::Expander::builder()
                .child(&tool_palette)
                .expanded(true)
                .visible(true)
                .can_focus(true)
                .tooltip_text("Overview tools")
                .halign(gtk::Align::End)
                .valign(gtk::Align::End)
                .build(),
        );
        obj.set_child(Some(&overlay));
        self.hide_empty.set(false);
        self.zoom_factor.set(1.0);

        self.grid
            .set(grid)
            .expect("Failed to initialize window state");
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: once_cell::sync::Lazy<Vec<ParamSpec>> =
            once_cell::sync::Lazy::new(|| {
                vec![
                    ParamSpecString::new(
                        // Name
                        "tab-title",
                        // Nickname
                        "tab-title",
                        // Short description
                        "tab-title",
                        // Default value
                        Some("overview"),
                        // The property can be read and written to
                        ParamFlags::READABLE,
                    ),
                    ParamSpecBoolean::new(
                        "tab-can-close",
                        "tab-can-close",
                        "tab-can-close",
                        false,
                        ParamFlags::READABLE,
                    ),
                    ParamSpecString::new("filter", "filter", "filter", None, ParamFlags::READWRITE),
                ]
            });
        PROPERTIES.as_ref()
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "tab-title" => "overview".to_value(),
            "tab-can-close" => false.to_value(),
            _ => unreachable!(),
        }
    }
}

impl WidgetImpl for GlyphsArea {}
impl ContainerImpl for GlyphsArea {}
impl BinImpl for GlyphsArea {}
impl EventBoxImpl for GlyphsArea {}

glib::wrapper! {
    pub struct GlyphsOverview(ObjectSubclass<GlyphsArea>)
        @extends gtk::Widget, gtk::Container, gtk::EventBox;
}

impl GlyphsOverview {
    pub fn new(app: gtk::Application, project: Arc<Mutex<Option<Project>>>) -> Self {
        let ret: Self = glib::Object::new(&[]).expect("Failed to create Main Window");
        let (mut col, mut row) = (0, 0);
        let grid = ret.imp().grid.get().unwrap();
        let project_copy = project.clone();
        let mut widgets = vec![];
        if let Some(p) = project.lock().unwrap().as_ref() {
            let mut glyphs = p.glyphs.values().collect::<Vec<&Rc<RefCell<Glyph>>>>();
            glyphs.sort();
            for glyph in glyphs {
                let glyph_box = GlyphBoxItem::new(app.clone(), project_copy.clone(), glyph.clone());
                grid.attach(&glyph_box, col, row, 1, 1);
                widgets.push(glyph_box);
                col += 1;
                if col == 4 {
                    col = 0;
                    row += 1;
                }
            }
        }
        ret.imp().app.set(app).unwrap();
        ret.imp().widgets.set(widgets).unwrap();
        ret.imp().project.set(project).unwrap();
        ret
    }

    fn update_grid(&self) {
        let hide_empty: bool = self.imp().hide_empty.get();
        let zoom_factor: f64 = self.imp().zoom_factor.get();
        let filter_input = self.imp().filter_input.borrow();
        let filter_input_uppercase: Option<String> =
            filter_input.as_ref().map(|s| s.to_ascii_uppercase());
        let filter_input_char: Option<char> = filter_input.as_ref().and_then(|s| {
            let mut iter = s.chars();
            let first = iter.next()?;
            if iter.next().is_some() {
                None
            } else {
                Some(first)
            }
        });
        let (mut col, mut row) = (0, 0);
        let grid = self.imp().grid.get().unwrap();
        let max_cols = self.imp().cols.get() as i32;
        let children = grid.children();
        for c in children {
            grid.remove(&c);
        }
        for c in self.imp().widgets.get().unwrap() {
            c.set_height_request((zoom_factor * GLYPH_BOX_HEIGHT) as i32);
            c.set_width_request((zoom_factor * GLYPH_BOX_WIDTH) as i32);
            //c.queue_draw();
            c.imp().zoom_factor.set(zoom_factor);
            let glyph = c.imp().glyph.get().unwrap().borrow();
            if hide_empty && glyph.is_empty() {
                continue;
            }
            if let Some(fu) = filter_input_uppercase.as_ref() {
                if !(glyph
                    .name2
                    .as_ref()
                    .map(|n| n.contains(fu.as_str()))
                    .unwrap_or(false)
                    || filter_input_char
                        .as_ref()
                        .map(|c| glyph.kind == GlyphKind::Char(*c))
                        .unwrap_or(false))
                {
                    continue;
                }
            }
            grid.attach(c, col, row, 1, 1);
            col += 1;
            if col == max_cols {
                col = 0;
                row += 1;
            }
        }
        grid.queue_draw();
    }
}

#[derive(Debug, Default)]
pub struct GlyphBox {
    pub app: OnceCell<gtk::Application>,
    pub project: OnceCell<Arc<Mutex<Option<Project>>>>,
    pub glyph: OnceCell<Rc<RefCell<Glyph>>>,
    pub focused: Cell<bool>,
    pub zoom_factor: Cell<f64>,
    pub drawing_area: OnceCell<gtk::DrawingArea>,
}

unsafe impl Send for GlyphBox {}
unsafe impl Sync for GlyphBox {}

unsafe impl Send for GlyphBoxItem {}
unsafe impl Sync for GlyphBoxItem {}
#[glib::object_subclass]
impl ObjectSubclass for GlyphBox {
    const NAME: &'static str = "GlyphBox";
    type Type = GlyphBoxItem;
    type ParentType = gtk::EventBox;
}

impl ObjectImpl for GlyphBox {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_height_request(GLYPH_BOX_HEIGHT as _);
        obj.set_width_request(GLYPH_BOX_WIDTH as _);
        obj.set_can_focus(true);
        obj.set_expand(false);

        obj.connect(
            "button-press-event",
            false,
            clone!(@weak obj => @default-return Some(false.to_value()), move |_| {
                obj.imp().app.get().unwrap().downcast_ref::<crate::GerbApp>().unwrap().imp().window.get().unwrap().emit_by_name::<()>("open-glyph-edit", &[&obj]);
                println!("open-glyph-edit emitted!");


                Some(true.to_value())
            }),
        );
        let drawing_area = gtk::DrawingArea::builder()
            .expand(true)
            .visible(true)
            .can_focus(true)
            .has_tooltip(true)
            .build();
        drawing_area.connect_query_tooltip(
            clone!(@weak obj => @default-return false, move |_self, _x: i32, _y: i32, _by_keyboard: bool, tooltip| {
                let glyph = obj.imp().glyph.get().unwrap().borrow();
                if let GlyphKind::Char(c) = glyph.kind {
                    let block_name = if let Some(idx) = c.char_block() {
                        UNICODE_BLOCKS[idx].1
                    } else {
                        "Unknown"
                    };
                    let unicode = format!("U+{:04X}", c as u32);

                    tooltip.set_text(Some(&format!("Name: {}\nUnicode: {}\nBlock: {}", glyph.name, unicode, block_name)));
                } else {
                    tooltip.set_text(Some(&format!("Name: {}\nComponent", glyph.name)));
                }
                true
            }));
        drawing_area.connect_draw(clone!(@weak obj => @default-return Inhibit(false), move |_drar: &gtk::DrawingArea, cr: &Context| {
            cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
            let is_focused: bool = obj.imp().focused.get();
            let zoom_factor: f64 = obj.imp().zoom_factor.get();
            //cr.scale(500f64, 500f64);
            //let (r, g, b) = crate::utils::hex_color_to_rgb("#c4c4c4").unwrap();
            //cr.set_source_rgb(r, g, b);
            cr.set_source_rgb(1., 1., 1.);
            cr.paint().expect("Invalid cairo surface state");

            let (x, y) = (0.01, 0.01);
            let glyph = obj.imp().glyph.get().unwrap().borrow();
            let c = &glyph.name;
            let label = match glyph.kind {
                GlyphKind::Char(c) => c.to_string(),
                GlyphKind::Component => c.to_string(),
            };
            cr.set_line_width(1.5);
            let (point, (width, height)) = crate::utils::draw_round_rectangle(cr, (x, y), (zoom_factor * GLYPH_BOX_WIDTH, zoom_factor * GLYPH_BOX_HEIGHT), 1.0, 1.5);
            let glyph_width = glyph.width.unwrap_or(1000) as f64 * (width * 0.8) / 1000.;
            if is_focused {
                cr.set_source_rgb(1., 250./255., 141./255.);
            } else {
                cr.set_source_rgb(1., 1., 1.);
            }
            cr.fill_preserve().expect("Invalid cairo surface state");
            cr.set_source_rgba(0., 0., 0., 0.5);
            cr.stroke_preserve().expect("Invalid cairo surface state");
            cr.clip();
            cr.new_path();
            cr.set_source_rgba(0., 0., 0., 0.4);
            cr.set_font_size(zoom_factor * 62.);
            let sextents = cr
                .text_extents(&label)
                .expect("Invalid cairo surface state");
            if glyph.is_empty() {
                cr.move_to(point.0 + width/2. - sextents.width/2., point.1+(height / 3.)+20.);
                cr.show_text(&label).expect("Invalid cairo surface state");
            } else {
                let mut matrix = gtk::cairo::Matrix::identity();
                matrix.translate((width - glyph_width) / 2., 0.);
                matrix.scale((width * 0.8) / 1000., (width * 0.8) / 1000.);
                let options = GlyphDrawingOptions {
                    outline: (0., 0., 0., 0.),
                    inner_fill: Some((0.35, 0.35, 0.35, 1.)),
                    highlight: None,
                    matrix,
                };
                glyph.draw(cr, options);
            }


            cr.set_line_width(2.);
            cr.set_source_rgb(0., 0., 0.);
            cr.move_to(x, point.1+ 2.* (height / 3.));
            cr.line_to(x+width*1.2, point.1+ 2.* (height / 3.));
            cr.stroke().expect("Invalid cairo surface state");
            cr.set_source_rgb(196./255., 196./255., 196./255.);
            cr.new_path();
            cr.rectangle(x, point.1+2.*(height/3.), width*1.2, 1.2*height/3.);
            cr.fill().expect("Invalid cairo surface state");
            cr.reset_clip();

            cr.set_source_rgb(0., 0., 0.);
            cr.set_font_size(zoom_factor * 12.);
            let sextents = cr
                .text_extents(&label)
                .expect("Invalid cairo surface state");
            cr.move_to(point.0 + width/2. - sextents.width/2., point.1+ 2.* (height / 3.)+20.);
            cr.show_text(&label).expect("Invalid cairo surface state");


            let label = match glyph.kind {
                GlyphKind::Char(c) => format!("U+{:04X}", c as u32),
                GlyphKind::Component => c.to_string(),
            };
            let extents = cr
                .text_extents(&label)
                .expect("Invalid cairo surface state");
            cr.move_to(point.0 + width/2. - extents.width/2., point.1+ 2.* (height / 3.)+22.0 + sextents.height);
            cr.show_text(&label).expect("Invalid cairo surface state");

            Inhibit(false)
        }
        ));
        obj.set_child(Some(&drawing_area));

        obj.set_events(
            gtk::gdk::EventMask::POINTER_MOTION_MASK
                | gtk::gdk::EventMask::ENTER_NOTIFY_MASK
                | gtk::gdk::EventMask::LEAVE_NOTIFY_MASK,
        );
        obj.connect_enter_notify_event(|_self, _event| -> Inhibit {
            //println!("obj has window {}", _self.has_window());
            if let Some(screen) = _self.window() {
                let display = screen.display();
                screen.set_cursor(Some(
                    &gtk::gdk::Cursor::from_name(&display, "pointer").unwrap(),
                ));
            }
            _self.imp().focused.set(true);
            _self.imp().drawing_area.get().unwrap().queue_draw();
            //println!("focus in {:?}", _self.imp().glyph.get().unwrap());
            Inhibit(false)
        });

        obj.connect_leave_notify_event(|_self, _event| -> Inhibit {
            //println!("focus out {:?}", _self.imp().glyph.get().unwrap());
            if let Some(screen) = _self.window() {
                let display = screen.display();
                screen.set_cursor(Some(
                    &gtk::gdk::Cursor::from_name(&display, "default").unwrap(),
                ));
            }
            _self.imp().focused.set(false);
            _self.imp().drawing_area.get().unwrap().queue_draw();

            Inhibit(false)
        });

        self.drawing_area
            .set(drawing_area)
            .expect("Failed to initialize window state");
        self.focused.set(false);
        self.zoom_factor.set(1.0);
    }
}

impl WidgetImpl for GlyphBox {}
impl ContainerImpl for GlyphBox {}
impl BinImpl for GlyphBox {}
impl EventBoxImpl for GlyphBox {}

glib::wrapper! {
    pub struct GlyphBoxItem(ObjectSubclass<GlyphBox>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::EventBox;
}

impl GlyphBoxItem {
    pub fn new(
        app: gtk::Application,
        project: Arc<Mutex<Option<Project>>>,
        glyph: Rc<RefCell<Glyph>>,
    ) -> Self {
        let ret: Self = glib::Object::new(&[]).expect("Failed to create Main Window");
        ret.imp().app.set(app).unwrap();
        ret.imp().project.set(project).unwrap();
        ret.imp().glyph.set(glyph).unwrap();
        ret
    }
}
