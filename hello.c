#include "bindings.h"
#include "dotenv.h"
#include <linux/limits.h>
#include <memory.h>
#include <stdio.h>

void on_result(void *owner, const struct IpConfigNative *ip_config) {
    printf("got ip_config\n");
    printf("ip: %s\n", ip_config->ip);
    printf("user_agent address: %p\n", ip_config->user_agent);
    // all variable returned from into_raw() need to be freed in Rust.
    // TODO:
}

void on_firewall_result(void *owner, const char *message) {
    printf("%s\n", message);
    // all variable returned from into_raw() need to be freed in Rust.
}

void on_error(void *owner, const char *err) { printf("%s\n", err); }

int main(int argc, char **argv) {
    // const char *ver = rust_version();
    // printf("version: %s\n", ver);
    printf("Creating webapi client\n");
    printf("%s\n", P_tmpdir);
    // get token_id & token_key from .env file
    env_load(".", false);
    char *token_id = getenv("SECRETID");
    char *token_key = getenv("SECRETKEY");
    if (NULL == token_id || NULL == token_key) {
        printf("Error: Failed to get SECRETID or SECRETKEY from .env file!\n");
        return 1;
    }
    // printf("%s\n", token_id);
    // printf("%s\n", token_key);
    char tmp_file_path[PATH_MAX];
    char *instance_id = "lhins-3jq1gki4";
    char *file_name = "update_qcloud_firewall_ip.txt";
    snprintf(tmp_file_path, strlen(P_tmpdir) + strlen(file_name) + 2, "%s/%s",
             P_tmpdir, file_name);
    printf("Tmp file will be saved into %s\n", tmp_file_path);
    struct WebClient *webClient =
        create_webapi_client(tmp_file_path, instance_id, token_id, token_key);
    //
    // IpConfigCallback callback = {
    //     .owner = webClient, .onResult = on_result, .onError = on_error};
    // printf("Gettting ip config.\n");
    // get_ip_config_native(webClient, callback);

    FirewallCallback callback2 = {.owner = webClient,
                                  .onResult = on_firewall_result,
                                  .onError = on_error};
    printf("Recreating firewall policy\n");
    // load iphone_payload.json
    char *payload_filename = "iphone_payload.json";
    FILE *f = fopen(payload_filename, "r");
    if (f == NULL) {
        printf("Failed to open file: %s\n", payload_filename);
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    fseek(f, 0, SEEK_SET); /* same as rewind(f); */

    char *payload = malloc(fsize + 1);
    fread(payload, fsize, 1, f);
    fclose(f);

    payload[fsize] = 0;
    recreate_firewall_policy(webClient, payload, callback2);

    printf("Freeing webapi client\n");
    // all variable returned from into_raw() need to be freed in Rust.
    free_swapi_client(webClient);

    free(payload);

    return 0;
}
