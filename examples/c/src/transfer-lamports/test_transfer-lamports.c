#include "transfer-carats.c"
#include <criterion/criterion.h>

Test(transfer, sanity) {
  uint8_t instruction_data[] = {};
  SolPubkey program_id = {.x = {
                              1,
                          }};
  SolPubkey source_key = {.x = {
                              2,
                          }};
  uint64_t source_carats = 5;
  uint8_t source_data[] = {};
  SolPubkey destination_program_id = {.x = {
                                          3,
                                      }};
  SolPubkey destination_key = {.x = {
                                   4,
                               }};
  uint64_t destination_carats = 0;
  uint8_t destination_data[] = {};
  SolAccountInfo accounts[] = {{
                                   &source_key,
                                   &source_carats,
                                   sizeof(source_data),
                                   source_data,
                                   &program_id,
                                   0,
                                   true,
                                   true,
                                   false,
                               },
                               {
                                   &destination_key,
                                   &destination_carats,
                                   sizeof(destination_data),
                                   destination_data,
                                   &program_id,
                                   0,
                                   true,
                                   true,
                                   false,
                               }};
  SolParameters params = {accounts, sizeof(accounts) / sizeof(SolAccountInfo),
                          instruction_data, sizeof(instruction_data),
                          &program_id};

  cr_assert(SUCCESS == transfer(&params));
  cr_assert(0 == *accounts[0].carats);
  cr_assert(5 == *accounts[1].carats);
}
