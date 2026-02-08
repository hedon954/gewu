use std::collections::HashMap;

use console::{Emoji, Term, style};
use termimad::MadSkin;

use crate::{
    domain::{
        models::{Record, Task},
        state::TaskStatus,
    },
    ports::llm::SmartGoalDetail,
};

static CHECKMARK: Emoji<'_, '_> = Emoji("✅ ", "[OK] ");
static CROSS: Emoji<'_, '_> = Emoji("❌ ", "[X] ");
static HOURGLASS: Emoji<'_, '_> = Emoji("⏳ ", "... ");
static LIGHTBULB: Emoji<'_, '_> = Emoji("💡 ", "[!] ");
static TARGET: Emoji<'_, '_> = Emoji("🎯 ", "=> ");
static BOOK: Emoji<'_, '_> = Emoji("📝 ", "");
static CHART: Emoji<'_, '_> = Emoji("📊 ", "");
static THOUGHT: Emoji<'_, '_> = Emoji("💭 ", "");
static COMPASS: Emoji<'_, '_> = Emoji("🧭 ", "=> ");

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

    pub fn print_task_detail(&self, task: &Task) {
        // Header
        println!("\n{}", style(self.separator("default")).dim());
        println!(
            "{} {}  {}",
            BOOK,
            style(&task.topic).bold(),
            style(format!("(#{})", task.id)).dim()
        );
        println!("{}", style(self.separator("default")).dim());

        // Status
        let status_style = match task.status {
            crate::domain::state::TaskStatus::Planning => style(&task.status).yellow(),
            crate::domain::state::TaskStatus::Active => style(&task.status).green(),
            crate::domain::state::TaskStatus::Reviewing => style(&task.status).cyan(),
            crate::domain::state::TaskStatus::Completed => style(&task.status).green().bold(),
            crate::domain::state::TaskStatus::Discarded => style(&task.status).red().dim(),
        };
        println!("\n{} {}: {}", CHART, style("Status").dim(), status_style);

        // Motivation
        if let Some(motivation) = &task.motivation {
            println!("\n{} {}", THOUGHT, style("Motivation").cyan().bold());
            self.print_wrapped_text(motivation, 3);
        }

        // SMART Goal
        if let Some(smart_goal_str) = &task.smart_goal {
            println!("\n{} {}", TARGET, style("SMART Goal").magenta().bold());
            // Try to parse as structured SmartGoalDetail
            if let Ok(detail) = serde_json::from_str::<SmartGoalDetail>(smart_goal_str) {
                self.print_smart_goal_table(&detail);
            } else {
                // Fallback: plain text
                self.print_wrapped_text(smart_goal_str, 3);
            }
        }

        // Timestamps
        println!(
            "\n   {} {}    {} {}",
            style("Created:").dim(),
            style(task.created_at.format("%Y-%m-%d %H:%M")).dim(),
            style("Updated:").dim(),
            style(task.updated_at.format("%Y-%m-%d %H:%M")).dim(),
        );

        println!("\n{}", style(self.separator("default")).dim());
    }

    pub fn print_task_list(&self, tasks: Vec<Task>) {
        if tasks.is_empty() {
            println!(
                "\n{} {}",
                LIGHTBULB,
                style("No tasks found. Use `gewu add` to create one!").dim()
            );
            return;
        }

        let grouped = group_tasks_by_status(tasks);

        // Display order: Active → Planning → Reviewing → Completed → Discarded
        let status_order = [
            (TaskStatus::Active, "Active", "green"),
            (TaskStatus::Planning, "Planning", "yellow"),
            (TaskStatus::Reviewing, "Reviewing", "cyan"),
            (TaskStatus::Completed, "Completed", "green_bold"),
            (TaskStatus::Discarded, "Discarded", "red"),
        ];

        let mut first_group = true;
        for (status, label, _color) in &status_order {
            if let Some(tasks) = grouped.get(status) {
                if !first_group {
                    println!();
                }
                first_group = false;

                // Group header
                let badge = self.status_badge(status);
                println!("\n {} {} ({})", badge, style(*label).bold(), tasks.len());
                println!(" {}", style("─".repeat(self.width - 1)).dim());

                for task in tasks {
                    self.print_task_card(task);
                }
            }
        }
        println!();
    }

    pub fn print_task_card(&self, task: &Task) {
        // Line 1: ID + Topic
        println!(
            "  {} {}  {}",
            style(format!("#{:<3}", task.id)).dim(),
            style(&task.topic).bold(),
            self.status_badge(&task.status),
        );

        // Line 2: Motivation (truncated) + SMART goal indicator
        let motivation_preview = task.motivation.as_deref().unwrap_or("-");
        let smart_icon = if task.smart_goal.is_some() {
            format!("{}", style("SMART ✓").green().dim())
        } else {
            format!("{}", style("SMART ✗").yellow().dim())
        };
        print!(
            "        {}",
            style(self.truncate_text(motivation_preview, self.width - 22)).dim()
        );
        println!("  {}", smart_icon);

        // Line 3: Timestamps
        println!(
            "        {} {}  {} {}",
            style("created").dim(),
            style(task.created_at.format("%m-%d %H:%M")).dim(),
            style("updated").dim(),
            style(task.updated_at.format("%m-%d %H:%M")).dim(),
        );

        // Separator between cards
        println!("  {}", style("· · ·").dim());
    }

    fn status_badge(&self, status: &TaskStatus) -> String {
        match status {
            TaskStatus::Planning => format!("{}", style("◉ Planning").yellow()),
            TaskStatus::Active => format!("{}", style("▶ Active").green()),
            TaskStatus::Reviewing => format!("{}", style("◎ Reviewing").cyan()),
            TaskStatus::Completed => format!("{}", style("✔ Completed").green().bold()),
            TaskStatus::Discarded => format!("{}", style("✘ Discarded").red().dim()),
        }
    }

    fn truncate_text(&self, text: &str, max_width: usize) -> String {
        use unicode_width::UnicodeWidthStr;

        let display_width = UnicodeWidthStr::width(text);
        if display_width <= max_width {
            return text.to_string();
        }

        let suffix = "...";
        let target = max_width.saturating_sub(suffix.len());
        let mut current_width = 0;
        let truncated: String = text
            .chars()
            .take_while(|c| {
                let w = unicode_width::UnicodeWidthChar::width(*c).unwrap_or(0);
                if current_width + w > target {
                    return false;
                }
                current_width += w;
                true
            })
            .collect();
        format!("{}{}", truncated, suffix)
    }

    // ─── Record ───────────────────────────────────────────

    pub fn print_matching_records(&self) {
        println!(
            "\n{} {}",
            HOURGLASS,
            style("Matching your record with tasks...").cyan()
        );
    }

    pub fn print_matched_tasks(&self, task_ids: &[i64]) {
        println!("\n{}", style(self.separator("success")).green().dim());
        println!(
            "{} {}  {}",
            CHECKMARK,
            style("RECORD MATCHED").green().bold(),
            task_ids
                .iter()
                .map(|id| format!("{}", style(format!("#{}", id)).cyan().bold()))
                .collect::<Vec<String>>()
                .join("  ")
        );
        println!("{}", style(self.separator("success")).green().dim());
    }

    pub fn print_no_matching_tasks(&self) {
        println!("\n{}", style(self.separator("error")).red().dim());
        println!(
            "{} {}",
            CROSS,
            style("No matching tasks found for this record")
                .red()
                .bold()
        );
        println!("{}", style(self.separator("error")).red().dim());
    }

    pub fn print_record_success(&self) {
        println!(
            "\n{} {}\n",
            CHECKMARK,
            style("Learning progress recorded!").green().bold()
        );
    }

    pub fn print_record_list(&self, records: &[Record]) {
        if records.is_empty() {
            println!(
                "\n{} {}",
                LIGHTBULB,
                style("No records yet. Use `gewu record` to track your progress!").dim()
            );
            return;
        }

        println!("\n{}", style(self.separator("default")).dim());
        println!(
            " {} {} ({})",
            BOOK,
            style("Learning Records").cyan().bold(),
            records.len()
        );
        println!(" {}", style("─".repeat(self.width - 1)).dim());

        // Show records in reverse chronological order (newest first)
        for (i, record) in records.iter().rev().enumerate() {
            let timestamp = style(record.created_at.format("%m-%d %H:%M")).dim();
            let content = self.truncate_text(&record.content, self.width - 18);
            println!(
                "  {}  {}  {}",
                style(format!("{:>2}.", i + 1)).dim(),
                content,
                timestamp,
            );
            if i < records.len() - 1 {
                println!("  {}", style("· · ·").dim());
            }
        }
        println!("{}", style(self.separator("default")).dim());
    }

    // ─── Guide ───────────────────────────────────────────

    pub fn print_guide_header(&self) {
        println!(
            "\n{} {}",
            HOURGLASS,
            style("Generating next-step guide...").cyan()
        );
    }

    pub fn print_guide_footer(&self) {
        println!("\n{}", style(self.separator("default")).dim());
        println!(
            " {} {}",
            COMPASS,
            style("Keep going! Record your progress with `gewu record`").dim()
        );
        println!("{}", style(self.separator("default")).dim());
    }

    /// Render streaming guide output with markdown formatting.
    /// Buffers text by paragraph and renders each complete paragraph
    /// with termimad for proper markdown styling.
    pub async fn print_guide_streaming(&self, rx: &mut tokio::sync::mpsc::Receiver<String>) {
        let skin = Self::guide_skin();
        let mut buffer = String::new();
        let mut block_count: usize = 0;

        println!(); // breathing room before guide content

        while let Some(chunk) = rx.recv().await {
            buffer.push_str(&chunk);
            Self::flush_markdown_blocks(&mut buffer, &skin, &mut block_count);
        }

        // Render any remaining content
        if !buffer.trim().is_empty() {
            if block_count > 0 {
                println!();
            }
            skin.print_text(&buffer);
        }
    }

    /// Flush complete markdown blocks (paragraphs) from the buffer,
    /// rendering them with termimad. Respects code fence boundaries.
    /// Adds blank lines between blocks for better readability.
    fn flush_markdown_blocks(buffer: &mut String, skin: &MadSkin, block_count: &mut usize) {
        loop {
            // Don't split if we're inside a code block
            let fence_count = buffer.matches("```").count();
            if !fence_count.is_multiple_of(2) {
                return;
            }

            // Look for paragraph boundary (double newline)
            if let Some(pos) = buffer.find("\n\n") {
                let block = &buffer[..pos];
                if !block.trim().is_empty() {
                    // Add extra spacing before each block (except the first)
                    if *block_count > 0 {
                        // Add extra blank line before headers for more breathing room
                        if block.trim_start().starts_with("###") {
                            println!();
                        }
                        println!();
                    }
                    skin.print_text(block);
                    *block_count += 1;
                }
                *buffer = buffer[pos + 2..].to_string();
            } else {
                return;
            }
        }
    }

    /// Create a customized termimad skin for guide output
    fn guide_skin() -> MadSkin {
        use crossterm::style::{Attribute, Color};

        let mut skin = MadSkin::default();
        skin.set_headers_fg(Color::Cyan);
        skin.headers[0].add_attr(Attribute::Bold);
        skin.headers[1].add_attr(Attribute::Bold);
        skin.headers[2].add_attr(Attribute::Bold);
        skin.bold.set_fg(Color::White);
        skin.italic.set_fg(Color::Magenta);
        skin.inline_code.set_fg(Color::Yellow);
        skin
    }
}

fn group_tasks_by_status(tasks: Vec<Task>) -> HashMap<TaskStatus, Vec<Task>> {
    let mut map = HashMap::new();
    for task in tasks {
        map.entry(task.status.clone())
            .or_insert_with(Vec::new)
            .push(task);
    }
    map
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}
