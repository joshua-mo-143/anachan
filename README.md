# Legendary Analytics Machine
A simple web analytics service that lets you track views for your website, per URI.

## Usage
Use the following `cargo shuttle` command (make sure you're logged in):
```bash
cargo shuttle init --from joshua-mo-143/legendary-analytics-machine
```
Follow the prompts, then cd to the folder.
Next, deploy it:
```bash
cargo shuttle deploy
```
Once it's deployed, you're ready to use it!

Simply add the following to the HTML head of any web page you own to start using:
```html
<script src="https://<your-project-name-here>.shuttleapp.rs/script.js"></script>
``` 

## Troubleshooting
Make sure your `cargo-shuttle` and other Shuttle-related dependencies on the project are up to date.

If there's any other issues, feel free to open an issue!

## Features 
- View page hits for the last 7 days on whatever domain or page you have that uses the JS script
- Simple to use

## To do
- Lock dashboard behind password
- Add support for buttons/links (button clickthrough rate, clicking on links, etc)
- Add more things that you can view (highest referrer link, etc)
- expand metrics
