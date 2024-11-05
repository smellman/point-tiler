use std::{collections::HashMap, error::Error, path::PathBuf};

use las::Reader;

use pcd_core::pointcloud::point::{
    BoundingVolume, Color, Metadata, Point, PointAttributes, PointCloud,
};

use super::{Parser, ParserProvider};

pub struct LasParserProvider {
    pub filenames: Vec<PathBuf>,
}

impl ParserProvider for LasParserProvider {
    fn get_parser(&self) -> Box<dyn Parser> {
        Box::new(LasParser {
            filenames: self.filenames.clone(),
        })
    }
}

pub struct LasParser {
    pub filenames: Vec<PathBuf>,
}

pub static SCALE_FACTOR: f64 = 0.001;

impl Parser for LasParser {
    fn parse(&self) -> Result<PointCloud, Box<dyn Error>> {
        let start = std::time::Instant::now();
        let mut reader = Reader::from_path(&self.filenames[0]).unwrap();
        println!("Read LAS time: {:?}", start.elapsed());

        let mut points = Vec::new();
        let mut bounding_volume = BoundingVolume {
            min: [f64::MAX, f64::MAX, f64::MAX],
            max: [f64::MIN, f64::MIN, f64::MIN],
        };
        let mut digits_x = 3;
        let mut digits_y = 3;
        let mut digits_z = 3;

        let mut point_count = 0;

        let start = std::time::Instant::now();
        for las_point in reader.points() {
            let las_point = las_point.unwrap();

            bounding_volume.max[0] = bounding_volume.max[0].max(las_point.x);
            bounding_volume.max[1] = bounding_volume.max[1].max(las_point.y);
            bounding_volume.max[2] = bounding_volume.max[2].max(las_point.z);
            bounding_volume.min[0] = bounding_volume.min[0].min(las_point.x);
            bounding_volume.min[1] = bounding_volume.min[1].min(las_point.y);
            bounding_volume.min[2] = bounding_volume.min[2].min(las_point.z);

            for (value, digits) in [
                (las_point.x, &mut digits_x),
                (las_point.y, &mut digits_y),
                (las_point.z, &mut digits_z),
            ] {
                let value_str = format!("{:.7}", value);
                if let Some(dot_index) = value_str.find('.') {
                    let fractional_part = &value_str[dot_index + 1..];
                    let fractional_part = fractional_part.trim_end_matches('0');
                    *digits = *digits.max(&mut fractional_part.len());
                }
            }

            point_count += 1;
        }

        let scale_x: f64 = format!("{:.*}", digits_x, 0.1_f64.powi(digits_x as i32)).parse()?;
        let scale_y: f64 = format!("{:.*}", digits_y, 0.1_f64.powi(digits_y as i32)).parse()?;
        let scale_z: f64 = format!("{:.*}", digits_z, 0.1_f64.powi(digits_z as i32)).parse()?;

        let min_x = bounding_volume.min[0];
        let min_y = bounding_volume.min[1];
        let min_z = bounding_volume.min[2];

        let offset_x = min_x / scale_x;
        let offset_y = min_y / scale_y;
        let offset_z = min_z / scale_z;

        let metadata = Metadata {
            point_count,
            bounding_volume,
            coordinate_system_wkt: "PROJCS[\"JGD2011 / Japan Plane Rectangular CS VII\",...]"
                .to_string(),
            scale: [scale_x, scale_y, scale_z],
            offset: [offset_x, offset_y, offset_z],
            other: HashMap::new(),
        };

        println!("Calc bounding_volume time: {:?}", start.elapsed());

        let start = std::time::Instant::now();
        // TODO: 1度目のループで消費されてしまうので、再度読み込みを行っているが、改修が必要
        let mut reader = Reader::from_path(&self.filenames[0]).unwrap();
        for las_point in reader.points() {
            let las_point = las_point.unwrap();

            let x = las_point.x;
            let y = las_point.y;
            let z = las_point.z;

            let color = las_point.color.map(|c| Color {
                r: c.red,
                g: c.green,
                b: c.blue,
            });

            let attributes = PointAttributes {
                intensity: Some(las_point.intensity),
                return_number: Some(las_point.return_number),
                classification: Some(format!("{:?}", las_point.classification)),
                scanner_channel: Some(las_point.user_data),
                scan_angle: Some(las_point.scan_angle),
                user_data: Some(las_point.user_data),
                point_source_id: Some(las_point.point_source_id),
                gps_time: Some(las_point.gps_time.unwrap_or(0.0)),
            };

            let point = Point {
                x: ((x * scale_x) + offset_x) as i32,
                y: ((y * scale_y) + offset_y) as i32,
                z: ((z * scale_z) + offset_z) as i32,
                color: color.unwrap_or(Color { r: 0, g: 0, b: 0 }),
                attributes,
            };

            points.push(point);
        }
        println!("Build PointCloud time: {:?}", start.elapsed());

        Ok(PointCloud { points, metadata })
    }
}
