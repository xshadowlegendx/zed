use crate::{
    IconAsset, Keybinding, Label, LabelColor, ListItem, ListItemSize, ModifierKeys, PaletteItem,
    ToggleState,
};

pub fn static_project_panel_project_items() -> Vec<ListItem> {
    vec![
        ListItem::new(Label::new("zed"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(0)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new(".cargo"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        ListItem::new(Label::new(".config"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        ListItem::new(Label::new(".git").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        ListItem::new(Label::new(".cargo"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        ListItem::new(Label::new(".idea").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        ListItem::new(Label::new("assets"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("cargo-target").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        ListItem::new(Label::new("crates"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("activity_indicator"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        ListItem::new(Label::new("ai"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        ListItem::new(Label::new("audio"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        ListItem::new(Label::new("auto_update"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        ListItem::new(Label::new("breadcrumbs"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        ListItem::new(Label::new("call"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        ListItem::new(Label::new("sqlez").color(LabelColor::Modified))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::NotToggled),
        ListItem::new(Label::new("gpui2"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("src"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("derrive_element.rs"))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(4),
        ListItem::new(Label::new("storybook").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("docs").color(LabelColor::Default))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("src").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("ui").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(4)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("component").color(LabelColor::Created))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(5)
            .set_toggle(ToggleState::Toggled),
        ListItem::new(Label::new("facepile.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        ListItem::new(Label::new("follow_group.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        ListItem::new(Label::new("list_item.rs").color(LabelColor::Created))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        ListItem::new(Label::new("tab.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        ListItem::new(Label::new("target").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        ListItem::new(Label::new(".dockerignore"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        ListItem::new(Label::new(".DS_Store").color(LabelColor::Hidden))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        ListItem::new(Label::new("Cargo.lock"))
            .left_icon(IconAsset::FileLock.into())
            .indent_level(1),
        ListItem::new(Label::new("Cargo.toml"))
            .left_icon(IconAsset::FileToml.into())
            .indent_level(1),
        ListItem::new(Label::new("Dockerfile"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        ListItem::new(Label::new("Procfile"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        ListItem::new(Label::new("README.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(1),
    ]
}

pub fn static_project_panel_single_items() -> Vec<ListItem> {
    vec![
        ListItem::new(Label::new("todo.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(0),
        ListItem::new(Label::new("README.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(0),
        ListItem::new(Label::new("config.json"))
            .left_icon(IconAsset::File.into())
            .indent_level(0),
    ]
}

pub fn static_collab_panel_current_call() -> Vec<ListItem> {
    vec![
        ListItem::new(Label::new("as-cii")).left_avatar("http://github.com/as-cii.png?s=50"),
        ListItem::new(Label::new("nathansobo"))
            .left_avatar("http://github.com/nathansobo.png?s=50"),
        ListItem::new(Label::new("maxbrunsfeld"))
            .left_avatar("http://github.com/maxbrunsfeld.png?s=50"),
    ]
}

pub fn static_collab_panel_channels() -> Vec<ListItem> {
    vec![
        ListItem::new(Label::new("zed"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(0),
        ListItem::new(Label::new("community"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(1),
        ListItem::new(Label::new("dashboards"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("feedback"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("teams-in-channels-alpha"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("current-projects"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(1),
        ListItem::new(Label::new("codegen"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("gpui2"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("livestreaming"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("open-source"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("replace"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("semantic-index"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("vim"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        ListItem::new(Label::new("web-tech"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
    ]
}

pub fn example_editor_actions() -> Vec<PaletteItem> {
    vec![
        PaletteItem::new("New File").keybinding(Keybinding::new(
            "N".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Open File").keybinding(Keybinding::new(
            "O".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Save File").keybinding(Keybinding::new(
            "S".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Cut").keybinding(Keybinding::new(
            "X".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Copy").keybinding(Keybinding::new(
            "C".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Paste").keybinding(Keybinding::new(
            "V".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Undo").keybinding(Keybinding::new(
            "Z".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Redo").keybinding(Keybinding::new(
            "Z".to_string(),
            ModifierKeys::new().control(true).shift(true),
        )),
        PaletteItem::new("Find").keybinding(Keybinding::new(
            "F".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Replace").keybinding(Keybinding::new(
            "R".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Jump to Line"),
        PaletteItem::new("Select All"),
        PaletteItem::new("Deselect All"),
        PaletteItem::new("Switch Document"),
        PaletteItem::new("Insert Line Below"),
        PaletteItem::new("Insert Line Above"),
        PaletteItem::new("Move Line Up"),
        PaletteItem::new("Move Line Down"),
        PaletteItem::new("Toggle Comment"),
        PaletteItem::new("Delete Line"),
    ]
}