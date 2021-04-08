#[cfg(feature = "jni")]
#[allow(non_snake_case)]
pub mod android {
  extern crate jni;

  use self::jni::JNIEnv;
  use self::jni::objects::{JClass, JString};
  use self::jni::sys::jstring;

  #[no_mangle]
  pub unsafe extern fn Java_com_example_rusty_RustyKt_hello(env: JNIEnv, _: JClass, name: JString) -> jstring {
    let name: String = env.get_string(name)
        .expect("Couldn't get Java string!")
        .into();

    let response = format!("Hello {}!", name);
    let output = env
        .new_string(response)
        .expect("Couldn't create a Java string!");

    output.into_inner()
  }
}
