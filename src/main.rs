// use std::path::PathBuf;
use structopt::StructOpt;
mod pr;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gen-pr",
    about = "Generate Pull request according to commit logs and given template"
)]
struct CustomOpt {
    // short flag (-d) that represents if we need to print out the logs
    #[structopt(short)]
    debug: bool,

    /// Set base branch
    #[structopt(short = "b", long = "base-branch", default_value = "")]
    base_branch: String,

    /// Set Pull request title
    #[structopt(short = "t", long = "title")]
    title: String,

    /// Set Pull request linked issue
    #[structopt(short = "i", long = "issue-link")]
    issue_link: String,

    /// Set extra description
    #[structopt(short = "e", long = "extra-desciption", default_value = "")]
    extra_description: String,

    // short and long flags (-f, --feature) that represent if it's a feature branch
    #[structopt(short = "f", long = "feature")]
    feature: bool,

    // short flag (-o) that represents if open pull request url after its creation
    #[structopt(short = "o")]
    open: bool,
}

fn main() {
    let args = CustomOpt::from_args();
    pr::pull_request_creator::gen(
        &args.title,
        &args.extra_description,
        &args.base_branch,
        &args.issue_link,
        args.feature,
        args.open,
        args.debug,
    );
}
