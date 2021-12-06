#![windows_subsystem = "windows"]

sixtyfps::sixtyfps!(import { MainWindow } from "src/ui/main.60";);

pub fn main() {
    let main_window = MainWindow::new();
    
    main_window.on_close({
        move || std::process::exit(0)
    });

    main_window.run();
}
