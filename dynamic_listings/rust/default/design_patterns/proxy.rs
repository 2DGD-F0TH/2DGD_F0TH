# #![allow(dead_code)]
struct HttpResponse(&'static str);

struct User {
    // This class represents a user, with their permissions
    // ...
}

impl User {
#   fn new() -> Self { todo!() }
    pub fn can_do_thing(&self) -> bool {
        // Changes according to user permissions
        // ...
#       todo!()
    }
}

struct Request {
    // This class represents a web request
    user: User,
}

impl Request {
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

trait WebPage {
    /*
     * Represents a call to a web page
     */
    // This function gets implemented in the concrete classes;
    fn get(&self, request: &Request) -> HttpResponse;
}

struct WebPageProxy(Box<dyn WebPage>);
# impl WebPageProxy {
#   fn new() -> Self { todo!() }
# }

impl WebPage for WebPageProxy {
    // Represents an authentication proxy for a web page
    fn get(&self, request: &Request) -> HttpResponse {
        // Get the requesting user
        let requesting_user = &request.user;
        if requesting_user.can_do_thing() {
            self.0.get(request)
        } else {
            HttpResponse("This user cannot access this page")
        }
    }
}

fn main() {
    let user = User::new();
    let page = WebPageProxy::new();
    // If the user has permission the page will come through.
    WebPage::get(&page, &Request::new(user));
}
