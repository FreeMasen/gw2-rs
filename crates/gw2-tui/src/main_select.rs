use tui_realm_stdlib::List;
use tuirealm::{State, StateValue};
use tuirealm::command::{Cmd, Direction, CmdResult};
use tuirealm::event::KeyModifiers;
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};

use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent, NoUserEvent,
};

use crate::{Msg, Screen};

pub(crate) struct MainList {
    component: List,
}

impl Default for MainList {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Thick)
                        .color(Color::White),
                )
                .title("Options", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::Cyan)
                .rewind(true)
                .step(1)
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("Characters").fg(Color::Black))
                        .add_row()
                        .add_col(TextSpan::from("Inventory").fg(Color::Black))
                        .add_row()
                        .build(),
                ),
        }
    }
}

impl MockComponent for MainList {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        self.component.view(frame, area)
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.component.attr(attr, value)
    }

    fn state(&self) -> tuirealm::State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> tuirealm::command::CmdResult {
        log::debug!("perform: {:?}", cmd);
        let ret = self.component.perform(cmd);
        log::debug!("performed: {:?}", ret);
        ret
    }
}

impl Component<Msg, NoUserEvent> for MainList {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        log::debug!("MainList::on {:?}", ev);
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                return Some(Msg::Updated)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                return Some(Msg::Updated)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers,
            }) => {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    return Some(Msg::AppClose);
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                log::debug!("Enter pressed: {}", self.component.states.list_index);
                let screen = match self.component.states.list_index {
                    0 => Screen::Characters,
                    _ => return None,
                };
                return Some(Msg::GoTo(screen));
            }
            _ => {}
        };
        None
    }
}
