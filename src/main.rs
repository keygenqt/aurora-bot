use std::io;

use ratatui_image::{picker::Picker, protocol::StatefulProtocol, Resize, StatefulImage};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
    DefaultTerminal,
    Frame,
};

mod utils;

// Check language, functions
fn main() {
    utils::run_tests();
}

// TUI
#[allow(dead_code)]
fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let picker = Picker::from_query_stdio().unwrap();
    let image_source = image::ImageReader::open("/home/keygenqt/Documents/Home/Projects/aurora-bot/assets/6048909d-cb71-4d59-964b-15e64d1bc9af.jpeg")?.decode()?;
    let image = picker.new_resize_protocol(image_source.clone());
    let mut app = App { image, is_image: false, exit: false };
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

pub struct App {
    image: StatefulProtocol,
    is_image: bool,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            if self.is_image {
                terminal.draw(|frame| self.draw_image(frame))?;
            } else {
                terminal.draw(|frame| self.draw(frame))?;
            }
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw_image(&mut self, frame: &mut Frame) {
        let title = Line::from(" Image ".bold());
        let instructions = Line::from(vec![
            " Back ".into(),
            "<B> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // Image
        let resize = Resize::Scale(None);
        let image = StatefulImage::default().resize(resize);
        let inner_area = block.inner(frame.area());
        frame.render_stateful_widget(image, inner_area, &mut self.image);

        // Block
        frame.render_widget(block, frame.area());

    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('b') => self.hide_image(),
            KeyCode::Char('i') => self.show_image(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn hide_image(&mut self) {
        self.is_image = false;
    }

    fn show_image(&mut self) {
        self.is_image = true;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Aurora Bot ".bold());
        let instructions = Line::from(vec![
            " Image ".into(),
            "<I> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let post_title = Span::styled(" 🎞️ Картографические библиотеки MFW для ОС Аврора.", Style::default().green().add_modifier(Modifier::BOLD));
        let body = Span::styled(" Делимся с вами записью выступления Дмитрия Лапшина, старшего инженера-разработчика ОМП, в котором он рассказывает про картографические библиотеки MFW для ОС Аврора 🗺", Style::default());
        let link = Span::styled(" 🔗 Смотреть видео", Style::default().blue());

        let author_raw = match utils::format(" Виталий Зарубин, 17 января 2025.") {
            Ok(it) => it,
            Err(_) => "Error format",
        };

        let author = Span::styled(author_raw , Style::default().italic().gray());

        let post: Vec<Line<'_>> = vec![
            post_title.into(),
            Span::from("").into(),
            body.into(),
            Span::from("").into(),
            link.into(),
            Span::from("").into(),
            Span::from("⭐⭐⭐⭐⭐  5.00").into(),
            Span::from("").into(),
            author.into(),
        ];

        Paragraph::new(post)
            .alignment(Alignment::Left)
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
