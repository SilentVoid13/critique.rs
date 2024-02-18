# critique.rs

critique.rs is a collection of tools to interact with [SensCritique](https://www.senscritique.com). 

## critique_api

A simple Rust library to interact with the SensCritique GraphQL API. It currently provides functionality to fetch user collections and other related data.

The full GraphQL schema can be found at [critique_api/gql/schema.graphql](https://github.com/SilentVoid13/critique.rs/blob/master/critique_api/gql/schema.graphql).
It is a rough approximation of the real SensCritique schema and may contain inaccuracies or omissions.

Some example GraphQL queries can be found at [critique_api/gql/query](https://github.com/SilentVoid13/critique.rs/blob/master/critique_api/gql/query).
Contributions to expand this set of queries are welcome. 

## critique2boxd

`critique2boxd` is a utility tool to export a SensCritique user collection to a CSV file that can later be imported to Letterboxd ([https://letterboxd.com/import/](https://letterboxd.com/import/)).

```txt
Usage: critique2boxd [OPTIONS] <USERNAME>

Arguments:
  <USERNAME>  SensCritique username

Options:
  -w, --with-reviews     Add reviews to the output
  -o, --output <OUTPUT>  Output CSV file [default: output.csv]
  -h, --help             Print help
```

## Contributing

Contributions to the project are welcome. Feel free to report bugs, suggest improvements or directly contribute to the codebase.

## License

critique.rs is licensed under the GNU AGPLv3 license. Refer to [LICENSE](LICENSE.txt) for more information.

## Support

Your support helps me continue to maintain and improve this project. If you find it useful and want to show your appreciation, consider sponsoring or donating:
- GitHub Sponsors: Preferred method. You can sponsor me on [GitHub Sponsors](https://github.com/sponsors/SilentVoid13). 
- PayPal: You can also make a donation via [PayPal](https://www.paypal.com/donate?hosted_button_id=U2SRGAFYXT32Q).

Every bit of support is greatly appreciated!

[![GitHub Sponsors](https://img.shields.io/github/sponsors/silentvoid13?label=Sponsor&logo=GitHub%20Sponsors&style=for-the-badge)](https://github.com/sponsors/silentvoid13)
[![Paypal](https://img.shields.io/badge/paypal-silentvoid13-yellow?style=social&logo=paypal)](https://www.paypal.com/donate?hosted_button_id=U2SRGAFYXT32Q)
