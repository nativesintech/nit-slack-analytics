# NiT Slack Analytics

## Intro

This project was built to take Slack analytics and create a table from them. Specifically analytics from the NiT Slack group. The goal of the project was to help with learning Rust but could potentially be built upon to be a CLI tool to interact with Slack analytics in JSON or CSV.

## Example Output

| 2017 | 2018 | 2019 | Total |
| ---- | ---- | ---- | ----- |
| 19   | 12   | 49   | 75    |

## Status

Unmaintained

## Usage

To use the app, download Slack analytics from your Slack group in JSON and put them into `src/data`.

From the command line, build the application using the command: `cargo build`.

Then, from the command line, run the application using the command: `cargo run`.

## Code of Conduct

The code of conduct governs interactions in this repository: [CODE_OF_CONDUCT](CODE_OF_CONDUCT).

## License

MIT licenses this software: [LICENSE](LICENSE).
