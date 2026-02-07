use console::{Emoji, Term, style};

use crate::ports::llm::SmartGoalDetail;

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
        let ch = match style_type {
            "success" => "━",
            "error" => "━",
            _ => "─",
        };
        ch.repeat(self.width)
    }

    // ─── Motivation ─────────────────────────────────────────

    pub fn print_checking_motivation(&self) {
        println!(
            "\n{} {}",
            HOURGLASS,
            style("Reviewing your motivation...").cyan()
        );
    }

    pub fn print_motivation_approved(&self, topic: &str, reason: &str, recommendation: &str) {
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
    }

    pub fn print_motivation_rejected(&self, reason: &str, recommendation: &str) {
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

    // ─── SMART Goal ─────────────────────────────────────────

    pub fn print_checking_smart_goal(&self) {
        println!(
            "\n{} {}",
            HOURGLASS,
            style("Evaluating your SMART goal...").cyan()
        );
    }

    pub fn print_smart_goal_approved(&self, reason: &str, detail: &SmartGoalDetail) {
        println!("\n{}", style(self.separator("success")).green().dim());
        println!(
            "{} {}",
            CHECKMARK,
            style("SMART GOAL APPROVED").green().bold()
        );
        println!("{}", style(self.separator("success")).green().dim());

        println!("\n{} {}", THOUGHT, style("Analysis").cyan().bold());
        self.print_wrapped_text(reason, 3);

        println!(
            "\n{} {}",
            TARGET,
            style("Refined SMART Goal").magenta().bold()
        );
        self.print_smart_goal_table(detail);

        println!("\n{}", style(self.separator("success")).green().dim());
    }

    pub fn print_smart_goal_rejected(&self, reason: &str, guidance: &str) {
        println!("\n{}", style(self.separator("error")).red().dim());
        println!(
            "{} {}",
            CROSS,
            style("SMART GOAL NEEDS IMPROVEMENT").red().bold()
        );
        println!("{}", style(self.separator("error")).red().dim());

        println!(
            "\n{} {}",
            THOUGHT,
            style("Why it needs improvement").yellow().bold()
        );
        self.print_wrapped_text(reason, 3);

        println!(
            "\n{} {}",
            LIGHTBULB,
            style("Think about this").cyan().bold()
        );
        self.print_wrapped_text(guidance, 3);

        println!("\n{}", style(self.separator("error")).red().dim());
    }

    pub fn print_smart_goal_saved(&self) {
        println!(
            "\n{} {}\n",
            CHECKMARK,
            style("SMART goal saved! Status → Active").green().bold()
        );
    }

    pub fn print_smart_goal_not_saved(&self) {
        println!(
            "\n{} {}\n",
            LIGHTBULB,
            style("SMART goal not saved. You can update it later.").yellow()
        );
    }

    // ─── SMART Goal Table ───────────────────────────────────

    fn print_smart_goal_table(&self, detail: &SmartGoalDetail) {
        let rows = [
            ("S", "Specific", &detail.specific),
            ("M", "Measurable", &detail.measurable),
            ("A", "Achievable", &detail.achievable),
            ("R", "Relevant", &detail.relevant),
            ("T", "Time-bound", &detail.time_bound),
        ];

        let label_width = 12;
        let content_width = self.width.saturating_sub(label_width + 7); // 7 = borders + padding

        // top border
        println!(
            "\n   {}{}{}",
            style("┌").dim(),
            style("─".repeat(label_width + 2)).dim(),
            style(format!("┬{}┐", "─".repeat(content_width + 2))).dim(),
        );

        for (i, (letter, label, content)) in rows.iter().enumerate() {
            let styled_label =
                format!("{} {}", style(*letter).cyan().bold(), style(*label).white());
            // The styled label has invisible escape codes, so we need to pad based on the
            // visible length
            let visible_label_len = letter.len() + 1 + label.len();
            let label_padding = label_width.saturating_sub(visible_label_len);

            // Wrap content
            let wrapped = self.wrap_text(content, content_width);

            // First line
            println!(
                "   {} {}{}{} {} {}",
                style("│").dim(),
                styled_label,
                " ".repeat(label_padding),
                style("│").dim(),
                wrapped[0],
                style("│").dim(),
            );

            // Continuation lines
            for line in wrapped.iter().skip(1) {
                println!(
                    "   {} {}{} {} {}",
                    style("│").dim(),
                    " ".repeat(label_width),
                    style("│").dim(),
                    line,
                    style("│").dim(),
                );
            }

            // row separator (not after last row)
            if i < rows.len() - 1 {
                println!(
                    "   {}{}{}",
                    style("├").dim(),
                    style("─".repeat(label_width + 2)).dim(),
                    style(format!("┼{}┤", "─".repeat(content_width + 2))).dim(),
                );
            }
        }

        // bottom border
        println!(
            "   {}{}{}",
            style("└").dim(),
            style("─".repeat(label_width + 2)).dim(),
            style(format!("┴{}┘", "─".repeat(content_width + 2))).dim(),
        );
    }

    // ─── Text Utilities ─────────────────────────────────────

    fn wrap_text(&self, text: &str, max_width: usize) -> Vec<String> {
        use unicode_width::UnicodeWidthStr;

        let mut lines = Vec::new();

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                lines.push(String::new());
                continue;
            }

            let words: Vec<&str> = trimmed.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let word_width = UnicodeWidthStr::width(word);

                // If the word itself is longer than max_width, force-wrap it
                if word_width > max_width {
                    // Flush current line first
                    if !current_line.is_empty() {
                        lines.push(self.pad_to_width(&current_line, max_width));
                        current_line.clear();
                    }
                    // Break the long word into multiple lines
                    lines.extend(self.force_wrap_long_word(word, max_width));
                    continue;
                }

                if current_line.is_empty() {
                    current_line = word.to_string();
                } else if UnicodeWidthStr::width(current_line.as_str()) + 1 + word_width
                    <= max_width
                {
                    current_line.push(' ');
                    current_line.push_str(word);
                } else {
                    lines.push(self.pad_to_width(&current_line, max_width));
                    current_line = word.to_string();
                }
            }

            if !current_line.is_empty() {
                lines.push(self.pad_to_width(&current_line, max_width));
            }
        }

        if lines.is_empty() {
            lines.push(" ".repeat(max_width));
        }

        lines
    }

    /// Force-wrap a long word that exceeds max_width by breaking it at character boundaries
    fn force_wrap_long_word(&self, word: &str, max_width: usize) -> Vec<String> {
        use unicode_width::UnicodeWidthChar;

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0;

        for ch in word.chars() {
            let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);

            if current_width + ch_width > max_width {
                // Current line is full, push it and start new line
                lines.push(self.pad_to_width(&current_line, max_width));
                current_line.clear();
                current_width = 0;
            }

            current_line.push(ch);
            current_width += ch_width;
        }

        if !current_line.is_empty() {
            lines.push(self.pad_to_width(&current_line, max_width));
        }

        lines
    }

    fn pad_to_width(&self, text: &str, width: usize) -> String {
        use unicode_width::UnicodeWidthStr;

        let display_width = UnicodeWidthStr::width(text);
        if display_width >= width {
            // Truncate by display width, respecting character boundaries
            let mut current_width = 0;
            let truncated: String = text
                .chars()
                .take_while(|c| {
                    let w = unicode_width::UnicodeWidthChar::width(*c).unwrap_or(0);
                    if current_width + w > width {
                        return false;
                    }
                    current_width += w;
                    true
                })
                .collect();
            let remaining = width.saturating_sub(UnicodeWidthStr::width(truncated.as_str()));
            format!("{}{}", truncated, " ".repeat(remaining))
        } else {
            format!("{}{}", text, " ".repeat(width - display_width))
        }
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
