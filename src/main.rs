use std::process::Command;

fn main() {
    abrir_terminal("xboxdrv --evdev \"/dev/input/event\" 
    --evdev-keymap \"N--=x,N--=y,N--=guide,N--=lb,N--=lt,N--=rb,N--=rt,N--=start,N--=back,N--=a,N--=b,N--=tl,N--=tr\" 
    --evdev-absmap \"N--=y1,N--=x1,N--=x2,N--=y2,N--=dpad_x,N--=dpad_y\" 
    --axismap \"-y1=y1,-y2=y2\" --mimic-xpad");
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
