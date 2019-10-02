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


OUT_DIR=bitbox02/generated

TARGETS=$(addprefix ${OUT_DIR}/, hww_pb2.py hww_pb2.pyi)

all: ${TARGETS}

${TARGETS}: ../../messages/hww.proto
	mkdir -p ${OUT_DIR}
	protoc --proto_path=../../messages --python_out=${OUT_DIR} --mypy_out=${OUT_DIR} $^

release-test: all
	rm -rf dist/*
	python3 setup.py sdist bdist_wheel
	python3 -m twine upload --sign --repository-url https://test.pypi.org/legacy/ dist/*

release-live: all
	rm -rf dist/*
	python3 setup.py sdist bdist_wheel
	python3 -m twine upload --sign dist/*
