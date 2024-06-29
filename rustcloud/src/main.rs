mod tests;
pub mod aws{
    pub mod aws_apis {
        pub mod compute {
            pub mod aws_ec2;
            pub mod aws_ecs;
            pub mod aws_eks;
            pub mod aws_paas;
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

mod gcp;
fn main() {
    println!("Hello, world!");
}