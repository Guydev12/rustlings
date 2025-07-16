// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of
//  a function. As input, we're going
// to give a list of strings and commands.
//  These commands determine what action
// is going to be applied to the string. 
// It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a 
// specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector 
//  of 2-length tuples,
//   the first element is the string, 
// the second one is the command.
// - The output element is going to be 
// a vector of strings.
#[derive(Debug)]
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as 
    // described above.
    pub fn transformer(input: Vec<(String, Command)>)->Vec<String> { 
        
		let mut results = Vec::new();
        for tup in input.iter()
        {
        	let (s, command) = tup;
        	let result = match command {
        		Command::Uppercase => s.to_uppercase(),
        		Command::Trim => s.trim().to_owned(),
        		Command::Append(x)=> {
        		    let mut s = s.to_string();
        		    let mut x = *x as u32;
        		    while x  > 0{
        				s.push_str("bar");
        				x -=1;
        			}
        			s
        		},
        	};

        	results.push(result);
        	
        }

        results
    }
}

fn main() {
    use my_module::*;
    // You can optionally experiment here.
    let input = vec![
        ("upper".to_owned(), Command::Uppercase),
        ("  trim   ".to_owned(), Command::Trim),
        ("foo".to_owned(), Command::Append(1)),
    ];

    println!("{:?}",transformer(input));
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import
    // to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
