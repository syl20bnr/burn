/// Define a left and a right region for the application.
/// Each region is divided in vertically stacked rectangles.
use std::marker::PhantomData;
use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{block::Position, Block, BorderType, Borders, Padding},
    Frame,
};

// Region Base ---------------------------------------------------------------

struct RegionInfo {
    pub width_percentage: u16,
}

struct RegionRectInfo {
    pub index: usize,
    pub title: &'static str,
    pub height_percentage: u16,
    pub hotkey: char,
}

trait GetRegionInfo {
    fn get_region_info() -> RegionInfo;
    fn get_rect_info(&self) -> RegionRectInfo;
}

pub(crate) struct Region<T: GetRegionInfo> {
    rects: Rc<[Rect]>,
    info: RegionInfo,
    _t: PhantomData<T>,
}

impl<T: GetRegionInfo> Region<T> {
    pub fn new(rects: Rc<[Rect]>) -> Self {
        Self {
            rects,
            info: T::get_region_info(),
            _t: PhantomData,
        }
    }
}

// Left Region --------------------------------------------------------------

pub(crate) enum LeftRegion {
    Top,
    Middle,
    Bottom,
}

impl GetRegionInfo for LeftRegion {
    fn get_region_info() -> RegionInfo {
        RegionInfo {
            width_percentage: 25,
        }
    }

    fn get_rect_info(&self) -> RegionRectInfo {
        match self {
            LeftRegion::Top => RegionRectInfo {
                index: 0,
                title: "Backend",
                height_percentage: 30,
                hotkey: 'b',
            },
            LeftRegion::Middle => RegionRectInfo {
                index: 1,
                title: "Benches",
                height_percentage: 60,
                hotkey: 'n',
            },
            LeftRegion::Bottom => RegionRectInfo {
                index: 2,
                title: "Action",
                height_percentage: 10,
                hotkey: 'a',
            },
        }
    }
}

// Right Region --------------------------------------------------------------

pub(crate) enum RightRegion {
    Top,
    Bottom,
}

impl GetRegionInfo for RightRegion {
    fn get_region_info() -> RegionInfo {
        RegionInfo {
            width_percentage: 100 - LeftRegion::get_region_info().width_percentage,
        }
    }

    fn get_rect_info(&self) -> RegionRectInfo {
        match self {
            RightRegion::Top => RegionRectInfo {
                index: 0,
                title: "Results",
                height_percentage: 90,
                hotkey: 'r',
            },
            RightRegion::Bottom => RegionRectInfo {
                index: 1,
                title: "Progress",
                height_percentage: 10,
                hotkey: 'p',
            },
        }
    }
}

// Regions definition --------------------------------------------------------

pub(crate) struct Regions<L: GetRegionInfo, R: GetRegionInfo> {
    pub left: Region<L>,
    pub right: Region<R>,
}

impl Regions<LeftRegion, RightRegion> {
    pub fn new(frame: &Frame) -> Self {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(LeftRegion::get_region_info().width_percentage),
                Constraint::Percentage(RightRegion::get_region_info().width_percentage),
            ])
            .split(frame.size());
        let left_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(LeftRegion::Top.get_rect_info().height_percentage),
                Constraint::Percentage(LeftRegion::Middle.get_rect_info().height_percentage),
                Constraint::Percentage(LeftRegion::Bottom.get_rect_info().height_percentage),
            ])
            .split(outer_layout[0]);
        let right_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(RightRegion::Top.get_rect_info().height_percentage),
                Constraint::Percentage(RightRegion::Bottom.get_rect_info().height_percentage),
            ])
            .split(outer_layout[1]);
        Self::new_with_rect(left_rects, right_rects)
    }

    pub fn draw(&self, frame: &mut Frame) {
        // Left region
        frame.render_widget(
            self.left.block(LeftRegion::Top),
            self.left.rect(LeftRegion::Top),
        );
        frame.render_widget(
            self.left.block(LeftRegion::Middle),
            self.left.rect(LeftRegion::Middle),
        );
        frame.render_widget(
            self.left.block(LeftRegion::Bottom),
            self.left.rect(LeftRegion::Bottom),
        );
        // Right region
        frame.render_widget(
            self.right.block(RightRegion::Top),
            self.right.rect(RightRegion::Top),
        );
        frame.render_widget(
            self.right.block(RightRegion::Bottom),
            self.right.rect(RightRegion::Bottom),
        );
    }
}

impl<L: GetRegionInfo, R: GetRegionInfo> Regions<L, R> {
    fn new_with_rect(left_rects: Rc<[Rect]>, right_rects: Rc<[Rect]>) -> Self {
        Self {
            left: Region {
                rects: left_rects,
                info: L::get_region_info(),
                _t: PhantomData,
            },
            right: Region {
                rects: right_rects,
                info: R::get_region_info(),
                _t: PhantomData,
            },
        }
    }
}

impl<P: GetRegionInfo> Region<P> {
    pub fn rect(&self, position: P) -> Rect {
        self.rects[position.get_rect_info().index]
    }

    /// Widget to draw the style of a region
    fn block(&self, position: P) -> Block {
        Block::default()
            .title(format!(
                "{} ({})",
                position.get_rect_info().title,
                position.get_rect_info().hotkey
            ))
            .title_position(Position::Top)
            .title_alignment(Alignment::Center)
            .borders(Borders::all())
            .border_style(Style::default().fg(Color::DarkGray))
            .border_type(BorderType::Rounded)
            .padding(Padding {
                left: 10,
                right: 10,
                top: 2,
                bottom: 2,
            })
            .style(Style::default().bg(Color::Black))
    }
}

fn create_region_block(title: &str) -> Block {
    todo!()
}