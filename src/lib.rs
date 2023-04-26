use core::num;

use image::{ImageBuffer, Pixel, Rgb};

pub fn adjust_exposure(pixel: &mut Rgb<u8>, exposure: f32) {
    pixel[0] = f32::max((pixel[0] as f32) * f32::powf(2.0, exposure), u8::MAX as f32) as u8;
    pixel[1] = f32::max((pixel[1] as f32) * f32::powf(2.0, exposure), u8::MAX as f32) as u8;
    pixel[2] = f32::max((pixel[2] as f32) * f32::powf(2.0, exposure), u8::MAX as f32) as u8;
}

pub struct Partition {
    pub xmin: u32,
    pub xmax: u32,
    pub ymin: u32,
    pub ymax: u32,
}

pub fn get_partitions(width: u32, height: u32, num_partitions: u32) -> Vec<Partition> {
    let mut partitions: Vec<Partition> = Vec::new();
    for i in 0..num_partitions {
        let partition = Partition {
            xmin: 0,
            xmax: width - 1,
            ymin: i * height / num_partitions,
            ymax: (i + 1) * height / num_partitions - 1,
        };
        partitions.push(partition);
    }
    partitions
}

mod tests {
    use super::*;

    #[test]
    fn test_get_partitions() {
        let partitions = get_partitions(100, 200, 5);
        assert_eq!(partitions.len(), 5);
        assert_eq!(partitions[0].xmin, 0);
        assert_eq!(partitions[0].xmax, 99);
        assert_eq!(partitions[0].ymin, 0);
        assert_eq!(partitions[0].ymax, 39);
        assert_eq!(partitions[1].xmin, 0);
        assert_eq!(partitions[1].xmax, 99);
        assert_eq!(partitions[1].ymin, 40);
        assert_eq!(partitions[1].ymax, 79);
        assert_eq!(partitions[2].xmin, 0);
        assert_eq!(partitions[2].xmax, 99);
        assert_eq!(partitions[2].ymin, 80);
        assert_eq!(partitions[2].ymax, 119);
        assert_eq!(partitions[3].xmin, 0);
        assert_eq!(partitions[3].xmax, 99);
        assert_eq!(partitions[3].ymin, 120);
        assert_eq!(partitions[3].ymax, 159);
        assert_eq!(partitions[4].xmin, 0);
        assert_eq!(partitions[4].xmax, 99);
        assert_eq!(partitions[4].ymin, 160);
        assert_eq!(partitions[4].ymax, 199);
    }
}
