use ratatui::layout::{Constraint, Direction, Layout, Rect, Size};

pub fn _center(area: Rect, max_size: Size) -> Rect {
    let horizontal_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Fill(1),
            Constraint::Max(max_size.width),
            Constraint::Fill(1),
        ],
    );
    let vertical_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Fill(1),
            Constraint::Max(max_size.height),
            Constraint::Fill(1),
        ],
    );
    let [_, horizontal_center, _] = *horizontal_layout.split(area) else {
        unreachable!("length is always three")
    };
    let [_, center, _] = *vertical_layout.split(horizontal_center) else {
        unreachable!("length is always three")
    };

    center
}
