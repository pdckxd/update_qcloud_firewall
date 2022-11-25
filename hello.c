#include <stdio.h>
#include "bindings.h"

void on_result(void *owner, const struct IpConfigNative * ip_config) {
    printf("got ip_config\n");
    printf("ip: %s\n", ip_config->ip);
    printf("user_agent address: %p\n", ip_config->user_agent);
}

void on_error(void *owner, const char *err) {
    printf("%s\n", err);
}

int main(int argc, char** argv) {
    // printf("helloworld\n");
    // int sum = add(1, 2);
    // printf("%d\n", sum);
    //
    printf("Creating webapi client\n");
    struct WebClient* webClient = create_webapi_client();

    IpConfigCallback callback = {
        .owner = webClient,
        .onResult = on_result,
        .onError = on_error
    };
    get_ip_config_native(webClient, callback);


    printf("Freeing webapi client\n");
    free_swapi_client(webClient);

    return 0;
}
