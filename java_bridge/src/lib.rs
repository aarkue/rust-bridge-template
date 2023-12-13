#![allow(non_snake_case)]

use jni::{
    objects::{JClass, JString},
    sys::jfloat,
    JNIEnv,
};
use jni_fn::jni_fn;

#[jni_fn("RustBridge")]
pub fn addTwoFloats<'local>(_env: JNIEnv<'local>, _: JClass, a: jfloat, b: jfloat) -> jfloat {
    return main_library::add_two_f32(a, b);
}

#[jni_fn("RustBridge")]
pub fn greetPerson<'local>(
    mut env: JNIEnv<'local>,
    _: JClass,
    person_json: JString<'local>,
) -> JString<'local> {
    let person = main_library::Person::from_json_str(
        env.get_string(&person_json).unwrap().to_str().unwrap(),
    );
    let greeting = main_library::get_greet_string(&person);
    return env.new_string(&greeting).unwrap();
}
