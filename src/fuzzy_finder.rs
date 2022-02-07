use ini::Ini;
use skim::prelude;
use std::{
    collections::{BTreeMap, HashMap},
    process::exit,
};

mod item;

pub struct FuzzyFinder {
    pub mapped_roles: HashMap<String, String>,
}

fn read_files(
    config_path: &str,
    creds_path: &str,
    tx: &prelude::Sender<prelude::Arc<dyn prelude::SkimItem>>,
) -> HashMap<String, String> {
    let config_file = Ini::load_from_file(config_path).unwrap();
    let credentials_file = Ini::load_from_file(creds_path).unwrap();
    let mut merged_entries = BTreeMap::new();
    let mut mapped_roles = HashMap::new();
    for (sec, prop) in credentials_file.iter() {
        let mut p = BTreeMap::new();
        for (k, v) in prop.iter() {
            p.insert(k.to_string(), v.to_string());
        }
        merged_entries.insert(sec.unwrap().to_string(), p);
        mapped_roles.insert(sec.unwrap().to_string(), sec.unwrap().to_string());
    }
    drop(credentials_file);

    for (sec, prop) in config_file.iter() {
        let role_name = sec.unwrap().strip_prefix("profile ").unwrap();
        if prop.get("sso_auto_populated").is_some() {
            let mut p = BTreeMap::new();
            for (k, v) in prop.iter() {
                p.insert(k.to_string(), v.to_string());
            }
            merged_entries.insert(format!("(SSO profile) {}", role_name), p);
            mapped_roles.insert(
                format!("(SSO profile) {}", role_name),
                role_name.to_string(),
            );
        } else {
            if merged_entries.contains_key(role_name) {
                let mut p = merged_entries.remove(role_name).unwrap();
                for (k, v) in prop.iter() {
                    p.insert(k.to_string(), v.to_string());
                }
                merged_entries.insert(format!("(profile) {}", role_name), p);
                mapped_roles.insert(format!("(profile) {}", role_name), role_name.to_string());
            }
        }
    }

    drop(config_file);

    for (k, v) in merged_entries {
        tx.send(prelude::Arc::new(item::Item {
            name: k.to_string(),
            attributes: v,
        }))
        .unwrap();
    }
    return mapped_roles;
}

impl FuzzyFinder {
    pub fn new(
        tx: prelude::Sender<prelude::Arc<dyn prelude::SkimItem>>,
        creds_path: &str,
        config_path: &str,
    ) -> FuzzyFinder {
        let mapped_roles = read_files(config_path, creds_path, &tx);

        drop(tx);
        return FuzzyFinder { mapped_roles };
    }

    pub fn run(
        &self,
        options: skim::SkimOptions,
        rx: prelude::Receiver<prelude::Arc<dyn prelude::SkimItem>>,
    ) {
        let skim_output = prelude::Skim::run_with(&options, Some(rx));
        match skim_output {
            Some(so) if so.is_abort => exit(1),
            Some(so) => {
                let selected = so
                    .selected_items
                    .last()
                    .map(|selected_item| {
                        let item = (**selected_item)
                            .as_any()
                            .downcast_ref::<item::Item>()
                            .unwrap()
                            .to_owned();
                        self.mapped_roles.get(item.name.as_str())
                    })
                    .unwrap();
                print!("{}", selected.unwrap())
            }
            _ => exit(1),
        }
    }
}
