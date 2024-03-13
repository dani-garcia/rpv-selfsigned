use std::{error::Error, time::Duration};

pub fn network_test() -> Result<String, Box<dyn Error>> {
    use rustls::{
        client::danger::ServerCertVerifier,
        pki_types::{CertificateDer, ServerName, UnixTime},
    };

    let cert = CertificateDer::from(&include_bytes!("../badssl.com.cer")[..]);
    let ca = CertificateDer::from(&include_bytes!("../badssl.com.ca.cer")[..]);
    let intermediates = vec![ca];

    let server_name = ServerName::try_from("untrusted-root.badssl.com")?;

    let ocsp_response = Vec::new();

    // This time is copied from the rustls-platform-verifier tests
    // Monday, March 11, 2024 8:30:25 PM UTC
    let now = UnixTime::since_unix_epoch(Duration::from_secs(1_710_189_025));

    let result = rustls_platform_verifier::Verifier::new().verify_server_cert(
        &cert,
        &intermediates,
        &server_name,
        &ocsp_response,
        now,
    )?;

    Ok(format!("Success: {:?}", result))
}

#[cfg(target_os = "android")]
mod android {
    use jni::{
        objects::{JClass, JObject},
        sys::jstring,
        JNIEnv,
    };

    #[no_mangle]
    pub extern "system" fn Java_com_example_myapplication_JniExample_networkTest(
        env: JNIEnv,
        _class: JClass,
        context: JObject,
    ) -> jstring {
        rustls_platform_verifier::android::init_hosted(&env, context).unwrap();

        let output = match super::network_test() {
            Ok(body) => env.new_string(body).unwrap(),
            Err(err) => env.new_string(format!("Error: {:?}", err)).unwrap(),
        };

        output.into_inner()
    }
}
