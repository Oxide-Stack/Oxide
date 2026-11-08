#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub verbose: bool,
    pub dry_run: bool,
    pub no_color: bool,
}

impl Output {
    pub fn new(verbose: bool, dry_run: bool, no_color: bool) -> Self {
        Self {
            verbose,
            dry_run,
            no_color,
        }
    }

    pub fn apply(&self) {
        console::set_colors_enabled(!self.no_color);
    }

    pub fn info(&self, msg: &str) {
        println!("{msg}");
    }

    pub fn ok(&self, msg: &str) {
        println!("{}", console::style(format!("ok: {msg}")).green().bold());
    }

    pub fn warn(&self, msg: &str) {
        eprintln!("{}", console::style(format!("warn: {msg}")).yellow().bold());
    }

    pub fn error(&self, msg: &str) {
        eprintln!("{}", console::style(format!("error: {msg}")).red().bold());
    }

    pub fn verbose(&self, msg: &str) {
        if self.verbose {
            println!("{}", console::style(format!("debug: {msg}")).dim());
        }
    }

    pub fn dry(&self, msg: &str) {
        if self.dry_run {
            println!("{}", console::style(format!("dry-run: {msg}")).cyan());
        }
    }
}
