use rouille::router;
use rouille::{try_or_400, post_input};
use std::sync::Mutex;

#[allow(unreachable_code)]
fn main() {
    let users_vec: Mutex<Vec<String>> = Mutex::new(Vec::new());

    let follow: Mutex<String> = Mutex::new("".to_string());
    let likert: Mutex<String> = Mutex::new("".to_string());
    let openurl: Mutex<String> = Mutex::new("".to_string());

    let temp_users_vec: Mutex<Vec<String>> = Mutex::new(Vec::new());


    println!("Now listening on localhost:8000");


    // The `start_server` starts listening forever on the given address.
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::html(FORM)
            },

            (POST) (/submit) => {
                let data = try_or_400!(post_input!(request, {
                    follow: String,
                    likert: String,
                    openurl: String
                }));

                println!("{:?}", data);

                let mut openurl = openurl.lock().unwrap();
                let mut follow = follow.lock().unwrap();
                let mut likert = likert.lock().unwrap();

                *openurl = data.openurl;
                *follow = data.follow;
                *likert = data.likert;

                let mut temp_users_vec = temp_users_vec.lock().unwrap();
                
                *temp_users_vec = users_vec.lock().unwrap().to_vec();

                rouille::Response::text("submitted")
            },

            (GET) (/get_openurl) => {

                if openurl.lock().unwrap().to_string() != "" {
                    return rouille::Response::text(openurl.lock().unwrap().to_string());
                } else {
                    return rouille::Response::text("")
                }
            },

            (GET) (/get_follow) => {
                if follow.lock().unwrap().to_string() != "" {
                    return rouille::Response::text("https://twitter.com/intent/user?screen_name=".to_string() + &follow.lock().unwrap().to_string());
                } else {
                    return rouille::Response::text("")
                }
                
            },

            (GET) (/get_likert) => {
                if likert.lock().unwrap().to_string() != "" {
                    return rouille::Response::text("https://twitter.com/intent/user?screen_name=".to_string() + &likert.lock().unwrap().to_string());
                } else {
                    return rouille::Response::text("")
                }
            },

            (GET) (/user_has_page_to_visit/{user: String}) => {
                if temp_users_vec.lock().unwrap().contains(&user) {
                    return rouille::Response::text("yes");
                } else {
                    return rouille::Response::text("no")
                }   

                
            },

            (GET) (/user/{user: String}) => {
                
                println!("User {} added", user);
                if !users_vec.lock().unwrap().contains(&user) {
                    users_vec.lock().unwrap().push(user);
                }

                rouille::Response::text(format!("{:?}", users_vec.lock().unwrap()))
            },

            (GET) (/visit_page/{user: String}) => {

                print!("User {} visited pages", user);
                temp_users_vec.lock().unwrap().retain(|value| *value != user);
                
                rouille::Response::text("ok")
            },

            _ => rouille::Response::empty_404()
        )
    });
}



static FORM: &'static str = r#"
<html>
    <head>
        <title>Form</title>
    </head>
    <body>
        <form action="submit" method="POST" enctype="multipart/form-data" target="_blank">
            
            <p>
                <label>Follow Twitter</label>
                <input type="text" name="follow" />
            </p>
            <p>
                <label>Like + RT + Follow</label>
                <input type="text" name="likert" />
            </p>
            <p>
                <label>Open URL</label>
                <input type="text" name="openurl" />
            </p>
            <p><button>Upload</button></p>
        </form>
    </body>
</html>
"#;