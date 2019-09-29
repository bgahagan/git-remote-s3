extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_s3::{
    DeleteObjectOutput, DeleteObjectRequest, GetObjectOutput, GetObjectRequest,
    ListObjectsV2Output, ListObjectsV2Request, PutObjectOutput, PutObjectRequest, S3Client, S3,
};

use std::fs::{File, OpenOptions};
use std::io::{copy, Read};
use std::path::Path;

use super::errors::*;

#[derive(Debug)]
pub struct Key {
    pub bucket: String,
    pub key: String,
}

pub fn get(s3: &S3Client, o: &Key, f: &Path) -> Result<GetObjectOutput> {
    let req = GetObjectRequest {
        bucket: o.bucket.to_owned(),
        key: o.key.to_owned(),
        ..Default::default()
    };
    let mut result = s3
        .get_object(req)
        .sync()
        .chain_err(|| "couldn't get item")?;
    let body = result.body.take().chain_err(|| "no body")?;
    let mut target = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(f)
        .chain_err(|| "open failed")?;
    copy(&mut body.into_blocking_read(), &mut target).chain_err(|| "copy failed")?;
    Ok(result)
}

pub fn put(s3: &S3Client, f: &Path, o: &Key) -> Result<PutObjectOutput> {
    let mut f = File::open(f).chain_err(|| "open failed")?;
    let mut contents: Vec<u8> = Vec::new();
    f.read_to_end(&mut contents).chain_err(|| "read failed")?;
    let req = PutObjectRequest {
        bucket: o.bucket.to_owned(),
        key: o.key.to_owned(),
        body: Some(contents.into()),
        ..Default::default()
    };
    s3.put_object(req)
        .sync()
        .chain_err(|| "Couldn't PUT object")
}

pub fn del(s3: &S3Client, o: &Key) -> Result<DeleteObjectOutput> {
    let req = DeleteObjectRequest {
        bucket: o.bucket.to_owned(),
        key: o.key.to_owned(),
        ..Default::default()
    };
    s3.delete_object(req)
        .sync()
        .chain_err(|| "Couldn't DELETE object")
}

pub fn list(s3: &S3Client, k: &Key) -> Result<ListObjectsV2Output> {
    let list_obj_req = ListObjectsV2Request {
        bucket: k.bucket.to_owned(),
        prefix: Some(k.key.to_owned()),
        ..Default::default()
    };
    s3.list_objects_v2(list_obj_req)
        .sync()
        .chain_err(|| "Couldn't list items in bucket")
}
