use std::io::{self, Write};

use crate::ips::IpComputation;

pub fn render_csv(ip_computation: IpComputation, writer: &mut dyn Write) -> io::Result<()> {
    writeln!(writer, "\"ips\"")?;
    for ip in ip_computation.ips {
        writeln!(writer, "\"{ip}\"")?;
    }

    Ok(())
}

pub fn render_json(ip_computation: IpComputation, writer: &mut dyn Write) -> io::Result<()> {
    write!(writer, "[")?;
    let mut first = true;
    for ip in ip_computation.ips {
        if first {
            first = false;
        } else {
            write!(writer, ",")?;
        }
        write!(writer, "\"{ip}\"")?;
    }
    writeln!(writer, "]")?;

    Ok(())
}

pub fn render_plain(ip_computation: IpComputation, writer: &mut dyn Write) -> io::Result<()> {
    writeln!(writer, "count: {}", ip_computation.total_host_count)?;
    for ip in ip_computation.ips {
        writeln!(writer, "{ip}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use std::net::IpAddr;

    use num_bigint::BigUint;

    use crate::ips::IpComputation;

    use super::render_plain;

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("forced write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn ip_addr(value: &str) -> IpAddr {
        value.parse::<IpAddr>().unwrap()
    }

    #[test]
    fn render_plain_propagates_writer_errors() {
        let computation = IpComputation {
            total_host_count: BigUint::from(2u8),
            ips: Box::new(vec![ip_addr("10.0.0.1"), ip_addr("10.0.0.2")].into_iter()),
        };

        let mut writer = FailingWriter;
        let err = render_plain(computation, &mut writer).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert_eq!(err.to_string(), "forced write failure");
    }
}
