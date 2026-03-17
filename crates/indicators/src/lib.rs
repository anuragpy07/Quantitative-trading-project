pub fn sma(data: &[f64], window_size: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];
    if window_size == 0 || window_size > data.len() {
        return result;
    }
    
    for i in 0..data.len() {
        if i + 1 >= window_size {
            let start = i + 1 - window_size;
            let window = &data[start..=i];
            let sum: f64 = window.iter().sum();
            result[i] = Some(sum / window_size as f64);
        }
    }
    result
}

pub fn ema(data: &[f64], window_size: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];
    if data.is_empty() {
        return result;
    }
    let alpha = 2.0 / (window_size as f64 + 1.0);
    let mut ema = data[0];
    result[0] = Some(ema);
    for i in 1..data.len() {
        ema = alpha * data[i] + (1.0 - alpha) * ema;
        result[i] = Some(ema);
    }
    result
}

pub fn rsi(data: &[f64], window_size: usize) -> Vec<Option<f64>> {
    let mut result = vec![None; data.len()];
    if window_size >= data.len() {
        return result;
    }

    let mut gains = Vec::with_capacity(data.len());
    let mut losses = Vec::with_capacity(data.len());

    for i in 1..data.len() {
        let diff = data[i] - data[i - 1];
        if diff >= 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-diff);
        }
    }

    let mut avg_gain: f64 = gains[0..window_size].iter().sum::<f64>() / window_size as f64;
    let mut avg_loss: f64 = losses[0..window_size].iter().sum::<f64>() / window_size as f64;

    if avg_loss != 0.0 {
        result[window_size] = Some(100.0 - (100.0 / (1.0 + avg_gain / avg_loss)));
    }

    for i in window_size..gains.len() {
        avg_gain = (avg_gain * (window_size - 1) as f64 + gains[i]) / window_size as f64;
        avg_loss = (avg_loss * (window_size - 1) as f64 + losses[i]) / window_size as f64;

        if avg_loss != 0.0 {
            result[i + 1] = Some(100.0 - (100.0 / (1.0 + avg_gain / avg_loss)));
        }
    }

    result
}
