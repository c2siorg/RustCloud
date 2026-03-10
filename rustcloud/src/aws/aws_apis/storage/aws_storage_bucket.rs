#![allow(clippy::result_large_err)]

use aws_sdk_s3::{
    primitives::ByteStream,
    types::{
        BucketCannedAcl, CreateBucketConfiguration, MetadataDirective, ObjectCannedAcl,
        ObjectOwnership, RequestPayer, ServerSideEncryption, StorageClass, TaggingDirective,
    },
    Client, Error,
};
use std::collections::HashMap;

pub async fn create_bucket(
    client: &Client,
    acl: BucketCannedAcl,
    bucket: String,
    create_bucket_configuration: CreateBucketConfiguration,
    grant_full_control: Option<String>,
    grant_read: Option<String>,
    grant_read_acp: Option<String>,
    grant_write: Option<String>,
    grant_write_acp: Option<String>,
    object_lock_enabled_for_bucket: Option<bool>,
    object_ownership: Option<ObjectOwnership>,
) -> Result<(), Error> {
    let resp = client
        .create_bucket()
        .acl(acl)
        .bucket(bucket)
        .create_bucket_configuration(create_bucket_configuration)
        .set_grant_full_control(grant_full_control)
        .set_grant_read(grant_read)
        .set_grant_read_acp(grant_read_acp)
        .set_grant_write(grant_write)
        .set_grant_write_acp(grant_write_acp)
        .set_object_lock_enabled_for_bucket(object_lock_enabled_for_bucket)
        .set_object_ownership(object_ownership)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("createbucket: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete(
    client: &Client,
    bucket: String,
    expected_bucket_owner: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .delete_bucket()
        .bucket(bucket)
        .set_expected_bucket_owner(expected_bucket_owner)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("delete: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_object(
    client: &Client,
    bucket: String,
    key: String,
    expected_bucket_owner: Option<String>,
    mfa: Option<String>,
    version_id: Option<String>,
    request_payer: Option<RequestPayer>,
) -> Result<(), Error> {
    let resp = client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .set_expected_bucket_owner(expected_bucket_owner)
        .set_mfa(mfa)
        .set_version_id(version_id)
        .set_request_payer(request_payer)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("deleteobject: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn list(client: &Client) -> Result<(), Error> {
    let resp = client.list_buckets().send().await;
    match resp {
        Ok(result) => {
            println!("list: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn put_object(
    client: &Client,
    bucket: String,
    key: String,
    body: ByteStream,
    content_type: Option<String>,
    content_length: Option<i64>,
    metadata: Option<HashMap<String, String>>,
    storage_class: Option<StorageClass>,
    server_side_encryption: Option<ServerSideEncryption>,
    expected_bucket_owner: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .set_content_type(content_type)
        .set_content_length(content_length)
        .set_metadata(metadata)
        .set_storage_class(storage_class)
        .set_server_side_encryption(server_side_encryption)
        .set_expected_bucket_owner(expected_bucket_owner)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("putobject: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn get_object(
    client: &Client,
    bucket: String,
    key: String,
    range: Option<String>,
    version_id: Option<String>,
    expected_bucket_owner: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .set_range(range)
        .set_version_id(version_id)
        .set_expected_bucket_owner(expected_bucket_owner)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("getobject: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn list_objects_v2(
    client: &Client,
    bucket: String,
    prefix: Option<String>,
    continuation_token: Option<String>,
    max_keys: Option<i32>,
    delimiter: Option<String>,
    start_after: Option<String>,
    fetch_owner: Option<bool>,
    expected_bucket_owner: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .list_objects_v2()
        .bucket(bucket)
        .set_prefix(prefix)
        .set_continuation_token(continuation_token)
        .set_max_keys(max_keys)
        .set_delimiter(delimiter)
        .set_start_after(start_after)
        .set_fetch_owner(fetch_owner)
        .set_expected_bucket_owner(expected_bucket_owner)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("listobjectsv2: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn head_object(
    client: &Client,
    bucket: String,
    key: String,
    if_match: Option<String>,
    if_none_match: Option<String>,
    range: Option<String>,
    version_id: Option<String>,
    expected_bucket_owner: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .head_object()
        .bucket(bucket)
        .key(key)
        .set_if_match(if_match)
        .set_if_none_match(if_none_match)
        .set_range(range)
        .set_version_id(version_id)
        .set_expected_bucket_owner(expected_bucket_owner)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("headobject: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn copy_object(
    client: &Client,
    bucket: String,
    key: String,
    copy_source: String,
    metadata_directive: Option<MetadataDirective>,
    acl: Option<ObjectCannedAcl>,
    storage_class: Option<StorageClass>,
    tagging_directive: Option<TaggingDirective>,
    server_side_encryption: Option<ServerSideEncryption>,
    expected_bucket_owner: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .copy_object()
        .bucket(bucket)
        .key(key)
        .copy_source(copy_source)
        .set_metadata_directive(metadata_directive)
        .set_acl(acl)
        .set_storage_class(storage_class)
        .set_tagging_directive(tagging_directive)
        .set_server_side_encryption(server_side_encryption)
        .set_expected_bucket_owner(expected_bucket_owner)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("copyobject: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

// async fn upload(client: &Client, body: ::aws_smithy_types::byte_stream::ByteStream,bucket: Option<String>,content_length: Option<i64>,content_md5: Option<String>,checksum_algorithm: Option<ChecksumAlgorithm>,checksum_crc32: Option<String>,checksum_crc32_c: Option<String>,checksum_sha1: Option<String>,checksum_sha256: Option<String>,key: Option<String>,part_number: Option<i32>,upload_id: Option<String>,sse_customer_algorithm: Option<String>,sse_customer_key: Option<String>,sse_customer_key_md5: Option<String>,request_payer: Option<RequestPayer>,expected_bucket_owner: Option<String>) -> Result<(), Error> {
//     let upload = client.upload_part().send().await?;
//     println!("upload: {:?}", upload);
//     Ok(())
// }
