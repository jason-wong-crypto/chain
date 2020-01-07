#!/bin/bash
set -e

echo "[Config] SGX_MODE=${SGX_MODE}"
echo "[Config] TX_QUERY_TIMEOUT=${TX_QUERY_TIMEOUT}"

LD_LIBRARY_PATH=/opt/intel/libsgx-enclave-common/aesm /opt/intel/libsgx-enclave-common/aesm/aesm_service &

echo "[aesm_service] Running in background ..."
# Wait for aesm_service to initialize
sleep 1

# assumes SPID + IAS_API_KEY are set

if [ x"${SPID}" == "x" ]; then
  echo "the environment SPID should be set"
  exit 1
fi

if [ x"${IAS_API_KEY}" == "x" ]; then
  echo "the environment IAS_API_KEY should be set"
  exit 1
fi

trap 'kill -TERM $PID' TERM INT
RUST_LOG=${RUST_LOG} ./tx-query-app 0.0.0.0:${APP_PORT_QUERY} ${TX_VALIDATION_CONN} ${TX_QUERY_TIMEOUT}
