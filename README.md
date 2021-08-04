# Cat

A UNIX style cat remake with rust, mainly for windows and other environments where cat isn't available. 
This isnt the whole implementation of cat, I'm doing this to learn rust and might add all the features in the future soon

This requires the rust toolchain, mainly [cargo](https://doc.rust-lang.org/stable/cargo/).

## Builing and running
Clone the project and then run 
```bash
   $ cd cat && cargo install --path .  
```
or you can do 
```bash
    $ cargo install --git https://github.com/Blaze2305/cat
```
This builds the binary for your architecture and stores in ~/.cargo so that you can use it throughout your system.

## Usage
```bash
    cat [FLAGS] [FILES]...
    FLAGS:
	    - -h, --help               Prints help information
	    - -n, --num                print the line number for each file
	    - -b, --number-nonblank    number non empty output lines, overrides -n
	    - -E, --show-ends          Show $ at the end of each line
	    - -t, --show-tabs          sdiaply TAB characters as ^I
	    - -s, --squeeze-blank      suppress repeated empty output lines
	    - -V, --version            Prints version information
    
    ARGS:
       - <FILES>...    List of files to cat
```