use png::ColorType;

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub channels: Vec<ImageChannel>,
}

#[derive(Clone)]
pub struct ImageChannel {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32, color_type: ColorType, data: &[u8]) -> Image {
        let num_channels = match color_type {
            png::ColorType::Rgb => 3,
            png::ColorType::Rgba => 4,
            _ => panic!("unexpected image color type"),
        };
        let mut chans: Vec<ImageChannel> = vec![];
        for i in 0..num_channels {
            let cbytes = data.iter().skip(i).step_by(num_channels).copied().collect();
            chans.push(ImageChannel {
                width,
                height,
                bytes: cbytes,
            });
        }
        Image {
            width,
            height,
            color_type,
            channels: chans,
        }
    }

    pub fn new_from_channels(
        width: u32,
        height: u32,
        color_type: ColorType,
        chans: Vec<ImageChannel>,
    ) -> Image {
        Image {
            width,
            height,
            color_type,
            channels: chans,
        }
    }

    pub fn apply_to_channels(&self, apply_func: &dyn Fn(&ImageChannel) -> ImageChannel) -> Image {
        // apply to each channel
        let mut out_channels: Vec<ImageChannel> = vec![];
        for ch in &self.channels {
            out_channels.push(apply_func(ch));
        }

        Image::new_from_channels(self.width, self.height, self.color_type, out_channels)
    }
}

impl From<Image> for Vec<u8> {
    fn from(value: Image) -> Self {
        let channel_size = value.channels[0].bytes.len();
        let mut out_bytes: Vec<u8> = Vec::new();
        for i in 0..channel_size {
            for chan in &value.channels {
                out_bytes.push(chan.bytes[i]);
            }
        }
        out_bytes
    }
}

impl ImageChannel {
    pub fn new(width: u32, height: u32, bytes: Vec<u8>) -> Self {
        ImageChannel {
            width,
            height,
            bytes,
        }
    }
}
