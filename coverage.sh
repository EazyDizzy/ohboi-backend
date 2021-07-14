cargo cov -- report --use-color \
    $( \
      for file in \
        $( \
          RUSTFLAGS="-Z instrument-coverage" \
            cargo test --no-run --message-format=json \
              | jq -r "select(.profile.test == true) | .filenames[]" \
              | grep -v dSYM - \
        ); \
      do \
        printf "%s %s " -object $file; \
      done \
    ) \
  --instr-profile=coverage/ohboi.profdata --summary-only | grep -v github
