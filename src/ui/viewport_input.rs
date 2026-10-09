use iced::advanced::{layout, mouse, renderer, widget::Tree, Layout, Widget};
use iced::{Element, Event, Length, Point, Rectangle, Size};
use crate::app::Message;
use std::sync::Arc;
use std::cell::RefCell;
use crate::scene::SelectionState;

#[derive(Default)]
pub struct ViewportInputState {
    last_move: Option<std::time::Instant>,
}

pub struct ViewportInput<'a> {
    pub last_cursor_screen: &'a std::cell::Cell<Point>,
    pub selection: Arc<RefCell<SelectionState>>,
    pub hardware_cursor: bool,
    pub pane_idx: Option<usize>,
}

impl<'a> ViewportInput<'a> {
    pub fn new(last_cursor_screen: &'a std::cell::Cell<Point>, selection: Arc<RefCell<SelectionState>>, hardware_cursor: bool, pane_idx: Option<usize>) -> Self {
        Self {
            last_cursor_screen,
            selection,
            hardware_cursor,
            pane_idx,
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
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _bounds: &Rectangle,
    ) {
        if !cursor.is_over(layout.bounds()) {
            return;
        }
        let state = tree.state.downcast_mut::<ViewportInputState>();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let now = std::time::Instant::now();
                let emit = if self.hardware_cursor {
                    match state.last_move {
                        Some(last) => now.duration_since(last).as_millis() > 66, // ~15 fps tick for heavy UI rebuilds/snapping
                        None => true,
                    }
                } else {
                    true
                };
                
                if !self.hardware_cursor {
                    shell.request_redraw();
                }

                if emit {
                    if self.hardware_cursor {
                        shell.request_redraw();
                    }
                    
                    let b = layout.bounds();
                    let local = iced::Point::new(position.x - b.x, position.y - b.y);
                    if let Some(idx) = self.pane_idx {
                        shell.publish(Message::PaneMove(idx, local));
                    } else {
                        shell.publish(Message::ViewportMove(local));
                    }
                    state.last_move = Some(now);
                }

                shell.capture_event()
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(_pos) = cursor.position() {
                    if let Some(idx) = self.pane_idx { shell.publish(Message::PanePress(idx)); } else { shell.publish(Message::ViewportLeftPress); }
                    shell.capture_event()
                } else {
                    
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(_pos) = cursor.position() {
                    if let Some(idx) = self.pane_idx { shell.publish(Message::PaneRelease(idx)); } else { shell.publish(Message::ViewportLeftRelease); }
                    shell.capture_event()
                } else {
                    
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                if let Some(_pos) = cursor.position() {
                    if let Some(idx) = self.pane_idx { shell.publish(Message::PaneRightPress(idx)); } else { shell.publish(Message::ViewportRightPress); }
                    shell.capture_event()
                } else {
                    
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) => {
                if let Some(_pos) = cursor.position() {
                    if let Some(idx) = self.pane_idx { shell.publish(Message::PaneRightRelease(idx)); } else { shell.publish(Message::ViewportRightRelease); }
                    shell.capture_event()
                } else {
                    
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) => {
                if let Some(_pos) = cursor.position() {
                    if let Some(idx) = self.pane_idx { shell.publish(Message::PaneMiddlePress(idx)); } else { shell.publish(Message::ViewportMiddlePress); }
                    shell.capture_event()
                } else {
                    
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Middle)) => {
                if let Some(_pos) = cursor.position() {
                    if let Some(idx) = self.pane_idx { shell.publish(Message::PaneMiddleRelease(idx)); } else { shell.publish(Message::ViewportMiddleRelease); }
                    shell.capture_event()
                } else {
                    
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if let Some(idx) = self.pane_idx { shell.publish(Message::PaneScroll(idx, *delta)); } else { shell.publish(Message::ViewportScroll(*delta)); }
                shell.capture_event()
            }
            Event::Mouse(mouse::Event::CursorLeft) => {
                shell.publish(Message::ViewportExit);
                shell.capture_event()
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
