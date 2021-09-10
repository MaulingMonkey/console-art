pub trait Ratio                 { fn to_0n_clamped(self, width: usize) -> usize; }
impl Ratio for f32              { fn to_0n_clamped(self, width: usize) -> usize { let n = width as Self; if self.is_nan() { 0 } else { (self * n).clamp(0.0, n) as _ } } }
impl Ratio for f64              { fn to_0n_clamped(self, width: usize) -> usize { let n = width as Self; if self.is_nan() { 0 } else { (self * n).clamp(0.0, n) as _ } } }
impl Ratio for (f32, f32)       { fn to_0n_clamped(self, width: usize) -> usize { (self.0 / self.1).to_0n_clamped(width) } }
impl Ratio for (f64, f64)       { fn to_0n_clamped(self, width: usize) -> usize { (self.0 / self.1).to_0n_clamped(width) } }

impl Ratio for (u64, u64) {
    fn to_0n_clamped(self, width: usize) -> usize {
        let width = width as u64;
        let n = self.1;
        let i = self.0.min(n);
        match n.checked_mul(width) {
            Some(_)             => (i * width / n) as usize,
            None if n < width   => (i as f64, n as f64).to_0n_clamped(width as usize),
            None                => (i / (n / width)).min(width) as usize,
        }
    }
}

impl Ratio for (usize, usize) {
    fn to_0n_clamped(self, width: usize) -> usize {
        let n = self.1;
        let i = self.0.min(n);
        match n.checked_mul(width) {
            Some(_)             => i * width / n,
            None if n < width   => (i as f64, n as f64).to_0n_clamped(width),
            None                => (i / (n / width)).min(width),
        }
    }
}

impl Ratio for (u32, u32) { fn to_0n_clamped(self, width: usize) -> usize { (self.0 as usize, self.1 as usize).to_0n_clamped(width) } }
impl Ratio for (u16, u16) { fn to_0n_clamped(self, width: usize) -> usize { (self.0 as usize, self.1 as usize).to_0n_clamped(width) } }
impl Ratio for (u8,  u8 ) { fn to_0n_clamped(self, width: usize) -> usize { (self.0 as usize, self.1 as usize).to_0n_clamped(width) } }

#[test] fn ratios_float_120() {
    assert_eq!(0.0f32.to_0n_clamped(120),   0);
    assert_eq!(1.0f32.to_0n_clamped(120), 120);
    assert_eq!(0.0f64.to_0n_clamped(120),   0);
    assert_eq!(1.0f64.to_0n_clamped(120), 120);
}

#[test] fn ratios_float_float_120() {
    assert_eq!((0.0f32, 2.0f32).to_0n_clamped(120),   0);
    assert_eq!((2.0f32, 2.0f32).to_0n_clamped(120), 120);
    assert_eq!((0.0f64, 2.0f64).to_0n_clamped(120),   0);
    assert_eq!((2.0f64, 2.0f64).to_0n_clamped(120), 120);
}

#[test] fn ratios_7_120() {
    assert_eq!((0u8,    7u8     ).to_0n_clamped(  0),   0);
    assert_eq!((7u8,    7u8     ).to_0n_clamped(120), 120);
    assert_eq!((0u16,   7u16    ).to_0n_clamped(  0),   0);
    assert_eq!((7u16,   7u16    ).to_0n_clamped(120), 120);
    assert_eq!((0u32,   7u32    ).to_0n_clamped(  0),   0);
    assert_eq!((7u32,   7u32    ).to_0n_clamped(120), 120);
    assert_eq!((0u64,   7u64    ).to_0n_clamped(  0),   0);
    assert_eq!((7u64,   7u64    ).to_0n_clamped(120), 120);
    assert_eq!((0usize, 7usize  ).to_0n_clamped(  0),   0);
    assert_eq!((7usize, 7usize  ).to_0n_clamped(120), 120);
}

#[test] fn ratios_8_120() {
    assert_eq!((0u8,    8u8     ).to_0n_clamped(  0),   0);
    assert_eq!((8u8,    8u8     ).to_0n_clamped(120), 120);
    assert_eq!((0u16,   8u16    ).to_0n_clamped(  0),   0);
    assert_eq!((8u16,   8u16    ).to_0n_clamped(120), 120);
    assert_eq!((0u32,   8u32    ).to_0n_clamped(  0),   0);
    assert_eq!((8u32,   8u32    ).to_0n_clamped(120), 120);
    assert_eq!((0u64,   8u64    ).to_0n_clamped(  0),   0);
    assert_eq!((8u64,   8u64    ).to_0n_clamped(120), 120);
    assert_eq!((0usize, 8usize  ).to_0n_clamped(  0),   0);
    assert_eq!((8usize, 8usize  ).to_0n_clamped(120), 120);
}

#[test] fn ratios_9_120() {
    assert_eq!((0u8,    9u8     ).to_0n_clamped(  0),   0);
    assert_eq!((9u8,    9u8     ).to_0n_clamped(120), 120);
    assert_eq!((0u16,   9u16    ).to_0n_clamped(  0),   0);
    assert_eq!((9u16,   9u16    ).to_0n_clamped(120), 120);
    assert_eq!((0u32,   9u32    ).to_0n_clamped(  0),   0);
    assert_eq!((9u32,   9u32    ).to_0n_clamped(120), 120);
    assert_eq!((0u64,   9u64    ).to_0n_clamped(  0),   0);
    assert_eq!((9u64,   9u64    ).to_0n_clamped(120), 120);
    assert_eq!((0usize, 9usize  ).to_0n_clamped(  0),   0);
    assert_eq!((9usize, 9usize  ).to_0n_clamped(120), 120);
}

#[test] fn ratios_max_120() {
    assert_eq!(( 0u8,       !0u8    ).to_0n_clamped(  0),   0);
    assert_eq!((!0u8,       !0u8    ).to_0n_clamped(120), 120);
    assert_eq!(( 0u16,      !0u16   ).to_0n_clamped(  0),   0);
    assert_eq!((!0u16,      !0u16   ).to_0n_clamped(120), 120);
    assert_eq!(( 0u32,      !0u32   ).to_0n_clamped(  0),   0);
    assert_eq!((!0u32,      !0u32   ).to_0n_clamped(120), 120);
    assert_eq!(( 0u64,      !0u64   ).to_0n_clamped(  0),   0);
    assert_eq!((!0u64,      !0u64   ).to_0n_clamped(120), 120);
    assert_eq!(( 0usize,    !0usize ).to_0n_clamped(  0),   0);
    assert_eq!((!0usize,    !0usize ).to_0n_clamped(120), 120);
}



#[test] fn ratios_float_max() {
    assert_eq!(0.0f32.to_0n_clamped( !0),   0);
    assert_eq!(1.0f32.to_0n_clamped( !0),  !0);
    assert_eq!(0.0f64.to_0n_clamped( !0),   0);
    assert_eq!(1.0f64.to_0n_clamped( !0),  !0);
}

#[test] fn ratios_float_float_max() {
    assert_eq!((0.0f32, 2.0f32).to_0n_clamped( !0),   0);
    assert_eq!((2.0f32, 2.0f32).to_0n_clamped( !0),  !0);
    assert_eq!((0.0f64, 2.0f64).to_0n_clamped( !0),   0);
    assert_eq!((2.0f64, 2.0f64).to_0n_clamped( !0),  !0);
}

#[test] fn ratios_7_max() {
    assert_eq!((0u8,    7u8     ).to_0n_clamped(  0),   0);
    assert_eq!((7u8,    7u8     ).to_0n_clamped( !0),  !0);
    assert_eq!((0u16,   7u16    ).to_0n_clamped(  0),   0);
    assert_eq!((7u16,   7u16    ).to_0n_clamped( !0),  !0);
    assert_eq!((0u32,   7u32    ).to_0n_clamped(  0),   0);
    assert_eq!((7u32,   7u32    ).to_0n_clamped( !0),  !0);
    assert_eq!((0u64,   7u64    ).to_0n_clamped(  0),   0);
    assert_eq!((7u64,   7u64    ).to_0n_clamped( !0),  !0);
    assert_eq!((0usize, 7usize  ).to_0n_clamped(  0),   0);
    assert_eq!((7usize, 7usize  ).to_0n_clamped( !0),  !0);
}

#[test] fn ratios_8_max() {
    assert_eq!((0u8,    8u8     ).to_0n_clamped(  0),   0);
    assert_eq!((8u8,    8u8     ).to_0n_clamped( !0),  !0);
    assert_eq!((0u16,   8u16    ).to_0n_clamped(  0),   0);
    assert_eq!((8u16,   8u16    ).to_0n_clamped( !0),  !0);
    assert_eq!((0u32,   8u32    ).to_0n_clamped(  0),   0);
    assert_eq!((8u32,   8u32    ).to_0n_clamped( !0),  !0);
    assert_eq!((0u64,   8u64    ).to_0n_clamped(  0),   0);
    assert_eq!((8u64,   8u64    ).to_0n_clamped( !0),  !0);
    assert_eq!((0usize, 8usize  ).to_0n_clamped(  0),   0);
    assert_eq!((8usize, 8usize  ).to_0n_clamped( !0),  !0);
}

#[test] fn ratios_9_max() {
    assert_eq!((0u8,    9u8     ).to_0n_clamped(  0),   0);
    assert_eq!((9u8,    9u8     ).to_0n_clamped( !0),  !0);
    assert_eq!((0u16,   9u16    ).to_0n_clamped(  0),   0);
    assert_eq!((9u16,   9u16    ).to_0n_clamped( !0),  !0);
    assert_eq!((0u32,   9u32    ).to_0n_clamped(  0),   0);
    assert_eq!((9u32,   9u32    ).to_0n_clamped( !0),  !0);
    assert_eq!((0u64,   9u64    ).to_0n_clamped(  0),   0);
    assert_eq!((9u64,   9u64    ).to_0n_clamped( !0),  !0);
    assert_eq!((0usize, 9usize  ).to_0n_clamped(  0),   0);
    assert_eq!((9usize, 9usize  ).to_0n_clamped( !0),  !0);
}

#[test] fn ratios_max_max() {
    assert_eq!(( 0u8,       !0u8    ).to_0n_clamped(  0),   0);
    assert_eq!((!0u8,       !0u8    ).to_0n_clamped( !0),  !0);
    assert_eq!(( 0u16,      !0u16   ).to_0n_clamped(  0),   0);
    assert_eq!((!0u16,      !0u16   ).to_0n_clamped( !0),  !0);
    assert_eq!(( 0u32,      !0u32   ).to_0n_clamped(  0),   0);
    assert_eq!((!0u32,      !0u32   ).to_0n_clamped( !0),  !0);
    assert_eq!(( 0u64,      !0u64   ).to_0n_clamped(  0),   0);
    assert_eq!((!0u64,      !0u64   ).to_0n_clamped( !0),  !0);
    assert_eq!(( 0usize,    !0usize ).to_0n_clamped(  0),   0);
    assert_eq!((!0usize,    !0usize ).to_0n_clamped( !0),  !0);
}
