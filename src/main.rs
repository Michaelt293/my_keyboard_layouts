extern crate clap;
#[macro_use]
extern crate kinesis_layout;
extern crate rayon;

use std::convert::AsRef;
use std::fs::File;
use std::io::Write;

use clap::{App, Arg};
use rayon::prelude::*;

use kinesis_layout::configure::*;
use kinesis_layout::keys::*;
use kinesis_layout::layout::*;
use kinesis_layout::macros::*;

fn main() {
    use Modifier::*;
    use NonModifier::*;

    let email = "Michaelt293@gmail.com";

    let matches = App::new("My keyboard layouts")
        .version("0.1")
        .author(format!("Michael T. <{}>", email).as_ref())
        .about("Does awesome things")
        .arg(
            Arg::with_name("email")
                .long("work-email")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    fn layout(system: System, email: &str) -> Layout {
        Configure::new()
            .with_remappings(colemak())
            .set_system(system)
            .with_macro(
                Shortcut::keypad_off(btreeset! {LeftAlt}, K),
                MacroBuilder::from_string(email).make(),
            )
            .with_macro(
                Shortcut::keypad_off(btreeset! {LeftAlt}, S),
                MacroBuilder::from_string("Regards,\nMichael").make(),
            )
            .with_macro(
                Shortcut::keypad_off(btreeset! {}, PageUp),
                MacroBuilder::new().with_command(Command::Copy).make(),
            )
            .with_macro(
                Shortcut::keypad_off(btreeset! {}, PageDown),
                MacroBuilder::new().with_command(Command::Paste).make(),
            )
            .with_macro(
                Shortcut::keypad_off(btreeset! {}, International),
                MacroBuilder::new().with_command(Command::Cut).make(),
            )
            .with_macro(
                Shortcut::keypad_off(btreeset! {}, CapsLock),
                MacroBuilder::new()
                    .with_shortcut(Shortcut::keypad_off(btreeset! {RightShift}, Quote))
                    .make(),
            )
            .with_macro(
                Shortcut::keypad_off(btreeset! {RightShift}, CapsLock),
                MacroBuilder::from_string("\"\"\"").make(),
            )
            .with_macro(
                Shortcut::keypad_off(btreeset! {LeftShift}, Quote),
                MacroBuilder::from_string("```\n").make(),
            )
            .make()
    };

    let w_colemak: &'static str = "w_qwerty.txt";
    let m_colemak: &'static str = "m_qwerty.txt";

    fn create_file(file_name: &str) -> File {
        File::create("layouts/".to_owned() + file_name)
            .expect(&("Unable to create file: ".to_owned() + file_name))
    };

    fn write_file(file_name: &str, system: System, email: &str) -> () {
        create_file(file_name)
            .write_all(format!("{}", layout(system, email)).as_bytes())
            .expect("Unable to write data")
    };

    let system_file_names = [
        (
            w_colemak,
            System::PC,
            matches.value_of("email").expect("work email is required"),
        ),
        (m_colemak, System::Mac, email),
    ];

    system_file_names
        .par_iter()
        .for_each(|sys_file_name| write_file(sys_file_name.0, sys_file_name.1, sys_file_name.2));
}
