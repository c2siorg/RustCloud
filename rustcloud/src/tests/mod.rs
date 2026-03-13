#[cfg(feature = "live-tests-azure")]
mod azure_blob_operations;

#[cfg(feature = "live-tests-aws")]
mod aws_archival_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_block_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_bucket_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_dns_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_dynamodb_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_ec2_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_ecs_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_eks_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_iam_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_kms_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_loadbalancer_operations;
#[cfg(feature = "live-tests-aws")]
mod aws_monitoring_operations;

#[cfg(feature = "live-tests-gcp")]
mod gcp_automl_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_bigtable_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_bigquery_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_compute_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_dns_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_kubernetes_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_loadbalancer_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_notification_operations;
#[cfg(feature = "live-tests-gcp")]
mod gcp_storage_operations;
