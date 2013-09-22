use std::{io, run};

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    let mut x = 0;
    let mut hist = std::vec::from_elem(100,~"test");	

    loop {
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
	hist[x]=line;
x=x+1;
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
		~"history"  => {
			int::range(0,x){|i|
			println(hist[i]);
}
		
}
		~"cd"	    => {println("RUST");}
                _           => {run::process_status(program, argv);}
            }
	    // Implement history list here.
	    
        }
    }
}
