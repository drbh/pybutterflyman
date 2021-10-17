use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bid {
    pub price: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ask {
    pub price: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub strike: f64,
    pub ask_info: Ask,
    pub bid_info: Bid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixed {
    pub breakeven_range: f64,
    pub range_to_target: f64,
    pub slope: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Space {
    pub max_loss_lower: f64,
    pub breakeven_lower: f64,
    pub target: f64,
    pub breakeven_upper: f64,
    pub max_loss_upper: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableOnSize {
    pub cost_per_bundle: f64,
    pub max_gain: f64,
    pub gain_per_cost: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Context {
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CondorContext {
    pub left: usize,
    pub ileft: usize,
    pub iright: usize,
    pub right: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub context: Context,
    pub space: Space,
    pub variable_on_size: VariableOnSize,
    pub fixed: Fixed,

    pub metric: f64,
    pub new_metric: f64,
    pub shares: i64,
    pub max: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CondorStats {
    pub context: CondorContext,
}

// for api
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestItem {
    pub strike: f64,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub symbol: String,
    pub target: f64,
    pub chain: Vec<RequestItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub input: RequestBody,
    pub output: Vec<Stats>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CondorOutput {
    pub input: RequestBody,
    pub output: Vec<CondorStats>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetNotInChain;
