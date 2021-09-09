use std::convert::*;



pub trait TryIntoXY                                 { fn try_into_xy(self) -> Option<(usize, usize)>; }
impl<T: Copy + TryInto<usize>> TryIntoXY for (T, T) { fn try_into_xy(self) -> Option<(usize, usize)> { Some((self.0.try_into().ok()?, self.1.try_into().ok()?)) } }
impl<T: Copy + TryInto<usize>> TryIntoXY for [T; 2] { fn try_into_xy(self) -> Option<(usize, usize)> { Some((self[0].try_into().ok()?, self[1].try_into().ok()?)) } }
