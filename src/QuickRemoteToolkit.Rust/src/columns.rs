use egui_extras::Column;

pub struct Columns<const N: usize> {
    pub order: [usize; N],
    pub widths: [f32; N],
    pub reset_pending: bool,
    stretch: usize,
}
impl<const N: usize> Columns<N> {
    pub fn new(widths: [f32; N], stretch: usize) -> Self {
        Self {
            order: std::array::from_fn(|i| i),
            widths,
            reset_pending: false,
            stretch,
        }
    }
    pub fn spec(&self, column: usize) -> Column {
        if column == self.stretch {
            Column::remainder()
        } else {
            Column::initial(self.widths[column])
        }
        .at_least(20.)
        .clip(true)
    }
    pub fn minimum_width(&self) -> f32 {
        self.widths
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != self.stretch)
            .map(|(_, w)| *w)
            .sum::<f32>()
            + 20.
    }
    pub fn remember_widths(&mut self, order: &[usize; N], widths: &[f32]) {
        for (&column, &width) in order.iter().zip(widths) {
            self.widths[column] = width;
        }
    }
    pub fn move_to(&mut self, column: usize, destination: usize) {
        if let (Some(from), Some(to)) = (
            self.order.iter().position(|c| *c == column),
            self.order.iter().position(|c| *c == destination),
        ) && from != to
        {
            if from < to {
                self.order[from..=to].rotate_left(1);
            } else {
                self.order[to..=from].rotate_right(1);
            }
            self.reset_pending = true;
        }
    }
}
#[derive(Clone)]
pub struct DraggedColumn {
    pub table: &'static str,
    pub column: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reordering_keeps_logical_widths_and_other_tables_independent() {
        let mut a = Columns::new([48., 58., 190., 150.], 3);
        let b = Columns::new([155., 180., 230., 20.], 3);
        a.remember_widths(&[0, 1, 2, 3], &[60., 70., 210., 300.]);
        a.move_to(2, 0);
        assert_eq!(a.order, [2, 0, 1, 3]);
        assert_eq!(a.widths[2], 210.);
        assert_eq!(b.order, [0, 1, 2, 3]);
        a.move_to(2, 3);
        assert_eq!(a.order, [0, 1, 3, 2]);
        assert!(a.reset_pending);
        a.move_to(99, 0);
        assert_eq!(a.order, [0, 1, 3, 2]);
    }
}
