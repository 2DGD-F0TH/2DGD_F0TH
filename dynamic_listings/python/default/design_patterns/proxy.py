class User:
    # This class represents a user, with their permissions
    # ...
    def has_permission(self):
        # Changes according to user permissions
        # ...


class Request:
    # This class represents a web request
    def __init__(self, user: User):
        self.user = user


class WebPage:
    # Represents a call to a web page
    # This function gets implemented in the concrete classes;
    def get(self, request: Request):
        raise NotImplementedError


class WebPageProxy(WebPage):
    # Represents an authentication proxy for a web page
    def get(self, request: Request):
        # Get the requesting user
        requesting_user = request.user
        if (requesting_user.has_permission()):
            return super().get(request)
        else:
            return HttpResponse("This user cannot access this page")


if __name__ == '__main__':
    user = User()
    # The following line is valid due to polimorphism
    page = WebPageProxy()
    # If the user has permission the page will come through.
    page.get(Request(user))
