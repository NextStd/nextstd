#ifndef NS_PROCESS_H
#define NS_PROCESS_H

#include <stdbool.h>
#include "ns_error.h"

#ifdef __cplusplus
extern "C" {
#endif

  typedef struct ns_process ns_process;

  /**
   * Spawns a process in the background 
   * Returns a handle to the process on success, NULL on failure
   */
  ns_process* ns_process_spawn(const char* command);

  /**
   * Checks if the background process is still running 
   */ 
  bool ns_process_is_running(ns_process* proc);

  /**
   * Forcefully terminates the process 
   */ 
  ns_error_t ns_process_kill(ns_process* proc);

  /**
   * Waits for process to finish and frees the handle 
   */ 
  void ns_process_free(ns_process* proc);

#ifdef __cplusplus

}

#endif 

#endif // !DEBUG
