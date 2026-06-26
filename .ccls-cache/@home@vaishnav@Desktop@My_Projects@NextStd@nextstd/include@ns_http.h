#ifndef NS_HTTP_H
#define NS_HTTP_H

#include "ns_string.h"
#include "ns_error.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

typedef struct ns_http_response {
  ns_string body;
  int status_code;
} ns_http_response;

/*
 * Performs an HTTP GET request
*/
ns_error_t ns_http_get(const char* url, ns_http_response* output);

/*
 * Performs an HTTP POST request with a string payload
*/
ns_error_t ns_http_post(const char* url, const char* body_data, ns_http_response* output);

/**
 * Memory cleanup
*/
void ns_http_free(ns_http_response* resp);

/**
 * Auto memory cleanup for RAII
*/
static inline void ns_http_cleanup_helper(ns_http_response* ptr) {
    if (ptr) {
        ns_http_free(ptr);
    }
}

// The RAII Macro!
#define ns_autohttp __attribute__((cleanup(ns_http_cleanup_helper)))

#ifdef __cplusplus
}

#endif
#endif
