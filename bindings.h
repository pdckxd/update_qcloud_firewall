#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct WebClient {
  char *tmp_file_path;
  char *instance_id;
  char *token_id;
  char *token_key;
} WebClient;

typedef struct UserAgentNative {
  const char *product;
  const char *comment;
  const char *version;
  const char *raw_value;
} UserAgentNative;

typedef struct IpConfigNative {
  const char *ip;
  uint32_t ip_decimal;
  const char *country;
  const char *country_iso;
  uint8_t country_eu;
  float latitude;
  float longitude;
  const char *time_zone;
  const char *asn;
  const char *asn_org;
  struct UserAgentNative *user_agent;
} IpConfigNative;

/**
 * you need reference to owner context to return data
 */
typedef struct IpConfigCallback {
  void *owner;
  void (*onResult)(void *owner, const struct IpConfigNative *arg);
  void (*onError)(void *owner, const char *arg);
} IpConfigCallback;

typedef struct FirewallCallback {
  void *owner;
  void (*onResult)(void *owner, const char *arg);
  void (*onError)(void *owner, const char *arg);
} FirewallCallback;

const char *rust_version(void);

/**
 * Create WebClient. For C, it creates a WebClient struct pointer
 * # Safety
 */
struct WebClient *create_webapi_client(const char *tmp_file_path,
                                       const char *instance_id,
                                       const char *token_id,
                                       const char *token_key);

/**
 * .
 *
 * # Panics
 *
 * Panics if client is null.
 *
 * # Safety
 *
 * Release WebClient memory because it is wrapped into Box.
 * For C, Release memory of WebClient struct pointer
 */
void free_swapi_client(struct WebClient *client);

/**
 * .
 *
 * # Panics
 *
 * Panics if .
 *
 * # Safety
 *
 * .
 */
void free_string(char *s);

/**
 * .
 *
 * # Panics
 *
 * Panics if .
 *
 * # Safety
 *
 * .
 */
void get_ip_config_native(struct WebClient *client, struct IpConfigCallback outer_listener);

/**
 * .
 *
 * # Panics
 *
 * Panics if .
 *
 * # Safety
 *
 * .
 */
void recreate_firewall_policy(struct WebClient *client,
                              const char *payload,
                              struct FirewallCallback outer_listener);
