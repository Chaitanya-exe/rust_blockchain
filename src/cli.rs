use clap::{Command, arg};
use crate::errors::Result;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub struct Cli{
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli>{
        Ok(Cli{
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("Chaitanya Yadav, jeetkrishna3333@gmail.com")
            .about("A blockchain implemented in Rust")
            .subcommand(Command::new("printchain").about("prints all the blocks in the blockchain"))
            .subcommand(
                Command::new("addblock")
                .about("add a block in blockchain")
                .arg(arg!(<DATA>"'the blockchain data'")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock"){
            if let Some(c) = matches.get_one::<String>("DATA"){
                self.addblock(vec![])?;
            } else{
                println!("Not printing testing lists...");
            }
        }

        if let Some(_) =  matches.subcommand_matches("printchain") {
            self.print_chain();
        } 

        Ok(())
    }

    pub fn addblock(&mut self, data: Vec<Transaction>) -> Result<()>{
        self.bc.add_block(data)
    }

    pub fn print_chain(&mut self){
        for b in self.bc.iter(){
            println!("block: {:#?}", b);
        }
    }
}