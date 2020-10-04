use array2d::Array2D;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pointer {
    LEFT,
    SUBST,
    UP,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PointingScore {
    score: f64,
    pointer: Pointer,
}

#[derive(Debug, PartialEq)]
pub struct AlignmentMtx {
    array: Array2D<PointingScore>
}

impl AlignmentMtx {
    pub const INITIAL_ELEMENT: PointingScore = PointingScore { score: 0.0, pointer: Pointer::SUBST };
    const OUT_OF_BOUNDS_MSG: &'static str = "Index is out of bounds.";

    fn of(num_rows: usize, num_columns: usize) -> AlignmentMtx {
        AlignmentMtx {
            array: Array2D::filled_with(AlignmentMtx::INITIAL_ELEMENT, num_rows, num_columns)
        }
    }

    fn get(&self, row_num: usize, column_num: usize) -> &PointingScore {
        self.array.get(row_num, column_num)
            .expect(AlignmentMtx::OUT_OF_BOUNDS_MSG)
    }

    fn get_score(&self, row_num: usize, column_num: usize) -> f64 {
        self.get_by(row_num, column_num, |el| el.score)
    }

    fn get_pointer(&self, row_num: usize, column_num: usize) -> Pointer {
        self.get_by(row_num, column_num, |el| el.pointer)
    }

    fn get_by<T, F: Fn(&PointingScore) -> T>(&self, row_num: usize, column_num: usize, getter: F) -> T {
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

    fn set<F: FnMut(&mut PointingScore)>(&mut self, row_num: usize, column_num: usize, setter: F) {
        self.array.get_mut(row_num, column_num)
            .map(setter)
            .expect(AlignmentMtx::OUT_OF_BOUNDS_MSG);
    }
}

#[cfg(test)]
mod tests {
    use super::{Pointer, PointingScore, AlignmentMtx};
    use array2d::Array2D;

    #[test]
    fn test_element() {
        let score = 0.0;
        let pointer: Pointer = Pointer::LEFT;
        let el = PointingScore { score, pointer };
        assert_eq!(el, PointingScore { score: 0.0, pointer: Pointer::LEFT });
        assert_ne!(el, PointingScore { score: 0.01, pointer: Pointer::LEFT });
        assert_ne!(el, PointingScore { score: 0.01, pointer: Pointer::UP });
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