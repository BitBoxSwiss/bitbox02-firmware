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

PROTO_FILES=hww.proto common.proto backup_commands.proto bitbox02_system.proto bitboxbase.proto btc.proto eth.proto mnemonic.proto random_number.proto system.proto perform_attestation.proto keystore.proto

TARGETS=$(addprefix ${OUT_DIR}/, $(PROTO_FILES:.proto=_pb2.py))

all: ${TARGETS}

# To avoid protobuf name collisions with other packages used elsewhere like the
# electrum app, the pb files for Python are generated from a copy
# of ../../messages where all protos specify a non-empty package directive.
# The proto files need not be committed to the repo. They are created at build time.
PY_PROTO_DIR=.proto
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
	sed -i 's/^from \([^.]*_pb2\)/from .\1/g'  ${OUT_DIR}/*.pyi
	sed -i 's/^import \([^.]*_pb2\) as \([^.]*__pb2\)/from . import \1 as \2/g' ${OUT_DIR}/*.py

# Grab proto files and insert a package name in each one.
# There can be only one "syntax = ..." directive in a proto file.
# So, that's our anchor and a good place to put the package name at.
${PY_PROTO}: $(addprefix ../../messages/,${PROTO_FILES})
	mkdir -p ${PY_PROTO_DIR}
	cp $^ ${PY_PROTO_DIR}
	sed -i 's/^syntax =.*/&\npackage shiftcrypto.bitbox02;/' ${PY_PROTO_DIR}/*.proto

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
	rm -rf ${PY_PROTO_DIR}
