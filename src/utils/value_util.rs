use serde_json::Value;
use serde_json::json;

//深度取值。例如a.b.c 最终得到c.如果不存在返回Value::Null
pub fn GetDeepValue(arg: &str, value: &Value) -> Value {
    let splits: Vec<&str> = arg.split(".").collect();

    let mut v = value;
    for item in splits {
        if item.is_empty() {
            continue;
        }
        let ropt = v.get(item);
        if ropt.is_none() {
            return Value::Null;
        } else {
            v = ropt.unwrap();
        }
    }
    return v.clone();
}

#[test]
pub fn TestGetDeepValue() {
    let john = json!({
        "a": {
           "name":"job",
        },
    });

    let v = GetDeepValue("a.name", &john);
    println!("{}", v);
}