use std::{fs::File, io::BufWriter};

use tuirealm::{
    application::PollStrategy,
    event::NoUserEvent,
    terminal::TerminalBridge,
    tui::layout::{Constraint, Direction, Layout},
    Application, AttrValue, Attribute, EventListenerCfg, MockComponent, Sub, SubClause,
    SubEventClause, Update,
};

mod main_select;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    GoTo(Screen),
    Updated,
}

struct Model {
    app: tuirealm::Application<u64, Msg, NoUserEvent>,
    quit: bool,
    redraw: bool,
    terminal: TerminalBridge,
    screen: Screen,
}

#[derive(Debug, PartialEq)]
pub enum Screen {
    Main,
    Characters,
    Inventory,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            screen: Screen::Main,
        }
    }
}

impl Model {
    pub fn view(&mut self) {
        match &self.screen {
            Screen::Main => {
                self.terminal
                    .raw_mut()
                    .draw(|f| {
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .margin(1)
                            .constraints([Constraint::Length(10)].as_ref())
                            .split(f.size());
                        self.app.view(&0, f, chunks[0])
                    })
                    .ok();
            }
            Screen::Characters => {}
            Screen::Inventory => {}
        }
    }
    pub fn init_app() -> Application<u64, Msg, NoUserEvent> {
        log::debug!("init_app");
        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(std::time::Duration::from_millis(20))
                .poll_timeout(std::time::Duration::from_millis(10))
                .tick_interval(std::time::Duration::from_secs(1)),
        );
        app.mount(
            0,
            Box::new(main_select::MainList::default()),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )
        .unwrap();
        app.active(&0).unwrap();
        app
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        match msg {
            Some(Msg::AppClose) => {
                self.quit = true;
                None
            }
            Some(Msg::Updated) => {
                self.redraw = true;
                None
            }
            _ => None,
        }
    }
}

fn main() {
    let log_file = File::options()
        .create(true)
        .append(true)
        .open("logs")
        .unwrap();
    env_logger::builder()
        .default_format()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
    // Setup model
    let mut model = Model::default();
    // Enter alternate screen
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                log::error!("Application error: {}", err);
            }
            Ok(messages) => {
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}
