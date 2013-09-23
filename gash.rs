use std:: {io, run};

fn main() {
	static CMD_PROMPT: &'static str = "gash > ";
	let mut ct = 0;
	let mut size = 2;
	let mut hist = ~std::vec::from_elem(size,~"test");
	
	loop {
		//print(CMD_PROMPT);
		print( "gash:"+std::os::getcwd().to_str()+"$ ");
		let line = io::stdin().read_line();
		debug!(fmt!("line: %?", line));
		// Fringe case; history buffer is at capacity. Double size.
		if (ct == size - 2) {
			let temp = ~std::vec::from_elem(size,~"test");
			let fin  = ~std::vec::concat([(*hist),(*temp)]);
			size = size * 2;
			hist = fin;
		}

		// Adding the line to history.
		(*hist)[ct] = std::str::to_owned(line);
		ct += 1;

		// Read in the "command" in the shell.
		let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "").transform(|x| x.to_owned()).collect();
		debug!(fmt!("argv %?", argv));

		if (argv.len() > 0) {
			let program = argv.remove(0);
			match program {
				// Internal command implementations here.
				~"exit" => {return;}
				~"ls" => {
					if(argv.len() > 0) { // non-zero check
						let lookhere = std::path::PosixPath("./../"+argv[0]); // makes the make_absolute part work
						let contents = std::os::list_dir(&std::os::make_absolute(&lookhere));
						for contents.iter().advance |s| { // s is a pointer to the strings, comprising the contents vector
							print(*s+" ");
						}
					}
					else {
						let contents = std::os::list_dir(&std::os::getcwd());
						for contents.iter().advance |s| {
							print(*s+" ");
						}
					}
					println("");
				} // end ls
				~"history" => {
					for std::uint::range(0,ct) |i| { // line blow is pretty straight forward
						println(fmt!("%s  %s",std::uint::to_str(i+1),(*hist)[i]));
					}
				} // end history
				~"cd" => {
					if(argv.len() != 0) {
						match argv[0] {
							~"~" => { // home dir
								let x:Option<std::path::PosixPath> = std::os::homedir();
								match x {
									Some(y) => {std::os::change_dir(&y);}
									None => {}
								} //end change path
							}
							~".." => { // parent directory (via ugly workaround)
								let target_path = "../";
								std::os::change_dir(&std::os::make_absolute(&std::path::PosixPath(target_path)));
							}
							_ => { // in case of argument
								let target_path = std::os::make_absolute(&std::path::PosixPath("./"+argv[0]));
								std::os::change_dir(&target_path);
							}
						} // end cd match
					} // end non-zero length (cd arg) check
					// If no input, go to home directory.	
					else {
						let x:Option<std::path::PosixPath> = std::os::homedir();
						match x {
							Some(y) => {std::os::change_dir(&y);}
							None => {}
						}
					} // end else
				} // end cd
				_ => {run::process_status(program, argv);}
			} // end command match

		} // end non-zero length (cmd) check
	} // end loop
} // end main
