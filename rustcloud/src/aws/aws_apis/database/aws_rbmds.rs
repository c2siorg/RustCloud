#![allow(clippy::result_large_err)]

use std::collections::HashMap;
use tokio;
use aws_sdk_rds::{operation::{create_db_cluster::CreateDBCluster, create_db_instance::CreateDbInstanceInput}, Client, Error};



pub struct CreateDbInstanceInput {
    pub db_name: ::std::option::Option<::std::string::String>,
    pub db_instance_identifier: ::std::option::Option<::std::string::String>,
    pub allocated_storage: ::std::option::Option<i32>,
    pub db_instance_class: ::std::option::Option<::std::string::String>,
    pub engine: ::std::option::Option<::std::string::String>,
    pub master_username: ::std::option::Option<::std::string::String>,
    pub master_user_password: ::std::option::Option<::std::string::String>,
    pub db_security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub vpc_security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub availability_zone: ::std::option::Option<::std::string::String>,
    pub db_subnet_group_name: ::std::option::Option<::std::string::String>,
    pub preferred_maintenance_window: ::std::option::Option<::std::string::String>,
    pub db_parameter_group_name: ::std::option::Option<::std::string::String>,
    pub backup_retention_period: ::std::option::Option<i32>,
    pub preferred_backup_window: ::std::option::Option<::std::string::String>,
    pub port: ::std::option::Option<i32>,
    pub multi_az: ::std::option::Option<bool>,
    pub engine_version: ::std::option::Option<::std::string::String>,
    pub auto_minor_version_upgrade: ::std::option::Option<bool>,
    pub license_model: ::std::option::Option<::std::string::String>,
    pub iops: ::std::option::Option<i32>,
    /// <p>The option group to associate the DB instance with.</p>
    /// <p>Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be removed from an option group. Also, that option group can't be removed from a DB instance after it is associated with a DB instance.</p>
    /// <p>This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.</p>
    pub option_group_name: ::std::option::Option<::std::string::String>,
    /// <p>For supported engines, the character set (<code>CharacterSet</code>) to associate the DB instance with.</p>
    /// <p>This setting doesn't apply to the following DB instances:</p>
    /// <ul>
    /// <li>
    /// <p>Amazon Aurora - The character set is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p></li>
    /// <li>
    /// <p>RDS Custom - However, if you need to change the character set, you can change it on the database itself.</p></li>
    /// </ul>
    pub character_set_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the NCHAR character set for the Oracle DB instance.</p>
    /// <p>This setting doesn't apply to RDS Custom DB instances.</p>
    pub nchar_character_set_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether the DB instance is publicly accessible.</p>
    /// <p>When the DB instance is publicly accessible, its Domain Name System (DNS) endpoint resolves to the private IP address from within the DB instance's virtual private cloud (VPC). It resolves to the public IP address from outside of the DB instance's VPC. Access to the DB instance is ultimately controlled by the security group it uses. That public access is not permitted if the security group assigned to the DB instance doesn't permit it.</p>
    /// <p>When the DB instance isn't publicly accessible, it is an internal DB instance with a DNS name that resolves to a private IP address.</p>
    /// <p>Default: The default behavior varies depending on whether <code>DBSubnetGroupName</code> is specified.</p>
    /// <p>If <code>DBSubnetGroupName</code> isn't specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
    /// <ul>
    /// <li>
    /// <p>If the default VPC in the target Region doesn’t have an internet gateway attached to it, the DB instance is private.</p></li>
    /// <li>
    /// <p>If the default VPC in the target Region has an internet gateway attached to it, the DB instance is public.</p></li>
    /// </ul>
    /// <p>If <code>DBSubnetGroupName</code> is specified, and <code>PubliclyAccessible</code> isn't specified, the following applies:</p>
    /// <ul>
    /// <li>
    /// <p>If the subnets are part of a VPC that doesn’t have an internet gateway attached to it, the DB instance is private.</p></li>
    /// <li>
    /// <p>If the subnets are part of a VPC that has an internet gateway attached to it, the DB instance is public.</p></li>
    /// </ul>
    pub publicly_accessible: ::std::option::Option<bool>,
    /// <p>Tags to assign to the DB instance.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The identifier of the DB cluster that this DB instance will belong to.</p>
    /// <p>This setting doesn't apply to RDS Custom DB instances.</p>
    pub db_cluster_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The storage type to associate with the DB instance.</p>
    /// <p>If you specify <code>io1</code>, <code>io2</code>, or <code>gp3</code>, you must also include a value for the <code>Iops</code> parameter.</p>
    /// <p>This setting doesn't apply to Amazon Aurora DB instances. Storage is managed by the DB cluster.</p>
    /// <p>Valid Values: <code>gp2 | gp3 | io1 | io2 | standard</code></p>
    /// <p>Default: <code>io1</code>, if the <code>Iops</code> parameter is specified. Otherwise, <code>gp2</code>.</p>
    pub storage_type: ::std::option::Option<::std::string::String>,
    /// <p>The ARN from the key store with which to associate the instance for TDE encryption.</p>
    /// <p>This setting doesn't apply to Amazon Aurora or RDS Custom DB instances.</p>
    pub tde_credential_arn: ::std::option::Option<::std::string::String>,
    /// <p>The password for the given ARN from the key store in order to access the device.</p>
    /// <p>This setting doesn't apply to RDS Custom DB instances.</p>
    pub tde_credential_password: ::std::option::Option<::std::string::String>,
    /// <p>Specifes whether the DB instance is encrypted. By default, it isn't encrypted.</p>
    /// <p>For RDS Custom DB instances, either enable this setting or leave it unset. Otherwise, Amazon RDS reports an error.</p>
    /// <p>This setting doesn't apply to Amazon Aurora DB instances. The encryption for DB instances is managed by the DB cluster.</p>
    pub storage_encrypted: ::std::option::Option<bool>,
    /// <p>The Amazon Web Services KMS key identifier for an encrypted DB instance.</p>
    /// <p>The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN.</p>
    /// <p>This setting doesn't apply to Amazon Aurora DB instances. The Amazon Web Services KMS key identifier is managed by the DB cluster. For more information, see <code>CreateDBCluster</code>.</p>
    /// <p>If <code>StorageEncrypted</code> is enabled, and you do not specify a value for the <code>KmsKeyId</code> parameter, then Amazon RDS uses your default KMS key. There is a default KMS key for your Amazon Web Services account. Your Amazon Web Services account has a different default KMS key for each Amazon Web Services Region.</p>
    /// <p>For Amazon RDS Custom, a KMS key is required for DB instances. For most RDS engines, if you leave this parameter empty while enabling <code>StorageEncrypted</code>, the engine uses the default KMS key. However, RDS Custom doesn't use the default key when this parameter is empty. You must explicitly specify a key.</p>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The Active Directory directory ID to create the DB instance in. Currently, you can create only Db2, MySQL, Microsoft SQL Server, Oracle, and PostgreSQL DB instances in an Active Directory Domain.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/kerberos-authentication.html"> Kerberos Authentication</a> in the <i>Amazon RDS User Guide</i>.</p>
    /// <p>This setting doesn't apply to the following DB instances:</p>
    /// <ul>
    /// <li>
    /// <p>Amazon Aurora (The domain is managed by the DB cluster.)</p></li>
    /// <li>
    /// <p>RDS Custom</p></li>
    /// </ul>
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p>The fully qualified domain name (FQDN) of an Active Directory domain.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be longer than 64 characters.</p></li>
    /// </ul>
    /// <p>Example: <code>mymanagedADtest.mymanagedAD.mydomain</code></p>
    pub domain_fqdn: ::std::option::Option<::std::string::String>,
    /// <p>The Active Directory organizational unit for your DB instance to join.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must be in the distinguished name format.</p></li>
    /// <li>
    /// <p>Can't be longer than 64 characters.</p></li>
    /// </ul>
    /// <p>Example: <code>OU=mymanagedADtestOU,DC=mymanagedADtest,DC=mymanagedAD,DC=mydomain</code></p>
    pub domain_ou: ::std::option::Option<::std::string::String>,
    /// <p>The ARN for the Secrets Manager secret with the credentials for the user joining the domain.</p>
    /// <p>Example: <code>arn:aws:secretsmanager:region:account-number:secret:myselfmanagedADtestsecret-123456</code></p>
    pub domain_auth_secret_arn: ::std::option::Option<::std::string::String>,
    /// <p>The IPv4 DNS IP addresses of your primary and secondary Active Directory domain controllers.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Two IP addresses must be provided. If there isn't a secondary domain controller, use the IP address of the primary domain controller for both entries in the list.</p></li>
    /// </ul>
    /// <p>Example: <code>123.124.125.126,234.235.236.237</code></p>
    pub domain_dns_ips: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specifies whether to copy tags from the DB instance to snapshots of the DB instance. By default, tags are not copied.</p>
    /// <p>This setting doesn't apply to Amazon Aurora DB instances. Copying tags to snapshots is managed by the DB cluster. Setting this value for an Aurora DB instance has no effect on the DB cluster setting.</p>
    pub copy_tags_to_snapshot: ::std::option::Option<bool>,
    /// <p>The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collection of Enhanced Monitoring metrics, specify <code>0</code>.</p>
    /// <p>If <code>MonitoringRoleArn</code> is specified, then you must set <code>MonitoringInterval</code> to a value other than <code>0</code>.</p>
    /// <p>This setting doesn't apply to RDS Custom DB instances.</p>
    /// <p>Valid Values: <code>0 | 1 | 5 | 10 | 15 | 30 | 60</code></p>
    /// <p>Default: <code>0</code></p>
    pub monitoring_interval: ::std::option::Option<i32>,
    pub monitoring_role_arn: ::std::option::Option<::std::string::String>,
    pub domain_iam_role_name: ::std::option::Option<::std::string::String>,
    pub promotion_tier: ::std::option::Option<i32>,
    pub timezone: ::std::option::Option<::std::string::String>,
    pub enable_iam_database_authentication: ::std::option::Option<bool>,
    pub enable_performance_insights: ::std::option::Option<bool>,
    pub performance_insights_kms_key_id: ::std::option::Option<::std::string::String>,
    pub performance_insights_retention_period: ::std::option::Option<i32>,
    pub enable_cloudwatch_logs_exports: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub processor_features: ::std::option::Option<::std::vec::Vec<crate::types::ProcessorFeature>>,
    pub deletion_protection: ::std::option::Option<bool>,
    pub max_allocated_storage: ::std::option::Option<i32>,
    pub enable_customer_owned_ip: ::std::option::Option<bool>,
    pub custom_iam_instance_profile: ::std::option::Option<::std::string::String>,
    pub backup_target: ::std::option::Option<::std::string::String>,
    pub network_type: ::std::option::Option<::std::string::String>,
    pub storage_throughput: ::std::option::Option<i32>,
    pub manage_master_user_password: ::std::option::Option<bool>,
    pub master_user_secret_kms_key_id: ::std::option::Option<::std::string::String>,
    pub ca_certificate_identifier: ::std::option::Option<::std::string::String>,
    pub db_system_id: ::std::option::Option<::std::string::String>,
    pub dedicated_log_volume: ::std::option::Option<bool>,
    pub multi_tenant: ::std::option::Option<bool>,
    pub engine_lifecycle_support: ::std::option::Option<::std::string::String>,
}

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn create_database(client: &Client, ) -> Result<(), Error> {
    let create_db = client.create_db_instance().send().await?;
    CreateDbInstanceInput
    Ok(())
}
