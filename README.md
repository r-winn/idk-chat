# IDK Chat Desktop

Native desktop shell for the hosted IDK Chat application. Mobile users continue to use the existing PWA.

The standalone IDK Nova repository is not vendored or modified here. The Nova navigation inside IDK Chat loads the official Nova web release.

## Local development

1. Install dependencies with `npm install`.
2. Run `npm run desktop:dev`.
3. Build installers with `npm run desktop:build`.

Desktop releases use Tauri signed update artifacts. The updater private key must stay outside this repository and be supplied through `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` when building a release.
