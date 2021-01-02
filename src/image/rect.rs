use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Rect {
    pub pos:  I2,
    pub dims: I2,
}

impl Rect {
    #[inline(always)] pub const fn new(pos: I2, dims: I2) -> Self
    { Self { pos, dims } }

    #[inline(always)] pub fn area(&self) -> I { self.dims.product() }

    #[inline(always)] pub fn length(&self) -> I { self.dims.reduce(Num::max) }

    #[inline(always)] pub fn flatten_rel_pos(&self, pos: I2) -> I
    { pos[Y] * self.dims[X] + pos[X] }

    #[inline(always)] pub fn flatten_abs_pos(&self, pos: I2) -> I
    { self.flatten_rel_pos(pos - self.pos) }

    #[inline(always)]
    pub fn chunks(&self) -> impl Iterator<Item=Self> {
        let Self { pos: A2(px, py), dims: A2(dx, dy) } = *self;
        let end = A2(px+dx, py+dy);

        (py..end[Y]).step_by(BLOCK_SIZE[Y] as usize).flat_map(move |y| {
            (px..end[X]).step_by(BLOCK_SIZE[X] as usize).map(move |x| {
                let p = A2(x, y);
                Self::new(p, BLOCK_SIZE.zip(end - p, Num::min))
            })
        })
    }

    #[inline(always)] pub fn positions(&self) -> impl Iterator<Item=I2> {
        let A2(px, py) = self.pos;
        let end = A2(px+self.dims[X], py+self.dims[Y]);
        (py..end[Y]).flat_map(move |y| (px..end[X]).map(move |x| A2(x, y)))
    }
}
