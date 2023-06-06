use super::lookuptable::LookUpTable;

pub struct Range {
    pub low: u64,       // lower bound of current interval for arithmetic coding
    pub high: u64,      // upper bound of current interval for arithmetic coding
    width: u64,         // initial difference between high and low (not changed value)
    pub half: u64,      // half of the initial upper bound (not changed value)
    pub quarter: u64,   // quarter of the initial upper bound (not changed value)
    three_quarter: u64, // 3/4 of the initail upper bound (not changed value)
}

impl Range {
    pub fn new(precision: u8) -> Self {
        /*
            Update range for current symbol

            Input:
            precision: a fixed limit of number at that encoder rounds the calculated fractions to their nearest equivalents
        */

        // If precision is greater than 53, there is a loss in converting width to f64.
        assert_eq!(
            (precision > 29) && (precision < 53),
            true,
            "Please enter the number of precision greater than 29 and less than 53"
        );

        // Initialize parameters for range update
        let high: u64 = 1 << precision;
        Self {
            low: 0,
            high,
            width: high,
            half: high / 2,
            quarter: high / 4,
            three_quarter: (high / 4) * 3, //If change order of caculation, then overflow occurs
        }
    }

    pub fn init_range(&mut self, source: &LookUpTable, symbol: usize) {
        /*
            Initialize range for first quality score

            Input:
                source: look up table of probability
                symbol: first quality score
        */

        let range = source.get_init_probability(symbol);
        self.high = (self.width as f64 * range.1) as u64;
        self.low = (self.width as f64 * range.0) as u64;
    }

    pub fn update_range(&mut self, source: &LookUpTable, pre_symbol: usize, current_symbol: usize) {
        /*
            Update range for current quality score, given previous quality score

            Input:
                source: look up table of probabilities
                pre_symbol: previous quality score
                current_symbol: current quality score
        */

        let width = self.high - self.low;
        let range = source.get_probability(pre_symbol, current_symbol);
        self.high = self.low + (width as f64 * range.1) as u64;
        self.low = self.low + (width as f64 * range.0) as u64;
    }

    pub fn find_range(
        &mut self,
        source: &LookUpTable,
        pre_symbol: usize,
        current_symbol: usize,
    ) -> (u64, u64) {
        /*
            For decoding, find range to which current quality score corresponds

            Input:
                source: look up table of probabilities
                pre_symbol: previous quality score
                current_symbol: current quality score
            Output:
                (range_low, range_high): cumulative probabilities for arithmetic coding
                range_low is P(x<current_symbol|previous_symbol)
                range_high is P(x<=current_symbol|previous_symbol)
        */

        let width = self.high - self.low;
        let range = source.get_probability(pre_symbol, current_symbol);
        let high = self.low + (width as f64 * range.1) as u64;
        let low = self.low + (width as f64 * range.0) as u64;
        (low, high)
    }

    pub fn in_upper(&self) -> bool {
        /*
            In order to find prefix corresponding 1(bit), this notify whether current range is higher than half
        */

        self.low > self.half
    }

    pub fn in_lower(&self) -> bool {
        /*
            In order to find prefix coresponding 0(bit), this notify whether current range is lower than half
        */

        self.high < self.half
    }

    pub fn in_middle(&self) -> bool {
        /*
            In order to find numbers of pending bits, this function notify whether current range is higher
            than quarter and lower than 3*quarter
        */

        (self.high < self.three_quarter) && (self.low > self.quarter)
    }

    pub fn in_quarter(&self) -> bool {
        /*
            For ending the encoding , this function notify whether lower bound of current range is lower than quarter
        */

        self.low <= self.quarter
    }

    pub fn upper_renormalize(&mut self) {
        /*
            After finding that current range belongs to in_upper,
            this function renormalize for keeping the finite precision
            from becoming a limit on the total number of scores that can be encoded
            this function renormalize by first subtracting half from high and low, and shifting them to the left
        */

        // to prevent overflow, delete MSB by subtracting half first.
        self.high = (self.high - self.half) << 1;
        self.low = (self.low - self.half) << 1;
    }

    pub fn lower_renormalize(&mut self) {
        /*
            After finding that current range belongs to in_lower,
            this function renormalize for keeping the finite precision from becoming a limit on the total number of scores that can be encoded
            this function renormalize by shifting high to the left
        */

        self.high <<= 1;
        self.low <<= 1;
    }

    pub fn middle_renormalize(&mut self) {
        /*
            After finding that current range belongs to in_upper,
            this function renormalize for keeping the finite precision
            from becoming a limit on the total number of scores that can be encoded
            this function renormalize by first subtracting quarter from high and low, and shifting them to the left
        */

        self.high = (self.high - self.quarter) << 1;
        self.low = (self.low - self.quarter) << 1;
    }
}
