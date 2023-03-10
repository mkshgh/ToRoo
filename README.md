# ToRoo
Really Simple TODO CLI in RUST

### ToDo Application
- add topic
- add description list

### View
- topic

### Search
- topic 
- Description
- All

### Delete
- Topic
- Description
- All

### Sections of App
- Listed -> can be moved to Doing
- Doing -> can be moved to Listed or Done
- Done  -> can be moved to Doing 

### Custom Keyboard Bindings
- I -> Insert New to the list
- J/K -> Move Task Up or Down
- D -> Remove the Task
- E -> Edit the Task
- Double Esc -> Close the Application

### Integrations
- DB in DSV for starters (~|~) Tilde pipeline Tilde Separator
- Sync to Google / Onedrive
- Export/Import using csv or other formats

### Problems
- When deleting or updating at the end of the file, there is a added new line in the db. How to detect and delete that.

## References
- [Not Building for Windows](https://stackoverflow.com/questions/55603111/unable-to-compile-rust-hello-world-on-windows-linker-link-exe-not-found)
- [Reding line from terminal](https://www.tutorialspoint.com/rust/rust_input_output.htm)
- [Printing Individual Lines]()
- [Time in Format: chrono](https://docs.rs/chrono/latest/chrono/#formatting-and-parsing)
- [Writing to File](https://www.includehelp.com/rust/append-data-into-an-existing-file.aspx) and [How it works](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/fs/struct.OpenOptions.html)
- [Creating a File](https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html)
- [Rename File in rust](https://doc.rust-lang.org/std/fs/fn.rename.html)
- [Remove File in rust](https://doc.rust-lang.org/std/fs/fn.remove_file.html)
- [Time in Strftime Format](https://docs.rs/chrono/latest/chrono/format/strftime/)
- [Writing to JSON](https://www.youtube.com/watch?v=cyVLw_7Vhb8)