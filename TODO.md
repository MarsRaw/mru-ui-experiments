# A list of todos:

- the confirm exit dialog should spawn at cursor, not at the top-right.
- put darkmode back in 
```rust
ui.horizontal(|ui| {
    egui::widgets::global_dark_light_mode_buttons(ui);
});
```
- successfully use download an image via the interop
- write the interop(s) functionality
- map out an architecture for where we'll store temp files etc
- suss out what the save/export options are going to be.
- suss out the lhs tabs (download, gallery, worktable etc)
- dummy imagery for the gallery
- gallery images are selectable
- selected images' larger version show in the preview window
- tools & tool-window constructors
- one of everything so that others can contribude.