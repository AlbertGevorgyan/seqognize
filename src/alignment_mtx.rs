use array2d::Array2D;
use delegate::delegate;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pointer {
    LEFT,
    SUBST,
    UP,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub score: f64,
    pointer: Pointer,
}

impl Element {
    pub fn of(score: f64, pointer: Pointer) -> Self {
        Element { score, pointer }
    }
}

#[derive(Debug, PartialEq)]
pub struct AlignmentMtx {
    array: Array2D<Element>
}

impl AlignmentMtx {
    pub const INITIAL_ELEMENT: Element = Element { score: 0.0, pointer: Pointer::SUBST };
    const OUT_OF_BOUNDS_MSG: &'static str = "Index is out of bounds.";

    pub fn of(num_rows: usize, num_columns: usize) -> AlignmentMtx {
        AlignmentMtx {
            array: Array2D::filled_with(AlignmentMtx::INITIAL_ELEMENT, num_rows, num_columns)
        }
    }

    delegate! {
        to self.array {
            pub fn row_len(&self) -> usize;
            pub fn row_iter(&self, row_index: usize) -> impl Iterator<Item = &Element>;
        }
    }

    pub fn get(&self, row_num: usize, column_num: usize) -> &Element {
        self.array.get(row_num, column_num)
            .expect(AlignmentMtx::OUT_OF_BOUNDS_MSG)
    }

    fn get_score(&self, row_num: usize, column_num: usize) -> f64 {
        self.get_by(row_num, column_num, |el| el.score)
    }

    fn get_pointer(&self, row_num: usize, column_num: usize) -> Pointer {
        self.get_by(row_num, column_num, |el| el.pointer)
    }

    fn get_by<T, F: Fn(&Element) -> T>(&self, row_num: usize, column_num: usize, getter: F) -> T {
        self.array.get(row_num, column_num)
            .map(getter)
            .expect(AlignmentMtx::OUT_OF_BOUNDS_MSG)
    }

    fn set_score(&mut self, row_num: usize, column_num: usize, score: f64) {
        self.set(row_num, column_num, |el| el.score = score);
    }

    fn set_pointer(&mut self, row_num: usize, column_num: usize, pointer: Pointer) {
        self.set(row_num, column_num, |el| el.pointer = pointer);
    }

    fn set<F: FnMut(&mut Element)>(&mut self, row_num: usize, column_num: usize, setter: F) {
        self.array.get_mut(row_num, column_num)
            .map(setter)
            .expect(AlignmentMtx::OUT_OF_BOUNDS_MSG);
    }
}

#[cfg(test)]
mod tests {
    use super::{Pointer, Element, AlignmentMtx};
    use array2d::Array2D;

    #[test]
    fn test_element() {
        let score = 0.0;
        let pointer: Pointer = Pointer::LEFT;
        let el = Element { score, pointer };
        assert_eq!(el, Element { score: 0.0, pointer: Pointer::LEFT });
        assert_ne!(el, Element { score: 0.01, pointer: Pointer::LEFT });
        assert_ne!(el, Element { score: 0.01, pointer: Pointer::UP });
    }

    #[test]
    fn test_mtx() {
        assert_eq!(
            AlignmentMtx::of(10, 5),
            AlignmentMtx {
                array: Array2D::filled_with(AlignmentMtx::INITIAL_ELEMENT, 10, 5)
            }
        )
    }

    #[test]
    fn test_row_len() {
        let mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        assert_eq!(
            mtx.array.row_len(),
            5
        )
    }

    #[test]
    fn test_get() {
        let mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        assert_eq!(
            mtx.get(0, 0),
            &AlignmentMtx::INITIAL_ELEMENT
        )
    }

    #[test]
    fn test_get_score() {
        let mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        assert_eq!(
            mtx.get_score(0, 0),
            0.0
        )
    }

    #[test]
    fn test_get_pointer() {
        let mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        assert_eq!(
            mtx.get_pointer(0, 0),
            Pointer::SUBST
        )
    }

    #[test]
    fn test_set_score() {
        let mut mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        mtx.set_score(0, 0, 1.0);
        assert_eq!(
            mtx.get_score(0, 0),
            1.0
        )
    }

    #[test]
    fn test_set_pointer() {
        let mut mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        mtx.set_pointer(0, 0, Pointer::UP);
        assert_eq!(
            mtx.get_pointer(0, 0),
            Pointer::UP
        )
    }

    #[test]
    #[should_panic(expected = "Index is out of bounds.")]
    fn test_get_score_out_of_bounds() {
        let mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        mtx.get_score(50, 3);
    }

    #[test]
    #[should_panic(expected = "Index is out of bounds.")]
    fn test_set_pointer_out_of_bounds() {
        let mut mtx: AlignmentMtx = AlignmentMtx::of(10, 5);
        mtx.set_pointer(50, 50, Pointer::UP);
    }
}