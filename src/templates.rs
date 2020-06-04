use std;
use std::borrow::Cow;

pub fn adduser<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/adduser.templates")));
}

pub fn apparmor<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/apparmor.templates")));
}

pub fn apt_listchanges<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/apt-listchanges.templates"
    )));
}

pub fn base_passwd<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/base-passwd.templates"
    )));
}

pub fn dash<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/dash.templates")));
}

pub fn gpm<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/gpm.templates")));
}

pub fn irqbalance<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/irqbalance.templates")));
}

pub fn wamerican<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/wamerican.templates")));
}

pub fn getlines<'a>(input: &str, line_start: u32, line_end: u32) -> Cow<'a, str> {
    let mut foo = String::from(input);
    let mut lines = foo.lines();
    let mut counter = 0;
    let mut Output = Vec::new();
    loop {
        let first_line = match lines.next() {
            Some(p) => p,
            None => break,
        };
        if (counter >= line_start && counter <= line_end) {
            Output.push(String::from(first_line));
        }
        counter += 1;
        if (counter > line_end) {
            break;
        }
    }
    Cow::Owned(String::from(Output.join("\n")))
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first_line() {
        let first_line = getlines(&dash(), 0, 0);
        assert!(first_line == "Template: dash/sh");
    }

    #[test]
    fn test_second_line() {
        let line = getlines(&dash(), 1, 1);
        println!("line {:?}", line);
        assert!(line == "Type: boolean");
    }

    #[test]
    fn test_getlines() {
        let line = getlines(&dash(), 0, 1);
        println!("line {:?}", line);
        assert!(line == "Template: dash/sh\nType: boolean");
    }

    #[test]
    fn test_getlines_decription() {
        let line = getlines(&dash(), 3, 8);
        println!("line {:?}", line);
        assert!(
            line == r#"Description: Use dash as the default system shell (/bin/sh)?
 The system shell is the default command interpreter for shell scripts.
 .
 Using dash as the system shell will improve the system's overall
 performance. It does not alter the shell presented to interactive
 users."#
        );
    }
}
