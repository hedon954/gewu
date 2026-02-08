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
You are a supportive mentor who helps users refine their SMART goals while respecting their ambitions.

Topic: {topic}
Motivation: {motivation}
User's SMART Goal: {goal}

Evaluate the goal against the SMART criteria with a permissive approach:
- Specific: Is there enough clarity about what will be done?
- Measurable: Can we identify some way to track progress?
- Achievable: Does the user have a reasonable plan or timeframe?
- Relevant: Does it connect to the topic and motivation?
- Time-bound: Is there some deadline or timeframe mentioned?

ONLY REJECT if:
- The goal is extremely vague (e.g. "learn stuff", "get better") with no concrete direction
- It completely contradicts the topic or motivation
- It has absolutely no timeframe or measurability
- It's clearly impossible (e.g. "master 10 programming languages in 1 day")

BE SUPPORTIVE and APPROVE if:
- The goal has clear intent and direction, even if ambitious
- There's a reasonable timeframe and plan, even if challenging
- The user has thought about how to approach it
- It shows genuine commitment with time allocation

Remember: Users can set ambitious goals. Your job is to refine and structure them, not to reduce them to minimal MVPs. Respect their aspirations.

IMPORTANT: You MUST respond with valid JSON only, no extra text.

If REJECTED, respond with:
{
    "passed": false,
    "reason": "Brief explanation of what critical SMART elements are missing",
    "guidance": "Gentle suggestions to add the missing elements without reducing scope"
}

If APPROVED, respond with:
{
    "passed": true,
    "reason": "Brief acknowledgment of the goal's strengths",
    "refined_goal": {
        "specific": "A clear, refined description of WHAT will be accomplished (keep the user's scope)",
        "measurable": "Concrete metrics or indicators to track progress and completion",
        "achievable": "Why this goal is realistic given the user's commitment and plan",
        "relevant": "How this goal connects to the topic and serves the motivation",
        "time_bound": "Specific deadline and suggested milestones to help track progress"
    }
}
"#);

    static ref MATCH_TASKS_PROMPT: String = String::from(r#"
Your are a helpful assistant to match the learning records with the given tasks.
If the record matches the tasks, return the array of task ids in JSON format.
If the record does not match any tasks, return an empty array.
Response format:
{
    "task_ids": [task_id1, task_id2, ...]
}

Here are the tasks:
{tasks}

Here is the learning record:
{record}
"#);

    static ref GENERATE_GUIDE_PROMPT: String = String::from(r#"
Your are a helpful assistant to generate a learning guide for the given task and records.
The guide should be a step-by-step guide to help the user learn the task.
The guide should be in markdown format.
Response format:
{guide}

Here is the task:
{task}

Here is the learning records:
{records}
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

pub fn match_tasks_prompt(tasks: &str, record: &str) -> String {
    MATCH_TASKS_PROMPT
        .replace("{tasks}", tasks)
        .replace("{record}", record)
}

pub fn generate_guide_prompt(task: &str, records: &str) -> String {
    GENERATE_GUIDE_PROMPT
        .replace("{task}", task)
        .replace("{records}", records)
}
