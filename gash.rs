use std::{io, run};

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    let mut ct = 0;
    let mut size = 2;
    let mut hist = ~std::vec::from_elem(size,~"test");	

    loop {
        print(CMD_PROMPT);
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
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
		// Internal command implementations here.
                ~"exit"     => {return; }
		~"history"  => {
			for std::uint::range(0,ct) |i| {
				println(fmt!("%s  %s",std::uint::to_str(i+1),(*hist)[i]));
			}
	        }
		~"cd"	    => {println("RUST");}
                _           => {run::process_status(program, argv);}
            }
	    // Implement history list here.
        }
    }
}
