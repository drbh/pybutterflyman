use crate::structs::RequestBody;
use crate::structs::RequestItem;

use pyo3::prelude::*;

mod butterfly;
mod structs;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn hunt_for_call_butterflies(symbol: String, target: f64, json_chain: String) -> PyResult<String> {
    let chain: Vec<RequestItem> = serde_json::from_str(&json_chain).unwrap();

    match butterfly::hunt_for_call_butterflies(RequestBody {
        symbol,
        target,
        chain,
    }) {
        Ok(res) => Ok(res),
        Err(_) => Ok("There was a problem".to_string()),
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn hunt_for_call_condors(symbol: String, target: f64, json_chain: String) -> PyResult<String> {
    let chain: Vec<RequestItem> = serde_json::from_str(&json_chain).unwrap();

    match butterfly::hunt_for_call_condors(RequestBody {
        symbol,
        target,
        chain,
    }) {
        Ok(res) => Ok(res),
        Err(_) => Ok("There was a problem".to_string()),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pybutterflyman(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(hunt_for_call_butterflies, m)?)?;
    m.add_function(wrap_pyfunction!(hunt_for_call_condors, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::*;

    #[test]
    fn butterfly_test() {
        let mut option_chain = vec![
            RequestItem {
                strike: 340.0,
                price: 0.67,
            },
            RequestItem {
                strike: 335.0,
                price: 2.21,
            },
            RequestItem {
                strike: 330.0,
                price: 5.59,
            },
        ];

        let current_price = 334.97;

        // sort that chain
        option_chain.sort_by(|a, b| a.strike.partial_cmp(&b.strike).unwrap());

        let all_option_stats = butterfly::hunt_for_call_butterflies(RequestBody {
            symbol: "RANGERDAVE".to_string(),
            target: current_price,
            chain: option_chain,
        })
        .unwrap();

        assert_eq!(
            all_option_stats,
            r#"{"input":{"symbol":"RANGERDAVE","target":334.97,"chain":[{"strike":330.0,"price":5.59},{"strike":335.0,"price":2.21},{"strike":340.0,"price":0.67}]},"output":[{"context":{"left":0,"right":2},"space":{"max_loss_lower":330.0,"breakeven_lower":331.84,"target":335.0,"breakeven_upper":338.16,"max_loss_upper":340.0},"variable_on_size":{"cost_per_bundle":1.8399999999999999,"max_gain":3.160000000000025,"gain_per_cost":1.7173913043478397},"fixed":{"breakeven_range":6.32000000000005,"range_to_target":0.018865671641791194,"slope":0.368},"metric":91.03260869565217,"new_metric":33.5,"shares":5,"max":15.800000000000125}]}"#
        );
    }

    #[test]
    fn condor_test() {
        let mut option_chain = vec![
            RequestItem {
                strike: 400.0,
                price: 0.67,
            },
            RequestItem {
                strike: 390.0,
                price: 0.67,
            },
            RequestItem {
                strike: 380.0,
                price: 0.67,
            },
            RequestItem {
                strike: 370.0,
                price: 0.67,
            },
            RequestItem {
                strike: 360.0,
                price: 0.67,
            },
            RequestItem {
                strike: 350.0,
                price: 0.67,
            },
            RequestItem {
                strike: 340.0,
                price: 2.21,
            },
            RequestItem {
                strike: 330.0,
                price: 2.21,
            },
            RequestItem {
                strike: 320.0,
                price: 5.59,
            },
            RequestItem {
                strike: 315.0,
                price: 5.59,
            }, // this one shouldnt work because 5
            RequestItem {
                strike: 310.0,
                price: 5.59,
            },
            RequestItem {
                strike: 300.0,
                price: 5.59,
            },
            RequestItem {
                strike: 290.0,
                price: 5.59,
            },
            RequestItem {
                strike: 280.0,
                price: 5.59,
            },
            RequestItem {
                strike: 270.0,
                price: 5.59,
            },
        ];

        let current_price = 333.97;

        // sort that chain
        option_chain.sort_by(|a, b| a.strike.partial_cmp(&b.strike).unwrap());

        let all_option_stats = butterfly::hunt_for_call_condors(RequestBody {
            symbol: "RANGERDAVE".to_string(),
            target: current_price,
            chain: option_chain,
        })
        .unwrap();

        // println!("{}", all_option_stats);

        assert_eq!(
            all_option_stats,
            r#"{"input":{"symbol":"RANGERDAVE","target":333.97,"chain":[{"strike":270.0,"price":5.59},{"strike":280.0,"price":5.59},{"strike":290.0,"price":5.59},{"strike":300.0,"price":5.59},{"strike":310.0,"price":5.59},{"strike":315.0,"price":5.59},{"strike":320.0,"price":5.59},{"strike":330.0,"price":2.21},{"strike":340.0,"price":2.21},{"strike":350.0,"price":0.67},{"strike":360.0,"price":0.67},{"strike":370.0,"price":0.67},{"strike":380.0,"price":0.67},{"strike":390.0,"price":0.67},{"strike":400.0,"price":0.67}]},"output":[{"context":{"left":0,"ileft":1,"iright":13,"right":14}},{"context":{"left":0,"ileft":2,"iright":12,"right":14}},{"context":{"left":0,"ileft":3,"iright":11,"right":14}},{"context":{"left":0,"ileft":4,"iright":10,"right":14}},{"context":{"left":0,"ileft":6,"iright":9,"right":14}},{"context":{"left":0,"ileft":7,"iright":8,"right":14}},{"context":{"left":1,"ileft":2,"iright":12,"right":13}},{"context":{"left":1,"ileft":3,"iright":11,"right":13}},{"context":{"left":1,"ileft":4,"iright":10,"right":13}},{"context":{"left":1,"ileft":6,"iright":9,"right":13}},{"context":{"left":1,"ileft":7,"iright":8,"right":13}},{"context":{"left":2,"ileft":3,"iright":11,"right":12}},{"context":{"left":2,"ileft":4,"iright":10,"right":12}},{"context":{"left":2,"ileft":6,"iright":9,"right":12}},{"context":{"left":2,"ileft":7,"iright":8,"right":12}},{"context":{"left":3,"ileft":4,"iright":10,"right":11}},{"context":{"left":3,"ileft":6,"iright":9,"right":11}},{"context":{"left":3,"ileft":7,"iright":8,"right":11}},{"context":{"left":4,"ileft":6,"iright":9,"right":10}},{"context":{"left":4,"ileft":7,"iright":8,"right":10}},{"context":{"left":6,"ileft":7,"iright":8,"right":9}}]}"#
        );
    }
}
