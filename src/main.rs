#![crate_name = "note"]
#![crate_type = "bin"]

use std::os;
use std::io::File;
use std::string::String;


// A small example of a commandline app in rust.
// Saves and returns notes.
// Ex. To create note
// > ./note post <title> <text>
//
// Ex. To get the note back
// > ./note <title>

fn main() -> (){

    match os::args().as_slice().tail() {
        [ref command, ref title, ref text] => {
            if command.as_slice() == "post" {
              post_note(title.as_slice(), text.as_slice());
            } else {
                fail!("{} is an unknown command", command);
            }
        },
        [ref title] => {
          let text = get_note(title.as_slice());

          println!("{}", text);
        },
        args => {
            fail!("Unexpected combination of arguments: {}", args);
        }
    }

}

fn post_note(title:&str, text:&str) -> () {

  let path = get_path(title);
  let mut file = File::create(&path);
  let _ = file.write(text.as_bytes());

}

fn get_note(title:&str) -> String {

  let path = get_path(title);
  let contents = File::open(&path).read_to_string();

  return match contents {
    Ok(s) => s,
    _ => fail!("Could not find note")
  }

}

fn get_path(title:&str) -> Path {
  let file_name = String::from_str(title).append(".txt");
  return Path::new(file_name);
}
