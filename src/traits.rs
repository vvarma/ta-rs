// Indicator traits
//

/// Resets an indicator to the initial state.
pub trait Reset {
    fn reset(&mut self);
}

/// Return the period used by the indicator.
pub trait Period {
    fn period(&self) -> usize;
}

/// Consumes a data item of type `T` and returns `Output`.
///
/// Typically `T` can be `f64` or a struct similar to [DataItem](struct.DataItem.html), that implements
/// traits necessary to calculate value of a particular indicator.
///
/// In most cases `Output` is `f64`, but sometimes it can be different. For example for
/// [MACD](indicators/struct.MovingAverageConvergenceDivergence.html) it is `(f64, f64, f64)` since
/// MACD returns 3 values.
///
pub trait Next<T> {
    type Output;
    fn next(&mut self, input: T) -> Self::Output;
}

/// Open price of a particular period.
pub trait Open {
    fn open(&self) -> f64;
}

/// Close price of a particular period.
pub trait Close {
    fn close(&self) -> f64;
}

/// Lowest price of a particular period.
pub trait Low {
    fn low(&self) -> f64;
}

/// Highest price of a particular period.
pub trait High {
    fn high(&self) -> f64;
}

/// Trading volume of a particular trading period.
pub trait Volume {
    fn volume(&self) -> f64;
}

pub trait CandleLike: Open + Close + High + Low {
    fn is_red(&self) -> bool {
        self.close() < self.open()
    }
    fn is_green(&self) -> bool {
        !self.is_red()
    }
    fn body(&self) -> f64 {
        (self.open() - self.close()).abs()
    }
}
#[cfg(test)]
pub mod tests {
    use crate::{test_helper::Bar, CandleLike};
    #[test]
    fn test_candlelike() {
        let candle = candle!(10.0, 60, 5, 40, 100);
        assert!(candle.is_green());
        assert_eq!(candle.body(), 30.0);
    }
}
