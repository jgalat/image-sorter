use tui::layout::Rect;

pub enum RouteId {
    Main,
    Bindings,
    ResultScript,
}

pub struct App<'a> {
    pub images: Vec<&'a str>,
    pub current: usize,
    pub route: RouteId,
    pub size: Rect,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            images: vec![],
            current: 0,
            route: RouteId::Main,
            size: Rect::default(),
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
}
