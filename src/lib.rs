use std::{error::Error, sync::Arc};

pub fn network_test() -> Result<String, Box<dyn Error>> {
    // Reqwest is still using rustls 0.21, so won't work with rpv 0.2
    /*let client = reqwest::blocking::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()?;

    let response = client.get("https://untrusted-root.badssl.com/").send()?;
    let body = response.text()?;*/

    let agent = ureq::builder()
        .tls_config(Arc::new(rustls_platform_verifier::tls_config()))
        .build();

    let body = agent
        .get("https://untrusted-root.badssl.com/")
        .call()?
        .into_string()?;

    Ok(body)
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
