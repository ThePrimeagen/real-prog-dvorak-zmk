fn get_macro(name: &str, action: &str) -> String {
    return format!("ZMK_MACRO({},
    wait-ms = <0>;
    tap-ms = <0>;
    bindings
    = <&macro_release &kp LSHFT>
    , <&macro_tap {}>
    , <&macro_press &kp LSHFT>
    , <&macro_pause_for_release>;
)", name, action);
}

/*
odotw: zero_dotw {
    compatible = "zmk,behavior-mod-morph";
    label = "ZERO_DOTW";
    #binding-cells = <0>;
    bindings = <&kp N0>, <&dot_dot>;
    mods = <(MOD_LSFT)>;
};
*/

struct Config {
    name: String,
    behavior: String
}

fn get_config(a: &str, b: &str) -> Config {
    let short_name = a.chars().take(1).chain(b.chars()).collect::<String>();
    let long_name = format!("{}_{}", a, b);
    let key_a = a.to_uppercase();
    let key_b = format!("&kp {}", b.to_uppercase());

    return Config {
        name: short_name.clone(),
        behavior: format!("{}: {} {{
    compatible = \"zmk,behavior-mod-morph\";
    label = \"{}\";
    #binding-cells = <0>;
    bindings = <&kp {}>, <{}>;
    mods = <(MOD_LGUI|MOD_LSFT|MOD_RGUI|MOD_RSFT)>;
}};", short_name, long_name, long_name, key_a, key_b),
    };
}

fn main() {
    let mut configs = vec![];

    configs.push(get_config("pipe", "pipe"));
    configs.push(get_config("plus", "kp_n1"));
    configs.push(get_config("left_bracket", "kp_n2"));
    configs.push(get_config("left_brace", "kp_n3"));
    configs.push(get_config("left_parenthesis", "kp_n4"));
    configs.push(get_config("amps", "kp_n5"));
    configs.push(get_config("equal", "kp_n6"));
    configs.push(get_config("right_parenthesis", "kp_n7"));
    configs.push(get_config("right_brace", "kp_n8"));
    configs.push(get_config("right_bracket", "kp_n9"));
    configs.push(get_config("star", "kp_n0"));
    configs.push(get_config("excl", "prcnt"));
    configs.push(get_config("grave", "grave"));

    for config in &configs {
        println!("{}", config.behavior);
    }

    for config in &configs {
        print!("&{}  ", config.name);
    }
    println!();
    println!();
    println!("My other sides");

    let mut configs = vec![];
    configs.push(get_config("dllr", "grave"));
    configs.push(get_config("grave", "grave"));
    configs.push(get_config("bslh", "hash"));
    configs.push(get_config("at", "caret"));

    for config in &configs {
        println!("{}", config.behavior);
    }

    for config in &configs {
        print!("&{}  ", config.name);
    }
}
