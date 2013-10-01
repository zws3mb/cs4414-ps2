use std::{io, run};
/*
main runs a loop--iterating through the command line, flags are thrown for various shell-specific parameters (<,>,|,&,cd,ls, history).
To be implemented, specific shell behavior for index-dependent functions.
struct shellState is used to hold these flags and allow main() to access the history array generated.
*/
fn execute ( gstate:&mut shellState, line:~[~str])
{
	let mut program = copy line;
	let orig = program.remove(0);
	if(gstate.backg) {
	let mut bargs = copy line;
	let bcom = bargs.remove(0);
	let cobargs = bargs;
do std::task::spawn_unlinked {
run::process_status(copy bcom, copy cobargs);
}

}
		if(gstate.output) { //>
			let outstr = program.remove(line.len()-2);
			let outpath = &std::os::make_absolute(&std::path::PosixPath(outstr));
			println(outpath.to_str());
	
			let writer = &std::io::file_writer(outpath, &[io::Create, io::Truncate]).unwrap();
		//	let outfile = std::io::rt::File::open(outstr, Create, Write);
			let mut p = run::Process::new(orig, program, run::ProcessOptions::new());
			let readit = p.output();
			while (!readit.eof()) {
				writer.write_line(readit.read_line());
			}
			println(">" + gstate.opstr[gstate.opstr.len()-1]);
		//create a filewriter to name line[1]/program[0]
		//gstate.opstr reset!!
p.finish();
		}
		if(gstate.input) {//<
			let instr = program.remove(line.len()-2);
			let inpath = &std::os::make_absolute(&std::path::PosixPath(instr));
			//println("<" + gstate.opstr[gstate.opstr.len()-1]);
			let reader = &std::io::file_reader(inpath).get();
			let mut p = run::Process::new(orig, &[], run::ProcessOptions::new());
			let stdin = p.input();			
			while (!reader.eof()) {
				stdin.write_line(reader.read_line());
			}
			let out = p.finish_with_output();
			print(std::str::from_bytes(out.output.tail()));
			//pull from the last argument
		}
		if(gstate.piper) {//|
			println("|" + gstate.opstr[gstate.opstr.len()-1]);
			//also pull from last argument
		}
		if (gstate.fire == true && orig != ~"exit") {
			match orig {
				// Internal command implementations here.
				~"exit" => {gstate.exitstatus=true;}
				~"ls" => {
					if(program.len() > 0) { // non-zero check
						let lookhere = std::path::PosixPath("./../"+program[0]); // creates pathway to look in
						let contents = std::os::list_dir(&std::os::make_absolute(&lookhere)); //complete the path from parent directories, get a list of the conents of the path
						for contents.iter().advance |s| { // print out the contents of the directory
							print(*s+" ");
						}
					}
					else {	//without argument, view cwd
						let contents = std::os::list_dir(&std::os::getcwd());
						for contents.iter().advance |s| {
							print(*s+" ");
						}
					}
					println("");
				} // end ls
				~"history" => {
					println(fmt!("%u",gstate.get_ct()));
					for std::uint::range(0,gstate.get_ct()) |i| { // line below is pretty straight forward
						println(fmt!("%s  %s",std::uint::to_str(i+1),(gstate.hist)[i]));
					}
				} // end history
				~"cd" => {
					if(program.len() != 0) {
						match program[0] {
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
								let target_path = std::os::make_absolute(&std::path::PosixPath("./"+program[0]));
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
				_ => {
if(!(gstate.piper || gstate.input || gstate.output || gstate.backg)){
run::process_status(orig, program);
}

}
			} // end command match
		}//fire flag

		else
		if(orig == ~"exit") {gstate.exitstatus=true;}
		gstate.opstr=std::vec::from_elem(0,~"test");
	//} //end backg
}//end execute

struct shellState {
	size:uint,
	ct:uint,
	hist:~[~str],
	opstr:~[~str],
	input:bool,
	output:bool,
	piper:bool,
	backg:bool,
	fire:bool,
	exitstatus:bool
}

impl shellState {
	fn new(size:uint,ct:uint, hist: ~[~str], opstr: ~[~str], input:bool, output:bool, piper:bool,backg:bool,fire:bool,exitstatus:bool)->shellState {
		shellState{size:size,ct:ct,hist:hist,opstr:opstr, input:input,output:output,piper:piper,backg:backg,fire:fire,exitstatus:exitstatus}
	}
	fn get_size(&self)->uint{self.size}
	fn get_ct(&self)->uint{self.ct}
	fn get_input(&self)->bool{self.input}
	fn get_output(&self)->bool{self.output}
	fn get_piper(&self)->bool{self.piper}
	fn get_backg(&self)->bool{self.backg}
	//fn get_opstr(&self)->~[~str]{return self.opstr}
}//end impl

fn main() {
	let mut x = shellState::new(2,0,std::vec::from_elem(0,~"test"), std::vec::from_elem(0,~"test"), false, false, false, false,false,false);
	
	loop {
		//print(CMD_PROMPT);
		print( "gash:"+std::os::getcwd().to_str()+"$ ");
		let inline = io::stdin().read_line();
		debug!(fmt!("line: %?", inline));
		// println("pushing " + inline);		
		x.hist.push(copy inline);
		x.ct = x.ct+1;

		// Read in the "command" in the shell.
		let argv: ~[~str] = inline.split_iter(' ').filter(|&q| q != "").transform(|q| q.to_owned()).collect();
		debug!(fmt!("argv %?", argv));
		// let argarray= &mut argv;
		let y=&mut x;
		if (argv.len() > 0) {
			for std::uint::range(0,argv.len()) |s| {
				if(s < argv.len()) {
					let c_str = copy std::str::to_owned(argv[s]);	
					match c_str {
						~"&" => {y.backg=(true);}
						~"<" => {y.input=(true);}
						~">" => {y.output=(true);}
	 					~"|" => {y.piper=(true);}
						_    => {y.opstr.push(c_str);}
					} //end match for pipe comparison
				}
				let argarray = copy argv;		
				//let temp = copy y.opstr;
//				let length = argarray.len();
				if( (y.output||y.input) && ((s+1) <= argarray.len()) ) {
					y.fire = true;
					y.opstr.push(copy argarray[s+1]);
					execute(y,copy y.opstr);
				}
				if(y.piper && ((s+1) <= argarray.len())) {
					//let y.opstr
					y.fire = true;
					y.opstr.push(argarray[s+1]);
					execute(y,copy y.opstr);
				}
				if(y.backg) {
					y.fire = true;
					// Nothing more here? Are we just writing two distinct execute methods, or an if...?
					execute(y, copy y.opstr);
				}	
				
				if(y.exitstatus==true) {break;}
				y.backg=false;
				y.input=false;
				y.output=false;
				y.fire=false;			
			} // end pipe parser
			//let mut v = &mut x;
			y.fire=true;
					execute(y, copy y.opstr);
			} // end non-zero length (cmd) check
		if(y.exitstatus==true) {break;}	
	} // end loop
} // end main
