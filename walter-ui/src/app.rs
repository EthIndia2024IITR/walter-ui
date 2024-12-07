use ratatui::widgets::ListState;

#[derive(Debug, Default, Clone, Copy)]
pub enum Screen {
    #[default]
    Home,
    Storage,
    Nodes,
    Settings,
}

#[derive(Default)]
pub struct App {
    pub current_screen: Screen,
    pub list_state: ListState,
    pub menu_items: Vec<&'static str>,
}

impl App {
    pub fn default() -> Self {
        let mut app = Self {
            current_screen: Screen::Home,
            list_state: ListState::default(),
            menu_items: vec!["Home", "Storage", "Nodes", "Settings"],
        };
        app.list_state.select(Some(0));
        app
    }

    pub fn next_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Home => Screen::Storage,
            Screen::Storage => Screen::Nodes,
            Screen::Nodes => Screen::Settings,
            Screen::Settings => Screen::Home,
        };
    }

    pub fn previous_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Home => Screen::Settings,
            Screen::Storage => Screen::Home,
            Screen::Nodes => Screen::Storage,
            Screen::Settings => Screen::Nodes,
        };
    }

    pub fn next_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.menu_items.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => (i + self.menu_items.len() - 1) % self.menu_items.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_current_item(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            match self.menu_items[selected] {
                "Home" => self.current_screen = Screen::Home,
                "Storage" => self.current_screen = Screen::Storage,
                "Nodes" => self.current_screen = Screen::Nodes,
                "Settings" => self.current_screen = Screen::Settings,
                _ => {}
            }
        }
    }
}
