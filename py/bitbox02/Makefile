# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


OUT_DIR=bitbox02/communication/generated

PROTO_FILES=hww.proto common.proto backup_commands.proto bitbox02_system.proto btc.proto cardano.proto eth.proto mnemonic.proto system.proto perform_attestation.proto keystore.proto antiklepto.proto

TARGETS=$(addprefix ${OUT_DIR}/, $(PROTO_FILES:.proto=_pb2.py))

all: ${TARGETS}

PY_PROTO_DIR=../../messages
PY_PROTO=$(addprefix ${PY_PROTO_DIR}/,${PROTO_FILES})

# Explicit rules cannot have multiple outputs (it will be interpreted as multiple rules and
# therefore the same command will be executed multiple times)
# Instead we create a single implicit rule. Implicit rules allow multiple outputs without executing
# the same command multiple times when multiple threads are used.
# https://stackoverflow.com/questions/19822435/multiple-targets-from-one-recipe-and-parallel-execution
TMP_TARGETS=$(addprefix %/, $(PROTO_FILES:.proto=_pb2.py))
${TMP_TARGETS}: ${PY_PROTO}
	mkdir -p ${OUT_DIR}
	protoc --proto_path=${PY_PROTO_DIR} --python_out=${OUT_DIR} --mypy_out=${OUT_DIR} $^
	touch ${@D}/__init__.py
# The hack below is due to protobuf-python generating absolute paths (which are frowned upon) instead of relative paths.
# https://github.com/protocolbuffers/protobuf/issues/1491
	sed -i 's/^import \([^.]*_pb2\)/from . import \1/g'  ${OUT_DIR}/*.pyi
	sed -i 's/^import \([^.]*_pb2\) as \([^.]*__pb2\)/from . import \1 as \2/g' ${OUT_DIR}/*.py

release-test: all
	rm -rf dist/*
	python3 setup.py sdist bdist_wheel
	python3 -m twine upload --sign --repository-url https://test.pypi.org/legacy/ dist/*

release-live: all
	rm -rf dist/*
	python3 setup.py sdist bdist_wheel
	python3 -m twine upload --sign dist/*

clean:
	rm -f ${TARGETS}
