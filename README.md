# Point Tiler

A tool for converting point cloud data (las/laz and csv) into "3D Tiles v1.1".

## install

Rust must be installed. You can easily install it from the following page.

[Getting started](https://www.rust-lang.org/learn/get-started)

After installing Rust, download this repository.

## Usage

### LAS/LAZ

- `input`: Specify the `.las/.laz/.csv/.txt` file. Multiple files can be input separated by spaces.
- `output`: Specify the output folder. Output `tileset.json` and glb.
- `epsg`: Input the epsg code of the las file. All point clouds are recognized as being in the same coordinate system.
- `min`: Specify the minimum zoom level you want to output.
- `max`: Specify the maximum zoom level you want to output.

In the repository root, the following commands can be executed.

```sh
cargo run -r -p app -- --input app/examples/data/sample.las \
    --output app/examples/data/output \
    --epsg 6677 \
    --min 15 \
    --max 18
```

### CSV/TXT

It supports .csv and .txt file extensions.

In CSV format, any columns other than XYZRGB will be ignored.
Also, the column names must be “x”, “y”, “z” and “r”, “g”, “b” or “red”, “green”, “blue”.
(The case of the letters does not matter.)

For example, the following data is valid.

```csv
“X“,”Y“,”Z“,”Intensity“,”ReturnNumber“,”NumberOfReturns“,”ScanDirectionFlag“,”EdgeOfFlightLine“,”Classification“,”Synthetic“,”KeyPoint“,”Withheld“,”Overlap“,”ScanAngleRank“,”UserData“,”PointSourceId“,”GpsTime“,”Red“,”Green“,”Blue”
-5599.971,-35106.097,3.602,680.000,1.000,1.000,1.000,0.000,1.000,0.000,0.00 0,0.000,0.000,-4.000,0.000,95.000,188552.207,19532.000,20046.000,20560.000
-5599.976,-35111.458,3.440,715.000,1.000,1.000,1.000,0.000,1.000,0.000,0.00 0,0.000,0.000,-4.000,0.000,95.000,188552.287,20560.000,21074.000,21588.000
-5599.978,-35115.695,3.543,311.000,1.000,1.000,1.000,0.000,1.000,0.000,0.00 0,0.000,0.000,-4.000,0.000,95.000,188552.347,19018.000,19275.000,20303.000
-5599.992,-35119.757,3.603,722.000,1.000,1.000,1.000,0.000,1.000,0.000,0.00 0,0.000,0.000,-4.000,0.000,95.000,188552.407,18504.000,20046.000,20817.000
-5599.992,-35129.327,3.431,505.000,1.000,1.000,1.000,0.000,1.000,0.000,0.00 0,0.000,0.000,-4.000,0.000,95.000,188552.547,18504.000,19789.000,21074.000
```
