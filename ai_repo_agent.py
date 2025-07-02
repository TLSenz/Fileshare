import os
from github import Github
import json
import requests
from datetime import datetime

# --- Configuration ---
# Replace with your GitHub Personal Access Token (PAT)
# It's highly recommended to set this as an environment variable (e.g., GITHUB_TOKEN)
GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN", "YOUR_GITHUB_TOKEN")
REPO_OWNER = "TLSenZ"  # Replace with your GitHub username
REPO_NAME = "fileshare"        # Replace with your repository name
FEATURES_FILE_PATH = "README.md"  # Path to your features list file in the repo

# Gemini API configuration
# Leave API key empty; Canvas will inject it at runtime for gemini-2.0-flash
GEMINI_API_KEY = ""
GEMINI_MODEL_NAME = "gemini-2.0-flash"
GEMINI_API_URL = f"https://generativelanguage.googleapis.com/v1beta/models/{GEMINI_MODEL_NAME}:generateContent?key={GEMINI_API_KEY}"

# --- GitHub Utility Functions ---

def get_github_repo():
    """Authenticates with GitHub and returns the repository object."""
    try:
        g = Github(GITHUB_TOKEN)
        repo = g.get_user().get_repo(REPO_NAME)
        print(f"Successfully connected to GitHub repository: {REPO_OWNER}/{REPO_NAME}")
        return repo
    except Exception as e:
        print(f"Error connecting to GitHub: {e}")
        exit(1)

def get_file_content(repo, file_path):
    """Fetches the content of a file from the repository."""
    try:
        contents = repo.get_contents(file_path)
        # contents.decoded_content is bytes, decode to utf-8
        return contents.decoded_content.decode('utf-8')
    except Exception as e:
        print(f"Error getting content of {file_path}: {e}")
        return None

def create_github_issue(repo, title, body, labels=None):
    """Creates a new GitHub issue."""
    try:
        # Check if an issue with a similar title already exists to avoid duplicates
        open_issues = repo.get_issues(state='open')
        for issue in open_issues:
            if issue.title.strip().lower() == title.strip().lower():
                print(f"Skipping duplicate issue: '{title}' already exists.")
                return None

        issue = repo.create_issue(title=title, body=body, labels=labels)
        print(f"Successfully created GitHub issue: #{issue.number} - {issue.title}")
        return issue
    except Exception as e:
        print(f"Error creating GitHub issue '{title}': {e}")
        return None

# --- AI (LLM) Interaction Functions ---

async def call_gemini_api(prompt, schema=None):
    """
    Calls the Gemini API to generate content.
    If a schema is provided, it requests a structured JSON response.
    """
    chat_history = [{"role": "user", "parts": [{"text": prompt}]}]
    payload = {"contents": chat_history}

    if schema:
        payload["generationConfig"] = {
            "responseMimeType": "application/json",
            "responseSchema": schema
        }

    headers = {'Content-Type': 'application/json'}

    try:
        # Using requests for synchronous call, as fetch is JS-specific.
        # In a real async Python app, you'd use aiohttp or httpx.
        response = requests.post(GEMINI_API_URL, headers=headers, data=json.dumps(payload))
        response.raise_for_status()  # Raise an exception for HTTP errors
        result = response.json()

        if result.get("candidates") and result["candidates"][0].get("content") and result["candidates"][0]["content"].get("parts"):
            text_content = result["candidates"][0]["content"]["parts"][0]["text"]
            if schema:
                try:
                    return json.loads(text_content)
                except json.JSONDecodeError:
                    print(f"Warning: LLM response was not valid JSON: {text_content}")
                    return None
            return text_content
        else:
            print(f"LLM response structure unexpected: {result}")
            return None
    except requests.exceptions.RequestException as e:
        print(f"Error calling Gemini API: {e}")
        return None

async def analyze_code_for_bugs(code_content):
    """
    Uses LLM to analyze code for potential bugs.
    Returns a list of dictionaries, each with 'title' and 'description'.
    """
    prompt = f"""
    Review the following code snippet for potential bugs, logical errors, security vulnerabilities, or areas of improvement.
    Focus on functional correctness and common pitfalls.
    Provide your findings as a JSON array of objects, where each object has a 'title' (short summary of the bug/issue)
    and a 'description' (detailed explanation and suggested fix).
    If no issues are found, return an empty JSON array `[]`.

    Code:
    ```
    {code_content}
    ```
    """
    schema = {
        "type": "ARRAY",
        "items": {
            "type": "OBJECT",
            "properties": {
                "title": {"type": "STRING"},
                "description": {"type": "STRING"}
            },
            "required": ["title", "description"]
        }
    }
    print("Analyzing code for bugs...")
    issues = await call_gemini_api(prompt, schema)
    return issues if isinstance(issues, list) else []

async def generate_feature_issue(feature_description):
    """
    Uses LLM to generate a GitHub issue title and description for a new feature.
    Returns a dictionary with 'title' and 'description'.
    """
    prompt = f"""
    Generate a concise GitHub issue title and a detailed description for the following new feature idea.
    The description should include a brief explanation of the feature, its purpose, and potential steps to implement it.
    Provide the output as a JSON object with 'title' and 'description' keys.

    Feature Idea:
    {feature_description}
    """
    schema = {
        "type": "OBJECT",
        "properties": {
            "title": {"type": "STRING"},
            "description": {"type": "STRING"}
        },
        "required": ["title", "description"]
    }
    print(f"Generating issue for feature: '{feature_description}'...")
    issue_data = await call_gemini_api(prompt, schema)
    return issue_data if isinstance(issue_data, dict) else None

# --- Main Agent Logic ---

async def run_agent():
    """Main function to run the AI agent."""
    print(f"AI Agent started at {datetime.now()}")
    repo = get_github_repo()

    # --- 1. Code Bug Analysis ---
    print("\n--- Starting Code Bug Analysis ---")
    code_files_to_check = [
        "src/main.rs", # Example Rust file
        "src/lib.rs",
        "src/auth/mod.rs",
        "src/services/mod.rs",
        # Add more relevant code file paths here
    ]
    all_bug_issues = []

    for file_path in code_files_to_check:
        print(f"Fetching code from: {file_path}")
        code_content = get_file_content(repo, file_path)
        if code_content:
            bugs = await analyze_code_for_bugs(code_content)
            if bugs:
                print(f"Found {len(bugs)} potential issues in {file_path}.")
                for bug in bugs:
                    all_bug_issues.append(bug)
            else:
                print(f"No significant issues found in {file_path}.")
        else:
            print(f"Could not retrieve content for {file_path}. Skipping analysis.")

    # Create GitHub issues for bugs
    if all_bug_issues:
        print("\n--- Creating GitHub Issues for Bugs ---")
        for bug_issue in all_bug_issues:
            create_github_issue(
                repo,
                f"[Bug] {bug_issue['title']}",
                f"**Found by AI Agent (Code Scan)**\n\n{bug_issue['description']}\n\n*Source File: (Consider adding the file path here)*",
                labels=["bug", "ai-scan"]
            )
    else:
        print("\nNo new bug issues to create.")

    # --- 2. Feature List Analysis ---
    print("\n--- Starting Feature List Analysis ---")
    features_content = get_file_content(repo, FEATURES_FILE_PATH)
    if features_content:
        # Simple parsing: assume each line is a feature, or parse markdown list
        # For a more robust solution, you might parse a structured format (YAML/JSON)
        # or compare against a previous run's features to find *new* ones.
        feature_ideas = [line.strip() for line in features_content.split('\n') if line.strip() and not line.strip().startswith('#')]
        print(f"Found {len(feature_ideas)} feature ideas in {FEATURES_FILE_PATH}.")

        for feature_idea in feature_ideas:
            issue_data = await generate_feature_issue(feature_idea)
            if issue_data:
                create_github_issue(
                    repo,
                    f"[Feature] {issue_data['title']}",
                    f"**Generated by AI Agent (Feature List Scan)**\n\n{issue_data['description']}",
                    labels=["feature", "enhancement", "ai-scan"]
                )
    else:
        print(f"Could not retrieve content for {FEATURES_FILE_PATH}. Skipping feature analysis.")

    print(f"\nAI Agent finished at {datetime.now()}")

# To run this, you'd typically use an async runner like asyncio:
# import asyncio
# if __name__ == "__main__":
#     asyncio.run(run_agent())

# For Canvas environment, we can run it directly if not in a web context
# or provide instructions for local execution.
# Since this is a Python script and not a web app, it would be run directly.
# The user will need to execute this script in their environment.

# Note: The `await` calls here are illustrative. If running this directly as a script
# without an explicit `asyncio.run()`, you'd need to ensure the environment supports it
# or convert `call_gemini_api` to be synchronous (using `requests` as shown).
# For simplicity, I've used `requests` for the LLM call, making it synchronous.
# If you want to use `asyncio` for better performance with multiple LLM calls,
# you'd replace `requests` with an async HTTP client like `httpx` or `aiohttp`.

# Re-checking the instructions, I should provide a complete, runnable example.
# The `requests` library is synchronous, so the `async` keywords on `call_gemini_api`,
# `analyze_code_for_bugs`, `generate_feature_issue`, and `run_agent` are technically
# not strictly necessary for this specific implementation, but they are good practice
# if you plan to introduce true async I/O (e.g., for concurrent LLM calls).
# For a simple script, we can remove `async/await` and use direct synchronous calls.

# Let's refine for a direct synchronous script execution for simplicity in a non-web context.

# --- REVISED SYNCHRONOUS AI Agent Logic ---

def call_gemini_api_sync(prompt, schema=None):
    """
    Synchronous call to the Gemini API to generate content.
    """
    chat_history = [{"role": "user", "parts": [{"text": prompt}]}]
    payload = {"contents": chat_history}

    if schema:
        payload["generationConfig"] = {
            "responseMimeType": "application/json",
            "responseSchema": schema
        }

    headers = {'Content-Type': 'application/json'}

    try:
        response = requests.post(GEMINI_API_URL, headers=headers, data=json.dumps(payload))
        response.raise_for_status()
        result = response.json()

        if result.get("candidates") and result["candidates"][0].get("content") and result["candidates"][0]["content"].get("parts"):
            text_content = result["candidates"][0]["content"]["parts"][0]["text"]
            if schema:
                try:
                    return json.loads(text_content)
                except json.JSONDecodeError:
                    print(f"Warning: LLM response was not valid JSON: {text_content}")
                    return None
            return text_content
        else:
            print(f"LLM response structure unexpected: {result}")
            return None
    except requests.exceptions.RequestException as e:
        print(f"Error calling Gemini API: {e}")
        return None

def analyze_code_for_bugs_sync(code_content):
    """
    Synchronous version: Uses LLM to analyze code for potential bugs.
    Returns a list of dictionaries, each with 'title' and 'description'.
    """
    prompt = f"""
    Review the following code snippet for potential bugs, logical errors, security vulnerabilities, or areas of improvement.
    Focus on functional correctness and common pitfalls.
    Provide your findings as a JSON array of objects, where each object has a 'title' (short summary of the bug/issue)
    and a 'description' (detailed explanation and suggested fix).
    If no issues are found, return an empty JSON array `[]`.

    Code:
    ```
    {code_content}
    ```
    """
    schema = {
        "type": "ARRAY",
        "items": {
            "type": "OBJECT",
            "properties": {
                "title": {"type": "STRING"},
                "description": {"type": "STRING"}
            },
            "required": ["title", "description"]
        }
    }
    print("Analyzing code for bugs...")
    issues = call_gemini_api_sync(prompt, schema)
    return issues if isinstance(issues, list) else []

def generate_feature_issue_sync(feature_description):
    """
    Synchronous version: Uses LLM to generate a GitHub issue title and description for a new feature.
    Returns a dictionary with 'title' and 'description'.
    """
    prompt = f"""
    Generate a concise GitHub issue title and a detailed description for the following new feature idea.
    The description should include a brief explanation of the feature, its purpose, and potential steps to implement it.
    Provide the output as a JSON object with 'title' and 'description' keys.

    Feature Idea:
    {feature_description}
    """
    schema = {
        "type": "OBJECT",
        "properties": {
            "title": {"type": "STRING"},
            "description": {"type": "STRING"}
        },
        "required": ["title", "description"]
    }
    print(f"Generating issue for feature: '{feature_description}'...")
    issue_data = call_gemini_api_sync(prompt, schema)
    return issue_data if isinstance(issue_data, dict) else None


def run_agent_sync():
    """Main synchronous function to run the AI agent."""
    print(f"AI Agent started at {datetime.now()}")
    repo = get_github_repo()

    # --- 1. Code Bug Analysis ---
    print("\n--- Starting Code Bug Analysis ---")
    code_files_to_check = [
        "src/main.rs", # Example Rust file
        "src/lib.rs",
        "src/auth/mod.rs",
        "src/services/mod.rs",
        # Add more relevant code file paths here based on your repo structure
    ]
    all_bug_issues = []

    for file_path in code_files_to_check:
        print(f"Fetching code from: {file_path}")
        code_content = get_file_content(repo, file_path)
        if code_content:
            bugs = analyze_code_for_bugs_sync(code_content)
            if bugs:
                print(f"Found {len(bugs)} potential issues in {file_path}.")
                for bug in bugs:
                    all_bug_issues.append(bug)
            else:
                print(f"No significant issues found in {file_path}.")
        else:
            print(f"Could not retrieve content for {file_path}. Skipping analysis.")

    # Create GitHub issues for bugs
    if all_bug_issues:
        print("\n--- Creating GitHub Issues for Bugs ---")
        for bug_issue in all_bug_issues:
            create_github_issue(
                repo,
                f"[AI Bug] {bug_issue['title']}", # Added "AI Bug" prefix for clarity
                f"**Found by AI Agent (Code Scan)**\n\n{bug_issue['description']}\n\n*Source File: `{file_path}`*", # Included file path
                labels=["bug", "ai-scan"]
            )
    else:
        print("\nNo new bug issues to create.")

    # --- 2. Feature List Analysis ---
    print("\n--- Starting Feature List Analysis ---")
    features_content = get_file_content(repo, FEATURES_FILE_PATH)
    if features_content:
        # Simple parsing: assume each line is a feature, or parse markdown list
        # For a more robust solution, you might parse a structured format (YAML/JSON)
        # or compare against a previous run's features to find *new* ones.
        # This example just processes every non-empty, non-comment line.
        feature_ideas = [line.strip() for line in features_content.split('\n') if line.strip() and not line.strip().startswith('#')]
        print(f"Found {len(feature_ideas)} feature ideas in {FEATURES_FILE_PATH}.")

        for feature_idea in feature_ideas:
            issue_data = generate_feature_issue_sync(feature_idea)
            if issue_data:
                create_github_issue(
                    repo,
                    f"[AI Feature] {issue_data['title']}", # Added "AI Feature" prefix
                    f"**Generated by AI Agent (Feature List Scan)**\n\n{issue_data['description']}",
                    labels=["feature", "enhancement", "ai-scan"]
                )
    else:
        print(f"Could not retrieve content for {FEATURES_FILE_PATH}. Skipping feature analysis.")

    print(f"\nAI Agent finished at {datetime.now()}")

if __name__ == "__main__":
    run_agent_sync()
