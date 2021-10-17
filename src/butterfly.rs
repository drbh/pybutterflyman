use crate::structs::*;
use crate::structs::{Ask, Bid, Fixed, Item, Space, Stats, TargetNotInChain, VariableOnSize};
use std::cmp::Ordering;

fn butterfly_hunting(option_chain: Vec<Item>, target_price: f64) -> Vec<Stats> {
    // get the starting point by iterating over the strikes
    let (target_option_index, target_option) =
        find_starting_option(&option_chain, target_price).unwrap();

    // get len of items
    let n_items = &option_chain.len();

    // start list pointers to the left/right of our starting point
    let mut next_rightside = target_option_index + 1;
    let mut next_leftside = target_option_index - 1;

    // we'll sink all the results into this vec
    let mut all_option_stats = vec![];

    // loop until complete
    while next_rightside < *n_items || next_leftside > 0 {
        // calculate absolute distance from strike on both sides
        let left_diff = (option_chain[next_leftside].strike - target_option.strike).abs();
        let right_diff = (option_chain[next_rightside].strike - target_option.strike).abs();

        match left_diff
            .partial_cmp(&right_diff)
            .expect("I dont like wrong numbers")
        {
            Ordering::Less => {
                next_leftside -= 1;
            }
            Ordering::Greater => {
                next_rightside += 1;
            }
            Ordering::Equal => {
                // this cost function assumes that we'll pay `ask` price
                // and we'll sell at `bid` - hence the worst case
                let cost = (option_chain[next_leftside].ask_info.price
                    + option_chain[next_rightside].ask_info.price)
                    - (target_option.bid_info.price * 2.0);

                let left_breakeven = option_chain[next_leftside].strike + cost;
                let right_breakeven = option_chain[next_rightside].strike - cost;

                let rise = cost;
                let run = target_option.strike - option_chain[next_leftside].strike;
                let slope = rise / run;

                let breakeven_range = right_breakeven - left_breakeven;
                let max_val = breakeven_range / 2.0;

                let result = Stats {
                    context: crate::structs::Context {
                        left: next_leftside,
                        right: next_rightside,
                    },
                    space: Space {
                        max_loss_lower: option_chain[next_leftside].strike,
                        breakeven_lower: left_breakeven,
                        target: target_option.strike,
                        breakeven_upper: right_breakeven,
                        max_loss_upper: option_chain[next_rightside].strike,
                    },
                    variable_on_size: VariableOnSize {
                        cost_per_bundle: cost,
                        max_gain: max_val,
                        gain_per_cost: max_val / cost,
                    },
                    fixed: Fixed {
                        breakeven_range,
                        range_to_target: breakeven_range / target_option.strike,
                        slope,
                    },
                    metric: (max_val / cost) / (breakeven_range / target_option.strike),
                    new_metric: ((max_val / cost) / (breakeven_range / target_option.strike))
                        * slope,
                    shares: (10.0 / cost).floor() as i64,
                    max: (10.0 / cost).floor() * max_val,
                };

                all_option_stats.push(result);
                next_rightside += 1;
            }
        }
    }
    all_option_stats
}

fn condor_hunting(option_chain: Vec<Item>, target_price: f64) -> Vec<CondorStats> {
    let (target_option_index, _target_option) =
        find_starting_option(&option_chain, target_price).unwrap();

    let inner_leftside_index = target_option_index;
    let inner_rightside_index = target_option_index + 1;

    let inner_leftside = option_chain[inner_leftside_index].clone();
    let inner_rightside = option_chain[inner_rightside_index].clone();

    let mut ranges = vec![(
        inner_leftside_index,
        inner_rightside_index,
        inner_leftside,
        inner_rightside,
    )];

    let n_items = &option_chain.len();
    let mut outer_leftside_index = target_option_index - 1;
    let mut outer_rightside_index = target_option_index + 2;

    let mut _all_option_stats = vec![];

    // loop until complete
    while outer_rightside_index < *n_items || outer_leftside_index > 0 {
        // get diffs a distance from outer to inner
        let left_diff = (option_chain[outer_leftside_index].strike
            - option_chain[inner_leftside_index].strike)
            .abs();
        let right_diff = (option_chain[outer_rightside_index].strike
            - option_chain[inner_rightside_index].strike)
            .abs();

        match left_diff
            .partial_cmp(&right_diff)
            .expect("I dont like wrong numbers")
        {
            Ordering::Less => {
                outer_leftside_index -= 1;
            }
            Ordering::Greater => {
                outer_rightside_index += 1;
            }
            Ordering::Equal => {
                let outer_leftside = option_chain[outer_leftside_index].clone();
                let outer_rightside = option_chain[outer_rightside_index].clone();

                ranges.push((
                    outer_leftside_index,
                    outer_rightside_index,
                    outer_leftside.clone(),
                    outer_rightside.clone(),
                ));
                outer_rightside_index += 1;
            }
        }
    }

    ranges = ranges.into_iter().rev().collect();

    for i in 0..ranges.len() {
        let x = &ranges[i];

        let _outer_leftside_index = x.0;
        let _outer_rightside_index = x.1;
        let _outer_leftside = x.2.clone();
        let _outer_rightside = x.3.clone();

        // more rusty way to write
        // for j in i..ranges.len() {
        for (j, _) in ranges.iter().enumerate().skip(i) {
            if j == i {
                continue;
            }

            let y = &ranges[j];

            let _inner_leftside_index = y.0;
            let _inner_rightside_index = y.1;
            let _inner_leftside = y.2.clone();
            let _inner_rightside = y.3.clone();

            let result = CondorStats {
                context: crate::structs::CondorContext {
                    left: x.0,
                    ileft: y.0,
                    iright: y.1,
                    right: x.1,
                },
            };

            _all_option_stats.push(result)
        }
    }
    _all_option_stats
}

fn find_starting_option(
    option_chain: &[Item],
    target_price: f64,
) -> Result<(usize, Item), TargetNotInChain> {
    // iterate over items until we are at closes to target
    let mut min_distance = std::f64::INFINITY;
    let mut resp: Option<(usize, Item)> = None;

    // search
    for (i, opt) in option_chain.iter().enumerate() {
        // get diff
        let delta = (target_price - opt.strike).abs();

        // return item if closest
        resp = match delta {
            _delta if _delta < min_distance => {
                min_distance = delta;
                None // nope, try the next iteration
            }
            _delta if _delta > min_distance => {
                Some((i - 1, option_chain[i - 1].clone())) // return one before this
            }
            _ => None, // this means the target has no values greater
        };

        // stop looking if we found it
        if resp.is_some() {
            break;
        }
    }

    // return or error
    match resp {
        Some(found_starting_point) => Ok(found_starting_point),
        None => Err(TargetNotInChain),
    }
}

pub fn hunt_for_call_butterflies(recieved: RequestBody) -> std::io::Result<String> {
    let mut option_chain: Vec<Item> = recieved
        .chain
        .iter()
        .map(|x| Item {
            strike: x.strike,
            ask_info: Ask {
                price: x.price,
                volume: 0.0,
            },
            bid_info: Bid {
                price: x.price,
                volume: 0.0,
            },
        })
        .collect();
    option_chain.sort_by(|a, b| a.strike.partial_cmp(&b.strike).unwrap());

    let all_option_stats = butterfly_hunting(option_chain, recieved.target);

    let output = Output {
        input: recieved,
        output: all_option_stats,
    };
    Ok(serde_json::to_string(&output).unwrap())
}

pub fn hunt_for_call_condors(recieved: RequestBody) -> std::io::Result<String> {
    let mut option_chain: Vec<Item> = recieved
        .chain
        .iter()
        .map(|x| Item {
            strike: x.strike,
            ask_info: Ask {
                price: x.price,
                volume: 0.0,
            },
            bid_info: Bid {
                price: x.price,
                volume: 0.0,
            },
        })
        .collect();
    option_chain.sort_by(|a, b| a.strike.partial_cmp(&b.strike).unwrap());

    let all_option_stats = condor_hunting(option_chain, recieved.target);

    let output = CondorOutput {
        input: recieved,
        output: all_option_stats,
    };
    Ok(serde_json::to_string(&output).unwrap())
}
