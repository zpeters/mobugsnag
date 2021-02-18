//! This is my beginning of a crate for Bugsnag alerting
//! right now it is very minimal and just for my testing use.
use std::fmt;
use std::fmt::Display;

/// A Bugsnag event has three severity levels, defined as follows:
/// error - The default for unhandled errors.
/// warning - The default when Bugsnag.notify is called.
/// info - Can be used in manual Bugsnag.notify calls
///
/// In our library the severity level is just based on what function
/// the user chooses
enum SeverityLevel {
    Error,
    Warning,
    Info,
}

/// Converts our SeverityLevel enum to the expected string
impl Display for SeverityLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            SeverityLevel::Error => f.write_str("error"),
            SeverityLevel::Warning => f.write_str("warning"),
            SeverityLevel::Info => f.write_str("info"),
        }
    }
}

/// The Bugsnag api specifies that requests are processed asynchornously.
/// Therefore, a 'bad' request does not necessarily return an error status
/// Errors returned from the api are likely formatting, structure or low-level
/// network issues
pub struct BugsnagError {
    /// Default of 0, future use
    code: usize,
    message: String,
}

/// Implement std::fmt::Display for AppError
impl fmt::Display for BugsnagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            0 => "An error occured posting to bugsnag",
            _ => "Some unknown error occured",
        };
        write!(f, "{}", err_msg)
    }
}

/// Implement std::fmt::Debug for AppError
impl fmt::Debug for BugsnagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BugsnagError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

/// In an info or error the `class` is at type or grouping for messages (see the [`documentation`]).  You can
/// think of this as a 'context' or just a way to group relevant errors
///
/// The message is your alerting details.
///
/// [`documentation`]: https://bugsnagerrorreportingapi.docs.apiary.io/#reference/0/notify/send-error-reports
/// # Examples
///
/// ## Informational message
/// ```
/// use mobugsnag::Bugsnag;
/// let snag = Bugsnag{apikey: "MYAPIKEY".to_string()};
/// snag.info("My Class", "My actual Message").unwrap();
/// ```
///
/// ## Warning message
/// ```
/// use mobugsnag::Bugsnag;
/// let snag = Bugsnag{apikey: "MYAPIKEY".to_string()};
/// snag.warning("My Class", "My actual Message").unwrap();
/// ```
///
/// ## Error message
/// ```
/// use mobugsnag::Bugsnag;
/// let snag = Bugsnag{apikey: "MYAPIKEY".to_string()};
/// snag.error("An Error Type", "My error message").unwrap();
/// ```
pub struct Bugsnag {
    /// Your bugsnag project api key. See [`here`] for details on generating it
    ///
    /// [`here`]: https://docs.bugsnag.com/product/getting-started/
    pub apikey: String,
}

impl Bugsnag {
    fn notify(&self, level: SeverityLevel, class: &str, msg: &str) -> Result<(), BugsnagError> {
        let resp = ureq::post("http://notify.bugsnag.com/")
            .set("Content-Type", "application/json")
            .set("Bugsnag-Api-Key", &self.apikey)
            .set("Bugsnag-Payload-Version", "5")
            .send_json(ureq::json!({
                "notifier": {
                    "name": "mobugsnag - my own bugsnag",
                    "version": "0.0.1",
                    "url": "http://bcianswers.com"
                },
                "events": [
                {"exceptions":[
                    {
                        "errorClass": class,
                        "message": msg
                    }
                ]}
                ],
                "severity": level.to_string()
            }));
        match resp {
            Ok(_) => Ok(()),
            Err(e) => Err(BugsnagError {
                code: 0,
                message: format!("{:?}", e),
            }),
        }
    }

    pub fn info(&self, class: &str, msg: &str) -> Result<(), BugsnagError> {
        self.notify(SeverityLevel::Info, class, msg)
    }
    pub fn warning(&self, class: &str, msg: &str) -> Result<(), BugsnagError> {
        self.notify(SeverityLevel::Warning, class, msg)
    }
    pub fn error(&self, class: &str, msg: &str) -> Result<(), BugsnagError> {
        self.notify(SeverityLevel::Error, class, msg)
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_levels() {
        let s1 = SeverityLevel::Error;
        let s2 = SeverityLevel::Warning;
        let s3 = SeverityLevel::Info;
        assert_eq!(format!("{}", s1), "error");
        assert_eq!(format!("{}", s2), "warning");
        assert_eq!(format!("{}", s3), "info");
    }
}
