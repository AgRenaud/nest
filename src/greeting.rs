pub const LOGO: &str = "                                        
███╗   ██╗███████╗███████╗████████╗
████╗  ██║██╔════╝██╔════╝╚══██╔══╝
██╔██╗ ██║█████╗  ███████╗   ██║   
██║╚██╗██║██╔══╝  ╚════██║   ██║   
██║ ╚████║███████╗███████║   ██║   
╚═╝  ╚═══╝╚══════╝╚══════╝   ╚═╝  
";

pub fn greets(addr: &String) {
    println!("{}", LOGO);
    println!("\nWelcome to Nest ! The API is available at {}\n", addr);
}
