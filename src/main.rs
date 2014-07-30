use std::os;
use std::io::File;
use std::string::String;

/*
  A small example of a commandline app in rust.
  Saves and returns notes.
  Ex. To create note
  > ./target/note post <title> <text>

  Ex. To get the note back
  > ./target/note <title>
*/

fn main(){

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

  let file_name = String::from_str(title).append(".txt");

  let mut file = File::create(&Path::new(file_name));
  let _ = file.write(text.as_bytes());

}

fn get_note(title:&str) -> String {

  let file_name = String::from_str(title).append(".txt");

  let contents = File::open(&Path::new(file_name)).read_to_string();
  return match contents {
    Ok(s) => s,
    _ => fail!("Could not find note")
  }

}
