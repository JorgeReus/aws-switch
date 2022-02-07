use dirs;
use skim::prelude;

mod fuzzy_finder;

pub fn main() {
    let options = prelude::SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .inline_info(true)
        .preview(Some(""))
        .preview_window(Some("right:50%"))
        .color(Some("dark,bg+:0,fg+:#DEBC08,prompt:#4B4DE1,matched:#3ECE21,matched_bg:-1,current_match_bg:#008282"))
        .build()
        .unwrap();

    let (tx, rx): (prelude::SkimItemSender, prelude::SkimItemReceiver) = prelude::unbounded();
    let home_path = match dirs::home_dir() {
        Some(hm) => hm.to_owned(),
        _ => panic!("No home directory was found."),
    };
    let home_dir = home_path.display();

    let config_path = format!("{}/.aws/config", home_dir);
    let creds_path = format!("{}/.aws/credentials", home_dir);

    fuzzy_finder::FuzzyFinder::new(tx, creds_path.as_str(), config_path.as_str()).run(options, rx);
}
