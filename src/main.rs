use std::os;
use std::io::File;
use std::string::String;

/*
  A small example of a commandline app in rust.
  Saves and returns notes.
  Ex. To create note
  > ./note post <title> <text>

  Ex. To get the note back
  > ./note <title>
*/

fn main(){

  let args = os::args();

  if args[1] == String::from_str("post") {

    let title = args[2].as_slice();
    let text = args[3].as_slice();

    post_note(title, text);

  }else{

    let title = args[1].as_slice();
    let text = get_note(title);

    println!("{}", text);

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
