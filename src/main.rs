trait Button{
    fn paint(&self);
}
struct MacButton{}
struct WinButton{}

impl  Button for MacButton {
    fn paint(&self) {
        println!("mac os button");
    }
}
impl  Button for WinButton {
    fn paint(&self) {
        println!("windows os button");
    }
}

trait CheckBox{
    fn paint(&self);
}
struct MacCheckBox{}
struct WinCheckBox{}

impl  CheckBox for MacCheckBox {
    fn paint(&self) {
        println!("mac os checkbox");
    }
}
impl  CheckBox for WinCheckBox {
    fn paint(&self) {
        println!("windows os checkbox");
    }
}

trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn CheckBox>;
}

fn main() {
    println!("Hello, world!");
    
}
pub struct WinFactory{}
impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton {})
    }
    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(WinCheckBox {})
    }
}
pub struct MacFactory{}
impl GUIFactory for MacFactory {
    
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton {})
    }
    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(MacCheckBox {})
    }
}
