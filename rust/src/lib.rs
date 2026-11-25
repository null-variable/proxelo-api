//! Generated types for the Proxelo snapshot format, distribution service,
//! simulator protocol and expression context.

pub mod snapshot {
    pub mod v1 {
        tonic::include_proto!("proxelo.snapshot.v1");
    }
}

pub mod distrib {
    pub mod v1 {
        tonic::include_proto!("proxelo.distrib.v1");
    }
}

pub mod sim {
    pub mod v1 {
        tonic::include_proto!("proxelo.sim.v1");
    }
}

pub mod expr {
    pub mod v1 {
        tonic::include_proto!("proxelo.expr.v1");
    }
}
