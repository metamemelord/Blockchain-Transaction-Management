use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();

    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_addr).unwrap();

    print!("Input difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).unwrap();
    let difficulty = difficulty
        .trim()
        .parse::<u32>()
        .expect("Unable to parse difficulty");

    println!("Generating genesis block");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), difficulty);

    loop {
        println!("\n\nMenu:");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("0. Exit\n");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();
        choice.clear();

        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse().unwrap() {
            0 => process::exit(0),
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sender).unwrap();

                print!("Enter receiver address:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut receiver).unwrap();

                print!("Enter amount:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount).unwrap();

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block...");
                match chain.generate_new_block() {
                    true => println!("Block generated successfully"),
                    false => println!("Failed to generate a block"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).unwrap();

                match chain.update_difficulty(new_diff.trim().parse().unwrap()) {
                    true => println!("Difficulty updated successfully"),
                    false => println!("Failed to change difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).unwrap();

                match chain.update_reward(new_reward.trim().parse().unwrap()) {
                    true => println!("Reward updated successfully"),
                    false => println!("Failed to change reward"),
                }
            }
            _ => println!("Wrong option, try again"),
        }
    }
}
