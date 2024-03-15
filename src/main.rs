use std::process::Command;

fn main() {
    abrir_terminal("xboxdrv --evdev \"/dev/input/event4\" --evdev-keymap \"BTN_TOP=x,BTN_TRIGGER=y,BTN_BASE3=guide,BTN_TOP2=lb,BTN_BASE=lt,BTN_PINKIE=rb,BTN_BASE2=rt,BTN_BASE4=start,BTN_BASE3=back,BTN_THUMB2=a,BTN_THUMB=b,BTN_BASE5=tl,BTN_BASE6=tr\" --evdev-absmap \"ABS_Y=y1,ABS_X=x1,ABS_RZ=x2,ABS_Z=y2,ABS_HAT0X=dpad_x,ABS_HAT0Y=dpad_y\" --axismap \"-y1=y1,-y2=y2\" --mimic-xpad");
}

fn abrir_terminal(comando: &str) {
    let terminal = "gnome-terminal";
    let argumento = format!("bash -c \"{}; exec bash\"", comando);

    Command::new(terminal)
        .args(&["-e"])
        .arg(&argumento)
        .spawn()
        .expect("Falha ao abrir o terminal.");
}
