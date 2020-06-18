use std;
use std::borrow::Cow;

#[cfg(test)]
pub fn adduser<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/adduser.templates")));
}

#[cfg(test)]
pub fn apparmor<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/apparmor.templates")));
}

#[cfg(test)]
pub fn apt_listchanges<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/apt-listchanges.templates"
    )));
}

#[cfg(test)]
pub fn base_passwd<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/base-passwd.templates"
    )));
}

#[cfg(test)]
pub fn ca_certificates<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/ca-certificates.templates"
    )));
}

#[cfg(test)]
pub fn console_setup<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/console-setup.templates"
    )));
}

#[cfg(test)]
pub fn cups<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/cups.templates")));
}

#[cfg(test)]
pub fn dash<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/dash.templates")));
}

#[cfg(test)]
pub fn debconf<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/debconf.templates")));
}

#[cfg(test)]
pub fn dictionaries_common<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/dictionaries-common.templates"
    )));
}

#[cfg(test)]
pub fn discover<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/discover.templates")));
}

#[cfg(test)]
pub fn exim4_base<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/exim4-base.templates")));
}

#[cfg(test)]
pub fn exim4_config<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/exim4-config.templates"
    )));
}

#[cfg(test)]
pub fn exim4_daemon_light<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!(
        "templates/exim4-daemon-light.templates"
    )));
}

#[cfg(test)]
pub fn exim4<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/exim4.templates")));
}

#[cfg(test)]
pub fn gpm<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/gpm.templates")));
}

#[cfg(test)]
pub fn irqbalance<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/irqbalance.templates")));
}

#[cfg(test)]
pub fn wamerican<'a>() -> Cow<'a, str> {
    return Cow::Owned(String::from(include_str!("templates/wamerican.templates")));
}

#[cfg(test)]
pub fn getlines<'a>(input: &str, line_start: u32, line_end: u32) -> Cow<'a, str> {
    let mut foo = String::from(input);
    let mut lines = foo.lines();
    let mut counter = 0;
    let mut output = Vec::new();
    loop {
        let first_line = match lines.next() {
            Some(p) => p,
            None => break,
        };
        if (counter >= line_start && counter <= line_end) {
            output.push(String::from(first_line));
        }
        counter += 1;
        if (counter > line_end) {
            break;
        }
    }
    Cow::Owned(String::from(output.join("\n")))
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::parser::template_parser;
    use nom::error::convert_error;
    use nom::error::VerboseError;
    use nom::Err;

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

    #[test]
    fn test_adduser_all() {
        let line = String::from(getlines(&adduser(), 0, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                // Test must fail as starts with blank line
                assert!(false);
            }
            Err(err) => {
                println!("err {:?}", err);
                //assert!(false);
            }
        }
    }

    #[test]
    fn test_adduser_all_skip_line() {
        let line = String::from(getlines(&adduser(), 1, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_apparmor_all() {
        let line = String::from(getlines(&apparmor(), 0, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_irqbalance_all() {
        let line = String::from(getlines(&irqbalance(), 0, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_apt_listchanges_all() {
        let line = String::from(getlines(&apt_listchanges(), 0, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_apt_listchanges_all_skip_line() {
        let line = String::from(getlines(&apt_listchanges(), 1, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                //println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_base_passwd_all() {
        let line = String::from(getlines(&base_passwd(), 0, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                //println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_ca_certificates_all() {
        let line = String::from(getlines(&ca_certificates(), 1, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                //println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_console_setup_all() {
        let line = String::from(getlines(&console_setup(), 1, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                //println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_cups_all() {
        let line = String::from(getlines(&cups(), 1, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                //println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_dash_all() {
        let line = String::from(getlines(&dash(), 0, 9999));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }
    #[test]
    fn test_debconf_all() {
        let line = String::from(getlines(&debconf(), 0, 169));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }

    #[test]
    fn test_discover_all() {
        let line = String::from(getlines(&discover(), 0, 169));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }
    #[test]
    fn test_exim4_base_all() {
        let line = String::from(getlines(&exim4_base(), 0, 169));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }
    #[test]
    fn test_exim4_config_all() {
        let line = String::from(getlines(&exim4_config(), 0, 169));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }
    #[test]
    fn test_exim4_daemon_light_all() {
        let line = String::from(getlines(&exim4_daemon_light(), 0, 169));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }
    #[test]
    fn test_exim4_all() {
        let line = String::from(getlines(&exim4(), 0, 169));
        match template_parser::<VerboseError<&str>>(&line) {
            Ok((i, value)) => {
                println!("value {:?}", value);
                println!("i {:?}", i);
                assert!(i == "");
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "template_parser::<VerboseError>(data):\n{}",
                    convert_error(&line, e)
                );
                assert!(false);
            }
            Err(Err::Incomplete(e)) => {}
        }
    }
}
