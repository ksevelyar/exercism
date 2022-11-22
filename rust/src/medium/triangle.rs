pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    fn triangle_inequality(sides: &[u64]) -> bool {
        !sides.iter().any(|side| {
            let other_sides_sum = sides.iter().sum::<u64>() - *side;
            *side == 0 || *side > other_sides_sum
        })
    }

    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if Self::triangle_inequality(&sides) {
            return Some(Triangle { sides });
        }

        None
    }

    fn equal_sides(&self) -> usize {
        self.sides
            .iter()
            .zip(self.sides.iter().cycle().skip(1))
            .filter(|(a, b)| a == b)
            .count()
    }

    pub fn is_equilateral(&self) -> bool {
        self.equal_sides() == 3
    }

    pub fn is_scalene(&self) -> bool {
        self.equal_sides() == 0
    }

    pub fn is_isosceles(&self) -> bool {
        self.equal_sides() >= 1
    }
}

#[test]
fn positive_length_sides_are_ok() {
    let sides = [2, 2, 2];

    let triangle = Triangle::build(sides);

    assert!(triangle.is_some());
}

#[test]
fn one_length_zero_side_second() {
    let sides = [2, 0, 2];

    let triangle = Triangle::build(sides);

    assert!(triangle.is_none());
}

#[test]
fn scalene_triangle_has_no_equal_sides_four() {
    let sides = [5, 4, 2];

    let triangle = Triangle::build(sides).unwrap();

    assert!(!triangle.is_equilateral());

    assert!(!triangle.is_isosceles());

    assert!(triangle.is_scalene());
}

#[test]
fn isosceles_triangles_have_two_equal_sides_one() {
    let sides = [3, 4, 4];

    let triangle = Triangle::build(sides).unwrap();

    assert!(triangle.is_isosceles());
}

#[test]
fn sum_of_two_sides_must_equal_or_exceed_the_remaining_side_one() {
    let sides = [7, 3, 2];

    let triangle = Triangle::build(sides);

    assert!(triangle.is_none());
}
