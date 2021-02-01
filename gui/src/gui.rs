// 纯GUI窗口运行, 无命令行窗口
#![windows_subsystem = "windows"]

use iced::{
    button, scrollable, window, Application, Button, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Settings, Space, Text,
};

// pub fn main() -> iced::Result {
//     App::run(Settings::default())
// }

pub struct App {
    routes: Routes,
    debug: bool,
    window_mode: window::Mode,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
}

impl App {
    pub fn start() -> iced::Result {
        App::run(Settings::default())
    }
}

impl Application for App {
    type Message = EventMessage;

    fn new(_flags: ()) -> (App, Command<EventMessage>) {
        (
            App {
                routes: Routes::new(),
                debug: false,
                window_mode: window::Mode::Windowed,
                scroll: scrollable::State::new(),
                back_button: button::State::new(),
                next_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("数据冰箱 - {}", self.routes.title())
    }

    fn mode(&self) -> window::Mode {
        self.window_mode
    }

    fn update(&mut self, event: EventMessage) -> Command<EventMessage> {
        match event {
            EventMessage::BackPressed => {
                self.routes.go_back();
                self.window_mode = window::Mode::Fullscreen;
            }
            EventMessage::NextPressed => {
                self.routes.advance();
                self.window_mode = window::Mode::Windowed;
            }
            EventMessage::RouteMessage(route_msg) => {
                self.routes.update(route_msg);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<EventMessage> {
        let App {
            routes,
            scroll,
            back_button,
            next_button,
            ..
        } = self;

        let mut controls = Row::new();

        if routes.has_previous() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(EventMessage::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if routes.can_continue() {
            controls = controls.push(
                button(next_button, "Next")
                    .on_press(EventMessage::NextPressed)
                    .style(style::Button::Primary),
            );
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(routes.view().map(EventMessage::RouteMessage))
            .push(controls)
            .into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }

    type Executor = iced::executor::Default;

    type Flags = ();
}

#[derive(Debug, Clone)]
pub enum EventMessage {
    BackPressed,
    NextPressed,
    RouteMessage(RouteMessage),
}

struct Routes {
    routes: Vec<Route>,
    current: usize,
}

impl Routes {
    fn new() -> Routes {
        Routes {
            routes: vec![Route::Welcome],
            current: 0,
        }
    }

    fn update(&mut self, msg: RouteMessage) {
        self.routes[self.current].update(msg);
    }

    fn view(&mut self) -> Element<RouteMessage> {
        self.routes[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.routes.len() && self.routes[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.routes[self.current].title()
    }
}

enum Route {
    Welcome,
}

#[derive(Debug, Clone)]
pub enum RouteMessage {
    Welcome,
}

impl<'a> Route {
    fn update(&mut self, msg: RouteMessage) {
        match msg {
            RouteMessage::Welcome => (),
        };
    }

    fn title(&self) -> &str {
        match self {
            Route::Welcome => "Welcome",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Route::Welcome => true,
        }
    }

    fn view(&mut self) -> Element<RouteMessage> {
        match self {
            Route::Welcome => Self::welcome(),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, RouteMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn welcome() -> Column<'a, RouteMessage> {
        Self::container("Welcome!")
            .push(Text::new(
                "This is a simple tour meant to showcase a bunch of widgets \
                 that can be easily implemented on top of Iced.",
            ))
            .push(Text::new(
                "Iced is a cross-platform GUI library for Rust focused on \
                 simplicity and type-safety. It is heavily inspired by Elm.",
            ))
            .push(Text::new(
                "It was originally born as part of Coffee, an opinionated \
                 2D game engine for Rust.",
            ))
            .push(Text::new(
                "On native platforms, Iced provides by default a renderer \
                 built on top of wgpu, a graphics library supporting Vulkan, \
                 Metal, DX11, and DX12.",
            ))
            .push(Text::new(
                "Additionally, this tour can also run on WebAssembly thanks \
                 to dodrio, an experimental VDOM library for Rust.",
            ))
            .push(Text::new(
                "You will need to interact with the UI in order to reach the \
                 end!",
            ))
    }
}

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
