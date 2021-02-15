    use std::error::Error;
     
    pub struct Bugsnag{
        pub apikey: String
    }

    impl Bugsnag {
    fn notify(&self, level: &str, class: &str, msg: &str) -> Result<(),Box<dyn Error>> {
        let resp = ureq::post("http://notify.bugsnag.com/")
            .set("Content-Type", "application/json")
            .set("Bugsnag-Api-Key", &self.apikey)
            .set("Bugsnag-Payload-Version", "5")
            .send_json(ureq::json!({
                "notifier": {
                    "name": "My Test Notifier",
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
            Err(e) => return Err(Box::new(e)),
        }
    }

    pub fn info(&self, class: &str, msg: &str) -> Result<(),Box<dyn Error>> {
        self.notify("info", class, msg)
    }
    pub fn error(&self, class: &str, msg: &str) -> Result<(),Box<dyn Error>> {
        self.notify("error", class, msg)
    }
    }
