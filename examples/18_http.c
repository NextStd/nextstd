#include "../include/ns.h"
#include "../include/ns_string.h"
#include "../include/ns_http.h"

int main(void)
{
  ns_println("=== NextStd HTTP GET and POST Demo ===");

  ns_println("\n[1] Performing HTTP GET...");
  {
    ns_autohttp ns_http_response out = {0};

    ns_error_t err = ns_http_get("https://jsonplaceholder.typicode.com/posts/1", &out);

    if (err == 0) {
      ns_println("GET Status: {}", out.status_code);
      ns_println("Body: \n{}", out.body);
    } else {
      ns_println("Failed to GET data: {}", ns_error_message(err));
    }
  }

  ns_println("\n[2]: Performing HTTP POST...");
  {
    ns_autohttp ns_http_response out = {0};

    const char* json_payload = "{\n"
                                   "  \"title\": \"NextStd HTTP\",\n"
                                   "  \"body\": \"Memory safe C is incredible!\",\n"
                                   "  \"userId\": 99\n"
                                   "}";

    ns_println("Sending Payload...");

    ns_error_t err = ns_http_post("https://jsonplaceholder.typicode.com/posts", json_payload, &out);

    if (err == 0) {
      ns_println("POST Status: {}", out.status_code);
      ns_println("POST Response: \n{}", out.body);
    } else {
      ns_println("POST request failed: {}", ns_error_message(err));
    }
  }

  return 0;
}
