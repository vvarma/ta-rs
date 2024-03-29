use crate::{errors::Result, CandleLike, Close, High, Low, Next, Open};
/// # Links
///
/// * [Heiken Ashi, Wikipedia](https://en.wikipedia.org/wiki/Heikin-Ashi_chart)
pub struct HeikenAshi {
    prev: Option<HeikenAshiOutput>,
}
#[derive(Debug, Clone, Copy)]
pub struct HeikenAshiOutput {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl HeikenAshi {
    pub fn new() -> Result<Self> {
        Ok(Self { prev: None })
    }
}
impl<T: CandleLike> Next<&T> for HeikenAshi {
    type Output = HeikenAshiOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let next = match self.prev.take() {
            Some(prev) => {
                let open = (prev.open + prev.close) / 2.0;
                let close = (input.open() + input.high() + input.low() + input.close()) / 4.0;
                Self::Output {
                    open,
                    high: open.max(close).max(input.high()),
                    low: open.min(close).min(input.low()),
                    close,
                }
            }
            None => Self::Output {
                open: (input.open() + input.close()) / 2.0,
                high: input.high(),
                low: input.low(),
                close: (input.high() + input.low() + input.open() + input.close()) / 4.0,
            },
        };
        self.prev = Some(next);
        next
    }
}

impl Open for HeikenAshiOutput {
    fn open(&self) -> f64 {
        self.open
    }
}
impl Close for HeikenAshiOutput {
    fn close(&self) -> f64 {
        self.close
    }
}
impl Low for HeikenAshiOutput {
    fn low(&self) -> f64 {
        self.low
    }
}
impl High for HeikenAshiOutput {
    fn high(&self) -> f64 {
        self.high
    }
}
impl CandleLike for HeikenAshiOutput {}

pub enum CandleColor {
    Red,
    Green,
}
pub fn candle_color<T: CandleLike>(candle: &T) -> CandleColor {
    if candle.close() > candle.open() {
        CandleColor::Green
    } else {
        CandleColor::Red
    }
}
