# This requires this file to be in the same folder:
# https://github.com/LedgerHQ/orakolo/blob/0b2d5e669ec61df9a824df9fa1a363060116b490/src/python/orakolo/HDEd25519.py
import HDEd25519

import os, json

HDEd25519.TRACE = False

n = 1000 # Generate this many tests.
H = 0x80000000
o = HDEd25519.BIP32Ed25519()

tests = []
for i in range(n):
    node = o.root_key_slip10(os.urandom(64))
    (kl, kr), pubkey, chaincode = node
    node_copy = node
    test = {
        "kl": kl.hex(),
        "kr": kr.hex(),
        "chain_code": chaincode.hex(),
        "private_derivations": [],
        "public_derivations":[],
    }
    private_paths = (
        (0, ),
        (H, ),
        (0, 1, 2, ),
        (H, H+1, H+2, ),
        (H+123456, H, 2**32-1, 0, 7891011, ),
    )
    for path in private_paths:
        node = node_copy
        for index in path:
            node = o.private_child_key(node, index)
        (kl, kr), pubkey, chaincode = node
        test["private_derivations"].append({
             "path": path,
            "expected_kl": kl.hex(),
            "expected_kr": kr.hex(),
            "expected_chain_code": chaincode.hex(),
            "expected_public_key": pubkey.hex(),
        })
    public_paths = (
        (0, ),
        (H-1, ),
        (0, 1, 2, ),
        (123456, 0, H-1, 76542345, 7891011, ),
    )
    for path in public_paths:
        _, pubkey, chaincode = node_copy
        node = (pubkey, chaincode)
        for index in path:
            node = o.public_child_key(node, index)
        (pubkey, chaincode) = node
        test["public_derivations"].append({
            "path": path,
            "expected_public_key": pubkey.hex(),
            "expected_chain_code": chaincode.hex(),
        })
    tests.append(test)
print(json.dumps(tests, indent=2))
