# Python scripts

**Important: pip >= 25 is required.**  
Editable installs (`pip install -e …`) are only supported with pip 25 or newer.  
Older pip versions will fail due to changes in how editable installs are handled (PEP 660).

To use the scripts (e.g. `send_message.py`, `load_firmware.py`):
```bash
pip3 install py/bitbox02
```

To work on the library/scripts and install in editable mode:
```bash
pip install -e py/bitbox02
```

It is highly recommended that you use the dockerized setup while developing, guide for setting it up
can be found in [BUILD.md](../BUILD.md).