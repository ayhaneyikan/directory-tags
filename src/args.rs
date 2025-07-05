use clap::{ArgGroup, Parser};

/// Tool to manage your directory paths
#[derive(Parser, Debug, Clone)]
#[command(name = "cdt", about = "Tool for rapid directory navigation")]
#[command(group(
    ArgGroup::new("mode").required(true).args(["add", "list", "list_tags", "tag"])
))]
pub struct Cli {
    /// Tag to assign to the directory
    #[arg(short, long)]
    pub add: Option<String>,

    /// (Used with add) [Optional] Path to map the tag to, defaults to current directory
    #[arg(short, long, requires("add"))]
    pub path: Option<String>,

    /// List all saved tag mappings
    #[arg(short, long)]
    pub list: bool,

    /// List all saved tags
    #[arg(long)]
    pub list_tags: bool,

    /// Tag to travel to (e.g., cd mytag)
    pub tag: Option<String>,
}
