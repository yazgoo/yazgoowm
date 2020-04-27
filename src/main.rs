use std::process::Command;
use yazgoowm::{Actions, Conf, yazgoowm, WindowBorder, DisplayBorder, Key, CustomAction, Meta};
use std::env;
use std::collections::HashMap;

fn main() -> Result<(), ()> {

    let mut wm_actions = HashMap::new();
    wm_actions.insert(' ', Actions::SwitchWindow);
    wm_actions.insert('w', Actions::CloseWindow);
    wm_actions.insert('f', Actions::ChangeLayout);

    let mut custom_actions : HashMap<Key, CustomAction> = HashMap::new();
    custom_actions.insert('r', Box::new(|| { Command::new("rofi").arg("-show").arg("run").spawn();}));
    custom_actions.insert('t', Box::new(|| { Command::new("kitty").spawn();}));
    custom_actions.insert('q', Box::new(|| std::process::exit(0)));

    let auto_float_types : Vec<String> = vec!["notification", "toolbar", "splash", "dialog", "popup_menu", "utility", "tooltip", "dock"].into_iter().map( |x|
            x.to_string()
        ).collect();

    let args: Vec<String> = env::args().collect();

    let conf = Conf {
        meta: if args.len() > 1 && args[1] == "mod4" { Meta::Mod4 } else { Meta::Mod1 },
        display_border: DisplayBorder {
            left: 0,
            right: 0,
            top: 20,
            bottom: 0,
        },
        border: WindowBorder {
            width: 1,
            gap: 10,
            focus_color: 0x906cff,
            normal_color: 0x000000,
        },
        workspaces_names: vec![ 'a', 'u', 'i', 'o', 'p' ],
        custom_actions: custom_actions,
        wm_actions: wm_actions,
        float_classes: vec!["screenkey", "audacious", "Download", "dropbox", "file_progress", "file-roller", "gimp",
                          "Komodo_confirm_repl", "Komodo_find2", "pidgin", "skype", "Transmission", "Update", "Xephyr", "obs"].into_iter().map( |x|
            x.to_string()
        ).collect(),
        auto_float_types: auto_float_types.clone(),
    };

    yazgoowm(conf).run();

    Ok(())
}
