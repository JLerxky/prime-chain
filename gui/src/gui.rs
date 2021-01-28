// 纯GUI窗口运行, 无命令行窗口
#![windows_subsystem = "windows"]

use iced::{
    button, scrollable, slider, text_input, window, Application, Button, Checkbox, Color, Column,
    Command, Container, Element, HorizontalAlignment, Length, Radio, Row, Scrollable, Settings,
    Slider, Space, Text, TextInput,
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
                self.routes.update(route_msg, &mut self.debug);
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
            .push(routes.view(self.debug).map(EventMessage::RouteMessage))
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
            routes: vec![
                Route::Welcome,
                Route::Slider {
                    state: slider::State::new(),
                    value: 50,
                },
                Route::RowsAndColumns {
                    layout: Layout::Row,
                    spacing_slider: slider::State::new(),
                    spacing: 20,
                },
                Route::Text {
                    size_slider: slider::State::new(),
                    size: 30,
                    color_sliders: [slider::State::new(); 3],
                    color: Color::BLACK,
                },
                Route::Radio { selection: None },
                Route::Image {
                    width: 300,
                    slider: slider::State::new(),
                },
                Route::Scrollable,
                Route::TextInput {
                    value: String::new(),
                    is_secure: false,
                    state: text_input::State::new(),
                },
                Route::Debugger,
                Route::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: RouteMessage, debug: &mut bool) {
        self.routes[self.current].update(msg, debug);
    }

    fn view(&mut self, debug: bool) -> Element<RouteMessage> {
        self.routes[self.current].view(debug)
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
    Slider {
        state: slider::State,
        value: u8,
    },
    RowsAndColumns {
        layout: Layout,
        spacing_slider: slider::State,
        spacing: u16,
    },
    Text {
        size_slider: slider::State,
        size: u16,
        color_sliders: [slider::State; 3],
        color: Color,
    },
    Radio {
        selection: Option<Language>,
    },
    Image {
        width: u16,
        slider: slider::State,
    },
    Scrollable,
    TextInput {
        value: String,
        is_secure: bool,
        state: text_input::State,
    },
    Debugger,
    End,
}

#[derive(Debug, Clone)]
pub enum RouteMessage {
    SliderChanged(u8),
    LayoutChanged(Layout),
    SpacingChanged(u16),
    TextSizeChanged(u16),
    TextColorChanged(Color),
    LanguageSelected(Language),
    ImageWidthChanged(u16),
    InputChanged(String),
    ToggleSecureInput(bool),
    DebugToggled(bool),
}

impl<'a> Route {
    fn update(&mut self, msg: RouteMessage, debug: &mut bool) {
        match msg {
            RouteMessage::DebugToggled(value) => {
                if let Route::Debugger = self {
                    *debug = value;
                }
            }
            RouteMessage::LanguageSelected(language) => {
                if let Route::Radio { selection } = self {
                    *selection = Some(language);
                }
            }
            RouteMessage::SliderChanged(new_value) => {
                if let Route::Slider { value, .. } = self {
                    *value = new_value;
                }
            }
            RouteMessage::TextSizeChanged(new_size) => {
                if let Route::Text { size, .. } = self {
                    *size = new_size;
                }
            }
            RouteMessage::TextColorChanged(new_color) => {
                if let Route::Text { color, .. } = self {
                    *color = new_color;
                }
            }
            RouteMessage::LayoutChanged(new_layout) => {
                if let Route::RowsAndColumns { layout, .. } = self {
                    *layout = new_layout;
                }
            }
            RouteMessage::SpacingChanged(new_spacing) => {
                if let Route::RowsAndColumns { spacing, .. } = self {
                    *spacing = new_spacing;
                }
            }
            RouteMessage::ImageWidthChanged(new_width) => {
                if let Route::Image { width, .. } = self {
                    *width = new_width;
                }
            }
            RouteMessage::InputChanged(new_value) => {
                if let Route::TextInput { value, .. } = self {
                    *value = new_value;
                }
            }
            RouteMessage::ToggleSecureInput(toggle) => {
                if let Route::TextInput { is_secure, .. } = self {
                    *is_secure = toggle;
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Route::Welcome => "Welcome",
            Route::Radio { .. } => "Radio button",
            Route::Slider { .. } => "Slider",
            Route::Text { .. } => "Text",
            Route::Image { .. } => "Image",
            Route::RowsAndColumns { .. } => "Rows and columns",
            Route::Scrollable => "Scrollable",
            Route::TextInput { .. } => "Text input",
            Route::Debugger => "Debugger",
            Route::End => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Route::Welcome => true,
            Route::Radio { selection } => *selection == Some(Language::Rust),
            Route::Slider { .. } => true,
            Route::Text { .. } => true,
            Route::Image { .. } => true,
            Route::RowsAndColumns { .. } => true,
            Route::Scrollable => true,
            Route::TextInput { value, .. } => !value.is_empty(),
            Route::Debugger => true,
            Route::End => false,
        }
    }

    fn view(&mut self, debug: bool) -> Element<RouteMessage> {
        match self {
            Route::Welcome => Self::welcome(),
            Route::Radio { selection } => Self::radio(*selection),
            Route::Slider { state, value } => Self::slider(state, *value),
            Route::Text {
                size_slider,
                size,
                color_sliders,
                color,
            } => Self::text(size_slider, *size, color_sliders, *color),
            Route::Image { width, slider } => Self::image(*width, slider),
            Route::RowsAndColumns {
                layout,
                spacing_slider,
                spacing,
            } => Self::rows_and_columns(*layout, spacing_slider, *spacing),
            Route::Scrollable => Self::scrollable(),
            Route::TextInput {
                value,
                is_secure,
                state,
            } => Self::text_input(value, *is_secure, state),
            Route::Debugger => Self::debugger(debug),
            Route::End => Self::end(),
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

    fn slider(state: &'a mut slider::State, value: u8) -> Column<'a, RouteMessage> {
        Self::container("Slider")
            .push(Text::new(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            ))
            .push(Text::new(
                "The following slider lets you choose an integer from \
                 0 to 100:",
            ))
            .push(Slider::new(
                state,
                0..=100,
                value,
                RouteMessage::SliderChanged,
            ))
            .push(
                Text::new(&value.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn rows_and_columns(
        layout: Layout,
        spacing_slider: &'a mut slider::State,
        spacing: u16,
    ) -> Column<'a, RouteMessage> {
        let row_radio = Radio::new(
            Layout::Row,
            "Row",
            Some(layout),
            RouteMessage::LayoutChanged,
        );

        let column_radio = Radio::new(
            Layout::Column,
            "Column",
            Some(layout),
            RouteMessage::LayoutChanged,
        );

        let layout_section: Element<_> = match layout {
            Layout::Row => Row::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
            Layout::Column => Column::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
        };

        let spacing_section = Column::new()
            .spacing(10)
            .push(Slider::new(
                spacing_slider,
                0..=80,
                spacing,
                RouteMessage::SpacingChanged,
            ))
            .push(
                Text::new(&format!("{} px", spacing))
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            );

        Self::container("Rows and columns")
            .spacing(spacing)
            .push(Text::new(
                "Iced uses a layout model based on flexbox to position UI \
                 elements.",
            ))
            .push(Text::new(
                "Rows and columns can be used to distribute content \
                 horizontally or vertically, respectively.",
            ))
            .push(layout_section)
            .push(Text::new(
                "You can also easily change the spacing between elements:",
            ))
            .push(spacing_section)
    }

    fn text(
        size_slider: &'a mut slider::State,
        size: u16,
        color_sliders: &'a mut [slider::State; 3],
        color: Color,
    ) -> Column<'a, RouteMessage> {
        let size_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("You can change its size:"))
            .push(Text::new(&format!("This text is {} pixels", size)).size(size))
            .push(Slider::new(
                size_slider,
                10..=70,
                size,
                RouteMessage::TextSizeChanged,
            ));

        let [red, green, blue] = color_sliders;

        let color_sliders = Row::new()
            .spacing(10)
            .push(color_slider(red, color.r, move |r| Color { r, ..color }))
            .push(color_slider(green, color.g, move |g| Color { g, ..color }))
            .push(color_slider(blue, color.b, move |b| Color { b, ..color }));

        let color_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("And its color:"))
            .push(Text::new(&format!("{:?}", color)).color(color))
            .push(color_sliders);

        Self::container("Text")
            .push(Text::new(
                "Text is probably the most essential widget for your UI. \
                 It will try to adapt to the dimensions of its container.",
            ))
            .push(size_section)
            .push(color_section)
    }

    fn radio(selection: Option<Language>) -> Column<'a, RouteMessage> {
        let question = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Iced is written in...").size(24))
            .push(Language::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, language| {
                    choices.push(Radio::new(
                        language,
                        language,
                        selection,
                        RouteMessage::LanguageSelected,
                    ))
                },
            ));

        Self::container("Radio button")
            .push(Text::new(
                "A radio button is normally used to represent a choice... \
                 Surprise test!",
            ))
            .push(question)
            .push(Text::new(
                "Iced works very well with iterators! The list above is \
                 basically created by folding a column over the different \
                 choices, creating a radio button for each one of them!",
            ))
    }

    fn image(width: u16, slider: &'a mut slider::State) -> Column<'a, RouteMessage> {
        Self::container("Image")
            .push(Text::new("An image that tries to keep its aspect ratio."))
            .push(ferris(width))
            .push(Slider::new(
                slider,
                100..=500,
                width,
                RouteMessage::ImageWidthChanged,
            ))
            .push(
                Text::new(&format!("Width: {} px", width.to_string()))
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn scrollable() -> Column<'a, RouteMessage> {
        Self::container("Scrollable")
            .push(Text::new(
                "Iced supports scrollable content. Try it out! Find the \
                 button further below.",
            ))
            .push(Text::new("Tip: You can use the scrollbar to scroll down faster!").size(16))
            .push(Column::new().height(Length::Units(4096)))
            .push(
                Text::new("You are halfway there!")
                    .width(Length::Fill)
                    .size(30)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(Column::new().height(Length::Units(4096)))
            .push(ferris(300))
            .push(
                Text::new("You made it!")
                    .width(Length::Fill)
                    .size(50)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn text_input(
        value: &str,
        is_secure: bool,
        state: &'a mut text_input::State,
    ) -> Column<'a, RouteMessage> {
        let text_input = TextInput::new(
            state,
            "Type something to continue...",
            value,
            RouteMessage::InputChanged,
        )
        .padding(10)
        .size(30);
        Self::container("Text input")
            .push(Text::new(
                "Use a text input to ask for different kinds of information.",
            ))
            .push(if is_secure {
                text_input.password()
            } else {
                text_input
            })
            .push(Checkbox::new(
                is_secure,
                "Enable password mode",
                RouteMessage::ToggleSecureInput,
            ))
            .push(Text::new(
                "A text input produces a message every time it changes. It is \
                 very easy to keep track of its contents:",
            ))
            .push(
                Text::new(if value.is_empty() {
                    "You have not typed anything yet..."
                } else {
                    value
                })
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn debugger(debug: bool) -> Column<'a, RouteMessage> {
        Self::container("Debugger")
            .push(Text::new(
                "You can ask Iced to visually explain the layouting of the \
                 different elements comprising your UI!",
            ))
            .push(Text::new(
                "Give it a shot! Check the following checkbox to be able to \
                 see element boundaries.",
            ))
            .push(if cfg!(target_arch = "wasm32") {
                Element::new(
                    Text::new("Not available on web yet!")
                        .color([0.7, 0.7, 0.7])
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
            } else {
                Element::new(Checkbox::new(
                    debug,
                    "Explain layout",
                    RouteMessage::DebugToggled,
                ))
            })
            .push(Text::new("Feel free to go back and take a look."))
    }

    fn end() -> Column<'a, RouteMessage> {
        Self::container("You reached the end!")
            .push(Text::new(
                "This tour will be updated as more features are added.",
            ))
            .push(Text::new("Make sure to keep an eye on it!"))
    }
}

fn ferris<'a>(width: u16) -> Container<'a, RouteMessage> {
    Container::new(Text::new("ferris").width(Length::FillPortion(width)))
        .width(Length::Fill)
        .center_x()
}

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

fn color_slider(
    state: &mut slider::State,
    component: f32,
    update: impl Fn(f32) -> Color + 'static,
) -> Slider<f64, RouteMessage> {
    Slider::new(state, 0.0..=1.0, f64::from(component), move |c| {
        RouteMessage::TextColorChanged(update(c as f32))
    })
    .step(0.01)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Other,
}

impl Language {
    fn all() -> [Language; 6] {
        [
            Language::C,
            Language::Elm,
            Language::Ruby,
            Language::Haskell,
            Language::Rust,
            Language::Other,
        ]
    }
}

impl From<Language> for String {
    fn from(language: Language) -> String {
        String::from(match language {
            Language::Rust => "Rust",
            Language::Elm => "Elm",
            Language::Ruby => "Ruby",
            Language::Haskell => "Haskell",
            Language::C => "C",
            Language::Other => "Other",
        })
    }
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
