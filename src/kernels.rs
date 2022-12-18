
pub struct Kernel {
  pub width: u32,
  pub data: &'static [f64],
}

pub const GAUSSIAN_SIMPLE: Kernel = Kernel { width: 3,
  data: &[0.03125f64, 0.09375f64, 0.03125f64,
          0.09375f64, 0.50000f64, 0.09375f64,
          0.03125f64, 0.09375f64, 0.03125f64] };

#[allow(unused)]
pub const IDENTITY1: Kernel = Kernel { width: 1, data: &[1f64] };
#[allow(unused)]
pub const IDENTITY2: Kernel = Kernel { width: 3, data: &[0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 0f64] };
#[allow(unused)]
pub const IDENTITY3: Kernel = Kernel { width: 3, data: &[0f64, 1f64, 0f64] };
// pub const GAUSSIAN_SIMPLE: [[f64; 3]; 3] = [&[0.0625f64, 0.1875f64, 0.0625f64],
//                                              &[0.1875f64, 0.5f64, 0.1875f64],
//                                              &[0.0625f64, 0.1875f64, 0.0625f64]];