#include "../include/ns.h"
#include "../include/ns_error.h"

int main() {
  ns_error_t err;

  int age;

  ns_println("----NextStd Error handling example---");
  ns_println("Try typing a valid number, or the text to see a safe fallback");
  ns_print("Enter your age: ");

  NS_TRY(err, ns_read(&age)) {
    ns_print("Success, you entered: "); 
    ns_println(age);
  } 
  NS_EXCEPT(err, NS_ERROR_INVALID_INPUT) {
    ns_println("That is not a valid number! Please enter digits only.");
  }
  NS_EXCEPT(err, NS_ERROR_IO_READ) {
    ns_println("Failed to read from standard input");
  }
  NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_print("An error occurred: ");
    ns_println(ns_error_message(err));
  }

  ns_println("");
  ns_println("----Testing Null pointer safety----");
  ns_println("Attempting to read a NULL pointer");

  int* ptr = 0;

  NS_TRY(err, ns_read(ptr)) {
    ns_println("CRITICAL FAILURE: This should not print!");
  } 
  NS_EXCEPT(err, NS_ERROR_ANY) {
    ns_print("Safely caught the bad pointer! Error Message: ");
    ns_println(ns_error_message(err));
  }

  return 0;
}
