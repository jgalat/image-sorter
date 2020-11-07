use tui::layout::Rect;

#[derive(Clone, Copy)]
pub enum TabId {
    Main,
    Script,
}

const TABS: [TabId; 2] = [TabId::Main, TabId::Script];

pub struct App<'a> {
    pub size: Rect,
    pub images: Vec<&'a str>,
    pub current: usize,
    pub tab: usize,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            size: Rect::default(),
            images: vec![],
            current: 0,
            tab: 0,
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            images: vec!["/home/jgalat/git/image-sorter/rember.png"],
            ..App::default()
        }
    }

    pub fn current_image(&self) -> &str {
        self.images[self.current]
    }

    pub fn current_tab(&self) -> TabId {
        TABS[self.tab]
    }

    pub fn switch_tab(&mut self) {
        self.tab = (self.tab + 1) % TABS.len()
    }
}
