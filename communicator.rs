#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        match self {
            Command::Power(increasing, value) => {
                if increasing == &true {
                    let mut output = String::from("Power increased by ");
                    output.push_str(&value.to_string());
                    output.push('%');
                    return output;
                } else {
                    let mut output = String::from("Power decreased by ");
                    output.push_str(&value.to_string());
                    output.push('%');
                    return output;
                }
            },
            Command::Missiles(increasing, value) => {
                if increasing == &true {
                    let mut output = String::from("Missiles increased by ");
                    output.push_str(&value.to_string());
                    return output;
                } else {
                    let mut output = String::from("Missiles decreased by ");
                    output.push_str(&value.to_string());
                    return output;
                }
            },
            Command::Shield(on) => { 
                if on == &true {
                    return String::from("Shield turned on");
                } else {
                    return String::from("Shield turned off");
                }
            },
            Command::Try => return String::from("Call attempt failed"),
            Command::Invalid => return String::from("Not a command")
        }
    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
    
    let mut current_match = s.strip_prefix("power ");
    match current_match {
        None => ,//Not a power command so check the others
        Some(power_string) => { // Possibly a power command so keep matching
            let mut inc_match = current_match.strip_prefix("inc"); // Look for the inc portion of the command
            let mut dec_match = current_match.strip_prefix("dec"); // Look for the dec portion of the command
            match inc_match {
                None => return Command::Invalid,
                Some(value_string)
            }
        
        }
    
    }
    
    
    
    
    
    
    
    
    
    
    
    
    if current_match == None {
        // If the current match is none then this is not a power command
        current_match = s.strip_suffix("missiles");
        if current_match == None {
            // If the current match is none, then it is not a missile command
            current_match = s.strip_prefix("shield");
            if current_match == None {
                // If the current match is none, then it is not a shield command

                if s[0..s.len()] == "try calling Miss Potts" {
                    return Command::Try;
                } else {
                    return Command::Invalid; // If we got to here then it is not a valid command
                }
            }
        }
    } else { // let mut current_match = s.strip_prefix("power ") == Some (somethign)
        let mut power_match = current_match.strip_prefix("inc");
    }

    return Command::Invalid;
}
