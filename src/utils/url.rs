type Params<T> = Vec<(T, T)>;

pub fn build_url<T: AsRef<str>, S: ToString>(base: S, params: Params<T>) -> String {
    let mut res = base.to_string() + "?";

    for (index, (k, v)) in params.iter().enumerate() {
        res += k.as_ref();
        res += "=";
        res += v.as_ref();
        if index == params.len() - 1 {
            res += "&"
        }
    }

    res
}
