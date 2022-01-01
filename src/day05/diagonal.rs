use std::convert::TryInto;

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
                
                todo!("Check for colinearity")
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

