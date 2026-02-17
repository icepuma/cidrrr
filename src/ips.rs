use std::net::IpAddr;

use crate::error::AppError;
use ipnet::IpNet;
use num_bigint::BigUint;

// show only that many addresses, unless "--danger-zone" is set
const HARD_LIMIT: usize = 1_048_576;

pub struct IpComputation {
    pub total_host_count: BigUint,
    pub ips: Box<dyn Iterator<Item = IpAddr>>,
}

fn calculate_total_host_count(ip_net: &IpNet) -> BigUint {
    match ip_net {
        IpNet::V4(v4_net) => {
            let prefix_len = v4_net.prefix_len();
            let host_bits = usize::from(32 - prefix_len);
            let total_addresses = BigUint::from(1u8) << host_bits;

            if prefix_len <= 30 {
                total_addresses - BigUint::from(2u8)
            } else {
                total_addresses
            }
        }
        IpNet::V6(v6_net) => {
            let host_bits = usize::from(128 - v6_net.prefix_len());
            BigUint::from(1u8) << host_bits
        }
    }
}

pub fn calculate_all_ips(
    cidr: &str,
    all: bool,
    danger_zone: bool,
) -> Result<IpComputation, AppError> {
    let ip_net = cidr
        .parse::<IpNet>()
        .map_err(|source| AppError::InvalidCidr {
            input: cidr.to_owned(),
            source,
        })?;
    let total_host_count = calculate_total_host_count(&ip_net);

    let ips: Box<dyn Iterator<Item = IpAddr>> = if all && danger_zone {
        Box::new(ip_net.hosts())
    } else if all {
        Box::new(ip_net.hosts().take(HARD_LIMIT))
    } else {
        let mut hosts = ip_net.hosts();

        Box::new(hosts.next().into_iter().chain(hosts.last()))
    };

    Ok(IpComputation {
        total_host_count,
        ips,
    })
}

#[cfg(test)]
mod tests {
    use crate::ips::HARD_LIMIT;

    use super::calculate_all_ips;
    use num_bigint::BigUint;
    use pretty_assertions::assert_eq;
    use std::net::IpAddr;

    fn ip_addr(value: &str) -> IpAddr {
        value.parse::<IpAddr>().unwrap()
    }

    fn host_count(value: &str) -> BigUint {
        value.parse::<BigUint>().unwrap()
    }

    #[test]
    fn test_calculate_all_ips_v4_show_first_and_last() {
        let result = calculate_all_ips("10.10.10.1/24", false, false).unwrap();
        let ips = result.ips.collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("10.10.10.1"), ip_addr("10.10.10.254")]);
        assert_eq!(result.total_host_count, host_count("254"));

        let result = calculate_all_ips("10.10.10.1/32", false, false).unwrap();
        let ips = result.ips.collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("10.10.10.1")]);
        assert_eq!(result.total_host_count, host_count("1"));
    }

    #[test]
    fn test_calculate_all_ips_v4_show_all() {
        let result = calculate_all_ips("10.10.10.1/8", true, false).unwrap();
        let ips = result.ips;

        assert_eq!(ips.count(), HARD_LIMIT);
        assert_eq!(result.total_host_count, host_count("16777214"));
    }

    #[test]
    fn test_calculate_all_ips_v4_show_all_in_danger_zone() {
        let result = calculate_all_ips("10.10.10.0/8", true, true).unwrap();
        let mut ips = result.ips;

        // network address and broadcast address are excluded
        assert_eq!(ips.by_ref().next().unwrap(), ip_addr("10.0.0.1"));
        assert_eq!(ips.by_ref().last().unwrap(), ip_addr("10.255.255.254"));
        assert_eq!(result.total_host_count, host_count("16777214"));
    }

    #[test]
    fn test_calculate_all_ips_v6_show_first_and_last() {
        let result = calculate_all_ips("2001:db8::/113", false, false).unwrap();
        let ips = result.ips.collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("2001:db8::"), ip_addr("2001:db8::7fff")]);
        assert_eq!(result.total_host_count, host_count("32768"));

        let result = calculate_all_ips("2001:db8::/128", false, false).unwrap();
        let ips = result.ips.collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("2001:db8::")]);
        assert_eq!(result.total_host_count, host_count("1"));
    }

    #[test]
    fn test_calculate_all_ips_v6_show_all() {
        let result = calculate_all_ips("2001:db8::/106", true, false).unwrap();
        let ips = result.ips;

        assert_eq!(ips.count(), HARD_LIMIT);
        assert_eq!(result.total_host_count, host_count("4194304"));
    }

    #[test]
    fn test_calculate_all_ips_v6_show_all_in_danger_zone() {
        let result = calculate_all_ips("2001:db8::/106", true, true).unwrap();
        let mut ips = result.ips;

        // ipv6 has no broadcast address
        assert_eq!(ips.by_ref().next().unwrap(), ip_addr("2001:db8::"));
        assert_eq!(ips.by_ref().last().unwrap(), ip_addr("2001:db8::3f:ffff"));
        assert_eq!(result.total_host_count, host_count("4194304"));
    }

    #[test]
    fn test_calculate_all_ips_v4_prefix_31_has_two_hosts() {
        let result = calculate_all_ips("10.0.0.0/31", false, false).unwrap();
        let ips = result.ips.collect::<Vec<IpAddr>>();

        assert_eq!(ips, vec![ip_addr("10.0.0.0"), ip_addr("10.0.0.1")]);
        assert_eq!(result.total_host_count, host_count("2"));
    }

    #[test]
    fn test_calculate_all_ips_v6_prefix_0_count_is_exact() {
        let result = calculate_all_ips("::/0", false, false).unwrap();

        assert_eq!(
            result.total_host_count,
            host_count("340282366920938463463374607431768211456")
        );
    }
}
