use awscreds::Credentials;
use pgrx::prelude::*;
use s3::bucket::Bucket;
use s3::region::Region;
use std::convert::TryInto;

::pgrx::pg_module_magic!(name, version);
fn bucket_create(
    server: String,
    input_bucket: String,
    input_identity: String,
    input_secret: String,
) -> Bucket {
    let mut bucket = Bucket::new(
        &input_bucket,
        Region::Custom {
            region: "none".into(),
            endpoint: server.into(),
        },
        Credentials::new_blocking(Some(&input_identity), Some(&input_secret), None, None, None)
            .unwrap(),
    )
    .unwrap();
    bucket.set_path_style();
    return bucket;
}

#[pg_extern]
fn pgx_s3sign_pre_get(
    server: String,
    input_bucket: String,
    input_identity: String,
    input_secret: String,
    input_file: String,
    duration: i32,
) -> String {
    let bucket = bucket_create(server, input_bucket, input_identity, input_secret);

    let url = bucket
        .presign_get(input_file, duration.try_into().unwrap())
        .unwrap();
    url
}

#[pg_extern]
fn pgx_s3sign_pre_put(
    server: String,
    input_bucket: String,
    input_identity: String,
    input_secret: String,
    input_file: String,
    duration: i32,
) -> String {
    let bucket = bucket_create(server, input_bucket, input_identity, input_secret);
    let url = bucket
        .presign_put(input_file, duration.try_into().unwrap())
        .unwrap();
    url
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
