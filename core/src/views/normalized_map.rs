pub trait NormalizedMap: 'static {
    fn normalized_to_display(&self, normalized: f32) -> String;

    fn snap(&self, normalized: f32) -> f32 {
        normalized
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayDecimals {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five
}

impl DisplayDecimals {
    pub fn display_value(&self, value: f32) -> String {
        match self {
            DisplayDecimals::Zero => format!("{:.0}", value),
            DisplayDecimals::One => format!("{:.1}", value),
            DisplayDecimals::Two => format!("{:.2}", value),
            DisplayDecimals::Three => format!("{:.3}", value),
            DisplayDecimals::Four => format!("{:.4}", value),
            DisplayDecimals::Five => format!("{:.5}", value),
        }
    }
}

impl Default for DisplayDecimals {
    fn default() -> Self {
        DisplayDecimals::One
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueScaling {
    Linear,
    Power(f32),
    Frequency,
}

impl ValueScaling {
    pub fn normalized_to_value(&self, normalized: f32, min: f32, max: f32) -> f32 {
        if normalized <= 0.0 {
            return min;
        } else if normalized >= 1.0 {
            return max;
        }
    
        let map = |x: f32| -> f32 {
            (x * (max - min)) + min
        };
    
        match self {
            ValueScaling::Linear => map(normalized),
    
            ValueScaling::Power(exponent) => map(normalized.powf(*exponent)),
    
            ValueScaling::Frequency => {
                let minl = min.log2();
                let range = max.log2() - minl;
                2.0f32.powf((normalized * range) + minl)
            }
        }
    }
    
    pub fn value_to_normalized(&self, value: f32, min: f32, max: f32) -> f32 {
        if value <= min {
            return 0.0;
        } else if value >= max {
            return 1.0;
        }
    
        let unmap = |x: f32| -> f32 {
            (x - min) / (max - min)
        };
    
        match self {
            ValueScaling::Linear => unmap(value),
    
            ValueScaling::Power(exponent) => unmap(value).powf(1.0 / *exponent),
    
            ValueScaling::Frequency => {
                let minl = min.log2();
                let range = max.log2() - minl;
                (value.log2() - minl) / range
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericMap {
    min: f32,
    max: f32,
    span_recip: f32, // Small optimization to avoid division operations.

    value_scaling: ValueScaling,

    display_decimals: DisplayDecimals,
    units: Option<String>,
}

impl GenericMap {
    pub fn new(min: f32, max: f32, value_scaling: ValueScaling, display_decimals: DisplayDecimals, units: Option<String>) -> Self {
        assert!(min < max);

        Self {
            min,
            max,
            span_recip: 1.0 / (max - min),
            value_scaling,
            display_decimals,
            units,
        }
    }

    pub fn min(&self) -> f32 {
        self.min
    }
    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn value_scaling(&self) -> &ValueScaling {
        &self.value_scaling
    }

    #[inline]
    pub fn normalized_to_value(&self, normalized: f32) -> f32 {
        self.value_scaling.normalized_to_value(normalized, self.min, self.max)
    }

    #[inline]
    pub fn value_to_normalized(&self, value: f32) -> f32 {
        self.value_scaling.value_to_normalized(value, self.min, self.max)
    }

    #[inline]
    pub fn clamp_value(&self, value: f32) -> f32 {
        value.min(self.max).max(self.min)
    }
}

impl NormalizedMap for GenericMap {
    fn normalized_to_display(&self, normalized: f32) -> String {
        let mut s = self.display_decimals.display_value(self.normalized_to_value(normalized));
        if let Some(units) = &self.units {
            s += units;
        }
        s
    }
}

#[derive(Debug, Clone)]
pub struct DecibelMap {
    min: f32,
    max: f32,

    value_scaling: ValueScaling,

    display_decimals: DisplayDecimals,
    display_units: bool,
}

impl DecibelMap {
    pub fn new(min_db: f32, max_db: f32, value_scaling: ValueScaling, display_decimals: DisplayDecimals, display_units: bool) -> Self {
        assert!(min_db < max_db);

        Self {
            min: min_db,
            max: max_db,
            value_scaling,
            display_decimals,
            display_units,
        }
    }

    pub fn min_db(&self) -> f32 {
        self.min
    }
    pub fn max_db(&self) -> f32 {
        self.max
    }

    pub fn value_scaling(&self) -> &ValueScaling {
        &self.value_scaling
    }

    #[inline]
    pub fn normalized_to_db(&self, normalized: f32) -> f32 {
        self.value_scaling.normalized_to_value(normalized, self.min, self.max)
    }

    #[inline]
    pub fn normalized_to_amplitude(&self, normalized: f32) -> f32 {
        db_to_amplitude(self.value_scaling.normalized_to_value(normalized, self.min, self.max))
    }

    #[inline]
    pub fn db_to_normalized(&self, db: f32) -> f32 {
        self.value_scaling.value_to_normalized(db, self.min, self.max)
    }

    #[inline]
    pub fn amplitude_to_normalized(&self, amplitude: f32) -> f32 {
        let db = amplitude_to_db(amplitude);
        self.db_to_normalized(db)
    }

    #[inline]
    pub fn clamp_db(&self, db: f32) -> f32 {
        db.min(self.max).max(self.min)
    }
}

impl NormalizedMap for DecibelMap {
    fn normalized_to_display(&self, normalized: f32) -> String {
        let mut s = self.display_decimals.display_value(self.normalized_to_db(normalized));
        if self.display_units {
            s += " dB"
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrequencyDisplayMode {
    OnlyHz(DisplayDecimals),
    HzThenKHz {
        under_1k: DisplayDecimals,
        over_1k: DisplayDecimals,
    }
}

impl Default for FrequencyDisplayMode {
    fn default() -> Self {
        FrequencyDisplayMode::HzThenKHz {
            under_1k: DisplayDecimals::One,
            over_1k: DisplayDecimals::Two,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FrequencyMap {
    min: f32,
    max: f32,

    value_scaling: ValueScaling,

    display_mode: FrequencyDisplayMode,
    display_units: bool,
}

impl FrequencyMap {
    pub fn new(
        min_hz: f32,
        max_hz: f32,
        value_scaling: ValueScaling,
        display_mode: FrequencyDisplayMode,
        display_units: bool,
    ) -> Self {
        assert!(min_hz < max_hz);

        Self {
            min: min_hz,
            max: max_hz,
            value_scaling,
            display_mode,
            display_units,
        }
    }

    pub fn min_hz(&self) -> f32 {
        self.min
    }
    pub fn max_hz(&self) -> f32 {
        self.max
    }

    pub fn value_scaling(&self) -> &ValueScaling {
        &self.value_scaling
    }

    #[inline]
    pub fn normalized_to_hz(&self, normalized: f32) -> f32 {
        self.value_scaling.normalized_to_value(normalized, self.min, self.max)
    }

    #[inline]
    pub fn hz_to_normalized(&self, hz: f32) -> f32 {
        self.value_scaling.value_to_normalized(hz, self.min, self.max)
    }

    #[inline]
    pub fn clamp_hz(&self, hz: f32) -> f32 {
        hz.min(self.max).max(self.min)
    }
}

impl NormalizedMap for FrequencyMap {
    fn normalized_to_display(&self, normalized: f32) -> String {
        let hz = self.normalized_to_hz(normalized);

        match self.display_mode {
            FrequencyDisplayMode::OnlyHz(display_decimals) => {
                let mut s = display_decimals.display_value(hz);
                if self.display_units {
                    s += " Hz"
                }
                s
            }
            FrequencyDisplayMode::HzThenKHz { under_1k, over_1k } => {
                if hz < 1_000.0 {
                    let mut s = under_1k.display_value(hz);
                    if self.display_units {
                        s += " Hz"
                    }
                    s
                } else {
                    let mut s = over_1k.display_value(hz / 1_000.0);
                    if self.display_units {
                        s += " kHz"
                    }
                    s
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct IntMap {
    min: i32,
    max: i32,
    span: f32,  // Small optimization.

    display_map: Option<&'static dyn Fn(i32) -> String>,
}

impl IntMap {
    pub fn new(min: i32, max: i32, display_map: Option<&'static dyn Fn(i32) -> String>) -> Self {
        assert!(min <= max);

        Self {
            min,
            max,
            span: (max - min) as f32,
            display_map,
        }
    }

    pub fn min(&self) -> i32 {
        self.min
    }
    pub fn max(&self) -> i32 {
        self.max
    }

    #[inline]
    pub fn normalized_to_int(&self, normalized: f32) -> i32 {
        if normalized <= 0.0 {
            return self.min;
        } else if normalized >= 1.0 {
            return self.max;
        }

        (normalized * self.span).round() as i32
    }

    #[inline]
    pub fn int_to_normalized(&self, int: i32) -> f32 {
        if self.min == self.max {
            // Value will always be the same, so avoid a divide
            // by zero.
            0.0
        } else {
            if int <= self.min {
                return 0.0;
            } else if int >= self.max {
                return 1.0;
            }

            (int as f32 - self.min as f32) / self.span
        }
    }

    #[inline]
    pub fn clamp_int(&self, int: i32) -> i32 {
        int.min(self.max).max(self.min)
    }
}

impl NormalizedMap for IntMap {
    fn normalized_to_display(&self, normalized: f32) -> String {
        let int = self.normalized_to_int(normalized);

        if let Some(display_map) = self.display_map {
            (display_map)(int)
        } else {
            // Display the plain integer instead.
            format!("{}", int)
        }
    }

    fn snap(&self, normalized: f32) -> f32 {
        let int = self.normalized_to_int(normalized);
        self.int_to_normalized(int)
    }
}

#[inline]
pub fn db_to_amplitude(db: f32) -> f32 {
    10.0f32.powf(db * 1.0 / 20.0)
}

#[inline]
pub fn amplitude_to_db(amp: f32) -> f32 {
    20.0f32 * amp.log10()
}