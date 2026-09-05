# #![allow(dead_code)]
struct FirstService {
    // Implementation here...
}
# impl FirstService {
#     fn new() -> Self { todo!() }
#     fn start(&self) -> bool { todo!() }
# }

struct SecondService {
    // Implementation here...
}
# impl SecondService {
#     fn new() -> Self { todo!() }
#     fn start(&self) -> bool { todo!() }
# }

struct Facade {
    /*
     * This class hides the complexities of using
     * FirstService and SecondService from the user
     * by "wrapping" them in a comfortable startAll
     * function
     */
    service1: FirstService,
    service2: SecondService,
}

impl Facade {
    pub fn new() -> Self {
        Self {
            service1: FirstService::new(),
            service2: SecondService::new(),
        }
    }

    fn start_all(&self) -> bool {
        /*
         * The facade starts all the services and does
         * some status checking, this is hidden from the
         * user.
         * Returns true if all services started successfully
         * false otherwise
         */
        let first_service_started = self.service1.start();
        if !first_service_started {
            return false;
        }
        let second_service_started = self.service2.start();
        if !second_service_started {
            return false;
        }
        // Here everything started successfully
        true
    }
}
