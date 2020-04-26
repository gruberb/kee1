# kee1
Transform a keepass CSV backup into a format which can be imported by 1Password

### Usage

1. `git clone git@github.com:gruberb/kee1.git && cd kee1`
2. `cargo run path/to/file.csv`

This will create a `1password.csv` in the root directory of your project folder. 

### Hints

1. The `keepass.csv` export has to have the following structure: `Group, Title, Username, Password, URL, Notes`
2. The `1password.csv` will have the following structure: `Title, Url, Username, Password`
3. The `1password.csv` can be imported via the 1Password web interface [like this](https://support.1password.com/import/#import-on-1password-com).
