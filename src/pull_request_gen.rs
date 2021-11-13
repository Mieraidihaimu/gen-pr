// import all dependencies

use std::process::Command;
use std::str;
use std::fs;

// Function: get all the commit logs against the base branch
fn get_commit_logs(base_branch: &str) -> Result<String, &'static str> {
    // The variable is initialized with the command to get the commit logs.
    let command = format!(
        "git log --reverse --no-merges --pretty=format:\"%h %s\" {}..HEAD",
        base_branch,
    );

    // run the command with `run_command` function
    let output = run_command(&command)?;

    // return error if output is empty
    if output.is_empty() {
        return Err("No commit logs found");
    }
    
    // Split output into lines, and each line runs the `format_commit_log` function.
    let lines = output.split("\n").map(|line| format_commit_log(line));
    
    // Join the lines with `\n` and return the result.
    return Ok(lines.collect::<Vec<String>>().join("\n"));
}

// Function: input a command and output is the output of the command
fn run_command(command: &str) -> Result<String, &'static str> {
    // run the command on shell, and return the output as String
    let cmd = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to run command");


    // check the cmd status code and return the error if the status code is not 0
    if cmd.status.success() {
        let output = String::from_utf8(cmd.stdout);
        return Ok(format!("{}", output.unwrap()));
    } else {
        return Err("Command failed to run");
    }
}

// Function: create a github pull request from the base branch to the head branch, and takes description, title, and base branch as input
fn create_pull_request(
    title: &str,
    description: &str,
    base_branch: &str
) -> Result<String, &'static str> {
    // Push the current branch to Github repository.
    let push_branch_command = format!("git push -u origin HEAD");
    // run the command with `run_command` function and check if the comman is successful
    if run_command(&push_branch_command).is_err() {
        // Return the error.
        return Err("Failed to push the branch.");
    }

    // The variable is initialized with the command to create the pull request using Github CLI
    let create_pr_command = format!(
        "gh pr create -t \"{}\" -b \"{}\" -B {} -d -a @me",
        title.to_string(), description.to_string(), base_branch.to_string()
    ).to_string();

    return run_command(&create_pr_command);
}

// Function: takes input base_branch, title, issue_link, extra descpription and boolean value is_feature_branch
pub fn gen<'a>(
    title: &'a str,
    description: &'a str,
    base_branch: &'a str,
    issue_link: &'a str,
    is_feature_branch: bool) {

    // Get a base branch name from .env file if it exists. Otherwise return the default value `main`
    let env_base_branch = get_env_value(String::from("BASE_BRANCH")).unwrap_or("master".to_string());
    let base = if base_branch.is_empty() { &env_base_branch } else { base_branch };

    // Get a github issue ticket prefix from .env file if it exists. Otherwise return the default value `#`
    let issue_prefix = get_env_value(String::from("ISSUE_PREFIX")).unwrap_or("".to_string());
    let issue = issue_prefix + issue_link;

    // First, define a string variable to store all the commit logs from running `get_commit_logs` function.
    let commit_logs = get_commit_logs(base);
    if commit_logs.is_err() {
        println!("{}", commit_logs.unwrap_err());
        return;
    }

    let logs = commit_logs.unwrap();

    // Second, define a string variable to store pr_description from the commit logs, issue link, pr types, pr template string.

    // Define a tuple to store the result from calling `get_pr_details` function.
    let pr_details = get_pr_details(is_feature_branch);

    let pr_description = format!(
        "#Related links\n\n{}\n\n#Why\n\n{}\n\n#How\n\nChanges included in this pull request:\n{}\n\n#Screenshots\n\n{}\n\n",
        issue.to_string(),
        pr_details.0.to_string(),
        logs.to_string(),
        pr_details.1.to_string()
    );
    
    // Then create a pull request with the `create_pull_request` function. 
    create_pull_request(title, description, base_branch);
}

// Function: takes input boolean is_feature, and returns a tuple with the why_description as string, screenshot_description as string
fn get_pr_details(is_feature_branch: bool) -> (String, String) {
    // If the branch is a feature branch, return the feature branch template.
    if is_feature_branch {
        return (
            String::from("As pet ticket above, Product wants to improve this feature, hence we are ..."), 
            String::from("Normal|Dark Mode|Accessibility |RTL\n---|---|---|---\n<img src= width=200 />|<img src= width=200 />|<img src= width=200 />|<img src= width=200 />\n")
        );
    } else {
        return (
            String::from("As pet ticket above, we need to fix the defect in this release."), 
            String::from("Before|After\n-|-\n<img src= width=200 />|<img src= width=200 />\n")
        );
    }
}

// Function: input a String, capitalize the first letter of first word in the string, and add prefix `- ` to the string
fn format_commit_log(string: &str) -> String {
    // Define a variable of type String.
    // The variable is initialized with the string.
    let mut string = string.to_string();

    // Capitalize the first letter of the first word in the string.
    // string.chars_mut().next().map(|c| c.make_ascii_uppercase());

    // Add prefix `- ` to the string.
    return format!("- {}", string);
}

// Function: input a String key name, and return the value to that key from .env file
fn get_env_value(key: String) -> Result<String, &'static str> {
    // Check if file exists
    if !fs::metadata(".env").is_ok() {
        return Err("No .env file found");
    }

    // Read the .env file and parse it into a HashMap.
    let env_file = fs::read_to_string(".env").unwrap();

    // Get the value of the key from the HashMap
    let line = env_file.split("\n").find(|line| line.contains(&key));

    // Get the value after splitting value by `=`
    let value = match line {
        Some(line) => line.split("=").last().unwrap(),
        None => return Err("No value found"),
    };

    // Check if the value is not empty and return the value, or return an error that states the key is not found.
    if value.is_empty() {
        return Err("No value found");
    }

    return Ok(value.to_string());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_value_found() {
        let value = get_env_value(String::from("BASE_BRANCH"));

        assert_eq!(value.unwrap(), "main");
    }

    #[test]
    fn test_get_env_value_not_found() {
        let value = get_env_value(String::from("BASE_BRANCH_MM"));

        assert!(value.is_err());
    }

    // test run_command function with a valid command
    #[test]
    fn test_run_command_valid() {
        let result = run_command("echo 'Hello World Mier!!!!'");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World Mier!!!!\n");
    }

    // test run_command function with an invalid command and panic
    #[test]
    fn test_run_command_invalid() {
        let result = run_command("halihali 'Hello World Mier!!!!'");

        assert!(result.is_err());
    }

    // test format_commit_log function with a valid string
    #[test]
    fn test_format_commit_log_valid() {
        let result = format_commit_log("Some other string");

        assert_eq!(result, "- Some other string");
    }

    // test git_commit_logs function with invalid branch
    #[test]
    fn test_git_commit_logs_invalid_branch() {
        let result = get_commit_logs("mier");

        assert!(result.is_err());
    }

    // test git_pr_details function with is_feature_branch = true
    #[test]
    fn test_git_pr_details_feature_branch() {
        let result = get_pr_details(true);

        assert_eq!(result.0, "As pet ticket above, Product wants to improve this feature, hence we are ...");
        assert_eq!(result.1, "Normal|Dark Mode|Accessibility |RTL\n---|---|---|---\n<img src= width=200 />|<img src= width=200 />|<img src= width=200 />|<img src= width=200 />\n");
    }

    // test git_pr_details function with is_feature_branch = false
    #[test]
    fn test_git_pr_details_not_feature_branch() {
        let result = get_pr_details(false);

        assert_eq!(result.0, "As pet ticket above, we need to fix the defect in this release.");
        assert_eq!(result.1, "Before|After\n-|-\n<img src= width=200 />|<img src= width=200 />\n");
    }
}