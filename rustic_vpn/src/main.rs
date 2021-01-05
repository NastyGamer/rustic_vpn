use crate::vpn_provider::VpnProvider;

mod vpn_provider;
mod nord_vpn;

fn main() {
    println!("{}", nord_vpn::NordVpn::is_logged_in());
}
