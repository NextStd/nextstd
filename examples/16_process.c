#include "../include/ns.h"
#include "../include/ns_process.h"
#include <unistd.h> // For sleep()

int main(void)
{
  ns_println("NextStd Asynchronous Process Demo");

  ns_println("\nTest 1: Spawning and waiting for completion");
  {
    ns_println("Spawning: sleep 2");
    ns_process* proc = ns_process_spawn("sleep 2");

    if (proc == NULL) {
      ns_println("Failed to spawn process.");
      return 1;
    }

    int tick = 1;
    // Loop continues while the background process is alive
    while (ns_process_is_running(proc)) {
      ns_println("C thread doing other work... tick {}", tick);
      sleep(1);
      tick++;
    }

    ns_println("Process finished naturally.");
    
    // Always free the process handle to prevent memory leaks and zombies
    ns_process_free(proc); 
  }

  ns_println("\nTest 2: Terminating a process early");
  {
    // In your scraper, this could be: ns_process_spawn("mpv https://animepahe...");
    ns_println("Spawning long-running task: sleep 10");
    ns_process* proc = ns_process_spawn("sleep 10");

    if (proc == NULL) {
      ns_println("Failed to spawn process.");
      return 1;
    }

    ns_println("Waiting 2 seconds before sending kill signal...");
    sleep(1);
    ns_println("Tick 1");
    sleep(1);
    ns_println("Tick 2");

    ns_println("User requested skip! Killing process early...");
    
    // Send the kill signal
    int err = ns_process_kill(proc);

    if (err == 0) { // 0 is NsError::Success
      ns_println("Kill signal sent successfully.");
    } else {
      ns_println("Failed to kill process.");
    }

    // Verify it actually died
    if (!ns_process_is_running(proc)) {
      ns_println("Verified: Process is no longer running.");
    }

    ns_process_free(proc);
  }

  ns_println("\nAll asynchronous tests completed safely!");
  return 0;
}
