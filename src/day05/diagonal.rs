use std::{convert::TryInto, ops::RangeInclusive};

#[derive(Debug, PartialEq)]
pub struct Diagonal {
    pub start: (u32, u32),
    pub length: u32,
    pub is_positive_y: bool,
}
impl Diagonal {
    pub fn points(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        std::iter::successors(Some(self.start), move |prev| {
            let y = if self.is_positive_y {
                Some(prev.1 + 1)
            } else {
                if prev.1 == 0 {
                    // for whatever reason, take appears to evaluate the underlying
                    // iterator even if the element shouldn't be taken, so this is
                    // not unreachable even if the start/length is correct.
                    None
                } else {
                    Some(prev.1 - 1)
                }
            }?;
            Some((prev.0 + 1, y))
        })
        .take(self.length.try_into().unwrap())
    }

    pub fn intersection(&self, other: &Diagonal) -> Option<(u32, u32)> {
        if self.is_positive_y == other.is_positive_y {
            // either (positive) -x + y == c, or (negative) x + y == c.
            // if c's are equal, the lines are colinear.
            fn c_coefficient(d: &Diagonal) -> i32 {
                if d.is_positive_y {
                    -(d.start.0 as i32) + (d.start.1 as i32)
                } else {
                    (d.start.0 + d.start.1) as i32
                }
            }
            if c_coefficient(self) == c_coefficient(other) {
                let overlap_range = RangeInclusive::new(
                    self.start.0.max(other.start.0),
                    (self.start.0 + self.length - 1).min(other.start.0 + other.length - 1)
                );
                if overlap_range.is_empty() {
                    None
                } else {
                    todo!("Enumerate colinear diagonal overlap")
                }
            } else {
                None
            }
        } else {
            let (pos, neg) = if self.is_positive_y {
                (self, other)
            } else {
                (other, self)
            };
            // a1x + b1y = c1
            // a2x + b2y = c2
            // -> x = (c' - b'y) / a'
            //      = a'.c' - b'.(c1 - a1.x) / a'.b1
            //      = [ a'.c' - b'.c1 / a'.b1 ] / (1 + b'.a1 / a'.b1)

            // or
            // ppos + (lpos, lpos) == pneg + (lneg, -lneg)
            // xpos - xneg == lneg - lpos
            // ypos - yneg == -lneg - lpos
            // -> dx + dy = -2 lpos
            return take_intersect(pos, neg);
            fn take_intersect(pos: &Diagonal, neg: &Diagonal) -> Option<(u32, u32)> {
                let pos_sum = pos.start.0 + pos.start.1;
                let neg_sum = neg.start.0 + neg.start.1;
                let diff = neg_sum.checked_sub(pos_sum)?;
                let lpos = if diff % 2 == 0 { Some(diff / 2) } else { None }?;
                let lneg = (pos.start.0 + lpos).checked_sub(neg.start.0)?;
                if lpos <= pos.length && lneg <= neg.length {
                    Some((pos.start.0 + lpos, pos.start.1 + lpos))
                } else {
                    None
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn range_can_be_inside_out() {
        let r = 10..=0;
        assert!(r.is_empty())
    }
}