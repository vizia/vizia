use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        dx12: { all(target_os = "windows", feature = "dx12") },
        metal: { all(target_os = "macos", feature = "metal") },
    }
}
