#ifndef NS_ERROR_H
#define NS_ERROR_H

#ifdef __cplusplus
extern "C" {
#endif

  typedef enum {
    NS_SUCCESS = 0,

    // Catch all 
    NS_ERROR_ANY = 1,

    //ns_io errors 
    NS_ERROR_IO_READ = 10,
    NS_ERROR_IO_WRITE = 11,
    NS_ERROR_INVALID_INPUT = 12,

    // ns_string errors
    NS_ERROR_STRING_ALLOC = 20,
    NS_ERROR_STRING_UTF8 = 21,

    NS_ERROR_OUT_OF_BOUNDS = 22,
  } ns_error_t;

  const char* ns_error_message(ns_error_t err);

#define NS_TRY(err_var, expr) \
  if (((err_var) = (expr)) == NS_SUCCESS)

#define NS_EXCEPT(err_var, err_type) \
  else if ((err_var) == (err_type) || (err_type) == NS_ERROR_ANY)

#ifdef __cplusplus

}

#endif // __cplusplus

#endif // !NS_ERROR_H
