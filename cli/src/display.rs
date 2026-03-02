use colored::{Color, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn banner() {
    let art = r#"
   ██████╗███████╗███████╗
  ██╔════╝██╔════╝██╔════╝
  ██║     ███████╗█████╗
  ██║     ╚════██║██╔══╝
  ╚██████╗███████║██║
   ╚═════╝╚══════╝╚═╝
    "#;
    println!("{}", art.bold().cyan());
    println!(
        "  {}  {}\n",
        "Cloud Service Foundry".bold(),
        format!("v{}", env!("CARGO_PKG_VERSION")).dimmed()
    );
}

pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

pub fn success(message: &str) {
    println!("{} {}", "✔".green().bold(), message);
}

pub fn error(message: &str) {
    eprintln!("{} {}", "✖".red().bold(), message);
}

pub fn warn(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

pub fn info(message: &str) {
    println!("{} {}", "●".cyan(), message);
}

pub fn section(title: &str) {
    println!("\n{}", title.bold().underline());
}

pub fn kv(key: &str, value: &str) {
    println!("  {:<20} {}", key.dimmed(), value);
}

pub fn kv_colored(key: &str, value: &str, color: Color) {
    println!("  {:<20} {}", key.dimmed(), value.color(color).bold());
}

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    color_fn: Option<Box<dyn Fn(usize, &str) -> Color>>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers.iter().map(|s| s.to_string()).collect(),
            rows: Vec::new(),
            color_fn: None,
        }
    }

    pub fn with_color<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, &str) -> Color + 'static,
    {
        self.color_fn = Some(Box::new(f));
        self
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    fn col_widths(&self) -> Vec<usize> {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        widths
    }

    pub fn print(&self) {
        let widths = self.col_widths();
        let total_width: usize = widths.iter().sum::<usize>() + widths.len() * 3;

        let header_line: String = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:<w$}", h.bold().to_string(), w = widths[i] + 10))
            .collect::<Vec<_>>()
            .join("  ");
        println!("  {}", header_line);

        let sep = "─".repeat(total_width.min(120));
        println!("  {}", sep.dimmed());

        for row in &self.rows {
            let row_line: String = row
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let w = widths.get(i).copied().unwrap_or(cell.len());
                    if let Some(ref f) = self.color_fn {
                        let color = f(i, cell);
                        format!("{:<w$}", cell.color(color).to_string(), w = w + 10)
                    } else {
                        format!("{:<w$}", cell, w = w)
                    }
                })
                .collect::<Vec<_>>()
                .join("  ");
            println!("  {}", row_line);
        }
    }
}

pub fn status_color(status: &str) -> Color {
    match status.to_lowercase().as_str() {
        "available" | "online" | "active" | "healthy" | "leader" => Color::Green,
        "creating" | "migrating" | "degraded" | "follower" => Color::Yellow,
        "error" | "offline" | "deleting" | "failed" => Color::Red,
        "inuse" | "in_use" => Color::Cyan,
        _ => Color::White,
    }
}
