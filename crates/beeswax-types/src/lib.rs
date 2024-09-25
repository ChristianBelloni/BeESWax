pub mod google;
pub mod build_event_stream {
    include! { concat!(env!("OUT_DIR"), "/build_event_stream.rs") }
}

pub mod blaze {
    include! { concat!(env!("OUT_DIR"), "/blaze.rs") }

    pub mod invocation_policy {
        include! { concat!(env!("OUT_DIR"), "/blaze.invocation_policy.rs") }
    }

    pub mod strategy_policy {
        include! { concat!(env!("OUT_DIR"), "/blaze.strategy_policy.rs") }
    }
}

pub mod failure_details {
    include! { concat!(env!("OUT_DIR"), "/failure_details.rs") }
}

pub mod command_line {
    include! { concat!(env!("OUT_DIR"), "/command_line.rs") }
}

pub mod options {
    include! { concat!(env!("OUT_DIR"), "/options.rs") }
}

pub mod devtools {
    pub mod build {
        pub mod lib {
            pub mod packages {
                pub mod metrics {

                    include! { concat!(env!("OUT_DIR"), "/devtools.build.lib.packages.metrics.rs") }
                }
            }
        }
    }
}
