use std::time::{Duration, Instant};
use vizia_style::{
    AnimationDirection, AnimationFillMode, AnimationIterationCount, AnimationPlayState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CssAnimationPhase {
    Before,
    Active,
    After,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CssAnimationTiming {
    pub duration: f32,
    pub delay: f32,
    pub iteration_count: AnimationIterationCount,
    pub direction: AnimationDirection,
    pub fill_mode: AnimationFillMode,
    pub play_state: AnimationPlayState,
}

impl Default for CssAnimationTiming {
    fn default() -> Self {
        Self {
            duration: 0.0,
            delay: 0.0,
            iteration_count: AnimationIterationCount::Number(1.0),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::None,
            play_state: AnimationPlayState::Running,
        }
    }
}

impl CssAnimationTiming {
    pub fn active_duration(self) -> f32 {
        if self.duration <= 0.0 {
            return 0.0;
        }
        match self.iteration_count {
            AnimationIterationCount::Infinite => f32::INFINITY,
            AnimationIterationCount::Number(count) => self.duration * count.max(0.0),
        }
    }

    fn reverse_for_iteration(self, iteration: u64) -> bool {
        match self.direction {
            AnimationDirection::Normal => false,
            AnimationDirection::Reverse => true,
            AnimationDirection::Alternate => iteration % 2 == 1,
            AnimationDirection::AlternateReverse => iteration % 2 == 0,
        }
    }

    fn directed_progress(self, iteration: u64, simple: f32) -> f32 {
        if self.reverse_for_iteration(iteration) { 1.0 - simple } else { simple }
    }

    fn initial_progress(self) -> f32 {
        self.directed_progress(0, 0.0)
    }

    fn final_iteration_and_progress(self) -> (u64, f32) {
        let count = match self.iteration_count {
            AnimationIterationCount::Infinite => return (0, self.initial_progress()),
            AnimationIterationCount::Number(count) => count.max(0.0),
        };

        if count == 0.0 {
            return (0, self.initial_progress());
        }

        let whole = count.floor();
        let fraction = count - whole;
        if fraction.abs() <= f32::EPSILON {
            let iteration = (whole as u64).saturating_sub(1);
            (iteration, self.directed_progress(iteration, 1.0))
        } else {
            let iteration = whole as u64;
            (iteration, self.directed_progress(iteration, fraction))
        }
    }

    pub fn sample(self, active_elapsed: f32) -> CssAnimationSample {
        let active_duration = self.active_duration();
        let local = active_elapsed - self.delay;

        if local < 0.0 {
            let progress =
                matches!(self.fill_mode, AnimationFillMode::Backwards | AnimationFillMode::Both)
                    .then(|| self.initial_progress());
            return CssAnimationSample {
                phase: CssAnimationPhase::Before,
                progress,
                current_iteration: 0,
                elapsed_active: 0.0,
                before: self.reverse_for_iteration(0),
                finished: false,
            };
        }

        if active_duration == 0.0 || local >= active_duration {
            let (final_iteration, final_progress) = self.final_iteration_and_progress();
            let progress =
                matches!(self.fill_mode, AnimationFillMode::Forwards | AnimationFillMode::Both)
                    .then_some(final_progress);
            return CssAnimationSample {
                phase: CssAnimationPhase::After,
                progress,
                current_iteration: final_iteration,
                elapsed_active: active_duration.min(local.max(0.0)),
                before: self.reverse_for_iteration(final_iteration),
                finished: true,
            };
        }

        let duration = self.duration.max(f32::MIN_POSITIVE);
        let position = local / duration;
        let iteration = position.floor() as u64;
        let simple = position - iteration as f32;
        CssAnimationSample {
            phase: CssAnimationPhase::Active,
            progress: Some(self.directed_progress(iteration, simple)),
            current_iteration: iteration,
            elapsed_active: local.max(0.0),
            before: self.reverse_for_iteration(iteration),
            finished: false,
        }
    }

    /// Sample a progress-based timeline. Delays are intentionally not wall-clock delays here: the
    /// external timeline owns progress and maps its full 0..=1 range across the effect iterations.
    /// Infinite iteration counts map to one reversible timeline iteration.
    pub fn sample_timeline_progress(self, progress: Option<f32>) -> CssAnimationSample {
        let Some(progress) = progress else {
            return CssAnimationSample {
                phase: CssAnimationPhase::Before,
                progress: None,
                current_iteration: 0,
                elapsed_active: 0.0,
                before: self.reverse_for_iteration(0),
                finished: false,
            };
        };
        let timeline_progress = progress.clamp(0.0, 1.0);
        let count = match self.iteration_count {
            AnimationIterationCount::Infinite => 1.0,
            AnimationIterationCount::Number(count) => count.max(0.0),
        };
        if count == 0.0 {
            return CssAnimationSample {
                phase: CssAnimationPhase::Active,
                progress: Some(self.initial_progress()),
                current_iteration: 0,
                elapsed_active: 0.0,
                before: self.reverse_for_iteration(0),
                finished: false,
            };
        }
        if timeline_progress >= 1.0 {
            let (iteration, value) = match self.iteration_count {
                AnimationIterationCount::Infinite => (0, self.directed_progress(0, 1.0)),
                AnimationIterationCount::Number(_) => self.final_iteration_and_progress(),
            };
            return CssAnimationSample {
                phase: CssAnimationPhase::Active,
                progress: Some(value),
                current_iteration: iteration,
                elapsed_active: self.duration.max(0.0) * count,
                before: self.reverse_for_iteration(iteration),
                finished: false,
            };
        }
        let position = timeline_progress * count;
        let iteration = position.floor() as u64;
        let simple = position - iteration as f32;
        CssAnimationSample {
            phase: CssAnimationPhase::Active,
            progress: Some(self.directed_progress(iteration, simple)),
            current_iteration: iteration,
            elapsed_active: timeline_progress * self.duration.max(0.0) * count,
            before: self.reverse_for_iteration(iteration),
            finished: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CssAnimationSample {
    pub phase: CssAnimationPhase,
    pub progress: Option<f32>,
    pub current_iteration: u64,
    pub elapsed_active: f32,
    /// CSS Easing's "before flag" is true while traversing a segment backwards. This matters at
    /// exact discontinuities for step timing functions.
    pub before: bool,
    pub finished: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct CssAnimationClock {
    pub timing: CssAnimationTiming,
    pub start_time: Instant,
    paused_at: Option<Instant>,
    paused_duration: Duration,
    playback_rate: f32,
    seek_offset: f32,
    runtime_paused: bool,
}

impl CssAnimationClock {
    pub fn new(timing: CssAnimationTiming, start_time: Instant) -> Self {
        let paused_at = (timing.play_state == AnimationPlayState::Paused).then_some(start_time);
        Self {
            timing,
            start_time,
            paused_at,
            paused_duration: Duration::ZERO,
            playback_rate: 1.0,
            seek_offset: 0.0,
            runtime_paused: false,
        }
    }

    pub fn effective_elapsed(&self, now: Instant) -> f32 {
        let end = self.paused_at.unwrap_or(now);
        self.seek_offset
            + end
                .saturating_duration_since(self.start_time)
                .saturating_sub(self.paused_duration)
                .as_secs_f32()
                * self.playback_rate
    }

    pub fn sample(&self, now: Instant) -> CssAnimationSample {
        self.timing.sample(self.effective_elapsed(now))
    }

    pub fn playback_rate(&self) -> f32 {
        self.playback_rate
    }

    pub fn is_paused(&self) -> bool {
        self.paused_at.is_some()
    }

    fn reanchor(&mut self, now: Instant, elapsed: f32) {
        let paused = self.is_paused();
        self.start_time = now;
        self.paused_duration = Duration::ZERO;
        self.seek_offset = elapsed;
        self.paused_at = paused.then_some(now);
    }

    pub fn pause(&mut self, now: Instant) {
        self.runtime_paused = true;
        if self.paused_at.is_none() {
            self.paused_at = Some(now);
        }
    }

    pub fn resume(&mut self, now: Instant) {
        self.runtime_paused = false;
        if self.timing.play_state == AnimationPlayState::Running {
            if let Some(paused_at) = self.paused_at.take() {
                self.paused_duration += now.saturating_duration_since(paused_at);
            }
        }
    }

    pub fn seek(&mut self, seconds: f32, now: Instant) {
        let paused = self.is_paused();
        self.start_time = now;
        self.paused_duration = Duration::ZERO;
        self.seek_offset = if seconds.is_finite() { seconds } else { 0.0 };
        self.paused_at = paused.then_some(now);
    }

    pub fn set_playback_rate(&mut self, rate: f32, now: Instant) {
        if !rate.is_finite() {
            return;
        }
        let elapsed = self.effective_elapsed(now);
        self.reanchor(now, elapsed);
        self.playback_rate = rate;
    }

    pub fn reverse(&mut self, now: Instant) {
        let rate =
            if self.playback_rate.abs() <= f32::EPSILON { -1.0 } else { -self.playback_rate };
        self.set_playback_rate(rate, now);
    }

    pub fn finish(&mut self, now: Instant) -> bool {
        let duration = self.timing.active_duration();
        if !duration.is_finite() {
            return false;
        }
        self.seek(self.timing.delay + duration, now);
        self.pause(now);
        true
    }

    pub fn map_timeline_progress(&self, progress: Option<f32>) -> Option<f32> {
        progress.map(|progress| {
            if self.playback_rate < 0.0 { 1.0 - progress } else { progress }.clamp(0.0, 1.0)
        })
    }

    pub(crate) fn apply_control(
        &mut self,
        control: crate::animation::CssAnimationControl,
        now: Instant,
    ) -> bool {
        use crate::animation::CssAnimationControl;
        match control {
            CssAnimationControl::Pause => self.pause(now),
            CssAnimationControl::Resume => self.resume(now),
            CssAnimationControl::Seek(seconds) => self.seek(seconds, now),
            CssAnimationControl::SetPlaybackRate(rate) => self.set_playback_rate(rate, now),
            CssAnimationControl::Reverse => self.reverse(now),
            CssAnimationControl::Finish => return self.finish(now),
        }
        true
    }

    pub fn update_timing(&mut self, timing: CssAnimationTiming, now: Instant) {
        let was_paused = self.paused_at.is_some();
        let should_pause = timing.play_state == AnimationPlayState::Paused || self.runtime_paused;
        match (was_paused, should_pause) {
            (false, true) => self.paused_at = Some(now),
            (true, false) => {
                if let Some(paused_at) = self.paused_at.take() {
                    self.paused_duration += now.saturating_duration_since(paused_at);
                }
            }
            _ => {}
        }
        self.timing = timing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timing() -> CssAnimationTiming {
        CssAnimationTiming {
            duration: 2.0,
            delay: 1.0,
            iteration_count: AnimationIterationCount::Number(1.0),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::None,
            play_state: AnimationPlayState::Running,
        }
    }

    #[test]
    fn runtime_seek_rate_reverse_and_pause_preserve_elapsed_time() {
        let start = Instant::now();
        let mut clock = CssAnimationClock::new(timing(), start);
        clock.seek(1.5, start);
        assert!((clock.effective_elapsed(start) - 1.5).abs() < 0.001);

        clock.set_playback_rate(2.0, start);
        assert!((clock.effective_elapsed(start + Duration::from_millis(250)) - 2.0).abs() < 0.001);

        clock.pause(start + Duration::from_millis(250));
        assert!((clock.effective_elapsed(start + Duration::from_secs(5)) - 2.0).abs() < 0.001);
        clock.resume(start + Duration::from_secs(5));
        assert!((clock.effective_elapsed(start + Duration::from_millis(5250)) - 2.5).abs() < 0.001);

        clock.reverse(start + Duration::from_millis(5250));
        assert_eq!(clock.playback_rate(), -2.0);
        assert!((clock.effective_elapsed(start + Duration::from_millis(5500)) - 2.0).abs() < 0.001);
    }

    #[test]
    fn runtime_finish_rejects_infinite_effects() {
        let start = Instant::now();
        let mut finite = CssAnimationClock::new(timing(), start);
        assert!(finite.finish(start));
        assert!(finite.is_paused());
        assert!(finite.sample(start).finished);

        let infinite_timing =
            CssAnimationTiming { iteration_count: AnimationIterationCount::Infinite, ..timing() };
        let mut infinite = CssAnimationClock::new(infinite_timing, start);
        assert!(!infinite.finish(start));
    }

    #[test]
    fn progress_timeline_is_reversible_and_ignores_wall_clock() {
        let t = CssAnimationTiming {
            delay: 99.0,
            iteration_count: AnimationIterationCount::Number(2.0),
            direction: AnimationDirection::Alternate,
            ..timing()
        };
        assert_eq!(t.sample_timeline_progress(Some(0.25)).progress, Some(0.5));
        assert_eq!(t.sample_timeline_progress(Some(0.75)).progress, Some(0.5));
        assert!(!t.sample_timeline_progress(Some(1.0)).finished);
        assert_eq!(t.sample_timeline_progress(None).progress, None);
    }

    #[test]
    fn phases_positive_and_negative_delay() {
        let t = timing();
        assert_eq!(t.sample(0.5).phase, CssAnimationPhase::Before);
        assert_eq!(t.sample(1.5).progress, Some(0.25));
        assert_eq!(t.sample(3.0).phase, CssAnimationPhase::After);

        let t = CssAnimationTiming { delay: -0.5, ..t };
        assert_eq!(t.sample(0.0).progress, Some(0.25));
    }

    #[test]
    fn fractional_iteration_finishes_part_way_through() {
        let t = CssAnimationTiming {
            iteration_count: AnimationIterationCount::Number(2.5),
            fill_mode: AnimationFillMode::Forwards,
            ..timing()
        };
        assert_eq!(t.sample(6.0).progress, Some(0.5));
    }

    #[test]
    fn direction_modes_cover_odd_and_even_iterations() {
        let base = CssAnimationTiming { delay: 0.0, ..timing() };
        let reverse = CssAnimationTiming { direction: AnimationDirection::Reverse, ..base };
        let reversed_sample = reverse.sample(0.5);
        assert!((reversed_sample.progress.unwrap() - 0.75).abs() < 0.001);
        assert!(reversed_sample.before);

        let alternate = CssAnimationTiming {
            direction: AnimationDirection::Alternate,
            iteration_count: AnimationIterationCount::Number(3.0),
            ..base
        };
        assert!((alternate.sample(2.5).progress.unwrap() - 0.75).abs() < 0.001);
        assert!(alternate.sample(2.5).before);

        let alternate_reverse = CssAnimationTiming {
            direction: AnimationDirection::AlternateReverse,
            iteration_count: AnimationIterationCount::Number(3.0),
            ..base
        };
        assert!((alternate_reverse.sample(0.5).progress.unwrap() - 0.75).abs() < 0.001);
        assert!(alternate_reverse.sample(0.5).before);
    }

    #[test]
    fn fill_modes_choose_values_outside_active_interval() {
        let backwards = CssAnimationTiming { fill_mode: AnimationFillMode::Backwards, ..timing() };
        assert_eq!(backwards.sample(0.0).progress, Some(0.0));

        let both_reverse = CssAnimationTiming {
            fill_mode: AnimationFillMode::Both,
            direction: AnimationDirection::Reverse,
            ..timing()
        };
        assert_eq!(both_reverse.sample(0.0).progress, Some(1.0));
        assert_eq!(both_reverse.sample(4.0).progress, Some(0.0));
    }

    #[test]
    fn zero_duration_and_zero_iterations_are_instantaneous() {
        let zero_duration = CssAnimationTiming {
            duration: 0.0,
            delay: 0.0,
            iteration_count: AnimationIterationCount::Infinite,
            fill_mode: AnimationFillMode::Forwards,
            ..timing()
        };
        assert!(zero_duration.sample(0.0).finished);

        let zero_iterations = CssAnimationTiming {
            delay: 0.0,
            iteration_count: AnimationIterationCount::Number(0.0),
            fill_mode: AnimationFillMode::Forwards,
            ..timing()
        };
        assert_eq!(zero_iterations.sample(0.0).progress, Some(0.0));
    }

    #[test]
    fn pause_freezes_delay_and_active_time() {
        let start = Instant::now();
        let mut clock = CssAnimationClock::new(timing(), start);
        clock.update_timing(
            CssAnimationTiming { play_state: AnimationPlayState::Paused, ..timing() },
            start + Duration::from_millis(500),
        );
        assert_eq!(clock.effective_elapsed(start + Duration::from_secs(5)), 0.5);
        clock.update_timing(timing(), start + Duration::from_secs(5));
        assert!((clock.effective_elapsed(start + Duration::from_millis(5500)) - 1.0).abs() < 0.001);
    }
}
