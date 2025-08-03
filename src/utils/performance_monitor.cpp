#include "performance_monitor.h"
#include <chrono>
#include <unordered_map>
#include <iostream>

#ifdef __APPLE__
#include <mach/mach.h>
#include <sys/resource.h>
#elif defined(_WIN32)
#include <windows.h>
#include <psapi.h>
#else
#include <sys/resource.h>
#include <fstream>
#endif

static std::unordered_map<std::string, double> timer_starts;

size_t PerformanceMonitor::GetMemoryUsageMB() {
#ifdef __APPLE__
    struct mach_task_basic_info info;
    mach_msg_type_number_t infoCount = MACH_TASK_BASIC_INFO_COUNT;
    if (task_info(mach_task_self(), MACH_TASK_BASIC_INFO,
                  (task_info_t)&info, &infoCount) != KERN_SUCCESS) {
        return 0;
    }
    return info.resident_size / (1024 * 1024);
#elif defined(_WIN32)
    PROCESS_MEMORY_COUNTERS pmc;
    if (GetProcessMemoryInfo(GetCurrentProcess(), &pmc, sizeof(pmc))) {
        return pmc.WorkingSetSize / (1024 * 1024);
    }
    return 0;
#else
    std::ifstream status_file("/proc/self/status");
    std::string line;
    while (std::getline(status_file, line)) {
        if (line.substr(0, 6) == "VmRSS:") {
            size_t kb = std::stoul(line.substr(7));
            return kb / 1024;
        }
    }
    return 0;
#endif
}

size_t PerformanceMonitor::GetPeakMemoryUsageMB() {
    struct rusage usage;
    if (getrusage(RUSAGE_SELF, &usage) == 0) {
#ifdef __APPLE__
        return usage.ru_maxrss / (1024 * 1024); // macOS returns bytes
#else
        return usage.ru_maxrss / 1024; // Linux returns KB
#endif
    }
    return 0;
}

float PerformanceMonitor::GetCPUUsagePercent() {
    // Simplified CPU usage - would need more sophisticated implementation
    return 25.0f; // Placeholder
}

float PerformanceMonitor::GetGPUUsagePercent() {
    // Placeholder - would integrate with GPU APIs
    return 15.0f;
}

size_t PerformanceMonitor::GetGPUMemoryUsageMB() {
    // Placeholder - would integrate with GPU APIs
    return 128;
}

double PerformanceMonitor::GetCurrentTimeSeconds() {
    auto now = std::chrono::high_resolution_clock::now();
    auto duration = now.time_since_epoch();
    return std::chrono::duration<double>(duration).count();
}

void PerformanceMonitor::StartTimer(const std::string& name) {
    timer_starts[name] = GetCurrentTimeSeconds();
}

double PerformanceMonitor::EndTimer(const std::string& name) {
    auto it = timer_starts.find(name);
    if (it != timer_starts.end()) {
        double elapsed = GetCurrentTimeSeconds() - it->second;
        timer_starts.erase(it);
        return elapsed;
    }
    return 0.0;
}