use indicatif::{MultiProgress, MultiProgressAlignment, ProgressBar, ProgressStyle};

pub enum MsbProgressTheme {
    Default,
    Custom(ProgressStyle),
}

pub struct MsbMultiProgress {
    #[cfg(feature = "cli")]
    progress: MultiProgress,
}

impl MsbMultiProgress {
    pub fn with_theme(theme: MsbProgressTheme) -> Self {
        #[cfg(not(feature = "cli"))]
        return Self {};

        #[cfg(feature = "cli")]
        {
            let mut progress = MultiProgress::new();
            theme.apply_to_multi_progress(&mut progress);
            Self { progress }
        }
    }

    pub fn add(&self, progress_bar: MsbProgressBar) {
        #[cfg(feature = "cli")]
        self.progress.add(progress_bar.progress);
    }
}

pub struct MsbProgressBar {
    #[cfg(feature = "cli")]
    progress: ProgressBar,
}

impl MsbProgressBar {
    pub fn with_theme(theme: MsbProgressTheme, len: u64) -> Self {
        #[cfg(not(feature = "cli"))]
        return Self {};

        #[cfg(feature = "cli")]
        {
            let mut progress = ProgressBar::new(len);
            theme.apply_to_progress(&mut progress);
            Self { progress }
        }
    }

    pub fn set_prefix(&self, prefix: String) {
        #[cfg(feature = "cli")]
        self.progress.set_prefix(prefix);
    }

    pub fn inc(&self, n: u64) {
        #[cfg(feature = "cli")]
        self.progress.inc(n);
    }
}

impl From<ProgressStyle> for MsbProgressTheme {
    fn from(style: ProgressStyle) -> Self {
        MsbProgressTheme::Custom(style)
    }
}

impl MsbProgressTheme {
    #[cfg(feature = "cli")]
    fn apply_to_multi_progress(&self, mp: &mut MultiProgress) {
        match self {
            MsbProgressTheme::Custom(_) => {}
            MsbProgressTheme::Default => mp.set_alignment(MultiProgressAlignment::Top),
        }
    }

    #[cfg(feature = "cli")]
    fn apply_to_progress(&self, p: &mut ProgressBar) {
        let style = match self {
            MsbProgressTheme::Custom(style) => style.clone(),
            MsbProgressTheme::Default => ProgressStyle::with_template(
                "{prefix:.bold.dim} {bar:40.green/green.dim} {bytes:.bold}/{total_bytes:.dim}",
            )
            .expect("Progress style template to be valid")
            .progress_chars("=+-"),
        };

        p.set_style(style);
    }
}
