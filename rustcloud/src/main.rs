mod tests;
pub mod aws{
    pub mod aws_apis {
        pub mod compute {
            pub mod aws_ec2;
            pub mod aws_ecs;
            pub mod aws_eks;
        }
        pub mod database {
            pub mod aws_dynamodb;
            pub mod aws_rbmds;
        }
        pub mod management {
           pub mod aws_monitoring; 
        }
        pub mod network {
            pub mod aws_dns;
            pub mod aws_loadbalancer;
        }
        pub mod security {
            pub mod aws_iam;
            pub mod aws_keymanagement;
        }
        pub mod storage {
            pub mod aws_archival_storage;
            pub mod aws_block_storage;
            pub mod aws_storage_bucket;
        }
    } 
}

pub mod gcp{
    pub mod gcp_apis{
        pub mod app_services{
            pub mod gcp_notification_service;
        }
        pub mod artificial_intelligence{
            pub mod gcp_automl;
        }
        pub mod compute{
            pub mod gcp_compute_engine;
            pub mod gcp_kubernetes;
        }
        pub mod database{
            pub mod gcp_bigtable;
        }
        pub mod network{
            pub mod gcp_dns;
            pub mod gcp_loadbalancer;
        }
        pub mod storage{
            pub mod gcp_storage;
        }
        pub mod auth{
            pub mod gcp_auth;
        }
    }
    pub mod types;
}
fn main() {
    println!("Hello, world!");
}