use csv::{Reader, ReaderBuilder};
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use linfa::{DatasetBase, prelude::Records};
use ndarray::{Array2, ArrayBase, Axis, Dim, OwnedRepr};
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea, PathElement, Rectangle},
    series::LineSeries,
    style::{BLACK, Color, GREEN, IntoFont, RED, WHITE},
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs::File};

fn read_csv(path: &str) -> Result<Reader<DecodeReaderBytes<File, Vec<u8>>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    Ok(ReaderBuilder::new()
        .has_headers(true)
        .from_reader(transcoded))
}

fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" | "1" | "y" | "yes" => Ok(true),
        "false" | "0" | "n" | "no" => Ok(false),
        other => Err(serde::de::Error::custom(format!("invalid bool: {}", other))),
    }
}

#[derive(Debug, Deserialize)]
struct Product {
    #[serde(rename = "Category")]
    category: String,
    name: String,
    mrp: f64,
    #[serde(rename = "discountPercent")]
    discount_percent: f64,
    #[serde(rename = "availableQuantity")]
    available_quantity: f64,
    #[serde(rename = "discountedSellingPrice")]
    discounted_selling_price: f64,
    #[serde(rename = "weightInGms")]
    weight_in_gms: f64,
    #[serde(rename = "outOfStock", deserialize_with = "bool_from_string")]
    out_of_stock: bool,
    quantity: f64,
}

fn make_dataset(
    reader: &mut Reader<DecodeReaderBytes<File, Vec<u8>>>,
) -> Result<
    DatasetBase<
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
    >,
    Box<dyn Error>,
> {
    let mut records: Vec<Vec<f64>> = Vec::new();
    for result in reader.deserialize::<Product>() {
        let product = result?;
        records.push(vec![
            product.mrp,
            product.discount_percent,
            product.available_quantity,
            product.discounted_selling_price,
            product.quantity,
        ])
    }

    let nrows = records.len();
    let ncols = records[0].len();

    let flat = records.into_iter().flatten().collect();
    let data = Array2::from_shape_vec((nrows, ncols), flat)?;

    let (features, target_col) = data.view().split_at(Axis(1), ncols - 1);
    let features = features.to_owned();
    let targets = target_col.column(0).to_owned();

    let dataset = DatasetBase::new(features, targets);
    Ok(dataset)
}

fn plot(
    reader: &mut Reader<DecodeReaderBytes<File, Vec<u8>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut category_counts: HashMap<String, (i64, i64)> = HashMap::new();
    for result in reader.deserialize::<Product>() {
        let product = result?;
        let is_out_of_stock = matches!(product.out_of_stock, true);
        let entry = category_counts.entry(product.category).or_insert((0, 0));
        if is_out_of_stock {
            entry.1 += 1;
        } else {
            entry.0 += 1;
        }
    }

    let mut data: Vec<_> = category_counts.into_iter().collect();
    data.sort_by_key(|(_, (in_stock, out_of_stock))| -(in_stock + out_of_stock));

    let categories: Vec<String> = data.iter().map(|(c, _)| c.clone()).collect();
    let max_count = data
        .iter()
        .map(|(_, (in_stock, out_of_stock))| in_stock + out_of_stock)
        .max()
        .unwrap_or(0);

    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let num_categories = categories.len() as i32;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "In-Stock vs Out-of-Stock per Category",
            ("sans-serif", 30).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(150)
        .build_cartesian_2d(0i64..(max_count + 10), 0i32..num_categories)?;

    chart
        .configure_mesh()
        .x_desc("Number of Products")
        .y_desc("Category")
        .y_labels(categories.len())
        .y_label_formatter(&|i| {
            categories.get(*i as usize).cloned().unwrap_or_default()
        })
        .draw()?;

    chart
        .draw_series(
            data.iter()
                .enumerate()
                .map(|(i, (_, (in_stock, out_of_stock)))| {
                    let y = i as i32;
                    let in_stock_bar =
                        Rectangle::new([(0i64, y), (*in_stock, y + 1)], GREEN.filled());
                    let out_of_stock_bar = Rectangle::new(
                        [(*in_stock, y), (*in_stock + out_of_stock, y + 1)],
                        RED.filled(),
                    );
                    vec![in_stock_bar, out_of_stock_bar]
                })
                .flatten(),
        )?
        .label("categories");

    chart
        .draw_series(std::iter::empty::<Rectangle<(i64, i32)>>())?
        .label("In Stock")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], GREEN.filled()));

    chart
        .draw_series(std::iter::empty::<Rectangle<(i64, i32)>>())?
        .label("Out of Stock")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], RED.filled()));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

pub fn run() {
    let mut reader = read_csv("datasets/zepto-inventory/zepto_v2.csv").unwrap();
    plot(&mut reader).expect("Failed to create plot");

    // Create a new reader for make_dataset since the first one is exhausted
    let mut reader2 = read_csv("datasets/zepto-inventory/zepto_v2.csv").unwrap();
    let dataset = make_dataset(&mut reader2).unwrap();
    println!("{}, {}", dataset.nsamples(), dataset.nfeatures())
}
