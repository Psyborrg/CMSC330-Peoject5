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
    
    let mut power_inc_match = s.strip_prefix("power inc ");
    match power_inc_match {
        None => {   // Not an increase power command so check the others
            let mut power_dec_match = strip_prefix("power dec ");
            match power_dec_match {
                None => { // Not a decrease power command so check the others
                    let mut shield_on_match = strip_prefix("shield on");
                    match shield_on_match {
                        None => { // Not a shield on command so check the others
                            let mut shield_off_match = strip_prefix("shield off");
                            match shield_off_match {
                                None => { // Not a shield off command so check the others
                                    let mut try_match = strip_prefix("try calling Miss Potts");
                                    match try_match {
                                        None => { // Not a try command so check the others
                                            return Command::Invalid; // Nothing after this so not a command
                                        },
                                        Some(rest) => { // If the rest of the string is empty then this is a try command
                                            if rest == "" {
                                                return Command::Try;
                                            } else {
                                                return Command::Invalid;
                                            }
                                        }
                                    }
                                },
                                Some(rest) => { // If the rest of the string is empty then this is a proper shield off command
                                    if rest == "" {
                                        return Command::Shield(false);
                                    } else {
                                        return Command::Invalid;
                                    }
                                }
                            }
                        },
                        Some(rest) => { // If the rest of the string is empty then this is a proper shield on command
                            if rest == "" {
                                return Command::Shield(true);
                            } else {
                                return Command::Invalid;
                            }
                        }
                    }
                },
                Some(value_string) => { // Seems like a missles dec command so check the rest to see if it is an i32
                    let value_as_int = s1.parse::<i32>();

                    match test {
                        Ok(ok) => return Command::Missiles(false, ok),
                        Err(e) => return Command::Invalid, 
                    }  
                }
            }
        },
        Some(value_string) => { // Seems like a missles inc command so check the rest to see if it is an i32
            let value_as_int = s1.parse::<i32>();

            match test {
                Ok(ok) => return Command::Missiles(true, ok),
                Err(e) => return Command::Invalid, 
            }  
        }
    }
}
