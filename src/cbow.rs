use crate::sampler::*;
use crate::params::*;
use crate::window::*;
use crate::numerics::*;

pub struct CBOW {
    pub embed:   ParameterStore,
    pub predict: ParameterStore
}

impl CBOW {

    fn ids(&self, win: &Window) -> Vec<usize> {
        let mut ids = Vec::new();
        for i in 0 .. win.words.len() {
            if i != win.predict_pos {
                ids.push(win.words[i])
            }
        }
        for id in win.ids.iter() {
            ids.push(*id)
        }
        ids
    }

    fn embedding(&self, ids: Vec<usize>) -> Vec<f64> {
        let mut result = vec![0.0; self.embed.cols];
        self.embed.avg(&mut result, ids);
        result
    }    

    fn gradient(&self, label: usize, h: &[f64], truth: f64, rate: f64) -> (f64, f64) {
        let w = self.predict.at(label);
        let a = sigmoid(dot(&h, &w));
        let d = (truth - a) * rate;
        let e = -f64::ln(if (truth - 1f64).abs() < std::f64::EPSILON {a} else {1f64 - a});
        (d, e)
    } 

    pub fn negative_sampeling(&mut self, window: &Window, rate: f64, n_samples: usize, unigrams: &Sampler) -> f64 {
        let mut gradient_embed = vec![0.0; self.embed.cols];
        let h = self.embedding(self.ids(window));        
        let (pos, pos_err) = self.gradient(window.label(), &h, 1.0, rate);        
        let mut error = pos_err;
        let mut gradient_pos_predict = vec![0.0; self.predict.cols];
        for i in 0 .. self.embed.cols {        
            gradient_embed[i] += pos * self.predict.at(window.label())[i];
            gradient_pos_predict[i] += pos * h[i];
        }
        self.predict.update(window.label(), &gradient_pos_predict);
        for _sample in 0 .. n_samples {
            let token = unigrams.multinomial();
            let (neg, neg_error) = self.gradient(token, &h, 0.0, rate);
            error += neg_error;
            let mut gradient_neg_predict = vec![0.0; self.predict.cols];
            for i in 0 .. self.embed.cols {
                gradient_embed[i]       += neg * self.predict.at(token)[i];
                gradient_neg_predict[i] += neg * h[i];
            }
            self.predict.update(token, &gradient_neg_predict);
        }

        for i in self.ids(window).iter() {
            self.embed.update(*i, &gradient_embed);
        }
        error
    }
}

