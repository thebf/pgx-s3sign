use awscreds::Credentials;
use pgx::*;
use s3::bucket::Bucket;
use s3::region::Region;
use std::convert::TryInto;

pg_module_magic!();

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
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
    let my_creds =
        Credentials::new_blocking(Some(&input_identity), Some(&input_secret), None, None, None)
            .unwrap();

    let minio = Storage {
        name: "minio".into(),
        region: Region::Custom {
            region: "us-east-1".into(),
            endpoint: server.into(),
        },
        credentials: my_creds,
        bucket: input_bucket,
        location_supported: false,
    };

    let mut bucket = Bucket::new(&minio.bucket, minio.region, minio.credentials).unwrap();
    bucket.set_path_style();

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
    let my_creds =
        Credentials::new_blocking(Some(&input_identity), Some(&input_secret), None, None, None)
            .unwrap();

    let minio = Storage {
        name: "minio".into(),
        region: Region::Custom {
            region: "us-east-1".into(),
            endpoint: server.into(),
        },
        credentials: my_creds,
        bucket: input_bucket,
        location_supported: false,
    };

    let mut bucket = Bucket::new(&minio.bucket, minio.region, minio.credentials).unwrap();
    bucket.set_path_style();

    let url = bucket
        .presign_put(input_file, duration.try_into().unwrap())
        .unwrap();
    url
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_my_extension() {
        assert_eq!("Hello, my_extension", crate::hello_my_extension());
    }
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

//todo:implement tests.
