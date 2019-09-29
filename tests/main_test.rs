extern crate assert_cmd;
extern crate rusoto_s3;

//use git_remote_s3;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::{
    CreateBucketRequest, DeleteBucketRequest, DeleteObjectRequest, ListObjectsV2Request, S3Client,
    S3,
};

use tempfile::Builder;

use assert_cmd::cargo::cargo_bin;
use assert_cmd::prelude::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn git(pwd: &Path, args: &str) -> Command {
    let my_path = cargo_bin("git-remote-s3");
    let my_path = my_path.parent().unwrap();
    let my_path = my_path.to_str().unwrap();
    let new_path = format!("{}:{}", my_path, env::var("PATH").unwrap());

    let mut command = Command::new("git");
    command.current_dir(pwd);
    command.env("PATH", new_path);
    command.env("S3_ENDPOINT", "http://localhost:9001");
    command.env("AWS_ACCESS_KEY_ID", "test");
    command.env("AWS_SECRET_ACCESS_KEY", "test1234");
    cmd_args(&mut command, args);
    command
}

fn cmd_args(command: &mut Command, args: &str) {
    let words: Vec<_> = args.split_whitespace().collect();

    for word in words {
        command.arg(word);
    }
}

fn delete_object(client: &S3Client, bucket: &str, filename: &str) {
    let del_req = DeleteObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .delete_object(del_req)
        .sync()
        .expect("Couldn't delete object");
    println!("{:#?}", result);
}

fn list_keys_in_bucket(client: &S3Client, bucket: &str) -> Vec<String> {
    let list_obj_req = ListObjectsV2Request {
        bucket: bucket.to_owned(),
        ..Default::default()
    };
    let result = client.list_objects_v2(list_obj_req).sync();
    match result {
        Ok(r) => r
            .contents
            .unwrap()
            .into_iter()
            .map(|o| o.key.unwrap())
            .collect(),
        _ => vec![],
    }
}

fn create_bucket(client: &S3Client, bucket: &str) {
    let create_bucket_req = CreateBucketRequest {
        bucket: bucket.to_owned(),
        ..Default::default()
    };

    let result = client
        .create_bucket(create_bucket_req)
        .sync()
        .expect("Couldn't create bucket");
    println!("{:?}", result);
}

fn delete_bucket(client: &S3Client, bucket: &str) {
    let delete_bucket_req = DeleteBucketRequest {
        bucket: bucket.to_owned(),
    };

    let result = client.delete_bucket(delete_bucket_req).sync();
    println!("{:?}", result);
}

fn delete_bucket_recurse(client: &S3Client, bucket: &str) {
    let keys = list_keys_in_bucket(client, bucket);
    for k in keys {
        delete_object(client, bucket, &k);
    }
    delete_bucket(client, bucket);
}

fn git_rev(pwd: &Path) -> String {
    let out = git(pwd, "rev-parse --short HEAD").output().unwrap();
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}

#[test]
fn integration() {
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:9001".to_owned(),
    };

    let s3 = S3Client::new_with(
        HttpClient::new().unwrap(),
        StaticProvider::new_minimal("test".to_string(), "test1234".to_string()),
        region,
    );

    let test_dir = Builder::new()
        .prefix("git_s3_test")
        .tempdir()
        .expect("mktemp dir failed");

    println!("Test dir: {}", test_dir.path().display());

    let repo1 = test_dir.path().join("repo1");
    let repo2 = test_dir.path().join("repo2");
    //test_dir.into_path();

    fs::create_dir(&repo1).unwrap();
    fs::create_dir(&repo2).unwrap();

    // Setup s3 bucket
    delete_bucket_recurse(&s3, "git-remote-s3");
    create_bucket(&s3, "git-remote-s3");

    println!("test: pushing from repo1");
    git(&repo1, "init").assert().success();
    git(&repo1, "commit --allow-empty -am r1_c1")
        .assert()
        .success();
    git(&repo1, "remote add origin s3://git-remote-s3/test")
        .assert()
        .success();
    git(&repo1, "push origin").assert().success();
    let sha = git_rev(&repo1);

    println!("test: cloning into repo2");
    git(&repo2, "clone s3://git-remote-s3/test .")
        .assert()
        .success();
    git(&repo2, "log --oneline --decorate=short")
        .assert()
        .stdout(format!(
            "{} (HEAD -> master, origin/master, origin/HEAD) r1_c1\n",
            sha
        ));

    println!("test: push form repo2 and pull into repo1");
    git(&repo2, "commit --allow-empty -am r2_c1")
        .assert()
        .success();
    git(&repo2, "push origin").assert().success();
    let sha = git_rev(&repo2);
    git(&repo1, "pull origin master").assert().success();
    git(&repo1, "log --oneline --decorate=short -n 1")
        .assert()
        .stdout(format!("{} (HEAD -> master, origin/master) r2_c1\n", sha));

    println!("test: force push form repo2");
    git(&repo1, "commit --allow-empty -am r1_c2")
        .assert()
        .success();
    git(&repo1, "push origin").assert().success();
    //let sha = git_rev(&repo1);
    git(&repo2, "commit --allow-empty -am r2_c2")
        .assert()
        .success();
    let sha = git_rev(&repo2);
    //assert!(false, "abort");
    git(&repo2, "push origin").assert().failure();
    git(&repo2, "push -f origin").assert().success();
    // TODO assert that there are 2 refs on s3 (the original was kept)
    git(&repo1, "pull origin master").assert().success();
    git(
        &repo1,
        format!("log --oneline --decorate=short -n 1 {}", sha).as_str(),
    )
    .assert()
    .stdout(format!("{} (origin/master) r2_c2\n", sha));
    git(&repo1, "push origin master").assert().success();
    // TODO assert that there is only one ref on s3

    assert_eq!(2 + 2, 4);
}
