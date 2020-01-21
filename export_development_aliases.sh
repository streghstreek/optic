#!/usr/bin/env bash
# usage: $ source ./export_development_aliases.sh
export OPTIC_SRC_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
echo "Optic development scripts will run from $OPTIC_SRC_DIR"
alias apidev="OPTIC_DAEMON_ENABLE_DEBUGGING=yes $OPTIC_SRC_DIR/workspaces/local-cli/bin/run"
alias watch-optic="cd $OPTIC_SRC_DIR && yarn wsrun -p '@useoptic/*' -c watch 'yarn run ws:build' './src'"
alias rescue-optic="rm -rf ~/.optic/daemon-lock.json.lock/ ~/.optic/daemon-lock.json"