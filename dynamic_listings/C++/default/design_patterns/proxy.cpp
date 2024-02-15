class HttpResponse;

class User{
    // This class represents a user, with their permissions
    // ...
    public:
        bool can_do_thing(){
            // Changes according to user permissions
            // ...
        }
};

class Request{
    // This class represents a web request
    public:
        User* user = nullptr;
        Request(User* user){
            user = user;
        }
};

class WebPage{
    /*
     * Represents a call to a web page
     */
    // This function gets implemented in the concrete classes;
    public:
        virtual HttpResponse get(Request request) = 0;
};

class WebPageProxy: public WebPage{
    // Represents an authentication proxy for a web page
    public:
        virtual HttpResponse get(Request request){
            // Get the requesting user
            User requesting_user = *request.user;
            if (requesting_user.can_do_thing()){
                return WebPage::get(request);
            }else{
                return HttpResponse("This user cannot access this page");
            }
        }
};

int main(){
    User* user = new User();
    // The following line is valid due to polimorphism
    WebPage page = WebPageProxy();
    // If the user has permission the page will come through.
    page.get(Request(user));
}
