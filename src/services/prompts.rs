lazy_static::lazy_static! {
    static ref AUDIT_MOTIVATION_PROMPT: String = String::from(r#"
You are a strict gatekeeper preventing "bookmark hoarding" - the habit of saving
learning resources that never get studied.

Topic: {topic}
Reason: {motivation}

Evaluate the quality of this learning motivation using these criteria:

REJECT if:
- The reason is vague ("improve myself", "might be useful someday")
- No specific use case or problem to solve
- No time pressure or deadline
- Purely driven by FOMO or "collecting knowledge"

APPROVE if:
- Addresses a current, specific problem or need
- Has a clear application scenario (project, task, challenge)
- Can answer: "What will you do differently after learning this?"
- Has some time sensitivity (not "someday maybe")

Response format:
{
    "passed": true/false,
    "reason": "Brief explanation focusing on specificity and use case",
    "recommendation": "If rejected, provide a specific, actionable suggestion for how to improve the motivation. If approved, offer a prompt or next step that helps the user turn their motivation into a specific SMART goal."
}
    "#);
}

pub fn audit_motivation_prompt(topic: &str, motivation: &str) -> String {
    AUDIT_MOTIVATION_PROMPT
        .replace("{topic}", topic)
        .replace("{motivation}", motivation)
}
