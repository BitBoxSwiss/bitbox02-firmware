# Changelog

## 9.4.0 [version may change, pending release]
- ETHPubRequest api call now fails if a an invalid contract address is provided also if `display` is
  false.
- Fix a memory leak (freeing a malloc'd string - no a functional or security issue)
- Title fixed when entering the 21st, 22nd and 23rd recovery word (was 21th, 22th, 23th) before.

## 9.3.1 [tagged 2020-12-01]
- Fix a bug where the device could freeze and become unresponsive.

## 9.3.0 [tagged 2020-11-23]
- Enter multisig account name on the device if the name in BTCRegisterScriptConfigRequest is empty.
- Allow new keypaths: m/48'/coin'/account' for multisig, to enable compatibility with the Nunchuk wallet.
- Multisig script type and derivation keypath are now also verified during account regisration.
