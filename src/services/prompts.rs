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

static ref EVALUATE_SMART_GOAL_PROMPT: String = String::from(r#"
You are a wise mentor who evaluates whether a user's SMART goal is well-formed and aligned with their learning topic and motivation.

Topic: {topic}
Motivation: {motivation}
User's SMART Goal: {goal}

Evaluate the goal against the SMART criteria:
- Specific: Is it clear and well-defined?
- Measurable: Can progress be tracked?
- Achievable: Is it realistic given the context?
- Relevant: Does it align with the topic and motivation?
- Time-bound: Does it have a clear deadline or timeframe?

REJECT if:
- It doesn't relate to the topic or motivation
- It lacks measurability or a timeframe
- It's unrealistically ambitious or trivially easy

APPROVE if:
- It meets most SMART criteria reasonably well
- It aligns with the stated topic and motivation

Response format:
{
    "passed": true/false,
    "reason": "Brief analysis of the goal against SMART criteria",
    "recommendation": "If rejected, provide specific suggestions for improvement. If approved, help the user further refine and improve their SMART goal."
}
"#);
}

pub fn audit_motivation_prompt(topic: &str, motivation: &str) -> String {
    AUDIT_MOTIVATION_PROMPT
        .replace("{topic}", topic)
        .replace("{motivation}", motivation)
}

pub fn evaluate_smart_goal_prompt(topic: &str, motivation: &str, goal: &str) -> String {
    EVALUATE_SMART_GOAL_PROMPT
        .replace("{topic}", topic)
        .replace("{motivation}", motivation)
        .replace("{goal}", goal)
}
