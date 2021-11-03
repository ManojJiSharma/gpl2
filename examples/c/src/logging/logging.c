/**
 * @brief A program demonstrating logging
 */
#include <gemachain_sdk.h>

extern uint64_t logging(SolParameters *params) {
  // Log a string
  gema_log("static string");

  // Log 5 numbers as u64s in hexadecimal format
  gema_log_64(params->data[0], params->data[1], params->data[2], params->data[3],
             params->data[4]);

  // Log a slice
  gema_log_array(params->data, params->data_len);

  // Log a public key
  gema_log_pubkey(params->program_id);

  // Log all the program's input parameters
  gema_log_params(params);

  // Log the number of compute units remaining that the program can consume.
  gema_log_compute_units();

  return SUCCESS;
}

extern uint64_t entrypoint(const uint8_t *input) {
  SolAccountInfo accounts[1];
  SolParameters params = (SolParameters){.ka = accounts};

  if (!gema_deserialize(input, &params, GEMA_ARRAY_SIZE(accounts))) {
    return ERROR_INVALID_ARGUMENT;
  }

  return logging(&params);
}
