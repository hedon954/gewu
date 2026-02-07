use console::{Emoji, Term, style};

static CHECKMARK: Emoji<'_, '_> = Emoji("✅ ", "[OK] ");
static CROSS: Emoji<'_, '_> = Emoji("❌ ", "[X] ");
static HOURGLASS: Emoji<'_, '_> = Emoji("⏳ ", "... ");
static LIGHTBULB: Emoji<'_, '_> = Emoji("💡 ", "[!] ");
static TARGET: Emoji<'_, '_> = Emoji("🎯 ", "=> ");
static BOOK: Emoji<'_, '_> = Emoji("📝 ", "");
static CHART: Emoji<'_, '_> = Emoji("📊 ", "");
static THOUGHT: Emoji<'_, '_> = Emoji("💭 ", "");

pub struct UI {
    width: usize,
}

impl UI {
    pub fn new() -> Self {
        let term = Term::stdout();
        let width = term.size().1 as usize;
        Self {
            width: width.min(80),
        }
    }

    fn separator(&self, style_type: &str) -> String {
        let char = match style_type {
            "success" => "━",
            "error" => "━",
            _ => "─",
        };
        char.repeat(self.width)
    }

    pub fn print_checking(&self) {
        println!(
            "\n{} {}",
            HOURGLASS,
            style("Reviewing your motivation...").cyan()
        );
    }

    pub fn print_approved(&self, topic: &str, reason: &str, recommendation: &str) {
        println!("\n{}", style(self.separator("success")).green().dim());
        println!(
            "{} {}",
            CHECKMARK,
            style("MOTIVATION APPROVED").green().bold()
        );
        println!("{}", style(self.separator("success")).green().dim());

        println!("\n{} {}", BOOK, style(topic).bold());
        println!(
            "{} {}",
            CHART,
            style("Status: Validating → Planning").yellow()
        );

        println!("\n{} {}", THOUGHT, style("Analysis").cyan().bold());
        self.print_wrapped_text(reason, 3);

        println!("\n{} {}", TARGET, style("Next Step").magenta().bold());
        self.print_wrapped_text(recommendation, 3);

        println!("\n{}", style(self.separator("success")).green().dim());
        println!(
            "\n{} Use {} to view all tasks\n",
            style("💡").cyan(),
            style("gewu list").yellow()
        );
    }

    pub fn print_rejected(&self, reason: &str, recommendation: &str) {
        println!("\n{}", style(self.separator("error")).red().dim());
        println!("{} {}", CROSS, style("MOTIVATION REJECTED").red().bold());
        println!("{}", style(self.separator("error")).red().dim());

        println!(
            "\n{} {}",
            THOUGHT,
            style("Why it was rejected").yellow().bold()
        );
        self.print_wrapped_text(reason, 3);

        println!("\n{} {}", LIGHTBULB, style("How to improve").cyan().bold());
        self.print_wrapped_text(recommendation, 3);

        println!("\n{}", style(self.separator("error")).red().dim());
        println!(
            "\n{} Try again with a more specific motivation\n",
            style("💡").cyan()
        );
    }

    fn print_wrapped_text(&self, text: &str, indent: usize) {
        let indent_str = " ".repeat(indent);
        let max_width = self.width.saturating_sub(indent + 2);

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                println!();
                continue;
            }

            let words: Vec<&str> = trimmed.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                if current_line.is_empty() {
                    current_line = word.to_string();
                } else if current_line.len() + word.len() < max_width {
                    current_line.push(' ');
                    current_line.push_str(word);
                } else {
                    println!("{}{}", indent_str, current_line);
                    current_line = word.to_string();
                }
            }

            if !current_line.is_empty() {
                println!("{}{}", indent_str, current_line);
            }
        }
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}
