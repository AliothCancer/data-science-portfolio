#![allow(unused_variables, dead_code)]


use std::{error::Error, path::PathBuf, time::Instant};

use clap::Parser;
use file_utility::{self, arg_parsing::Cli};
// use plotlars::{HeatMap, Palette, Plot, Text};
use polars::prelude::*;
use scirs2::{datasets::utils::Array2, stats::corrcoef};

/*
Todo:
1. Divide Nominal and Ordinal features
2. Visualization:
    - Ordinal:
        HEATMAP: Correlation matrix
        Report: R² rank
    - Nominal:
        HISTOGRAM: Absolute frequencies
3. Multiple input - Output analysis
4. Variance analysis
    - .. -> Sobel indexes
*/


fn main() -> Result<(), Box<dyn Error>> {
    let Cli {
        data_path,
        feature_desc_path,
    } = Cli::parse();


    let dataset = Dataset::try_new(data_path, feature_desc_path.unwrap())?;
    let numeric_data_owned = dataset.numerical_features.clone().collect()?;
    let numeric_data_owned_clone = numeric_data_owned.clone();
    let start = Instant::now();
    println!("{}", corr(numeric_data_owned_clone));
    let delta_time = start.elapsed();
    println!("polars corr: {} ms", delta_time.as_millis());

    let start = Instant::now();
    println!(
        "{}",
        heatmap(numeric_data_owned, dataset.numerical_features)?
    );
    let delta_time = start.elapsed();

    println!("scirs corr: {} ms", delta_time.as_millis());
    Ok(())
}

struct Dataset {
    data: LazyFrame,
    numerical_features: LazyFrame,
}
impl Dataset {
    fn try_new(data_path: PathBuf, feature_desc_path: PathBuf) -> Result<Self, PolarsError> {
        let data = read_and_return_df(data_path)?;
        let numerical_features = data
            .clone()
            .select([Selector::ByDType(DataTypeSelector::Numeric).as_expr()])
            .drop_nulls(None);

        let df_features = read_and_return_df(feature_desc_path)?;

        Ok(Dataset {
            data,
            numerical_features,
            // numerical_names,
            // numerical_col_names,
        })
    }
}

fn read_and_return_df(path: PathBuf) -> Result<LazyFrame, PolarsError> {
    LazyCsvReader::new(PlPath::Local(Arc::from(path)))
        .with_has_header(true)
        .with_ignore_errors(false)
        .with_infer_schema_length(Some(200))
        .with_missing_is_null(true)
        .with_null_values(Some(NullValues::AllColumns(vec!["NA".into()])))
        .finish()
}

fn heatmap(
    numeric_data_owned: DataFrame,
    numeric_data: LazyFrame,
) -> Result<DataFrame, Box<dyn Error>> {
    let numeric_data: Array2<f32> = numeric_data_owned.to_ndarray::<Float32Type>(IndexOrder::C)?;
    let mut x: Vec<u64> = Vec::new();
    let mut y: Vec<u64> = Vec::new();
    let mut z: Vec<f32> = Vec::new();

    for (row_index, row) in corrcoef(&numeric_data, "pearson")?
        .rows()
        .into_iter()
        .enumerate()
    {
        for (col_index, col_cell) in row.into_iter().enumerate() {
            x.push(col_index as u64);
            y.push(row_index as u64);
            z.push(*col_cell);
        }
    }
    let corr_mat = DataFrame::new(vec![
        Series::new("x".into(), x).into(),
        Series::new("y".into(), y).into(),
        Series::new("z".into(), z).into(),
    ])?;

    // HeatMap::builder()
    //     .data(&corr_mat)
    //     .x("x")
    //     .y("y")
    //     .z("z")
    //     .plot_title(Text::from("Heat Map").font("Arial").size(18))
    //     .color_scale(Palette::Cividis)
    //     .build()
    //     .plot();
    Ok(corr_mat)
}

fn corr(df: DataFrame) -> DataFrame {
    let col_names= df.get_column_names_str();
    let col_names_len = col_names.len();
    let mut x: Vec<u32> = Vec::new();
    let mut y: Vec<u32> = Vec::new();
    let mut z: Vec<f32> = Vec::new();
    // Genereting all the correlation expressions
    // between every pair combination
    let corr_df_exprs = col_names.iter()
        .flat_map(|col_ax1| {
            col_names.iter().map(move |col_ax2| {
                pearson_corr(col(*col_ax1), col(*col_ax2))
                        .alias(format!("{}{}", col_ax1, col_ax2))
            })
        })
        // .dedup()
        .collect::<Vec<Expr>>();
    // convert to ndarray::Array2, needs ndarray feature
    let tmp: Array2<f32> = df
        .clone()
        .lazy()
        .select(corr_df_exprs)
        .collect()
        .unwrap()
        .to_ndarray::<Float32Type>(IndexOrder::C)
        .unwrap();
    // at this point tmp has shape = (1, len*len)
    // with len: number of column from the starting df

    // squaring the Array2
    let corr_matrix = tmp
        .into_shape_with_order([col_names_len, col_names_len])
        .unwrap();

    // Iter over the Axis and build the series

    for (row_index, row) in corr_matrix.rows().into_iter().enumerate() {
        for (col_index, col_cell) in row.into_iter().enumerate() {
            x.push(col_index as u32);
            y.push(row_index as u32);
            z.push(*col_cell);
        }
    }

    DataFrame::new(vec![
        Series::new("x".into(), x).into(),
        Series::new("y".into(), y).into(),
        Series::new("z".into(), z).into(),
    ])
    .unwrap()
}

// fn take_corr(name: &str, paired_names:){

//     .filter_map(|name| {
//         // dbg!(name);
//         if name.starts_with("Id") {
//             Some(col(name))
//         } else {
//             None
//         }
//     }).collect_vec();
// }

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df; // Macro for creating DataFrames easily

    #[test]
    fn test_perfect_correlation() {
        // Create data with known linear relationships
        let df = df!(
            "a" => &[1.0, 2.0, 3.0, 4.0, 5.0],
            "b" => &[2.0, 4.0, 6.0, 8.0, 10.0],   // 2x (Perfect Positive)
            "c" => &[-1.0, -2.0, -3.0, -4.0, -5.0] // -1x (Perfect Negative)
        )
        .expect("Failed to create DF");

        let result = corr(df);

        // 1. Check Dimensions
        // 3 input cols -> Output should be 3 cols + 1 label col = 4 cols
        assert_eq!(result.width(), 4);
        assert_eq!(result.height(), 3);

        // 2. Validate "column_name" column exists and has correct order
        let names_col = result.column("column_name").unwrap().str().unwrap();
        assert_eq!(names_col.get(0), Some("a"));
        assert_eq!(names_col.get(1), Some("b"));
        assert_eq!(names_col.get(2), Some("c"));

        // 3. Check Correlation Values (Column "a" vs others)
        let col_a = result.column("a").unwrap().f32().unwrap();

        // Corr(a, a) should be 1.0
        assert!((col_a.get(0).unwrap() - 1.0).abs() < 1e-6);
        // Corr(a, b) should be 1.0
        assert!((col_a.get(1).unwrap() - 1.0).abs() < 1e-6);
        // Corr(a, c) should be -1.0
        assert!((col_a.get(2).unwrap() + 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_uncorrelated_data() {
        // Orthogonal-ish data
        let df = df!(
            "x" => &[1.0, 0.0, -1.0],
            "y" => &[0.0, 1.0, 0.0]
        )
        .unwrap();

        let result = corr(df);

        // Extract correlation between x and y
        // Column "x", row 1 (which corresponds to "y")
        let val = result.column("x").unwrap().f32().unwrap().get(1).unwrap();

        // Should be 0.0 (or very close to it)
        assert!(val.abs() < 1e-6);
    }

    #[test]
    fn test_structure_integrity() {
        // Test ensuring the extra column is always last
        let df = df!("val1" => &[1.0, 2.0], "val2" => &[3.0, 4.0]).unwrap();
        let result = corr(df);

        let column_names = result.get_column_names();

        // Based on your chain logic, "column_name" should be the last one added
        assert_eq!(column_names.last(), Some(&&PlSmallStr::from("column_name")));
    }
}
