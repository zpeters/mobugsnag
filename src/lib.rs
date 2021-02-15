//! This is my beginning of a crate for Bugsnag alerting
//! right now it is very minimal and just for my testing use.
use std::error::Error;

/// Bugsnag is the main struct, we create a new struct and can 
/// send an "Info" or "Alert".  This is just a different severity
/// level
///
/// In an info or error the `class` is at type or grouping for messages (see the [`documentation`]).  You can 
/// think of this as a 'context' or just a way to group relevant errors
///
/// The message is your alerting details.
///
/// [`documentation`]: https://bugsnagerrorreportingapi.docs.apiary.io/#reference/0/notify/send-error-reports
/// # Examples
///
/// ## Setup
/// ```
/// use mob::Bugsnag;
/// let snag = Bugsnag{apikey: "MYAPIKEY"};
/// ```
/// ## Informational message
/// ```
/// snag.info("My Class", "My actual Message")
/// ```
/// ## Error message
/// ```
/// snag.error("An Error Type", "My error message")
/// ```
pub struct Bugsnag {
    /// Your bugsnag project api key. See [`here`] for details on generating it
    ///
    /// [`here`]: https://docs.bugsnag.com/product/getting-started/
    pub apikey: String,
}

impl Bugsnag {
    fn notify(&self, level: &str, class: &str, msg: &str) -> Result<(), Box<dyn Error>> {
        let resp = ureq::post("http://notify.bugsnag.com/")
            .set("Content-Type", "application/json")
            .set("Bugsnag-Api-Key", &self.apikey)
            .set("Bugsnag-Payload-Version", "5")
            .send_json(ureq::json!({
                "notifier": {
                    "name": "mob - my own bugsnag",
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
                "severity": level
            }));
        match resp {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn info(&self, class: &str, msg: &str) -> Result<(), Box<dyn Error>> {
        self.notify("info", class, msg)
    }
    pub fn error(&self, class: &str, msg: &str) -> Result<(), Box<dyn Error>> {
        self.notify("error", class, msg)
    }
}
