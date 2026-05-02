#include "../include/ns.h"
#include "../include/ns_process.h"
#include <unistd.h> // For sleep()

int main(void)
{
  ns_println("=== NextStd Concurrent Processes Demo ===");

  // 1. Fire off a background download (Simulating the scraper)
  ns_println("Task 1: Starting background HTML download...");
  ns_process* curl_proc = ns_process_spawn(
      "curl -s 'https://en.wikipedia.org/wiki/Bleach_(manga)' -o bleach_data.html"
  );

  // 2. Fire off a media player
  // --idle and --force-window ensures it opens a window even without a video link
  ns_println("Task 2: Launching mpv...");
  ns_process* mpv_proc = ns_process_spawn("mpv --idle --force-window=yes");

  if (!curl_proc || !mpv_proc) {
      ns_println("Failed to spawn one or more processes.");
      return 1;
  }

  int tick = 1;

  // 3. The Main Thread Loop
  // This continues as long as AT LEAST ONE process is still running
  while (ns_process_is_running(curl_proc) || ns_process_is_running(mpv_proc)) {
      
      bool curl_alive = ns_process_is_running(curl_proc);
      bool mpv_alive = ns_process_is_running(mpv_proc);

      ns_println("Tick {}: Main thread monitoring... [Curl: {}] [MPV: {}]", 
                 tick, 
                 curl_alive ? "Active" : "Finished", 
                 mpv_alive ? "Active" : "Finished");

      sleep(1);
      tick++;

      // Let's force kill mpv after 5 seconds so you don't have to close it manually
      if (tick == 6 && mpv_alive) {
          ns_println("\nTimeout reached! Sending kill signal to mpv...");
          ns_process_kill(mpv_proc);
      }
  }

  ns_println("\nAll concurrent tasks finished safely!");

  // 4. Cleanup
  ns_process_free(curl_proc);
  ns_process_free(mpv_proc);

  return 0;
}
