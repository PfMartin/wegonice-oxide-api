ifneq (,$(wildcard .env))
include .env
export
endif

ADMIN_WEGONICE_CONNECTION_STRING=mongodb://${MONGO_INITDB_ROOT_USERNAME}:${MONGO_INITDB_ROOT_PASSWORD}@127.0.0.1:27017/${MONGO_WEGONICE_DB}?authSource=${MONGO_INITDB_DATABASE}
USER_WEGONICE_CONNECTION_STRING=mongodb://${MONGO_WEGONICE_USER}:${MONGO_WEGONICE_PASSWORD}@127.0.0.1:27017/${MONGO_WEGONICE_DB}?authSource=${MONGO_WEGONICE_DB}

DOCKER_EXECUTE_STRING=docker exec -it wegonice-db /bin/bash -c


db-create-user:
	${DOCKER_EXECUTE_STRING} "mongosh ${ADMIN_WEGONICE_CONNECTION_STRING} --eval 'db.createUser({user: \"${MONGO_WEGONICE_USER}\", pwd: \"${MONGO_WEGONICE_PASSWORD}\", roles: [{role: \"readWrite\", db: \"${MONGO_WEGONICE_DB}\"}]})'"

db-connect-admin:
	${DOCKER_EXECUTE_STRING} "mongosh ${ADMIN_WEGONICE_CONNECTION_STRING}"

db-connect-user:
	${DOCKER_EXECUTE_STRING} "mongosh ${USER_WEGONICE_CONNECTION_STRING}"

unit-tests:
	cargo llvm-cov --workspace --all-features --ignore-filename-regex="test_utils|main" -- --test unit_tests --test-threads=1 --nocapture

integration_tests:
	cargo test \
		--test integration_tests_heartbeat \
		--test integration_tests_register \
		--test integration_tests_login

fmt-check:
	cargo fmt --all --check

clippy-check:
	cargo clippy -- --D warnings

watch:
	cargo watch -q -w src -x run
