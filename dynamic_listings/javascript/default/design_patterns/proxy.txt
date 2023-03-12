class User{
    // This class represents a user, with their permissions
    // ...
    can_do_thing(){
        // Changes according to user permissions
        // ...
    }
}

class Request{
    // This class represents a web request
    constructor(user){
        this.user = user;
    }
}

class WebPage{
    /*
     * Represents a call to a web page
     */
    // This function gets implemented in the concrete classes;
    get(request){
        throw "Cannot use get() from abstract class"
    }
}

class WebPageProxy extends WebPage{
    // Represents an authentication proxy for a web page
    get(request){
        // Get the requesting user
        requesting_user = request.user;
        if (requesting_user.can_do_thing()){
            return super.get(request);
        }else{
            return HttpResponse("This user cannot access this page");
        }
    }
}

let user = User();
// The following line is valid due to polimorphism
let page = WebPageProxy();
// If the user has permission the page will come through.
page.get(Request(user));
