use iced::advanced::{layout, mouse, renderer, widget::Tree, Layout, Widget};
use iced::{Element, Event, Length, Rectangle, Size};
use crate::app::Message;


#[derive(Default)]
pub struct ViewportInputState {
    pub last_move: Option<std::time::Instant>,
}

pub struct ViewportInput<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> ViewportInput<'a> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, Theme, Renderer> Widget<Message, Theme, Renderer> for ViewportInput<'a>
where
    Renderer: renderer::Renderer,
{
    
    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(ViewportInputState::default())
    }

    fn size(&self) -> Size<Length> {

        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.max())
    }

    
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut iced::advanced::Shell<'_, Message>,
        bounds: &Rectangle,
    ) {
        if !cursor.is_over(*bounds) {
            return;
        }
        let state = tree.state.downcast_mut::<ViewportInputState>();

        if !cursor.is_over(*bounds) {
            return;
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let now = std::time::Instant::now();
                let emit = match state.last_move {
                    Some(last) => now.duration_since(last).as_millis() > 30, // ~30 fps tick
                    None => true,
                };
                if emit {
                    shell.publish(Message::ViewportMove(*position));
                    state.last_move = Some(now);
                } else {
                    shell.request_redraw();
                }
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(pos) = cursor.position() {
                    shell.publish(Message::ViewportLeftPress);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(pos) = cursor.position() {
                    shell.publish(Message::ViewportLeftRelease);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                if let Some(pos) = cursor.position() {
                    shell.publish(Message::ViewportRightPress);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) => {
                if let Some(pos) = cursor.position() {
                    shell.publish(Message::ViewportRightRelease);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) => {
                if let Some(pos) = cursor.position() {
                    shell.publish(Message::ViewportMiddlePress);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Middle)) => {
                if let Some(pos) = cursor.position() {
                    shell.publish(Message::ViewportMiddleRelease);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                shell.publish(Message::ViewportScroll(*delta));
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorLeft) => {
                shell.publish(Message::ViewportExit);
                shell.capture_event();
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {}
}

impl<'a, Theme, Renderer> From<ViewportInput<'a>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Theme: 'a,
{
    fn from(widget: ViewportInput<'a>) -> Self {
        Element::new(widget)
    }
}
