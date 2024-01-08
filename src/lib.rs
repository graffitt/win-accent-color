use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;
use regex::Regex;

/// returns current windows accent color in hex
///
/// default color - 0xffd77800 (4292311040) [src](https://learn.microsoft.com/en-us/windows-hardware/customize/desktop/unattend/microsoft-windows-shell-setup-themes-windowcolor#values)
pub fn get_accent_color() -> String{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Accent";

    let accent = match hkcu.open_subkey(path){
        Ok(_) => hkcu.open_subkey(path).unwrap(),
        Err(_error) => return String::from("#0078d7")
    };

    let color: u32 = accent.get_value("AccentColorMenu").unwrap_or_else(|_| {4292311040});

    return process_string(format!("{:x}", color))
}
/// ffd47800 -> #0078d4
fn process_string(input: String) -> String{
    let without_ff = input.trim_start_matches(['f', 'F']);

    let reg = Regex::new(r"(.{1,2})").unwrap();
    let mut temp: Vec<&str> = vec![];

    for (_, [group]) in reg.captures_iter(without_ff).map(|c| c.extract()) {
        temp.push(group)
    }
    temp.reverse();

    return format!("#{}", temp.join(""))
}