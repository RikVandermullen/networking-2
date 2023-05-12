use ldap3::{LdapConn, Scope, SearchEntry};
use ldap3::result::Result;

fn main() -> Result<()> {
    let mut ldap = LdapConn::new("LDAP://ldap.itd.umich.edu")?;
    let (rs, _res) = ldap.search(
        "ou=People,dc=umich,dc=edu",
        Scope::Subtree,
        "(&(objectClass=*)(cn=Amy Newman))",
        vec!["mail", "cn"]
    )?.success()?;
    for entry in rs {
        println!("{:#?}", SearchEntry::construct(entry));
    }
    Ok(ldap.unbind()?)
}
