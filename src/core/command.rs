use clap::{Arg, Command};

pub fn commands() {
	let flags = Command::new("")
		// Global properties.
		// Show `About`, `author`, and `version`.
		.about("nbeditor-cli")
		.author("\
		sakuraiyukina   <sakuraiyukina0205@yahoo.co.jp>\
		EB-wilson       <2534946881@qq.com>")
		.disable_help_flag(true)
		.version("git-23q3-01")



		// flags
		// -f,  --force
		.arg(Arg::new("force")
			.short('f')
			.long("force")
			.help("force overwrite of output file"))

		// -h,  --help
		.arg(Arg::new("help")
			.short('h')
			.long("help")
			.help("show this help message and exit"))

		// -i,  --stdin
		.arg(Arg::new("stdin")
			.short('i')
			.long("stdin")
			.help("read form standard input"))

		// -o,  --stdout
		.arg(Arg::new("stdout")
			.short('o')
			.long("stdout")
			.help("write to standard output"))

		// -t,  --threads
		.arg(Arg::new("threads")
			.short('t')
			.long("threads")
			.help("use at most NUM threads; the default is 1; set to 0 to use as many threads as there are processor cores"))

		// -v,  --verbose
		.arg(Arg::new("verbose")
			.short('v')
			.long("verbose")
			.help("be verbose"))

		// -F,  --format
		.arg(Arg::new("format")
			.short('F')
			.long("format")
			.help("target data format"))

		// -V,  --version
		.arg(Arg::new("version")
			.short('V')
			.long("version")
			.help("display the version number and exit"));

}