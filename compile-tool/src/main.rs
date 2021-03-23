use std::env;
use subprocess::Exec;

fn main() {
	let args = env::args().collect::<Vec<_>>();
	let lib_name = match args.len() {
		2 => (&args[1]),
		_ => {
			println!("Usage: {} gen <contract_file name>", args[0]);
			return;
		},
	};
    let tar_path_name = String::from("/root/wasm_contract_tools/contract/workspace/src/lib.rs");
    let tar_project_path = String::from("/root/wasm_contract_tools/contract/workspace/");
    // let tar_path_name = "/home/lsk/wasm_cdc/contract/workspace/src/lib.rs";
    // let tar_project_path = "/home/lsk/wasm_cdc/contract/workspace/";    
    // println!("{} {} {}",contract_path,contract_name,lib_path);
    Exec::shell(format!("cp /root/wasm_contract_tools/compile-tool/sources/{} {}",lib_name.to_owned(),tar_path_name.to_owned())).join().unwrap();        
    Exec::shell(format!("cd {} && bash build.sh",tar_project_path)).join().unwrap();
}
