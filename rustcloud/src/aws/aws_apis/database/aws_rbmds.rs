#![allow(clippy::result_large_err)]

use aws_sdk_rds::{Client, Error};
use aws_sdk_rds::types::{
    CreateDbClusterMessage, CreateDbSnapshotMessage, DeleteDbClusterMessage, DeleteDbSnapshotMessage,
    ModifyDbClusterMessage, RebootDbInstanceMessage
};

pub struct AwsRdbms {
    client: Client,
}

impl AwsRdbms {
    pub fn new(client: Client) -> Self {
        AwsRdbms { client }
    }

    pub async fn create_database(&self, params: CreateDbClusterMessage) -> Result<(), Error> {
        let response = self.client.create_db_cluster().set_request(params).send().await?;
        println!("{:?}", response);
        Ok(())
    }

    pub async fn create_snapshots(&self, params: CreateDbSnapshotMessage) -> Result<(), Error> {
        let response = self.client.create_db_snapshot().set_request(params).send().await?;
        println!("{:?}", response);
        Ok(())
    }

    pub async fn delete_database(&self, params: DeleteDbClusterMessage) -> Result<(), Error> {
        let response = self.client.delete_db_cluster().set_request(params).send().await?;
        println!("{:?}", response);
        Ok(())
    }

    pub async fn delete_db_snapshot(&self, params: DeleteDbSnapshotMessage) -> Result<(), Error> {
        let response = self.client.delete_db_snapshot().set_request(params).send().await?;
        println!("{:?}", response);
        Ok(())
    }

    pub async fn modify_db(&self, params: ModifyDbClusterMessage) -> Result<(), Error> {
        let response = self.client.modify_db_cluster().set_request(params).send().await?;
        println!("{:?}", response);
        Ok(())
    }

    pub async fn restore_database(&self, params: RebootDbInstanceMessage) -> Result<(), Error> {
        let response = self.client.reboot_db_instance().set_request(params).send().await?;
        println!("{:?}", response);
        Ok(())
    }
}
