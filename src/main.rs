// use std::path::PathBuf;
use structopt::StructOpt;
mod pull_request_gen;

#[derive(Debug, StructOpt)]
#[structopt(name = "gen-pr", about = "Generate Pull request according to commit logs and given template")]
struct CustomOpt {
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
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
}


fn main() {
    let args = CustomOpt::from_args();

    // list all the values in args
    pull_request_gen::gen(&args.title, &args.issue_link, &args.base_branch, &args.extra_description, args.feature);

    println!("Hello, world! {}", args.title);
}
