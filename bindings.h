#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct WebClient {

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

/**
 * Create WebClient. For C, it creates a WebClient struct pointer
 */
struct WebClient *create_webapi_client(void);

const char *rust_version(void);

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
void get_ip_config_native(struct WebClient *client, struct IpConfigCallback outer_listener);
