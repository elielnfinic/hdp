use std::str::FromStr;

use anyhow::{bail, Result};

pub mod integer;
pub mod string;

/// Aggregation function types
///
/// ### Defined
/// - AVG - Returns the average of the values
/// - BLOOM - Bloom filter
/// - MAX - Find the maximum value
/// - MIN - Find the minimum value
/// - MERKLE - Return the merkle root of the values
/// - STD - Standard deviation
/// - SUM - Sum of values
/// - COUNTIF - Count number of values that satisfy a condition
pub enum AggregationFunction {
    AVG,
    BLOOM,
    MAX,
    MIN,
    MERKLE,
    STD,
    SUM,
    COUNTIF,
}

/// Get [`AggregationFunction`] from function id
impl FromStr for AggregationFunction {
    type Err = anyhow::Error;

    fn from_str(function_id: &str) -> Result<Self, Self::Err> {
        match function_id.to_uppercase().as_str() {
            "AVG" => Ok(AggregationFunction::AVG),
            "BLOOM" => Ok(AggregationFunction::BLOOM),
            "MAX" => Ok(AggregationFunction::MAX),
            "MIN" => Ok(AggregationFunction::MIN),
            "MERKLE" => Ok(AggregationFunction::MERKLE),
            "STD" => Ok(AggregationFunction::STD),
            "SUM" => Ok(AggregationFunction::SUM),
            "COUNTIF" => Ok(AggregationFunction::COUNTIF),
            _ => bail!("Unknown aggregation function"),
        }
    }
}

impl AggregationFunction {
    pub fn get_index(&self) -> usize {
        match self {
            AggregationFunction::AVG => 0,
            AggregationFunction::BLOOM => 1,
            AggregationFunction::MAX => 2,
            AggregationFunction::MIN => 3,
            AggregationFunction::MERKLE => 4,
            AggregationFunction::STD => 5,
            AggregationFunction::SUM => 6,
            AggregationFunction::COUNTIF => 7,
        }
    }

    pub fn operation(&self, values: &[String], ctx: Option<String>) -> Result<String> {
        match self {
            AggregationFunction::AVG => integer::average(values),
            AggregationFunction::BLOOM => integer::bloom_filterize(values),
            AggregationFunction::MAX => integer::find_max(values),
            AggregationFunction::MIN => integer::find_min(values),
            AggregationFunction::MERKLE => string::merkleize(values),
            AggregationFunction::STD => integer::standard_deviation(values),
            AggregationFunction::SUM => integer::sum(values),
            AggregationFunction::COUNTIF => {
                if let Some(ctx) = ctx {
                    integer::count_if(values, &ctx)
                } else {
                    bail!("Context not provided for COUNTIF")
                }
            }
        }
    }
}
