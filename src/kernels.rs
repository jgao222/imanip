pub struct Kernel {
    pub width: u32,
    pub data: &'static [f64],
}

pub const GAUSSIAN_SIMPLE: Kernel = Kernel {
    width: 3,
    data: &[
        0.03125f64, 0.09375f64, 0.03125f64, 0.09375f64, 0.50000f64, 0.09375f64, 0.03125f64,
        0.09375f64, 0.03125f64,
    ],
};

pub const GAUSSIAN_X: Kernel = Kernel {
    width: 7,
    data: &[
        0.000817212785_f64,
        0.02804152135_f64,
        0.2339264272_f64,
        0.4744296773_f64,
        0.2339264272_f64,
        0.02804152135_f64,
        0.000817212785_f64,
    ],
};

pub const GAUSSIAN_Y: Kernel = Kernel {
    width: 1,
    data: &[
        0.000817212785_f64,
        0.02804152135_f64,
        0.2339264272_f64,
        0.4744296773_f64,
        0.2339264272_f64,
        0.02804152135_f64,
        0.000817212785_f64,
    ],
};

#[allow(unused)]
pub const IDENTITY1: Kernel = Kernel {
    width: 1,
    data: &[1f64],
};
#[allow(unused)]
pub const IDENTITY2: Kernel = Kernel {
    width: 3,
    data: &[0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 0f64],
};
#[allow(unused)]
pub const IDENTITY3: Kernel = Kernel {
    width: 3,
    data: &[0f64, 1f64, 0f64],
};

pub const LAPLACIAN: Kernel = Kernel {
    width: 3,
    data: &[0f64, 1f64, 0f64, 1f64, -4f64, 1f64, 0f64, 1f64, 0f64],
};
