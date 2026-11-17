use std::collections::BTreeSet;

#[derive(Default)]
pub struct Selection {
    pub rows: BTreeSet<usize>,
    pub current: Option<usize>,
    anchor: Option<usize>,
}
impl Selection {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    pub fn prepend(&mut self) {
        self.rows = self.rows.iter().map(|i| i + 1).collect();
        self.current = self.current.map(|i| i + 1);
        self.anchor = self.anchor.map(|i| i + 1);
    }
    pub fn select(&mut self, order: &[usize], row: usize, ctrl: bool, shift: bool) {
        if shift {
            let start = self.anchor.and_then(|i| order.iter().position(|v| *v == i));
            let end = order.iter().position(|v| *v == row);
            if let (Some(a), Some(b)) = (start, end) {
                if !ctrl {
                    self.rows.clear();
                }
                self.rows.extend(&order[a.min(b)..=a.max(b)]);
            } else {
                self.rows.insert(row);
            }
        } else {
            if ctrl {
                if !self.rows.remove(&row) {
                    self.rows.insert(row);
                }
            } else {
                self.rows.clear();
                self.rows.insert(row);
            }
            self.anchor = Some(row);
        }
        self.current = Some(row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ranges_follow_display_order_and_survive_incoming_logs() {
        let mut s = Selection::default();
        let order = [4, 1, 3, 0, 2];
        s.select(&order, 1, false, false);
        s.select(&order, 0, false, true);
        assert_eq!(s.rows, [0, 1, 3].into());
        s.select(&order, 3, true, false);
        assert_eq!(s.rows, [0, 1].into());
        s.prepend();
        assert_eq!(s.rows, [1, 2].into());
        assert_eq!(s.current, Some(4));
        s.clear();
        assert!(s.rows.is_empty());
        assert_eq!(s.current, None);
    }
}
