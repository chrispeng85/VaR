use crate::asset::Asset;
use crate::error::{Result, VaRError};
use std::fs::File;
use std::io::Write;

pub struct VaRCalculator {

    portfolio:Vec<asset>,

}

impl VaRCalculator {

    pub fn new() -> Self {

            VaRCalculator {
                portfolio: Vec::new(),
            }
    }



    pub fn add_asset(&mut self, asset: Asset) {
    self.portfolio.push(asset);

    }

    fn validate_inputs(&self, confidence_level: f64, holding_period:u32) ->Result<()> {

        if self.portfolio.is_empty() {
            return Err(VaRError::EmptyPortfolio);

        }

        if confidence_level <= 0.0 || confidence_level >= 1.0 {
            return Err(VaRError::InvalidConfidenceLevel(confidence_level));

        }

        if holding_period == 0 {
            return Err(VaRError::InvalidHoldingPeriod())

        }

        Ok(())

        }

        fn calculate_mean(data: &[f64]) -> f64 {

            let sum: f64 = data.iter().sum();
            sum / data.len() as f64


        } 

        fn calculate_std_dev(data: &[f64], mean: f64) -> f64 {

                let variance: f64 = data.iter().map(|&x| (x - mean)).powi(2)
                .sum::<f64>() / (data.len() - 1) as f64;
                variance.sqrt();

        }

        fn calculate_percentile(mut data: Vec<f64>, percentile: f64) -> f64 {

            data.sort_by(|a,b| a.partial_cmp(b).unwrap());
            let index = ((percentile * data.len() as f64).ceil() - 1.0 ) as usize;
            data[index]

        } 

        pub fn calculate_historical_var(&self, condifence_level: f64, holding_period:u32) ->Result<f64> {

            self.validate_inputs(condifence_level, holding_period)?;

            let mut portfolio_returns = Vec::new();
            let mut total_value = 0.0;

            for asset in &self.portfolio {

                    total_value += asset.current_value();

                    if portfolio_returns.is_empty() {

                            portfolio_returns = asset.returns().to_vec();

                    }

                    else {

                            for (i, ret) in asset.returns().iter().enumerate() {

                                    portfolio_returns[i] += ret;
                            }
                    }
            }


                let scaling_factor = (holding_period as f64).sqrt();
                portfolio_returns.iter_mut().for_each(|r| *r *= scaling_factor);

                let percentile = Self::calculate_percentile(portfolio_returns, 1.0 - condifence_level);
                Ok( - total_value * percentile)
        }
        
        pub fn calculate_parametric_var(&self, condifence_level: f64, holding_period: u32) -> Result<f64> {

            self.validate_inputs(confidence_level, holding_period)?;

            let mut portfolio_returns = Vec::new();
            let mut total_value = 0.0;

            for asset in &self.portfolio {
                total_value += asset.current_value();

                if portfolio_returns.is_empty() {
                    portfolio_returns = asset.returns().to_vec();

                }

                else {

                        for (i, ret) in asset.returns().iter().enumerate() {
                            portfolio_returns[i] += ret;

                        }
                }

            }

            let mean = Self::calculate_mean(&portfolio_returns);
            let mut std_dev = Self::calculate_std_dev(&portfolio_returns, mean);

            let z_score = match condifence_level {
                0.95 => 1.645,
                0.99 => 2.326,
                _ => 2.0,

            };

            std_dev *= (holding_period as f64).sqrt();

            Ok(total_value * z_score * std_dev)

        }


}